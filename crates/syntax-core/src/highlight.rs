//! Syntax highlighting using Tree-sitter queries.

use crate::error::SyntaxError;
use crate::language::LanguageId;
use tree_sitter::{Query, QueryCursor, Tree};

/// A highlight span in the document
#[derive(Debug, Clone)]
pub struct HighlightSpan {
    /// Start byte offset
    pub start: usize,
    /// End byte offset
    pub end: usize,
    /// Highlight type (maps to theme token categories)
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

    // Try to compile the query
    let query = match Query::new(&ts_lang, query_str) {
        Ok(q) => q,
        Err(e) => {
            // Log the error for debugging
            eprintln!("DEBUG: Tree-sitter query error for {}: {}", language.as_str(), e);
            // Return empty spans (plaintext) when query compilation fails
            return Ok(Vec::new());
        }
    };

    let mut cursor = QueryCursor::new();
    let root_node = tree.root_node();
    let mut spans = Vec::new();

    // In tree-sitter 0.26.8, we need to use captures() instead of matches()
    // The captures() method returns an iterator over QueryCapture items
    let captures = cursor.captures(&query, root_node, source.as_bytes());
    for (match_, capture_index) in captures {
        let capture = match_.captures[capture_index];
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

    // Sort spans by start position
    spans.sort_by_key(|span| span.start);
    
    Ok(spans)
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
        "variable.builtin" => Highlight::Type, // Built-in variables like 'self' use type color (amber in dark theme)
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
        
        // Markdown-specific captures (based on tree-sitter-markdown grammar)
        // These may vary between versions
        "heading" => Highlight::Type,
        "heading.1" => Highlight::Type,
        "heading.2" => Highlight::Type,
        "heading.3" => Highlight::Type,
        "heading.4" => Highlight::Type,
        "heading.5" => Highlight::Type,
        "heading.6" => Highlight::Type,
        "atx_heading" => Highlight::Type,
        "setext_heading" => Highlight::Type,
        "emphasis" => Highlight::Comment,
        "strong_emphasis" => Highlight::Keyword,
        "strong" => Highlight::Keyword,
        "link" => Highlight::Variable,
        "link_text" => Highlight::Variable,
        "link_destination" => Highlight::String,
        "link_url" => Highlight::String,
        "link_title" => Highlight::String,
        "inline_code_span" => Highlight::Constant,
        "inline_code" => Highlight::Constant,
        "code_fence" => Highlight::Property,
        "block_quote" => Highlight::Comment,
        "blockquote" => Highlight::Comment,
        "list" => Highlight::Property,
        "list_item" => Highlight::Property,
        "thematic_break" => Highlight::Operator,
        "html_block" => Highlight::Attribute,
        "html_inline" => Highlight::Attribute,
        "table" => Highlight::Property,
        "table_header" => Highlight::Type,
        "table_row" => Highlight::Property,
        "table_cell" => Highlight::Plain,
        _ => Highlight::Plain,
    }
}

pub fn get_query_for_language(language: LanguageId) -> Result<&'static str, SyntaxError> {
    let language_id = language.as_str();
    
    // Always try to load from the query cache first
    // Check if query exists in cache (but we need the query text, not the compiled query)
    // The cache stores compiled queries, but we need the raw text
    // So we'll always load from file for now
    let runtime = crate::runtime::Runtime::new();
    let query_path = runtime.language_dir(language_id).join("queries/highlights.scm");
    
    eprintln!("DEBUG: Loading query for {} from {}", language_id, query_path.display());
    
    if query_path.exists() {
        match std::fs::read_to_string(&query_path) {
            Ok(query_text) => {
                // Leak the string to make it static
                let leaked = Box::leak(query_text.into_boxed_str());
                Ok(leaked)
            }
            Err(e) => {
                eprintln!("DEBUG: Failed to read query file for {}: {}", language_id, e);
                Err(SyntaxError::LanguageNotSupported(
                    format!("failed to read query file: {}", e),
                ))
            }
        }
    } else {
        eprintln!("DEBUG: Query file doesn't exist for {}", language_id);
        Err(SyntaxError::LanguageNotSupported(
            format!("{} grammar not available", language_id),
        ))
    }
}
