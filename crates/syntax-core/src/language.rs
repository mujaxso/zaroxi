//! Language registry and abstraction for Tree-sitter grammars.

use std::collections::HashMap;
use std::path::Path;
use tree_sitter::{Language as TsLanguage, Parser};

use crate::highlight::HighlightConfiguration;

/// Supported language identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageId {
    Rust,
    Toml,
    PlainText,
}

impl LanguageId {
    /// Detect language from file extension
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => LanguageId::Rust,
            "toml" => Self::Toml,
            _ => Self::PlainText,
        }
    }

    /// Detect language from file path
    pub fn from_path(path: &Path) -> Self {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(Self::from_extension)
            .unwrap_or(Self::PlainText)
    }

    /// Get the Tree-sitter language for this language ID
    pub fn tree_sitter_language(&self) -> Option<TsLanguage> {
        match self {
            LanguageId::Rust => {
                #[cfg(feature = "rust")]
                {
                    // Use the raw C function to get the language pointer
                    extern "C" {
                        fn tree_sitter_rust() -> *const ();
                    }
                    let ptr = unsafe { tree_sitter_rust() };
                    // Convert raw pointer to Language using from_raw
                    // Safety: tree_sitter_rust() returns a valid language pointer
                    Some(unsafe { std::mem::transmute(ptr) })
                }
                #[cfg(not(feature = "rust"))]
                {
                    None
                }
            }
            LanguageId::Toml => {
                // TOML support is not currently implemented
                None
            }
            LanguageId::PlainText => None,
        }
    }

    /// Get the highlight query for this language
    pub fn highlight_query(&self) -> &'static str {
        match self {
            LanguageId::Rust => include_str!("../queries/rust/highlights.scm"),
            LanguageId::Toml => "",
            LanguageId::PlainText => "",
        }
    }
}

/// Registry for managing language configurations
pub struct LanguageRegistry {
    languages: HashMap<LanguageId, HighlightConfiguration>,
}

impl LanguageRegistry {
    /// Create a new language registry
    pub fn new() -> Self {
        let mut registry = Self {
            languages: HashMap::new(),
        };

        // Initialize supported languages
        registry.register_languages();

        registry
    }

    /// Register all supported languages
    fn register_languages(&mut self) {
        // Register Rust if available
        if let Some(lang) = LanguageId::Rust.tree_sitter_language() {
            if let Ok(config) = HighlightConfiguration::new(
                &lang,
                LanguageId::Rust.highlight_query(),
                "",
                "",
            ) {
                self.languages.insert(LanguageId::Rust, config);
            }
        }

        // TOML support is currently disabled
    }

    /// Get language configuration for a language ID
    pub fn get_config(&self, lang_id: LanguageId) -> Option<&HighlightConfiguration> {
        self.languages.get(&lang_id)
    }

    /// Create a parser for a specific language
    pub fn create_parser(&self, lang_id: LanguageId) -> Option<Parser> {
        let mut parser = Parser::new();
        
        if let Some(lang) = lang_id.tree_sitter_language() {
            if parser.set_language(&lang).is_ok() {
                return Some(parser);
            }
        }
        
        None
    }

    /// Detect language from file path and get its configuration
    pub fn detect_from_path(&self, path: &Path) -> (LanguageId, Option<&HighlightConfiguration>) {
        let lang_id = LanguageId::from_path(path);
        let config = self.get_config(lang_id);
        (lang_id, config)
    }
}

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}
