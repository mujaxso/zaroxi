//! Editor core for Zaroxi

pub mod document;
pub mod cursor;
pub mod selection;
pub mod viewport;
pub mod commands;
pub mod events;

use ropey::Rope;

/// Main editor structure
#[derive(Debug)]
pub struct Editor {
    /// The document content
    pub document: Rope,
    /// Cursor positions
    pub cursors: Vec<cursor::Cursor>,
}

impl Editor {
    /// Create a new editor with empty content
    pub fn new() -> Self {
        Self {
            document: Rope::new(),
            cursors: vec![cursor::Cursor::new(0)],
        }
    }
    
    /// Create a new editor with initial content
    pub fn from_str(text: &str) -> Self {
        Self {
            document: Rope::from_str(text),
            cursors: vec![cursor::Cursor::new(0)],
        }
    }
}
