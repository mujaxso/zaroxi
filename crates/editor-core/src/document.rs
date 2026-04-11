//! Text document model with rope-based storage.

use ropey::Rope;

/// A text document with efficient editing operations.
#[derive(Debug, Clone)]
pub struct Document {
    rope: Rope,
    version: u64,
    dirty: bool,
    path: Option<String>,
}

impl Document {
    /// Create a new empty document.
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            version: 0,
            dirty: false,
            path: None,
        }
    }

    /// Create a document from text.
    pub fn from_text(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            version: 0,
            dirty: false,
            path: None,
        }
    }

    /// Create a document from text with a file path.
    pub fn from_text_with_path(text: &str, path: String) -> Self {
        Self {
            rope: Rope::from_str(text),
            version: 0,
            dirty: false,
            path: Some(path),
        }
    }

    /// Get the document's text as a string.
    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    /// Get the number of characters in the document.
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Get the number of lines in the document.
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Check if the document is empty.
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Get a line by index (0-based).
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx >= self.rope.len_lines() {
            return None;
        }
        Some(self.rope.line(line_idx).to_string())
    }

    /// Get the character index for a line and column.
    pub fn line_col_to_char(&self, line: usize, col: usize) -> Option<usize> {
        if line >= self.rope.len_lines() {
            return None;
        }
        let line_start = self.rope.line_to_char(line);
        let line_len = self.rope.line(line).len_chars();
        if col > line_len {
            return None;
        }
        Some(line_start + col)
    }

    /// Get the line and column for a character index.
    pub fn char_to_line_col(&self, char_idx: usize) -> Option<(usize, usize)> {
        if char_idx > self.rope.len_chars() {
            return None;
        }
        let line = self.rope.char_to_line(char_idx);
        let line_start = self.rope.line_to_char(line);
        let col = char_idx - line_start;
        Some((line, col))
    }

    /// Get the character index for the start of a line.
    pub fn line_to_char(&self, line: usize) -> usize {
        self.rope.line_to_char(line)
    }

    /// Insert text at a character position.
    pub fn insert(&mut self, char_idx: usize, text: &str) -> Result<(), String> {
        if char_idx > self.rope.len_chars() {
            return Err(format!("Char index {} out of bounds", char_idx));
        }
        self.rope.insert(char_idx, text);
        self.version += 1;
        self.dirty = true;
        Ok(())
    }

    /// Delete a range of characters.
    pub fn delete(&mut self, start: usize, end: usize) -> Result<(), String> {
        if start > end {
            return Err(format!("Start {} greater than end {}", start, end));
        }
        if end > self.rope.len_chars() {
            return Err(format!("End {} out of bounds", end));
        }
        self.rope.remove(start..end);
        self.version += 1;
        self.dirty = true;
        Ok(())
    }

    /// Replace the entire document content.
    pub fn replace_all(&mut self, text: &str) {
        self.rope = Rope::from_str(text);
        self.version += 1;
        self.dirty = true;
    }

    /// Get a slice of the document.
    pub fn slice(&self, start: usize, end: usize) -> Result<String, String> {
        if start > end {
            return Err(format!("Start {} greater than end {}", start, end));
        }
        if end > self.rope.len_chars() {
            return Err(format!("End {} out of bounds", end));
        }
        Ok(self.rope.slice(start..end).to_string())
    }

    /// Mark the document as saved (clears dirty flag).
    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    /// Check if the document has unsaved changes.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Get the document version (increments on each change).
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Get the document's file path, if any.
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    /// Set the document's file path.
    pub fn set_path(&mut self, path: Option<String>) {
        self.path = path;
    }

    /// Check if the document is considered large (for performance considerations).
    pub fn is_large(&self) -> bool {
        self.len_chars() > 1_000_000
    }

    /// Check if the document is considered very large (read-only recommended).
    pub fn is_very_large(&self) -> bool {
        self.len_chars() > 10_000_000
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}
