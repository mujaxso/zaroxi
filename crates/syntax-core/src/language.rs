//! Language identification and grammar loading.

use std::path::Path;
use std::collections::HashMap;
use std::sync::OnceLock;
use tree_sitter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageId {
    Rust,
    Toml,
    Markdown,
    PlainText,
    Dynamic(&'static str),
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
        
        // Check for TOML files
        if ext.eq_ignore_ascii_case("toml") {
            return LanguageId::Toml;
        }
        
        // Check for Markdown files
        if ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown") {
            return LanguageId::Markdown;
        }
        
        // Check for specific TOML filenames
        match name.as_str() {
            "cargo.toml" | "rust-toolchain.toml" | "clippy.toml" | "rustfmt.toml" 
            | ".clippy.toml" | ".rustfmt.toml" | "pyproject.toml" | "taplo.toml" => {
                return LanguageId::Toml;
            }
            _ => {}
        }
        
        // Check for built-in languages first
        match ext {
            "rs" => return LanguageId::Rust,
            _ => {}
        }
        
        // Try to match against dynamic language registry
        // First check by filename
        if let Some(lang_id) = Self::from_filename_dynamic(&name) {
            return LanguageId::Dynamic(lang_id);
        }
        
        // Then check by extension
        if let Some(lang_id) = Self::from_extension_dynamic(ext) {
            return LanguageId::Dynamic(lang_id);
        }
        
        LanguageId::PlainText
    }

    fn from_filename_dynamic(name: &str) -> Option<&'static str> {
        use crate::grammar_registry::GrammarRegistry;
        
        static FILENAME_MAP: OnceLock<HashMap<String, &'static str>> = OnceLock::new();
        
        let map = FILENAME_MAP.get_or_init(|| {
            let mut map = HashMap::new();
            let registry = GrammarRegistry::global();
            
            for (lang_id, info) in registry.languages() {
                for filename in &info.filenames {
                    map.insert(filename.to_lowercase(), *lang_id);
                }
            }
            map
        });
        
        map.get(&name.to_lowercase()).copied()
    }

    fn from_extension_dynamic(ext: &str) -> Option<&'static str> {
        use crate::grammar_registry::GrammarRegistry;
        
        static EXTENSION_MAP: OnceLock<HashMap<String, &'static str>> = OnceLock::new();
        
        let map = EXTENSION_MAP.get_or_init(|| {
            let mut map = HashMap::new();
            let registry = GrammarRegistry::global();
            
            for (lang_id, info) in registry.languages() {
                for ext in &info.extensions {
                    map.insert(ext.to_lowercase(), *lang_id);
                }
                for filename in &info.filenames {
                    map.insert(filename.to_lowercase(), *lang_id);
                }
            }
            map
        });
        
        map.get(&ext.to_lowercase()).copied()
    }

    pub fn as_str(&self) -> &str {
        match self {
            LanguageId::Rust => "rust",
            LanguageId::Toml => "toml",
            LanguageId::Markdown => "markdown",
            LanguageId::PlainText => "plaintext",
            LanguageId::Dynamic(id) => id,
        }
    }

    /// Return the Tree-sitter language, loading dynamically if needed.
    pub fn tree_sitter_language(&self) -> Option<tree_sitter::Language> {
        match self {
            LanguageId::Rust => {
                eprintln!("DEBUG: tree_sitter_language for Rust");
                crate::dynamic_loader::load_language("rust")
            }
            LanguageId::Toml => {
                eprintln!("DEBUG: tree_sitter_language for Toml");
                crate::dynamic_loader::load_language("toml")
            }
            LanguageId::Markdown => {
                eprintln!("DEBUG: tree_sitter_language for Markdown");
                crate::dynamic_loader::load_language("markdown")
            }
            LanguageId::PlainText => None,
            LanguageId::Dynamic(id) => crate::dynamic_loader::load_language(id),
        }
    }
}
