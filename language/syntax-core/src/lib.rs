//! Syntax highlighting for Zaroxi

use tree_sitter::{Parser, Language};

/// Syntax highlighter
pub struct Highlighter {
    parser: Parser,
}

impl Highlighter {
    /// Create a new highlighter
    pub fn new() -> Self {
        let mut parser = Parser::new();
        // TODO: Initialize with a language
        Self { parser }
    }
}
