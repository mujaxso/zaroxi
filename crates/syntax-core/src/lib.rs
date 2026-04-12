//! Syntax layer for Neote IDE.
//!
//! This crate provides Tree-sitter-based syntax parsing, highlighting,
//! and language support for the editor. It's designed to be:
//! - Incremental: updates syntax trees efficiently after edits
//! - Modular: clean separation between parsing, highlighting, and UI
//! - Extensible: easy to add new languages and features
//! - Performant: minimal overhead for large files and frequent edits

pub mod error;
pub mod highlight;
pub mod language;
pub mod manager;

// Re-export main types for convenience
pub use error::SyntaxError;
pub use highlight::{Highlight, HighlightSpan};
pub use language::LanguageId;
pub use manager::SyntaxManager;
