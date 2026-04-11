//! Cursor position and movement.

/// Cursor movement operations.
#[derive(Debug, Clone, Copy)]
pub enum CursorMovement {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
    LineStart,
    LineEnd,
    DocumentStart,
    DocumentEnd,
}

/// Cursor position in the document (character index).
#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    position: usize,
}

impl Cursor {
    /// Create a new cursor at position 0.
    pub fn new() -> Self {
        Self { position: 0 }
    }

    /// Create a cursor at a specific position.
    pub fn at(position: usize) -> Self {
        Self { position }
    }

    /// Get the cursor position (character index).
    pub fn position(&self) -> usize {
        self.position
    }

    /// Set the cursor position.
    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    /// Move the cursor by a movement operation.
    pub fn move_by(&mut self, movement: CursorMovement, document: &crate::document::Document) {
        match movement {
            CursorMovement::Left(n) => {
                self.position = self.position.saturating_sub(n);
            }
            CursorMovement::Right(n) => {
                self.position = (self.position + n).min(document.len_chars());
            }
            CursorMovement::Up(_) => {
                // For now, simple implementation
                // In a real implementation, we'd need to track column
                if let Some((line, col)) = document.char_to_line_col(self.position) {
                    if line > 0 {
                        if let Some(new_pos) = document.line_col_to_char(line - 1, col) {
                            self.position = new_pos;
                        }
                    }
                }
            }
            CursorMovement::Down(_) => {
                if let Some((line, col)) = document.char_to_line_col(self.position) {
                    if let Some(new_pos) = document.line_col_to_char(line + 1, col) {
                        self.position = new_pos;
                    }
                }
            }
            CursorMovement::LineStart => {
                if let Some((line, _)) = document.char_to_line_col(self.position) {
                    self.position = document.line_to_char(line);
                }
            }
            CursorMovement::LineEnd => {
                if let Some((line, _)) = document.char_to_line_col(self.position) {
                    let line_start = document.line_to_char(line);
                    let line_len = document.line(line).map(|l| l.chars().count()).unwrap_or(0);
                    self.position = line_start + line_len;
                }
            }
            CursorMovement::DocumentStart => {
                self.position = 0;
            }
            CursorMovement::DocumentEnd => {
                self.position = document.len_chars();
            }
        }
    }

    /// Move the cursor without document context (for simple movements).
    pub fn move_by_simple(&mut self, movement: CursorMovement) {
        match movement {
            CursorMovement::Left(n) => {
                self.position = self.position.saturating_sub(n);
            }
            CursorMovement::Right(n) => {
                self.position = self.position + n;
            }
            _ => {
                // Other movements need document context
            }
        }
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}
