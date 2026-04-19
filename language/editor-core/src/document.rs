//! Document handling for the editor

use ropey::Rope;
use serde::{Deserialize, Serialize};

/// A document in the editor
#[derive(Debug)]
pub struct Document {
    /// The underlying rope data structure
    pub rope: Rope,
    /// Document path (if any)
    pub path: Option<String>,
}

impl Document {
    /// Create a new empty document
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            path: None,
        }
    }

    /// Create a document from a string
    pub fn from_str(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            path: None,
        }
    }

    /// Get the text content as a String
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    /// Insert text at a given position
    pub fn insert(&mut self, pos: usize, text: &str) {
        self.rope.insert(pos, text);
    }

    /// Remove text in a range
    pub fn remove(&mut self, start: usize, end: usize) {
        self.rope.remove(start..end);
    }

    /// Get line count
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// Get character count
    pub fn char_count(&self) -> usize {
        self.rope.len_chars()
    }
}
