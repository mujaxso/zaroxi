//! Syntax highlighting using Tree-sitter queries.

use crate::error::SyntaxError;
use crate::language::LanguageId;
use tree_sitter::{Query, QueryCursor, StreamingIterator, Tree};

/// A highlight span in the document
#[derive(Debug, Clone)]
pub struct HighlightSpan {
    /// Start byte offset
    pub start: usize,
    /// End byte offset
    pub end: usize,
    /// Highlight type (maps to zaroxi_theme token categories)
    pub highlight: Highlight,
}

/// Highlight types (maps to Tree-sitter capture names)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Highlight {
    Comment,
    String,
    Keyword,
    Function,
    Variable,
    Type,
    Constant,
    Attribute,
    Operator,
    Number,
    Property,
    Namespace,
    Plain,
}

/// The highlighting engine that manages parsing and query execution.
///
/// This struct is designed to be reused across multiple highlight operations
/// to avoid re-creating parsers and queries unnecessarily.
#[derive(Debug)]
pub struct HighlightEngine {
    /// Cached runtime for resolving language resources.
    runtime: crate::runtime::Runtime,
}

impl HighlightEngine {
    /// Create a new highlighting engine.
    pub fn new() -> Self {
        Self {
            runtime: crate::runtime::Runtime::new(),
        }
    }

    /// Highlight a syntax tree for a given language.
    ///
    /// Returns a vector of `HighlightSpan`s sorted by start position.
    /// Returns an empty vector if highlighting is not possible (e.g., unknown
    /// language, missing query, or large file).
    pub fn highlight(
        &self,
        language: LanguageId,
        source: &str,
        tree: &Tree,
    ) -> Result<Vec<HighlightSpan>, SyntaxError> {
        // For plaintext or unknown languages, return empty spans
        if language == LanguageId::PlainText {
            return Ok(Vec::new());
        }

        // Get the Tree-sitter language
        let ts_lang = match language.tree_sitter_language() {
            Some(lang) => lang,
            None => {
                // No grammar available, return empty spans
                return Ok(Vec::new());
            }
        };

        // Load and compile the highlighting query
        let query = match self.load_query(&language) {
            Some(q) => q,
            None => {
                // No query available, return empty spans
                return Ok(Vec::new());
            }
        };

        // Execute the query against the syntax tree
        let spans = self.execute_query(&query, source, tree);

        // Post-process spans: sort and filter covered plain spans
        let filtered = self.filter_spans(spans);

        Ok(filtered)
    }

    /// Load and compile the highlighting query for a language.
    ///
    /// Returns `None` if the query cannot be loaded or compiled.
    fn load_query(&self, language: &LanguageId) -> Option<Query> {
        let language_id = language.as_str();

        // Get the Tree-sitter language for query compilation
        let ts_lang = language.tree_sitter_language()?;

        // Read the query file from the runtime directory
        let query_path = self
            .runtime
            .language_dir(language_id)
            .join("queries")
            .join("highlights.scm");

        let query_text = match std::fs::read_to_string(&query_path) {
            Ok(text) => text,
            Err(_) => {
                // Query file not found or unreadable
                return None;
            }
        };

        if query_text.trim().is_empty() {
            return None;
        }

        // Compile the query
        match Query::new(&ts_lang, &query_text) {
            Ok(q) => Some(q),
            Err(e) => {
                // Log the error for debugging but don't crash
                eprintln!(
                    "Warning: Failed to compile query for {}: {}",
                    language_id, e
                );
                None
            }
        }
    }

    /// Execute a highlighting query against a syntax tree.
    fn execute_query(
        &self,
        query: &Query,
        source: &str,
        tree: &Tree,
    ) -> Vec<HighlightSpan> {
        let mut cursor = QueryCursor::new();
        let root_node = tree.root_node();
        let mut spans = Vec::new();

        // Use StreamingIterator to iterate over query matches
        let mut matches = cursor.matches(query, root_node, source.as_bytes());
        while let Some(match_) = StreamingIterator::next(&mut matches) {
            for capture in match_.captures {
                let node = capture.node;
                let start = node.start_byte();
                let end = node.end_byte();
                let capture_name = &query.capture_names()[capture.index as usize];
                let highlight = map_capture_name(capture_name);

                spans.push(HighlightSpan { start, end, highlight });
            }
        }

        spans
    }

    /// Post-process highlight spans: sort and filter covered plain spans.
    fn filter_spans(&self, mut spans: Vec<HighlightSpan>) -> Vec<HighlightSpan> {
        // Sort by start position
        spans.sort_by_key(|span| span.start);

        // Filter out plain spans that are completely covered by other spans
        let mut filtered = Vec::new();
        for (i, span) in spans.iter().enumerate() {
            if span.highlight == Highlight::Plain {
                let mut covered = false;
                for (j, other) in spans.iter().enumerate() {
                    if i != j
                        && other.highlight != Highlight::Plain
                        && other.start <= span.start
                        && other.end >= span.end
                    {
                        covered = true;
                        break;
                    }
                }
                if !covered {
                    filtered.push(span.clone());
                }
            } else {
                filtered.push(span.clone());
            }
        }

        filtered
    }
}

impl Default for HighlightEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub fn map_capture_name(name: &str) -> Highlight {
    match name {
        // Programming language captures
        "comment" => Highlight::Comment,
        "string" => Highlight::String,
        "string.escape" => Highlight::String,
        "escape" => Highlight::String,
        "string.special" => Highlight::String,
        "keyword" => Highlight::Keyword,
        "function" | "function.call" | "function.method" => Highlight::Function,
        "function.macro" | "macro" => Highlight::Function, // Macros use function color
        "variable" | "variable.parameter" => Highlight::Variable,
        "variable.builtin" => Highlight::Type, // Built-in variables like 'self' use type color (amber in dark zaroxi_theme)
        "type" | "type.builtin" => Highlight::Type,
        "constant" | "constant.builtin" => Highlight::Constant,
        "attribute" => Highlight::Attribute,
        "operator" => Highlight::Operator,
        "punctuation.bracket" => Highlight::Operator,
        "punctuation.delimiter" => Highlight::Operator,
        "number" => Highlight::Number,
        "boolean" => Highlight::Constant,
        "property" => Highlight::Property,
        "namespace" => Highlight::Namespace,
        "constructor" => Highlight::Type,
        "label" => Highlight::Variable,
        "mutable_specifier" => Highlight::Keyword,
        "lifetime" => Highlight::Type, // Lifetimes use type color

        // Markdown-specific captures (based on tree-sitter-markdown-inline grammar)
        // Based on debug output showing actual node types
        "emphasis" => Highlight::Comment,
        "strong_emphasis" => Highlight::Keyword,
        "code_span" => Highlight::Constant,
        "inline_code" => Highlight::Constant,
        "link_text" => Highlight::Variable,
        "link_destination" => Highlight::String,
        "link_title" => Highlight::String,
        "shortcut_link" => Highlight::Variable,
        "full_reference_link" => Highlight::Variable,
        "collapsed_reference_link" => Highlight::Variable,
        "inline_link" => Highlight::Variable,
        "image" => Highlight::Variable,
        "image.description" => Highlight::Variable,
        "html_tag" => Highlight::Attribute,
        "hard_line_break" => Highlight::Operator,
        "line_break" => Highlight::Operator,
        "strikethrough" => Highlight::Comment,
        "uri_autolink" => Highlight::String,
        "email_autolink" => Highlight::String,
        "backslash_escape" => Highlight::String,
        "latex" => Highlight::Constant,
        // Additional captures that might be present
        "link" => Highlight::Variable,
        "link.autolink" => Highlight::String,
        "html" => Highlight::Attribute,
        "html_block" => Highlight::Attribute,
        "html_inline" => Highlight::Attribute,
        // Fallback for block-level elements if they somehow appear
        "heading" => Highlight::Type,
        // These captures are kept for compatibility but not used in the current markdown query
        // They remain as fallbacks
        "heading.1" => Highlight::Type,
        "heading.2" => Highlight::Type,
        "heading.3" => Highlight::Type,
        "heading.4" => Highlight::Type,
        "heading.5" => Highlight::Type,
        "heading.6" => Highlight::Type,
        "atx_heading" => Highlight::Type,
        "setext_heading" => Highlight::Type,
        "setext_heading_text" => Highlight::Type,
        "code_block" => Highlight::Property,
        "code_block.delimiter" => Highlight::Operator,
        "code_block.content" => Highlight::Plain,
        "fenced_code_block" => Highlight::Property,
        "fenced_code_block_delimiter" => Highlight::Operator,
        "blockquote" => Highlight::Comment,
        "block_quote" => Highlight::Comment,
        "blockquote.marker" => Highlight::Operator,
        "block_quote_marker" => Highlight::Operator,
        "list" => Highlight::Property,
        "list.item" => Highlight::Property,
        "list.marker" => Highlight::Operator,
        "list_item" => Highlight::Property,
        "thematic_break" => Highlight::Operator,
        "table" => Highlight::Property,
        "table.header" => Highlight::Type,
        "table.row" => Highlight::Property,
        "table.cell" => Highlight::Plain,
        "table.delimiter" => Highlight::Operator,
        "paragraph" => Highlight::Plain,
        "reference_link" => Highlight::Variable,
        "reference_definition" => Highlight::Variable,
        "footnote_reference" => Highlight::Variable,
        "footnote_definition" => Highlight::Variable,
        "task_list_marker" => Highlight::Operator,
        "escape_sequence" => Highlight::String,
        "soft_line_break" => Highlight::Plain,
        "heading_content" => Highlight::Type,
        "url" => Highlight::String,
        "email" => Highlight::String,
        "inline" => Highlight::Plain,
        "block" => Highlight::Plain,
        "document" => Highlight::Plain,
        // nvim-treesitter style captures
        "text.literal" => Highlight::Constant,
        "text.reference" => Highlight::Variable,
        "text.title" => Highlight::Type,
        "text.uri" => Highlight::String,
        "text.emphasis" => Highlight::Comment,
        "text.strong" => Highlight::Keyword,
        "text.quote" => Highlight::Comment,
        "text.math" => Highlight::Constant,
        "text.environment" => Highlight::Property,
        "punctuation.special" => Highlight::Operator,
        "definition" => Highlight::Variable,
        // Additional markdown captures from the query file
        "atx_heading_marker" => Highlight::Operator,
        "plain" => Highlight::Plain,
        _ => Highlight::Plain,
    }
}

pub fn get_query_for_language(language: LanguageId) -> Result<&'static str, SyntaxError> {
    let language_id = language.as_str();

    // For plaintext, return an empty query
    if language_id == "plaintext" {
        return Ok("");
    }

    let runtime = crate::runtime::Runtime::new();
    let query_path = runtime.language_dir(language_id).join("queries/highlights.scm");

    if query_path.exists() {
        match std::fs::read_to_string(&query_path) {
            Ok(query_text) => {
                // Check if the query is not empty
                if query_text.trim().is_empty() {
                    eprintln!("DEBUG: Query file for {} is empty", language_id);
                    return Ok("");
                }

                // Leak the string to make it static
                let leaked = Box::leak(query_text.into_boxed_str());
                Ok(leaked)
            }
            Err(e) => {
                // If we can't read the query file, return an empty query instead of failing
                // This allows syntax highlighting to fall back gracefully
                eprintln!("Warning: Failed to read query file for {}: {}", language_id, e);
                Ok("")
            }
        }
    } else {
        // If no query file exists, return an empty query
        // This is better than failing, as it allows the editor to work without syntax highlighting
        eprintln!("Warning: No query file found for {} at {}", language_id, query_path.display());
        Ok("")
    }
}
