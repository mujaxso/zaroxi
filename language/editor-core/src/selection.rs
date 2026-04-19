//! Selection handling for the editor

use serde::{Deserialize, Serialize};

/// A text selection range
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Selection {
    /// Start position (inclusive)
    pub start: usize,
    /// End position (exclusive)
    pub end: usize,
}

impl Selection {
    /// Create a new selection
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Check if the selection is empty (cursor)
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Get the sorted range (start <= end)
    pub fn sorted(&self) -> (usize, usize) {
        if self.start <= self.end {
            (self.start, self.end)
        } else {
            (self.end, self.start)
        }
    }

    /// Get the length of the selection in characters
    pub fn len(&self) -> usize {
        let (start, end) = self.sorted();
        end - start
    }
}
