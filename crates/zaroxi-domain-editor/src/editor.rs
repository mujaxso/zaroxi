//! Minimal editor state: document, cursor, viewport, and syntax highlighting.

use crate::cursor::{Cursor, CursorMovement};
use crate::document::Document;
use crate::thresholds::FileClass;
use crate::viewport::Viewport;
use zaroxi_lang_syntax::highlight::{HighlightEngine, HighlightSpan};
use zaroxi_lang_syntax::theme_map::StyledSpan;
use zaroxi_theme::theme::SemanticColors;

/// The main editor state, combining document, cursor, viewport, and syntax highlighting.
#[derive(Debug)]
pub struct EditorState {
    document: Document,
    cursor: Cursor,
    viewport: Viewport,
    scroll_offset_y: f32, // vertical scroll offset in pixels
    /// The syntax highlighting engine.
    highlight_engine: HighlightEngine,
    /// Cached highlight spans for the current document content.
    cached_highlights: Vec<HighlightSpan>,
    /// The version of the document when highlights were last computed.
    cached_version: u64,
}

impl EditorState {
    /// Create a new editor state with an empty document.
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            cursor: Cursor::new(),
            viewport: Viewport::new(),
            scroll_offset_y: 0.0,
            highlight_engine: HighlightEngine::new(),
            cached_highlights: Vec::new(),
            cached_version: u64::MAX,
        }
    }

    /// Create editor state from an existing document.
    pub fn from_document(document: Document) -> Self {
        let mut state = Self {
            document,
            cursor: Cursor::new(),
            viewport: Viewport::new(),
            scroll_offset_y: 0.0,
            highlight_engine: HighlightEngine::new(),
            cached_highlights: Vec::new(),
            cached_version: u64::MAX,
        };
        // Trigger initial syntax highlighting
        state.highlights();
        state
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

    // ---------- syntax highlighting ----------

    /// Get the highlight spans for the current document content.
    ///
    /// This method caches the highlights and only recomputes them when the
    /// document version changes. For large files, it returns an empty vector.
    pub fn highlights(&mut self) -> Vec<HighlightSpan> {
        // For very large files, return empty highlights
        if self.document.file_class() == FileClass::Large {
            self.cached_highlights.clear();
            return Vec::new();
        }

        // Check if we need to recompute highlights
        let current_version = self.document.version();
        if current_version != self.cached_version {
            self.recompute_highlights();
        }

        self.cached_highlights.clone()
    }

    /// Recompute highlight spans for the current document.
    fn recompute_highlights(&mut self) {
        self.cached_highlights.clear();
        self.cached_version = self.document.version();

        eprintln!("DEBUG: recompute_highlights: version={}", self.cached_version);

        // Ensure we have a syntax tree
        if !self.document.ensure_syntax_tree() {
            eprintln!("DEBUG: recompute_highlights: ensure_syntax_tree returned false");
            return;
        }

        // Get the syntax tree and language
        let tree = match self.document.syntax_tree() {
            Some(t) => t,
            None => {
                eprintln!("DEBUG: recompute_highlights: syntax_tree is None after ensure");
                return;
            }
        };

        let language = self.document.language();
        eprintln!("DEBUG: recompute_highlights: language={:?}", language);

        // Run the highlighting engine
        let text = self.document.text();
        match self.highlight_engine.highlight(language, &text, tree.tree()) {
            Ok(spans) => {
                eprintln!("DEBUG: recompute_highlights: got {} spans", spans.len());
                self.cached_highlights = spans;
            }
            Err(e) => {
                eprintln!("Warning: Syntax highlighting failed: {}", e);
            }
        }
    }

    /// Get styled spans for the current document, applying the given theme.
    ///
    /// This is the main method for the rendering layer to get syntax-colored spans.
    /// All returned spans use **character offsets** (not byte offsets).
    pub fn styled_spans(&mut self, colors: &SemanticColors) -> Vec<StyledSpan> {
        let highlights = self.highlights();
        let mut spans = apply_theme(&highlights, colors);
        // Convert byte offsets to character offsets
        for span in &mut spans {
            let start_char = self.document.byte_to_char(span.start);
            let end_char = self.document.byte_to_char(span.end);
            span.start = start_char;
            span.end = end_char;
        }
        spans
    }

    /// Get styled spans for a specific line range, applying the given theme.
    ///
    /// This is optimized for rendering only visible lines.
    /// All returned spans use **character offsets** (not byte offsets) so they
    /// can be used directly by the frontend for text slicing.
    ///
    /// The returned spans are guaranteed to cover the entire requested character
    /// range: any gap between spans will be filled with a plain-text span using
    /// `colors.text_primary`. This ensures the frontend never has to fall back
    /// to a separate plain‑text rendering path.
    pub fn styled_spans_for_lines(
        &mut self,
        colors: &SemanticColors,
        start_line: usize,
        end_line: usize,
    ) -> Vec<StyledSpan> {
        // First, get the highlights (this may borrow self mutably)
        let highlights = self.highlights();

        // Now we can borrow self.document immutably because highlights is owned
        let total_lines = self.document.len_lines();

        // Clamp line range to document bounds
        let start_line = start_line.min(total_lines.saturating_sub(1));
        let end_line = end_line.min(total_lines);

        // Convert line range to character range (not byte range)
        let start_char = self.document.line_to_char(start_line);
        let end_char = if end_line >= total_lines {
            self.document.len_chars()
        } else {
            self.document.line_to_char(end_line)
        };

        let mut result = Vec::new();

        // Filter spans to the requested range, converting byte offsets to char offsets
        for span in &highlights {
            // Convert byte offsets to character offsets
            let span_start_char = self.document.byte_to_char(span.start);
            let span_end_char = self.document.byte_to_char(span.end);

            if span_end_char > start_char && span_start_char < end_char {
                let clamped_start = span_start_char.max(start_char);
                let clamped_end = span_end_char.min(end_char);
                if clamped_start < clamped_end {
                    let token_type = zaroxi_lang_syntax::theme_map::SemanticTokenType::from_highlight(span.highlight);
                    let color = token_type.theme_color(colors);
                    result.push(zaroxi_lang_syntax::theme_map::StyledSpan {
                        start: clamped_start,
                        end: clamped_end,
                        token_type,
                        color,
                    });
                }
            }
        }

        // Sort by start position
        result.sort_by_key(|s| s.start);

        // Fill gaps with plain-text spans
        let mut filled = Vec::new();
        let mut cursor = start_char;
        for span in &result {
            if span.start > cursor {
                // There is a gap before this span
                filled.push(zaroxi_lang_syntax::theme_map::StyledSpan {
                    start: cursor,
                    end: span.start,
                    token_type: zaroxi_lang_syntax::theme_map::SemanticTokenType::Plain,
                    color: colors.text_primary,
                });
            }
            filled.push(span.clone());
            cursor = span.end;
        }
        // Fill any remaining gap after the last span
        if cursor < end_char {
            filled.push(zaroxi_lang_syntax::theme_map::StyledSpan {
                start: cursor,
                end: end_char,
                token_type: zaroxi_lang_syntax::theme_map::SemanticTokenType::Plain,
                color: colors.text_primary,
            });
        }

        filled
    }

    /// Invalidate the highlight cache (e.g., after a large edit).
    pub fn invalidate_highlights(&mut self) {
        self.cached_version = 0;
        self.cached_highlights.clear();
        self.document.invalidate_syntax_tree();
    }

    // ---------- convenience ----------
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}
