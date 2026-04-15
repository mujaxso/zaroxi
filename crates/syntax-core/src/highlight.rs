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

    // Check if query is empty
    if query_str.trim().is_empty() {
        eprintln!("DEBUG: Query for {} is empty", language.as_str());
        return Ok(Vec::new());
    }

    // Try to compile the query
    let query = match Query::new(&ts_lang, query_str) {
        Ok(q) => q,
        Err(e) => {
            // Log the error for debugging
            eprintln!("DEBUG: Tree-sitter query error for {}: {}", language.as_str(), e);
            eprintln!("DEBUG: Query text that failed to compile:");
            eprintln!("{}", query_str);
            // Return empty spans (plaintext) when query compilation fails
            return Ok(Vec::new());
        }
    };

    // Debug: print capture names for markdown
    if language.as_str() == "markdown" {
        println!("DEBUG: Markdown capture names: {:?}", query.capture_names());
    }

    let mut cursor = QueryCursor::new();
    let root_node = tree.root_node();
    let mut spans = Vec::new();

    // In tree-sitter 0.26.8, QueryCursor::matches() returns QueryMatches which implements StreamingIterator
    // We need to use a while loop with next()
    let mut matches = cursor.matches(&query, root_node, source.as_bytes());
    let mut match_count = 0;
    while let Some(match_) = StreamingIterator::next(&mut matches) {
        match_count += 1;
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
        // Additional captures that might be present
        "paragraph" => Highlight::Plain,
        "fenced_code_block" => Highlight::Property,
        "code_span" => Highlight::Constant,
        "image" => Highlight::Variable,
        "reference_link" => Highlight::Variable,
        "reference_definition" => Highlight::Variable,
        "footnote_reference" => Highlight::Variable,
        "footnote_definition" => Highlight::Variable,
        "task_list_marker" => Highlight::Operator,
        "strikethrough" => Highlight::Comment,
        "escape_sequence" => Highlight::String,
        "hard_line_break" => Highlight::Operator,
        "soft_line_break" => Highlight::Plain,
        // More markdown captures
        "heading_content" => Highlight::Type,
        "list_marker" => Highlight::Operator,
        "link_label" => Highlight::Variable,
        "link_title" => Highlight::String,
        "url" => Highlight::String,
        "email" => Highlight::String,
        "html" => Highlight::Attribute,
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
        "label" => Highlight::Variable,
        "definition" => Highlight::Variable,
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
