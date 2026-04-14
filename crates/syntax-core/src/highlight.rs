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
    match language {
        LanguageId::Rust => highlight_with_query(language, source, tree),
        #[cfg(feature = "toml")]
        LanguageId::Toml => highlight_with_query(language, source, tree),
        #[cfg(not(feature = "toml"))]
        LanguageId::Toml => Ok(Vec::new()),
        #[cfg(feature = "markdown")]
        LanguageId::Markdown => highlight_with_query(language, source, tree),
        #[cfg(not(feature = "markdown"))]
        LanguageId::Markdown => Ok(Vec::new()),
        LanguageId::PlainText => Ok(Vec::new()),
    }
}

fn highlight_with_query(
    language: LanguageId,
    source: &str,
    tree: &Tree,
) -> Result<Vec<HighlightSpan>, SyntaxError> {
    let query_str = get_query_for_language(language)?;
    let ts_lang = language
        .tree_sitter_language()
        .ok_or_else(|| SyntaxError::LanguageNotSupported(language.as_str().to_string()))?;

    // If the query fails (e.g., because of unrecognized node types), we return empty highlights
    // rather than propagating an error. This allows the editor to keep working without syntax
    // highlighting for that particular language.
    let query = match Query::new(ts_lang, query_str) {
        Ok(q) => q,
        Err(e) => {
            // Log the error for debugging
            eprintln!("DEBUG: Tree-sitter query error for {}: {}", language.as_str(), e);
            return Ok(Vec::new());
        }
    };

    let mut cursor = QueryCursor::new();
    let root_node = tree.root_node();
    let mut spans = Vec::new();

    for match_ in cursor.matches(&query, root_node, source.as_bytes()) {
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
        _ => Highlight::Plain,
    }
}

pub fn get_query_for_language(language: LanguageId) -> Result<&'static str, SyntaxError> {
    match language {
        LanguageId::Rust => {
            #[cfg(feature = "rust")]
            {
                Ok(include_str!(
                    "../../../runtime/treesitter/languages/rust/queries/highlights.scm"
                ))
            }
            #[cfg(not(feature = "rust"))]
            Err(SyntaxError::LanguageNotSupported(
                "rust feature not enabled".to_string(),
            ))
        }
        LanguageId::Toml => {
            #[cfg(feature = "toml")]
            {
                // Use the official highlight query from the tree-sitter-toml crate
                // This ensures we're using the correct node types for the exact grammar version
                use tree_sitter_toml;
                Ok(tree_sitter_toml::HIGHLIGHT_QUERY)
            }
            #[cfg(not(feature = "toml"))]
            Err(SyntaxError::LanguageNotSupported(
                "toml support not compiled".to_string(),
            ))
        }
        LanguageId::Markdown => {
            #[cfg(feature = "markdown")]
            {
                // Try to load from runtime directory first
                // This follows the same pattern as Rust
                match std::fs::read_to_string("../../../runtime/treesitter/languages/markdown/queries/highlights.scm") {
                    Ok(query) => Ok(Box::leak(query.into_boxed_str())),
                    Err(_) => {
                        // Fall back to a minimal query if file doesn't exist
                        // This is better than hardcoding a large query
                        const MINIMAL_MARKDOWN_QUERY: &str = r#"
                            (atx_heading) @heading
                            (setext_heading) @heading
                            (emphasis) @emphasis
                            (strong_emphasis) @strong
                            (inline_code_span) @inline_code
                            (code_fence) @code_fence
                        "#;
                        Ok(MINIMAL_MARKDOWN_QUERY)
                    }
                }
            }
            #[cfg(not(feature = "markdown"))]
            Err(SyntaxError::LanguageNotSupported(
                "markdown support not compiled".to_string(),
            ))
        }
        LanguageId::PlainText => Err(SyntaxError::LanguageNotSupported(
            "plaintext has no syntax queries".to_string(),
        )),
    }
}
