//! Editor commands

use serde::{Deserialize, Serialize};

/// An editor command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    /// Insert text at cursor
    Insert {
        text: String,
    },
    /// Delete text in range
    Delete {
        start: usize,
        end: usize,
    },
    /// Move cursor
    MoveCursor {
        position: usize,
    },
    /// Select text
    Select {
        start: usize,
        end: usize,
    },
    /// Undo last operation
    Undo,
    /// Redo last undone operation
    Redo,
}

impl Command {
    /// Get a description of the command
    pub fn description(&self) -> &'static str {
        match self {
            Command::Insert { .. } => "Insert text",
            Command::Delete { .. } => "Delete text",
            Command::MoveCursor { .. } => "Move cursor",
            Command::Select { .. } => "Select text",
            Command::Undo => "Undo",
            Command::Redo => "Redo",
        }
    }
}
