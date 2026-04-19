//! Cursor handling for the editor

use serde::{Deserialize, Serialize};

/// A cursor position in the document
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cursor {
    /// Character position in the document
    pub position: usize,
    /// Preferred column for vertical movement
    pub preferred_column: Option<usize>,
}

impl Cursor {
    /// Create a new cursor at the given position
    pub fn new(position: usize) -> Self {
        Self {
            position,
            preferred_column: None,
        }
    }

    /// Move cursor to a new position
    pub fn move_to(&mut self, position: usize) {
        self.position = position;
    }

    /// Move cursor by an offset
    pub fn move_by(&mut self, offset: isize) {
        if offset >= 0 {
            self.position = self.position.saturating_add(offset as usize);
        } else {
            self.position = self.position.saturating_sub((-offset) as usize);
        }
    }
}
