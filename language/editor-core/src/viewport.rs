//! Viewport handling for the editor

use serde::{Deserialize, Serialize};

/// Viewport representing the visible area of the document
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Viewport {
    /// First visible line
    pub first_line: usize,
    /// Number of visible lines
    pub line_count: usize,
    /// Horizontal scroll offset in characters
    pub horizontal_scroll: usize,
}

impl Viewport {
    /// Create a new viewport
    pub fn new(first_line: usize, line_count: usize) -> Self {
        Self {
            first_line,
            line_count,
            horizontal_scroll: 0,
        }
    }

    /// Scroll the viewport by a number of lines
    pub fn scroll_by(&mut self, lines: isize) {
        if lines >= 0 {
            self.first_line = self.first_line.saturating_add(lines as usize);
        } else {
            self.first_line = self.first_line.saturating_sub((-lines) as usize);
        }
    }

    /// Scroll the viewport horizontally
    pub fn scroll_horizontal(&mut self, chars: isize) {
        if chars >= 0 {
            self.horizontal_scroll = self.horizontal_scroll.saturating_add(chars as usize);
        } else {
            self.horizontal_scroll = self.horizontal_scroll.saturating_sub((-chars) as usize);
        }
    }

    /// Check if a line is visible in the viewport
    pub fn is_line_visible(&self, line: usize) -> bool {
        line >= self.first_line && line < self.first_line + self.line_count
    }
}
