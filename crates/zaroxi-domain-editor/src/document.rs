//! Minimal text document model backed by a plain `String`.
//! Line start offsets are pre‑computed to allow O(1) line access.

use std::path::PathBuf;
use memmap2::Mmap;

/// Large file mode indicator.
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

/// A minimal text document.
#[derive(Debug)]
pub struct Document {
    text: String,
    /// Byte offset of the start of each line (0‑based).
    line_starts: Vec<usize>,
    version: u64,
    dirty: bool,
    path: Option<PathBuf>,
    large_file_mode: LargeFileMode,
}

fn compute_line_starts(text: &str) -> Vec<usize> {
    let mut starts = Vec::new();
    starts.push(0);
    for (i, ch) in text.char_indices() {
        if ch == '\n' {
            starts.push(i + 1);
        }
    }
    starts
}

impl Document {
    /// Create a new empty document.
    pub fn new() -> Self {
        Self {
            text: String::new(),
            line_starts: vec![0],
            version: 0,
            dirty: false,
            path: None,
            large_file_mode: LargeFileMode::Normal,
        }
    }

    /// Create a document from a plain string.
    pub fn from_text(text: &str) -> Self {
        let line_starts = compute_line_starts(text);
        Self {
            text: text.to_owned(),
            line_starts,
            version: 0,
            dirty: false,
            path: None,
            large_file_mode: LargeFileMode::Normal,
        }
    }

    /// Create a document from text with an associated file path.
    pub fn from_text_with_path(text: &str, path: String) -> Self {
        let mut doc = Self::from_text(text);
        doc.path = Some(PathBuf::from(path));
        doc
    }

    /// Re‑compute line start offsets (must be called after any edit).
    fn recompute_line_starts(&mut self) {
        self.line_starts = compute_line_starts(&self.text);
    }

    /// Number of Unicode scalar values.
    pub fn len_chars(&self) -> usize {
        self.text.chars().count()
    }

    /// Number of lines (including a final line without trailing newline).
    pub fn len_lines(&self) -> usize {
        self.line_starts.len()
    }

    /// Whether the document contains no text.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Return the text content of line `idx` (0‑based), without the trailing newline.
    pub fn line(&self, idx: usize) -> Option<&str> {
        if idx >= self.line_starts.len() {
            return None;
        }
        let start = self.line_starts[idx];
        let end = if idx + 1 < self.line_starts.len() {
            self.line_starts[idx + 1]
        } else {
            self.text.len()
        };
        let slice = &self.text[start..end];
        // strip trailing newline (and optional carriage return)
        Some(slice.trim_end_matches('\n').trim_end_matches('\r'))
    }

    /// Borrow the entire document text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Convert a character index to (line, column).
    pub fn char_to_line_col(&self, char_idx: usize) -> Option<(usize, usize)> {
        let mut line = 0usize;
        let mut col = 0usize;
        for (i, ch) in self.text.char_indices() {
            if i >= char_idx {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }
        Some((line, col))
    }

    /// Convert (line, column) to a character index, or `None` if out of bounds.
    pub fn line_col_to_char(&self, line: usize, col: usize) -> Option<usize> {
        if line >= self.line_starts.len() {
            return None;
        }
        let start_byte = self.line_starts[line];
        let mut byte = start_byte;
        let mut current_col = 0usize;
        for ch in self.text[start_byte..].chars() {
            if ch == '\n' {
                break;
            }
            if current_col == col {
                return Some(byte);
            }
            let ch_len = ch.len_utf8();
            byte += ch_len;
            current_col += 1;
        }
        None
    }

    /// Character index of the start of the given line.
    pub fn line_to_char(&self, line: usize) -> usize {
        self.line_starts.get(line).copied().unwrap_or(self.text.len())
    }

    /// Insert text at a given character index.
    pub fn insert(&mut self, char_idx: usize, ins: &str) -> Result<(), String> {
        if self.large_file_mode.is_read_only() {
            return Err("Read‑only large file".into());
        }
        let byte_pos = self.char_idx_to_byte(char_idx).ok_or("Invalid char index")?;
        self.text.insert_str(byte_pos, ins);
        self.version += 1;
        self.dirty = true;
        self.recompute_line_starts();
        Ok(())
    }

    /// Delete characters in the range `start..end`.
    pub fn delete_range(&mut self, start: usize, end: usize) -> Result<(), String> {
        if self.large_file_mode.is_read_only() {
            return Err("Read‑only large file".into());
        }
        if start > end {
            return Err("start > end".into());
        }
        let start_byte = self.char_idx_to_byte(start).ok_or("Invalid start")?;
        let end_byte = self.char_idx_to_byte(end).ok_or("Invalid end")?;
        if end_byte > self.text.len() {
            return Err("Out of bounds".into());
        }
        self.text.drain(start_byte..end_byte);
        self.version += 1;
        self.dirty = true;
        self.recompute_line_starts();
        Ok(())
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

    /// The large‑file mode.
    pub fn large_file_mode(&self) -> LargeFileMode {
        self.large_file_mode
    }

    /// Whether the file is “large” (may degrade performance).
    pub fn is_large(&self) -> bool {
        self.large_file_mode == LargeFileMode::Large
    }

    /// Whether the file is “very large” (read‑only recommended).
    pub fn is_very_large(&self) -> bool {
        self.large_file_mode == LargeFileMode::VeryLarge
    }

    /// Create a document from a memory‑mapped file.
    pub fn from_mmap(mmap: Mmap, path: String, size: u64) -> Self {
        let mode = LargeFileMode::from_size(size);
        let text = unsafe { std::str::from_utf8_unchecked(&mmap) };
        let line_starts = compute_line_starts(text);
        Self {
            text: text.to_owned(),
            line_starts,
            version: 0,
            dirty: false,
            path: Some(PathBuf::from(path)),
            large_file_mode: mode,
        }
    }

    /// Convert a byte offset to a character index.
    pub fn byte_to_char(&self, byte: usize) -> usize {
        let mut char_idx = 0usize;
        for (i, _) in self.text.char_indices() {
            if i >= byte {
                break;
            }
            char_idx += 1;
        }
        char_idx
    }

    /// Convert a character index to a byte offset.
    pub fn char_to_byte(&self, char_idx: usize) -> usize {
        let mut count = 0usize;
        for (i, _) in self.text.char_indices() {
            if count == char_idx {
                return i;
            }
            count += 1;
        }
        self.text.len()
    }

    /// Delete characters in the range `start..end` (alias for `delete_range`).
    pub fn delete(&mut self, start: usize, end: usize) -> Result<(), String> {
        self.delete_range(start, end)
    }

    // ------------------------------------------------------------------
    // Internal helpers
    // ------------------------------------------------------------------
    fn char_idx_to_byte(&self, char_idx: usize) -> Option<usize> {
        let mut count = 0usize;
        for (i, _) in self.text.char_indices() {
            if count == char_idx {
                return Some(i);
            }
            count += 1;
        }
        if count == char_idx {
            Some(self.text.len())
        } else {
            None
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}
