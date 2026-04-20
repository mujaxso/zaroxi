//! Syntax highlighting using Tree-sitter queries.

use crate::error::SyntaxError;
use crate::language::LanguageId;
use tree_sitter::{Query, QueryCursor, Tree, StreamingIterator};

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

/// Highlight a syntax tree for a given language.
pub fn highlight(
    language: LanguageId,
    source: &str,
    tree: &Tree,
) -> Result<Vec<HighlightSpan>, SyntaxError> {
    // Always try to highlight with query, regardless of language
    highlight_with_query(language, source, tree)
}

fn highlight_with_query(
    language: LanguageId,
    source: &str,
    tree: &Tree,
) -> Result<Vec<HighlightSpan>, SyntaxError> {
    // Get the Tree-sitter language
    let ts_lang = match language.tree_sitter_language() {
        Some(lang) => lang,
        None => {
            // No language available, return empty spans
            return Ok(Vec::new());
        }
    };

    // Try to get the query string
    let query_str = match get_query_for_language(language) {
        Ok(str) => str,
        Err(e) => {
            eprintln!("DEBUG: Failed to get query for {}: {:?}", language.as_str(), e);
            // Return empty spans when query can't be loaded
            return Ok(Vec::new());
        }
    };

    // Check if query is empty
    if query_str.trim().is_empty() {
        eprintln!("DEBUG: Query for {} is empty", language.as_str());
        return Ok(Vec::new());
    }

    // Try to compile the query
    let query = match Query::new(&ts_lang, query_str) {
        Ok(q) => {
            // Log success for debugging
            eprintln!("DEBUG: Query compiled successfully for {}", language.as_str());
            if language.as_str() == "markdown" {
                eprintln!("DEBUG: Markdown query has {} patterns", q.pattern_count());
                eprintln!("DEBUG: Markdown query captures: {:?}", q.capture_names());
            }
            q
        }
        Err(e) => {
            // Log the error for debugging
            eprintln!("DEBUG: Tree-sitter query error for {}: {}", language.as_str(), e);
            // For markdown, also log the query string to help debug
            if language.as_str() == "markdown" {
                eprintln!("DEBUG: Markdown query string: {:?}", query_str);
                eprintln!("DEBUG: Markdown language node count: {}", ts_lang.node_kind_count());
                // Print ALL node types for debugging
                for i in 0..ts_lang.node_kind_count() {
                    let kind = ts_lang.node_kind_for_id(i as u16);
                    if let Some(kind) = kind {
                        eprintln!("DEBUG: Node type {}: {}", i, kind);
                    }
                }
            }
            // Try to create an empty query as a fallback
            match Query::new(&ts_lang, "") {
                Ok(empty_query) => {
                    eprintln!("DEBUG: Using empty query as fallback for {}", language.as_str());
                    empty_query
                }
                Err(_) => {
                    // If even an empty query fails, return empty spans
                    return Ok(Vec::new());
                }
            }
        }
    };

    let mut cursor = QueryCursor::new();
    let root_node = tree.root_node();
    let mut spans = Vec::new();

    // In tree-sitter 0.26.8, QueryCursor::matches() returns QueryMatches which implements StreamingIterator
    // We need to use a while loop with next()
    let mut matches = cursor.matches(&query, root_node, source.as_bytes());
    while let Some(match_) = StreamingIterator::next(&mut matches) {
        for capture in match_.captures {
            let node = capture.node;
            let start = node.start_byte();
            let end = node.end_byte();
            let capture_name = &query.capture_names()[capture.index as usize];
            let highlight = map_capture_name(capture_name);
            
            spans.push(HighlightSpan {
                start,
                end,
                highlight,
            });
        }
    }

    // Sort spans by start position
    spans.sort_by_key(|span| span.start);
    
    // Filter out plain spans that are completely covered by other spans
    let mut filtered_spans = Vec::new();
    for (i, span) in spans.iter().enumerate() {
        if span.highlight == Highlight::Plain {
            let mut covered = false;
            for (j, other) in spans.iter().enumerate() {
                if i != j && other.highlight != Highlight::Plain &&
                   other.start <= span.start && other.end >= span.end {
                    covered = true;
                    break;
                }
            }
            if !covered {
                filtered_spans.push(span.clone());
            }
        } else {
            filtered_spans.push(span.clone());
        }
    }
    
    if language.as_str() == "markdown" {
        eprintln!("DEBUG: Generated {} highlight spans for markdown (filtered to {})", spans.len(), filtered_spans.len());
        for span in filtered_spans.iter().take(5) {
            eprintln!("DEBUG: Span {}..{} -> {:?}", span.start, span.end, span.highlight);
        }
    }
    
    Ok(filtered_spans)
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
        "lifetime" => Highlight::Type,  // Lifetimes use type color
        
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
        "escape" => Highlight::String,
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
