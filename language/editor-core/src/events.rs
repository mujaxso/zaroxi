//! Editor events

use serde::{Deserialize, Serialize};

/// An editor event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /// Document changed
    DocumentChanged,
    /// Cursor moved
    CursorMoved {
        position: usize,
    },
    /// Selection changed
    SelectionChanged {
        start: usize,
        end: usize,
    },
    /// Viewport scrolled
    ViewportScrolled {
        first_line: usize,
    },
    /// Error occurred
    Error {
        message: String,
    },
}

impl Event {
    /// Create a document changed event
    pub fn document_changed() -> Self {
        Event::DocumentChanged
    }

    /// Create a cursor moved event
    pub fn cursor_moved(position: usize) -> Self {
        Event::CursorMoved { position }
    }
}
