//! Minimal text document model backed by a Rope (ropey::Rope).
//!
//! The document enforces a **central file‑size policy**: for large files
//! (`FileClass::Large`) it never creates a rope, holding only a short preview
//! and pre‑computed line count.  That avoids OOM crashes and guarantees safe
//! read‑only behaviour for huge files.
//!
//! For normal and medium files the underlying storage is a Rope; editing
//! operations are supported and a version counter propagates to syntax
//! consumers (future integration).
//!
//! No hand‑rolled line‑start caching; the rope provides O(log n) line access.
use crate::thresholds::{self, FileClass};
use ropey::Rope;
use std::path::PathBuf;

/// A text document.
///
/// The concrete representation depends on the file class:
/// - `Normal` / `Medium` : a `Rope` is always present; full editing support.
/// - `Large` : no rope, only a short preview string and pre‑computed line
///   count.  The document is read‑only.
#[derive(Debug, Clone)]
pub struct Document {
    rope: Option<Rope>,
    /// Line count (available for every class, computed once at load time).
    line_count: usize,
    version: u64,
    dirty: bool,
    path: Option<PathBuf>,
    file_class: FileClass,
    /// For large files we keep a preview of the first few characters.
    preview: String,
}

impl Document {
    // ── Construction ───────────────────────────────────────────────────

    /// Create an empty document (used as a placeholder).
    pub fn new() -> Self {
        Self {
            rope: Some(Rope::new()),
            line_count: 0,
            version: 0,
            dirty: false,
            path: None,
            file_class: FileClass::Normal,
            preview: String::new(),
        }
    }

    /// Create a document from a full string (normal / medium mode).
    ///
    /// The caller is responsible for providing the correct `file_class`.
    /// This method creates a rope and computes the line count.
    pub fn from_text(text: &str, file_class: FileClass) -> Self {
        let rope = Rope::from_str(text);
        let line_count = rope.len_lines();
        let preview = if file_class == FileClass::Large {
            text.chars().take(Self::PREVIEW_MAX_CHARS).collect()
        } else {
            String::new()
        };

        Self {
            rope: Some(rope),
            line_count,
            version: 0,
            dirty: false,
            path: None,
            file_class,
            preview,
        }
    }

    /// Create a document from text with an associated file path.
    ///
    /// The file class is **re‑computed** here from the text length.
    pub fn from_text_with_path(text: &str, path: String) -> Self {
        let file_class = Self::compute_file_class_from_text(text);
        let mut doc = Self::from_text(text, file_class);
        doc.path = Some(PathBuf::from(path));
        doc
    }

    /// Create a **large‑file** document that stores no rope, only metadata.
    pub fn from_large_file_preview(
        preview: String,
        line_count: usize,
        path: String,
    ) -> Self {
        let path = PathBuf::from(path);
        Self {
            rope: None,
            line_count,
            version: 0,
            dirty: false,
            path: Some(path),
            file_class: FileClass::Large,
            preview,
        }
    }

    /// Create a document from a memory‑mapped file.
    ///
    /// For large files it never builds a rope – it only stores a preview
    /// and the line count (computed by scanning the mmap region).
    /// For normal / medium files a rope is built.
    pub fn from_mmap(mmap: &memmap2::Mmap, path: String, size: u64) -> Self {
        let text = unsafe { std::str::from_utf8_unchecked(&mmap) };
        let file_class = Self::compute_file_class(size, text);
        match file_class {
            FileClass::Large => {
                let preview: String = text.chars().take(Self::PREVIEW_MAX_CHARS).collect();
                let line_count = count_lines_of(text);
                Self {
                    rope: None,
                    line_count,
                    version: 0,
                    dirty: false,
                    path: Some(PathBuf::from(path)),
                    file_class,
                    preview,
                }
            }
            _ => Self::from_text_with_path(text, path),
        }
    }

    // ── Queries ────────────────────────────────────────────────────────

    pub fn len_chars(&self) -> usize {
        self.rope.as_ref().map(|r| r.len_chars()).unwrap_or(self.preview.len())
    }

    pub fn len_lines(&self) -> usize {
        self.line_count
    }

    pub fn is_empty(&self) -> bool {
        self.len_chars() == 0
    }

    /// Return the textual content of line `idx` (0‑based), without the
    /// trailing newline.  For large files this returns the line from the
    /// preview if the index is within the preview, otherwise `None`.
    pub fn line(&self, idx: usize) -> Option<String> {
        if let Some(rope) = &self.rope {
            rope.get_line(idx).map(|slice| {
                let s = slice.to_string();
                s.strip_suffix('\n')
                    .or_else(|| s.strip_suffix("\r\n"))
                    .unwrap_or(&s)
                    .to_owned()
            })
        } else {
            // Large file – answer from the preview string.
            line_from_preview(&self.preview, idx)
        }
    }

    /// Return the entire document content as an owned `String`.
    /// For large files this is the short preview.
    pub fn text(&self) -> String {
        self.rope
            .as_ref()
            .map(|r| r.to_string())
            .unwrap_or_else(|| self.preview.clone())
    }

    /// Convert a character index to a byte offset.
    pub fn char_to_byte(&self, char_idx: usize) -> usize {
        self.rope
            .as_ref()
            .map(|r| r.char_to_byte(char_idx))
            .unwrap_or(char_idx) // large files are treated as ascii offsets
    }

    pub fn byte_to_char(&self, byte: usize) -> usize {
        self.rope
            .as_ref()
            .map(|r| r.byte_to_char(byte))
            .unwrap_or(byte)
    }

    pub fn line_to_char(&self, line: usize) -> usize {
        self.rope
            .as_ref()
            .map(|r| r.line_to_char(line))
            .unwrap_or(0)
    }

    // ── Editing (only available for Normal / Medium files) ────────────

    pub fn insert(&mut self, char_idx: usize, ins: &str) -> Result<(), String> {
        if self.file_class.is_read_only() {
            return Err("Document is read‑only".into());
        }
        let rope = self.rope.as_mut().ok_or("Read‑only document")?;
        if char_idx > rope.len_chars() {
            return Err("Invalid char index".into());
        }
        let byte_pos = rope.char_to_byte(char_idx);
        rope.insert(byte_pos, ins);
        self.word_count_line_count();
        self.version += 1;
        self.dirty = true;
        Ok(())
    }

    pub fn delete_range(&mut self, start: usize, end: usize) -> Result<(), String> {
        if self.file_class.is_read_only() {
            return Err("Document is read‑only".into());
        }
        let rope = self.rope.as_mut().ok_or("Read‑only document")?;
        if start > end || end > rope.len_chars() {
            return Err("Invalid range".into());
        }
        let start_byte = rope.char_to_byte(start);
        let end_byte = rope.char_to_byte(end);
        rope.remove(start_byte..end_byte);
        self.word_count_line_count();
        self.version += 1;
        self.dirty = true;
        Ok(())
    }

    pub fn delete(&mut self, start: usize, end: usize) -> Result<(), String> {
        self.delete_range(start, end)
    }

    // ── Metadata / versioning ─────────────────────────────────────────

    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }
    pub fn is_dirty(&self) -> bool { self.dirty }
    pub fn version(&self) -> u64 { self.version }
    pub fn file_class(&self) -> FileClass { self.file_class }
    pub fn path(&self) -> Option<&std::path::Path> { self.path.as_deref() }
    pub fn set_path(&mut self, path: Option<String>) {
        self.path = path.map(std::path::PathBuf::from);
    }

    /// Whether the document is read‑only (Large class).
    pub fn is_read_only(&self) -> bool { self.file_class.is_read_only() }

    // ── Internal helpers ──────────────────────────────────────────────

    const PREVIEW_MAX_CHARS: usize = 50_000;

    fn word_count_line_count(&mut self) {
        if let Some(rope) = &self.rope {
            self.line_count = rope.len_lines();
        }
    }

    fn compute_file_class_from_text(text: &str) -> FileClass {
        Self::compute_file_class(text.len() as u64, text)
    }

    fn compute_file_class(byte_size: u64, text: &str) -> FileClass {
        let line_count = count_lines_of(text);
        let max_line_len = text
            .lines()
            .map(|l| l.len())
            .max()
            .unwrap_or(0);
        thresholds::classify_file(byte_size, line_count, max_line_len)
    }
}

// ------------------------------------------------------------------
// Helpers
// ------------------------------------------------------------------

/// Count the number of lines in a string slice.
fn count_lines_of(text: &str) -> usize {
    text.lines().count()
}

/// Return the text of line `idx` from a preview string.
/// The preview is taken as a plain text (no rope).
fn line_from_preview(preview: &str, idx: usize) -> Option<String> {
    let mut current_line = 0;
    if idx == 0 && preview.is_empty() {
        return Some(String::new());
    }
    for line in preview.lines() {
        if current_line == idx {
            return Some(line.to_owned());
        }
        current_line += 1;
    }
    None
}
