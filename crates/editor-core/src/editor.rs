//! Editor state management.

use crate::document::Document;
use crate::cursor::{Cursor, CursorMovement};
use crate::selection::Selection;
use crate::viewport::Viewport;

/// The main editor state, combining document, cursor, selection, and viewport.
#[derive(Debug, Clone)]
pub struct EditorState {
    document: Document,
    cursor: Cursor,
    selection: Option<Selection>,
    viewport: Viewport,
    scroll_offset: (f32, f32), // (horizontal, vertical) in pixels
}

impl EditorState {
    /// Create a new editor state with an empty document.
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            cursor: Cursor::new(),
            selection: None,
            viewport: Viewport::new(),
            scroll_offset: (0.0, 0.0),
        }
    }

    /// Create editor state from a document.
    pub fn from_document(document: Document) -> Self {
        Self {
            document,
            cursor: Cursor::new(),
            selection: None,
            viewport: Viewport::new(),
            scroll_offset: (0.0, 0.0),
        }
    }

    /// Get a reference to the document.
    pub fn document(&self) -> &Document {
        &self.document
    }

    /// Get a mutable reference to the document.
    pub fn document_mut(&mut self) -> &mut Document {
        &mut self.document
    }

    /// Get the cursor.
    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    /// Get a mutable reference to the cursor.
    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    /// Get the selection, if any.
    pub fn selection(&self) -> Option<&Selection> {
        self.selection.as_ref()
    }

    /// Set the selection.
    pub fn set_selection(&mut self, selection: Option<Selection>) {
        self.selection = selection;
    }

    /// Get the viewport.
    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    /// Get a mutable reference to the viewport.
    pub fn viewport_mut(&mut self) -> &mut Viewport {
        &mut self.viewport
    }

    /// Get the scroll offset.
    pub fn scroll_offset(&self) -> (f32, f32) {
        self.scroll_offset
    }

    /// Set the scroll offset.
    pub fn set_scroll_offset(&mut self, offset: (f32, f32)) {
        self.scroll_offset = offset;
    }

    /// Insert text at the cursor position.
    pub fn insert_text(&mut self, text: &str) -> Result<(), String> {
        let cursor_pos = self.cursor.position();
        self.document.insert(cursor_pos, text)?;
        
        // Move cursor forward by the length of inserted text
        self.cursor.move_by(CursorMovement::Right(text.len()), &self.document);
        Ok(())
    }

    /// Delete the character before the cursor (backspace).
    pub fn delete_backward(&mut self) -> Result<(), String> {
        let cursor_pos = self.cursor.position();
        if cursor_pos > 0 {
            self.document.delete(cursor_pos - 1, cursor_pos)?;
            self.cursor.move_by(CursorMovement::Left(1), &self.document);
        }
        Ok(())
    }

    /// Delete the character after the cursor (delete).
    pub fn delete_forward(&mut self) -> Result<(), String> {
        let cursor_pos = self.cursor.position();
        if cursor_pos < self.document.len_chars() {
            self.document.delete(cursor_pos, cursor_pos + 1)?;
        }
        Ok(())
    }

    /// Move the cursor.
    pub fn move_cursor(&mut self, movement: CursorMovement) {
        self.cursor.move_by(movement, &self.document);
    }

    /// Get the current line and column of the cursor.
    pub fn cursor_line_col(&self) -> Option<(usize, usize)> {
        self.document.char_to_line_col(self.cursor.position())
    }

    /// Get visible lines for the current viewport.
    pub fn visible_lines(&self, line_height: f32, viewport_height: f32) -> Vec<(usize, String)> {
        let start_line = (self.scroll_offset.1 / line_height).floor() as usize;
        let lines_in_viewport = (viewport_height / line_height).ceil() as usize + 1;
        
        let mut lines = Vec::new();
        for line_idx in start_line..(start_line + lines_in_viewport) {
            if let Some(line_text) = self.document.line(line_idx) {
                lines.push((line_idx, line_text));
            } else {
                break;
            }
        }
        lines
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}
