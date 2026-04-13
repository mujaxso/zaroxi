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
        Err(_) => {
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

fn map_capture_name(name: &str) -> Highlight {
    match name {
        "comment" => Highlight::Comment,
        "string" => Highlight::String,
        "keyword" => Highlight::Keyword,
        "function" | "function.call" => Highlight::Function,
        "variable" | "variable.parameter" => Highlight::Variable,
        "type" | "type.builtin" => Highlight::Type,
        "constant" | "constant.builtin" => Highlight::Constant,
        "attribute" => Highlight::Attribute,
        "operator" => Highlight::Operator,
        "number" => Highlight::Number,
        "property" => Highlight::Property,
        "namespace" => Highlight::Namespace,
        "macro" => Highlight::Function, // Treat macros like functions
        "punctuation" => Highlight::Operator, // Treat punctuation as operators
        _ => Highlight::Plain,
    }
}

fn get_query_for_language(language: LanguageId) -> Result<&'static str, SyntaxError> {
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
                Ok(include_str!(
                    "../../../runtime/treesitter/languages/toml/queries/highlights.scm"
                ))
            }
            #[cfg(not(feature = "toml"))]
            Err(SyntaxError::LanguageNotSupported(
                "toml support not compiled".to_string(),
            ))
        }
        LanguageId::PlainText => Err(SyntaxError::LanguageNotSupported(
            "plaintext has no syntax queries".to_string(),
        )),
    }
}
