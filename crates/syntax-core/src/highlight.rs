//! Syntax highlighting using Tree-sitter queries.

use crate::error::SyntaxError;
use crate::language::LanguageId;
use tree_sitter::Tree;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Plain,
}

/// Highlight a syntax tree for a given language.
pub fn highlight(
    language: LanguageId,
    source: &str,
    tree: &Tree,
) -> Result<Vec<HighlightSpan>, SyntaxError> {
    match language {
        LanguageId::Rust => highlight_rust(source),
        LanguageId::Toml => highlight_toml(source),
        LanguageId::PlainText => Ok(Vec::new()),
    }
}

fn highlight_rust(source: &str) -> Result<Vec<HighlightSpan>, SyntaxError> {
    let mut spans = Vec::new();
    // Simple line-based detection for demonstration.
    let mut offset = 0;
    for line in source.lines() {
        if let Some(pos) = line.find("//") {
            let start = offset + pos;
            let end = offset + line.len();
            spans.push(HighlightSpan {
                start,
                end,
                highlight: Highlight::Comment,
            });
        }
        // Detect string literals (simplistic)
        if let Some(pos) = line.find('\"') {
            // Find closing quote
            let line_remainder = &line[pos + 1..];
            if let Some(close) = line_remainder.find('\"') {
                let start = offset + pos;
                let end = offset + pos + 1 + close + 1;
                spans.push(HighlightSpan {
                    start,
                    end,
                    highlight: Highlight::String,
                });
            }
        }
        offset += line.len() + 1; // +1 for newline character (assuming \n)
    }
    Ok(spans)
}

fn highlight_toml(_source: &str) -> Result<Vec<HighlightSpan>, SyntaxError> {
    // Placeholder
    Ok(Vec::new())
}
