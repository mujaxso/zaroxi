//! Minimal editor state: document, cursor, viewport.

use crate::document::Document;
use crate::cursor::{Cursor, CursorMovement};
use crate::viewport::Viewport;
use crate::document::LargeFileMode;

/// The main editor state, combining document, cursor, and viewport.
#[derive(Debug)]
pub struct EditorState {
    document: Document,
    cursor: Cursor,
    viewport: Viewport,
    scroll_offset_y: f32, // vertical scroll offset in pixels
}

impl EditorState {
    /// Create a new editor state with an empty document.
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            cursor: Cursor::new(),
            viewport: Viewport::new(),
            scroll_offset_y: 0.0,
        }
    }

    /// Create editor state from an existing document.
    pub fn from_document(document: Document) -> Self {
        Self {
            document,
            cursor: Cursor::new(),
            viewport: Viewport::new(),
            scroll_offset_y: 0.0,
        }
    }

    // ---------- document ----------
    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn document_mut(&mut self) -> &mut Document {
        &mut self.document
    }

    // ---------- cursor ----------
    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    /// Move the cursor with the given movement.
    pub fn move_cursor(&mut self, movement: CursorMovement) {
        self.cursor.move_by(movement, &self.document);
    }

    /// Current (line, column) of the cursor, or `None` if the document is empty.
    pub fn cursor_line_col(&self) -> Option<(usize, usize)> {
        self.document.char_to_line_col(self.cursor.position())
    }

    /// Insert text at the cursor position and advance the cursor.
    pub fn insert_text(&mut self, text: &str) -> Result<(), String> {
        let pos = self.cursor.position();
        self.document.insert(pos, text)?;
        self.cursor.move_by(CursorMovement::Right(text.chars().count()), &self.document);
        Ok(())
    }

    /// Delete the character before the cursor (backspace).
    pub fn delete_backward(&mut self) -> Result<(), String> {
        let pos = self.cursor.position();
        if pos == 0 {
            return Err("Cannot delete before start of document".into());
        }
        self.document.delete_range(pos - 1, pos)?;
        self.cursor.move_by(CursorMovement::Left(1), &self.document);
        Ok(())
    }

    /// Delete the character after the cursor (delete / forward delete).
    pub fn delete_forward(&mut self) -> Result<(), String> {
        let pos = self.cursor.position();
        if pos >= self.document.len_chars() {
            return Err("Cannot delete after end of document".into());
        }
        self.document.delete_range(pos, pos + 1)?;
        Ok(())
    }

    // ---------- viewport ----------
    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn viewport_mut(&mut self) -> &mut Viewport {
        &mut self.viewport
    }

    /// Vertical scroll offset (pixels).
    pub fn scroll_offset_y(&self) -> f32 {
        self.scroll_offset_y
    }

    pub fn set_scroll_offset_y(&mut self, offset: f32) {
        self.scroll_offset_y = offset;
    }

    /// Compute the visible lines for the current viewport state.
    pub fn visible_lines(&self) -> Vec<(usize, String)> {
        let total_lines = self.document.len_lines();
        if total_lines == 0 {
            return Vec::new();
        }

        // convert scroll offset to first displayed line
        let first = (self.scroll_offset_y / self.viewport.line_height).floor() as usize;
        let count = self.viewport.visible_line_count;
        if count == 0 {
            return Vec::new();
        }

        let end = (first + count).min(total_lines);
        let mut lines = Vec::with_capacity(end.saturating_sub(first));
        for idx in first..end {
            if let Some(text) = self.document.line(idx) {
                lines.push((idx, text));
            }
        }
        lines
    }

    // ---------- convenience ----------
    pub fn text(&self) -> String {
        self.document.text()
    }

    pub fn path(&self) -> Option<&std::path::Path> {
        self.document.path()
    }

    pub fn large_file_mode(&self) -> LargeFileMode {
        self.document.large_file_mode()
    }

    pub fn is_large_file(&self) -> bool {
        self.document.is_large() || self.document.is_very_large()
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}
