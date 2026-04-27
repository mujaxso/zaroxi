//! Minimal text document model backed by a Rope (ropey::Rope).
//!
//! Every document, regardless of size, holds a rope so that editing
//! is possible.  The file‑class (`FileClass`) is still computed and
//! can be used by the UI to selectively disable expensive decorations
//! (line‑number gutter, syntax highlighting), but it no longer prevents
//! editing or forces a read‑only preview.
//!
//! No hand‑rolled line‑start caching; the rope provides O(log n) line access.

use crate::thresholds::{self, FileClass};
use ropey::Rope;
use std::path::PathBuf;

/// A text document.
#[derive(Debug, Clone)]
pub struct Document {
    rope: Rope,
    /// Line count (computed once at load time and kept up‑to‑date).
    line_count: usize,
    version: u64,
    dirty: bool,
    path: Option<PathBuf>,
    file_class: FileClass,
}

impl Document {
    // ── Construction ───────────────────────────────────────────────────

    /// Create an empty document.
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            line_count: 0,
            version: 0,
            dirty: false,
            path: None,
            file_class: FileClass::Normal,
        }
    }

    /// Create a document from a full string.  The file class is derived automatically.
    pub fn from_text(text: &str) -> Self {
        let rope = Rope::from_str(text);
        let line_count = rope.len_lines();
        let file_class = Self::compute_file_class_from_text(text);
        Self {
            rope,
            line_count,
            version: 0,
            dirty: false,
            path: None,
            file_class,
        }
    }

    /// Create a document from text with an associated file path.
    ///
    /// The file class is re‑computed from the text length.
    pub fn from_text_with_path(text: &str, path: String) -> Self {
        let mut doc = Self::from_text(text);
        doc.path = Some(PathBuf::from(path));
        doc
    }

    /// Create a **large‑file** document that still holds the full rope.
    ///
    /// This is the same as `from_text_with_path` except the caller can also
    /// provide pre‑calculated line count (used when scanning an mmap region).
    pub fn from_large_file_full(
        text: &str,
        line_count: usize,
        path: String,
    ) -> Self {
        let rope = Rope::from_str(text);
        let path = Some(PathBuf::from(path));
        let file_class = Self::compute_file_class(text.len() as u64, text);
        Self {
            rope,
            line_count,
            version: 0,
            dirty: false,
            path,
            file_class,
        }
    }

    /// Create a document from a memory‑mapped file.
    ///
    /// For **all** files a rope is built so that editing is possible.
    pub fn from_mmap(mmap: &memmap2::Mmap, path: String, _size: u64) -> Self {
        let text = unsafe { std::str::from_utf8_unchecked(&mmap) };
        Self::from_text_with_path(text, path)
    }

    // ── Queries ────────────────────────────────────────────────────────

    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn len_lines(&self) -> usize {
        self.line_count
    }

    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Return the textual content of line `idx` (0‑based), without the
    /// trailing newline.
    pub fn line(&self, idx: usize) -> Option<String> {
        self.rope.get_line(idx).map(|slice| {
            let s = slice.to_string();
            s.strip_suffix('\n')
                .or_else(|| s.strip_suffix("\r\n"))
                .unwrap_or(&s)
                .to_owned()
        })
    }

    /// Return the entire document content as an owned `String`.
    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    /// Convert a character index to a byte offset.
    pub fn char_to_byte(&self, char_idx: usize) -> usize {
        self.rope.char_to_byte(char_idx)
    }

    pub fn byte_to_char(&self, byte: usize) -> usize {
        self.rope.byte_to_char(byte)
    }

    pub fn line_to_char(&self, line: usize) -> usize {
        self.rope.line_to_char(line)
    }

    /// Convert a character index to a (line, column) pair.
    ///
    /// Column is measured in **characters** within the line (not bytes).
    /// Returns `None` for out‑of‑bounds positions.
    pub fn char_to_line_col(&self, char_idx: usize) -> Option<(usize, usize)> {
        if char_idx > self.rope.len_chars() {
            return None;
        }
        let byte_pos = self.rope.char_to_byte(char_idx);
        let line = self.rope.byte_to_line(byte_pos);
        if line >= self.rope.len_lines() {
            return None;
        }
        let line_slice = self.rope.line(line);
        let line_start_byte = self.rope.line_to_byte(line);
        let byte_in_line = byte_pos - line_start_byte;
        let col = line_slice.byte_to_char(byte_in_line);
        Some((line, col))
    }

    /// Convert a (line, column) pair to a character index.
    ///
    /// Column is measured in **characters** within the line (not bytes).
    /// Returns `None` if the line index is out of bounds or the column
    /// exceeds the line length.
    pub fn line_col_to_char(&self, line: usize, col: usize) -> Option<usize> {
        if line >= self.rope.len_lines() {
            return None;
        }
        let line_slice = self.rope.line(line);
        if col > line_slice.len_chars() {
            return None;
        }
        let byte_in_line = line_slice.char_to_byte(col);
        let line_start_byte = self.rope.line_to_byte(line);
        let byte_pos = line_start_byte + byte_in_line;
        let char_idx = self.rope.byte_to_char(byte_pos);
        Some(char_idx)
    }

    // ── Editing ────────────────────────────────────────────────────────

    pub fn insert(&mut self, char_idx: usize, ins: &str) -> Result<(), String> {
        if char_idx > self.rope.len_chars() {
            return Err("Invalid char index".into());
        }
        let byte_pos = self.rope.char_to_byte(char_idx);
        self.rope.insert(byte_pos, ins);
        self.word_count_line_count();
        self.version += 1;
        self.dirty = true;
        Ok(())
    }

    pub fn delete_range(&mut self, start: usize, end: usize) -> Result<(), String> {
        if start > end || end > self.rope.len_chars() {
            return Err("Invalid range".into());
        }
        let start_byte = self.rope.char_to_byte(start);
        let end_byte = self.rope.char_to_byte(end);
        self.rope.remove(start_byte..end_byte);
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
        self.path = path.map(PathBuf::from);
    }

    // ── Internal helpers ──────────────────────────────────────────────

    fn word_count_line_count(&mut self) {
        self.line_count = self.rope.len_lines();
    }

    fn compute_file_class_from_text(text: &str) -> FileClass {
        Self::compute_file_class(text.len() as u64, text)
    }

    fn compute_file_class(byte_size: u64, text: &str) -> FileClass {
        let line_count = text.lines().count();
        let max_line_len = text
            .lines()
            .map(|l| l.len())
            .max()
            .unwrap_or(0);
        thresholds::classify_file(byte_size, line_count, max_line_len)
    }
}
