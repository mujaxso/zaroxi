//! Per-document syntax state cache with version-aware invalidation.
//!
//! This module provides a cache that stores parsed syntax trees and
//! highlight spans for each open document, keyed by document version.
//! It ensures that scrolling does not trigger unnecessary re-parsing.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::Mutex;
use zaroxi_lang_syntax::highlight::{HighlightEngine, HighlightSpan};
use zaroxi_lang_syntax::parser::{SyntaxTree, ParserPool};
use zaroxi_lang_syntax::language::LanguageId;
use zaroxi_lang_syntax::theme_map::StyledSpan;
use zaroxi_theme::theme::SemanticColors;

/// Cached syntax state for a single document.
#[derive(Debug, Clone)]
pub struct DocumentSyntaxState {
    /// The parsed syntax tree, if available.
    pub syntax_tree: Option<SyntaxTree>,
    /// The parser pool for creating syntax trees.
    pub parser_pool: Arc<ParserPool>,
    /// The highlighting engine.
    pub highlight_engine: HighlightEngine,
    /// Cached highlight spans for the current document version.
    pub cached_highlights: Vec<HighlightSpan>,
    /// The document version when highlights were last computed.
    pub cached_version: u64,
    /// The language ID for syntax highlighting.
    pub language: LanguageId,
    /// Cached styled spans keyed by (start_line, end_line) for the current version.
    /// This allows reusing previously computed styled spans across viewport changes.
    pub styled_spans_cache: std::collections::HashMap<(usize, usize), Vec<StyledSpan>>,
    /// The version for which styled_spans_cache is valid.
    pub styled_spans_cache_version: u64,
}

impl DocumentSyntaxState {
    pub fn new(language: LanguageId) -> Self {
        Self {
            syntax_tree: None,
            parser_pool: Arc::new(ParserPool::new()),
            highlight_engine: HighlightEngine::new(),
            cached_highlights: Vec::new(),
            cached_version: u64::MAX,
            language,
            styled_spans_cache: std::collections::HashMap::new(),
            styled_spans_cache_version: u64::MAX,
        }
    }

    /// Ensure the syntax tree exists and is up-to-date for the given text.
    /// Returns true if a syntax tree is available after this call.
    pub fn ensure_syntax_tree(&mut self, text: &str, language: LanguageId) -> bool {
        // PlainText has no grammar – skip entirely
        if language == LanguageId::PlainText {
            self.syntax_tree = None;
            self.cached_highlights.clear();
            self.cached_version = u64::MAX;
            return false;
        }

        // If language changed, invalidate everything
        if self.language != language {
            self.syntax_tree = None;
            self.cached_highlights.clear();
            self.cached_version = u64::MAX;
            self.language = language;
        }

        // If we already have a syntax tree, try to reparse it
        if let Some(ref mut tree) = self.syntax_tree {
            if tree.reparse().is_ok() {
                return true;
            }
            // Reparse failed, will create new tree
            self.syntax_tree = None;
        }

        // Create a new syntax tree
        match SyntaxTree::new(self.parser_pool.clone(), text, language) {
            Ok(tree) => {
                self.syntax_tree = Some(tree);
                true
            }
            Err(e) => {
                eprintln!("Warning: Failed to create syntax tree: {}", e);
                false
            }
        }
    }

    /// Get highlight spans for the current document content.
    /// Returns cached highlights if the document version hasn't changed.
    pub fn get_highlights(&mut self, text: &str, version: u64) -> &[HighlightSpan] {
        if version != self.cached_version {
            self.recompute_highlights(text, version);
        }
        &self.cached_highlights
    }

    fn recompute_highlights(&mut self, text: &str, version: u64) {
        self.cached_highlights.clear();
        self.cached_version = version;

        // PlainText has no grammar – skip
        if self.language == LanguageId::PlainText {
            return;
        }

        if !self.ensure_syntax_tree(text, self.language) {
            return;
        }

        let tree = match &self.syntax_tree {
            Some(t) => t,
            None => return,
        };

        match self.highlight_engine.highlight(self.language, text, tree.tree()) {
            Ok(spans) => {
                self.cached_highlights = spans;
            }
            Err(e) => {
                eprintln!("Warning: Syntax highlighting failed: {}", e);
            }
        }
    }

    /// Get styled spans for a specific line range, applying the given theme.
    /// Uses a two-level cache: first checks the styled_spans_cache for the exact range,
    /// then falls back to computing from the full highlights.
    pub fn styled_spans_for_lines(
        &mut self,
        text: &str,
        version: u64,
        colors: &SemanticColors,
        start_line: usize,
        end_line: usize,
        line_to_char: impl Fn(usize) -> usize,
        byte_to_char: impl Fn(usize) -> usize,
        len_chars: usize,
        total_lines: usize,
    ) -> Vec<StyledSpan> {
        // Check if the styled spans cache is still valid for this version
        if version != self.styled_spans_cache_version {
            self.styled_spans_cache.clear();
            self.styled_spans_cache_version = version;
        }

        // Try to find the exact range in cache
        let cache_key = (start_line, end_line);
        if let Some(cached) = self.styled_spans_cache.get(&cache_key) {
            return cached.clone();
        }

        // Compute highlights if needed
        let highlights = self.get_highlights(text, version);

        // Clamp line range to document bounds
        let start_line = start_line.min(total_lines.saturating_sub(1));
        let end_line = end_line.min(total_lines);

        // Convert line range to character range
        let start_char = line_to_char(start_line);
        let end_char = if end_line >= total_lines {
            len_chars
        } else {
            line_to_char(end_line)
        };

        let mut result = Vec::new();

        // Filter spans to the requested range, converting byte offsets to char offsets
        for span in highlights {
            let span_start_char = byte_to_char(span.start);
            let span_end_char = byte_to_char(span.end);

            if span_end_char > start_char && span_start_char < end_char {
                let clamped_start = span_start_char.max(start_char);
                let clamped_end = span_end_char.min(end_char);
                if clamped_start < clamped_end {
                    let token_type = zaroxi_lang_syntax::theme_map::SemanticTokenType::from_highlight(span.highlight);
                    let color = token_type.theme_color(colors);
                    result.push(StyledSpan {
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
                filled.push(StyledSpan {
                    start: cursor,
                    end: span.start,
                    token_type: zaroxi_lang_syntax::theme_map::SemanticTokenType::Plain,
                    color: colors.text_primary,
                });
            }
            filled.push(span.clone());
            cursor = span.end;
        }
        if cursor < end_char {
            filled.push(StyledSpan {
                start: cursor,
                end: end_char,
                token_type: zaroxi_lang_syntax::theme_map::SemanticTokenType::Plain,
                color: colors.text_primary,
            });
        }

        // Cache the result for this exact range
        self.styled_spans_cache.insert(cache_key, filled.clone());

        filled
    }
}

/// Global cache of document syntax states, keyed by canonical file path.
#[derive(Debug)]
pub struct DocumentSyntaxCache {
    states: Mutex<HashMap<PathBuf, DocumentSyntaxState>>,
}

impl DocumentSyntaxCache {
    pub fn new() -> Self {
        Self {
            states: Mutex::new(HashMap::new()),
        }
    }

    /// Get or create a syntax state for the given path, returning a mutable reference.
    pub fn get_or_create_state(&self, path: &PathBuf, language: LanguageId) -> DocumentSyntaxState {
        let mut states = self.states.lock();
        states.entry(path.clone()).or_insert_with(|| {
            DocumentSyntaxState::new(language)
        }).clone()
    }

    /// Update the syntax state for a given path.
    pub fn update_state(&self, path: &PathBuf, state: DocumentSyntaxState) {
        let mut states = self.states.lock();
        states.insert(path.clone(), state);
    }

    /// Remove a document from the cache.
    pub fn remove(&self, path: &PathBuf) {
        let mut states = self.states.lock();
        states.remove(path);
    }

    /// Clear all cached states.
    pub fn clear(&self) {
        let mut states = self.states.lock();
        states.clear();
    }
}

impl Default for DocumentSyntaxCache {
    fn default() -> Self {
        Self::new()
    }
}
