//! Minimal text document model backed by a Rope (ropey::Rope).
//! No hand‑rolled line‑start caching; the rope provides O(log n) line access.

use std::path::PathBuf;
use memmap2::Mmap;
use ropey::Rope;
use crate::thresholds::{self, FileClass};

/// Large file mode indicator (kept for backward compatibility, but classification
/// now uses the richer `FileClass` enum from `thresholds`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LargeFileMode {
    Normal,
    Large,
    VeryLarge,
}

impl LargeFileMode {
    pub fn from_size(size: u64) -> Self {
        if size <= 10_000_000 {
            Self::Normal
        } else if size <= 100_000_000 {
            Self::Large
        } else {
            Self::VeryLarge
        }
    }

    /// Whether edits should be allowed.
    pub fn is_read_only(&self) -> bool {
        *self == Self::VeryLarge
    }
}

/// A minimal text document that uses a Rope as its backing storage.
#[derive(Debug)]
pub struct Document {
    rope: Rope,
    version: u64,
    dirty: bool,
    path: Option<PathBuf>,
    file_class: FileClass,
}

impl Document {
    /// Create a new empty document.
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            version: 0,
            dirty: false,
            path: None,
            file_class: FileClass::Normal,
        }
    }

    /// Create a document from a plain string.
    pub fn from_text(text: &str) -> Self {
        let rope = Rope::from_str(text);
        let file_class = Self::compute_file_class(&rope, text.len() as u64);
        Self {
            rope,
            version: 0,
            dirty: false,
            path: None,
            file_class,
        }
    }

    /// Create a document from text with an associated file path.
    pub fn from_text_with_path(text: &str, path: String) -> Self {
        let mut doc = Self::from_text(text);
        doc.path = Some(PathBuf::from(path));
        doc
    }

    // ------------------------------------------------------------------
    // Basic queries
    // ------------------------------------------------------------------

    /// Number of Unicode scalar values.
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Number of lines (0‑based). Rope counts a final line even without a trailing '\n'.
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Whether the document contains no text.
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Return the text content of line `idx` (0‑based), without the trailing newline.
    /// The returned `String` is owned and stripped of trailing line‑terminator characters.
    pub fn line(&self, idx: usize) -> Option<String> {
        self.rope.get_line(idx).map(|slice| {
            let s = slice.to_string();
            let trimmed = s.trim_end_matches('\n').trim_end_matches('\r');
            trimmed.to_owned()
        })
    }

    /// Return the entire document content as an owned `String`.
    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    // ------------------------------------------------------------------
    // Coordinate conversion
    // ------------------------------------------------------------------

    /// Convert a character index to (line, column).
    pub fn char_to_line_col(&self, char_idx: usize) -> Option<(usize, usize)> {
        if char_idx > self.rope.len_chars() {
            return None;
        }
        let line = self.rope.char_to_line(char_idx);
        let col = char_idx - self.rope.line_to_char(line);
        Some((line, col))
    }

    /// Convert (line, column) to a character index, or `None` if out of bounds.
    pub fn line_col_to_char(&self, line: usize, col: usize) -> Option<usize> {
        if line >= self.rope.len_lines() {
            return None;
        }
        let line_start = self.rope.line_to_char(line);
        let line_slice = self.rope.get_line(line)?;
        let line_chars = line_slice.chars().count();
        if col > line_chars {
            return None;
        }
        Some(line_start + col)
    }

    /// Character index of the start of the given line.
    pub fn line_to_char(&self, line: usize) -> usize {
        self.rope.line_to_char(line)
    }

    /// Convert a byte offset to a character index.
    pub fn byte_to_char(&self, byte: usize) -> usize {
        self.rope.byte_to_char(byte)
    }

    /// Convert a character index to a byte offset.
    pub fn char_to_byte(&self, char_idx: usize) -> usize {
        self.rope.char_to_byte(char_idx)
    }

    // ------------------------------------------------------------------
    // Editing operations
    // ------------------------------------------------------------------

    /// Insert text at a given character index.
    pub fn insert(&mut self, char_idx: usize, ins: &str) -> Result<(), String> {
        if self.file_class.is_read_only() {
            return Err("Read‑only large file".into());
        }
        if char_idx > self.rope.len_chars() {
            return Err("Invalid char index".into());
        }
        let byte_pos = self.rope.char_to_byte(char_idx);
        self.rope.insert(byte_pos, ins);
        self.version += 1;
        self.dirty = true;
        Ok(())
    }

    /// Delete characters in the range `start..end`.
    pub fn delete_range(&mut self, start: usize, end: usize) -> Result<(), String> {
        if self.file_class.is_read_only() {
            return Err("Read‑only large file".into());
        }
        if start > end {
            return Err("start > end".into());
        }
        if end > self.rope.len_chars() {
            return Err("Out of bounds".into());
        }
        let start_byte = self.rope.char_to_byte(start);
        let end_byte = self.rope.char_to_byte(end);
        self.rope.remove(start_byte..end_byte);
        self.version += 1;
        self.dirty = true;
        Ok(())
    }

    /// Delete characters in the range `start..end` (alias for `delete_range`).
    pub fn delete(&mut self, start: usize, end: usize) -> Result<(), String> {
        self.delete_range(start, end)
    }

    /// Mark the document as saved.
    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    /// Whether the document has unsaved changes.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Monotonically increasing version counter.
    pub fn version(&self) -> u64 {
        self.version
    }

    /// The file path, if any.
    pub fn path(&self) -> Option<&std::path::Path> {
        self.path.as_deref()
    }

    /// Set (or clear) the file path.
    pub fn set_path(&mut self, path: Option<String>) {
        self.path = path.map(PathBuf::from);
    }

    /// The large‑file mode (kept for backward compatibility).
    pub fn large_file_mode(&self) -> LargeFileMode {
        match self.file_class {
            FileClass::Normal => LargeFileMode::Normal,
            FileClass::Medium => LargeFileMode::Large, // map Medium to Large
            FileClass::Large => LargeFileMode::VeryLarge,
        }
    }

    /// Whether the file is “large” (may degrade performance).
    pub fn is_large(&self) -> bool {
        self.file_class == FileClass::Medium || self.file_class == FileClass::Large
    }

    /// Whether the file is “very large” (read‑only recommended).
    pub fn is_very_large(&self) -> bool {
        self.file_class == FileClass::Large
    }

    /// The centralised file class.
    pub fn file_class(&self) -> FileClass {
        self.file_class
    }

    /// Create a document from a memory‑mapped file.
    pub fn from_mmap(mmap: Mmap, path: String, size: u64) -> Self {
        let text = unsafe { std::str::from_utf8_unchecked(&mmap) };
        let rope = Rope::from_str(text);
        let file_class = Self::compute_file_class(&rope, size);
        Self {
            rope,
            version: 0,
            dirty: false,
            path: Some(PathBuf::from(path)),
            file_class,
        }
    }

    // ------------------------------------------------------------------
    // Internal helpers
    // ------------------------------------------------------------------
    fn compute_file_class(rope: &Rope, byte_size: u64) -> FileClass {
        let line_count = rope.len_lines();
        let max_line_len = rope.lines().map(|l| l.chars().count()).max().unwrap_or(0);
        thresholds::classify_file(byte_size, line_count, max_line_len)
    }

    fn _char_idx_to_byte(&self, char_idx: usize) -> Option<usize> {
        if char_idx > self.rope.len_chars() {
            return None;
        }
        Some(self.rope.char_to_byte(char_idx))
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}
