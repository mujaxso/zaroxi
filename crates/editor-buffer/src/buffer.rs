//! Text buffer implementation.
//!
//! Manages the underlying text storage, line tracking, and basic text
//! manipulation operations.

#[derive(Debug, Clone)]
pub struct TextBuffer {
    text: String,
    version: i32,
    dirty: bool,
}

impl TextBuffer {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            version: 0,
            dirty: false,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn replace_all(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.version += 1;
        self.dirty = true;
    }

    pub fn insert(&mut self, offset: usize, text: &str) -> Result<(), String> {
        if offset > self.text.len() {
            return Err(format!("Offset {} out of bounds (length {})", offset, self.text.len()));
        }
        self.text.insert_str(offset, text);
        self.version += 1;
        self.dirty = true;
        Ok(())
    }

    pub fn delete(&mut self, start: usize, end: usize) -> Result<(), String> {
        if start > end {
            return Err(format!("Start {} greater than end {}", start, end));
        }
        if end > self.text.len() {
            return Err(format!("End {} out of bounds (length {})", end, self.text.len()));
        }
        self.text.replace_range(start..end, "");
        self.version += 1;
        self.dirty = true;
        Ok(())
    }

    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn version(&self) -> i32 {
        self.version
    }
}
