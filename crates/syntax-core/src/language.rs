//! Language identification and grammar loading.

use std::path::Path;
use tree_sitter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageId {
    Rust,
    Toml,
    PlainText,
}

impl LanguageId {
    /// Determine language from file path.
    pub fn from_path(path: &Path) -> Self {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();
        match (ext, name.as_str()) {
            ("rs", _) => LanguageId::Rust,
            ("toml", _) => LanguageId::Toml,
            (_, "Cargo.toml") => LanguageId::Toml,
            (_, "rust-toolchain.toml") => LanguageId::Toml,
            _ => LanguageId::PlainText,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LanguageId::Rust => "rust",
            LanguageId::Toml => "toml",
            LanguageId::PlainText => "plaintext",
        }
    }

    /// Return the statically linked Tree-sitter language if available.
    pub fn tree_sitter_language(&self) -> Option<tree_sitter::Language> {
        match self {
            #[cfg(feature = "rust")]
            LanguageId::Rust => Some(tree_sitter_rust::language()),
            #[cfg(not(feature = "rust"))]
            LanguageId::Rust => None,
            LanguageId::Toml => None, // TOML support not currently compiled
            LanguageId::PlainText => None,
        }
    }
}
