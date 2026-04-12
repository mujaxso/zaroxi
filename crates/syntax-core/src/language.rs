//! Language registry and abstraction for Tree-sitter grammars.

use std::collections::HashMap;
use std::path::Path;
use tree_sitter::{Language as TsLanguage, Parser};

use crate::highlight::HighlightConfiguration;
use crate::metadata::LanguageMetadata;
use crate::runtime::Runtime;
use crate::SyntaxError;

/// Supported language identifiers (kept for compatibility).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageId {
    Rust,
    Toml,
    PlainText,
}

impl LanguageId {
    /// Convert to the string identifier used in the runtime.
    pub fn as_str(&self) -> &'static str {
        match self {
            LanguageId::Rust => "rust",
            LanguageId::Toml => "toml",
            LanguageId::PlainText => "plaintext",
        }
    }

    /// Create from a string identifier.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "rust" => Some(LanguageId::Rust),
            "toml" => Some(LanguageId::Toml),
            "plaintext" => Some(LanguageId::PlainText),
            _ => None,
        }
    }

    /// Detect language from file extension using the metadata registry.
    /// This is a fallback; the main detection happens in `LanguageRegistry::detect_from_path`.
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => LanguageId::Rust,
            "toml" => LanguageId::Toml,
            _ => LanguageId::PlainText,
        }
    }

    /// Detect language from file path (fallback).
    pub fn from_path(path: &Path) -> Self {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(Self::from_extension)
            .unwrap_or(LanguageId::PlainText)
    }

    /// Get the Tree-sitter language for this language ID.
    /// This will attempt to load from the runtime shared library; if that fails,
    /// it falls back to the statically linked grammar (if the feature is enabled).
    pub fn tree_sitter_language(&self, runtime: &Runtime) -> Option<TsLanguage> {
        match self {
            LanguageId::Rust => {
                // First try runtime loading
                if let Ok(lang) = runtime_load_language("rust", runtime) {
                    Some(lang)
                } else {
                    #[cfg(feature = "rust")]
                    {
                        // Use the statically linked grammar function
                        Some(tree_sitter_rust::LANGUAGE())
                    }
                    #[cfg(not(feature = "rust"))]
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

    /// Get human-readable name for this language.
    pub fn name(&self) -> &'static str {
        match self {
            LanguageId::Rust => "Rust",
            LanguageId::Toml => "TOML",
            LanguageId::PlainText => "Plain Text",
        }
    }
}

/// Attempt to load a Tree-sitter language from a runtime shared library.
fn runtime_load_language(lang_id: &str, runtime: &Runtime) -> Result<TsLanguage, SyntaxError> {
    use libloading::{Library, Symbol};

    let lib_path = runtime.grammar_library_path(lang_id);
    if !lib_path.is_file() {
        return Err(SyntaxError::GrammarLoadError(format!(
            "Grammar library not found at {}",
            lib_path.display()
        )));
    }

    unsafe {
        let lib = Library::new(&lib_path)
            .map_err(|e| SyntaxError::GrammarLoadError(format!("Failed to load library: {}", e)))?;
        let symbol_name = format!("tree_sitter_{}", lang_id);
        let lang_sym: Symbol<unsafe extern "C" fn() -> TsLanguage> = lib
            .get(symbol_name.as_bytes())
            .map_err(|e| SyntaxError::GrammarLoadError(format!("Symbol lookup failed: {}", e)))?;
        Ok(lang_sym())
    }
}

/// Registry for managing language configurations, now backed by runtime metadata.
pub struct LanguageRegistry {
    /// Runtime environment for locating assets.
    runtime: Runtime,
    /// Metadata for all known languages.
    metadata: Vec<LanguageMetadata>,
    /// Cached highlight configurations.
    highlight_configs: HashMap<String, HighlightConfiguration>,
    /// Cached parsers for each language.
    parsers: HashMap<String, Parser>,
}

impl LanguageRegistry {
    /// Create a new language registry that loads metadata from the runtime.
    pub fn new() -> Self {
        let runtime = Runtime::new();
        let metadata = LanguageMetadata::load_all(&runtime)
            .unwrap_or_else(|_| vec![LanguageMetadata::rust(), LanguageMetadata::plaintext()]);

        Self {
            runtime,
            metadata,
            highlight_configs: HashMap::new(),
            parsers: HashMap::new(),
        }
    }

    /// Get metadata for a language identifier.
    pub fn metadata(&self, lang_id: &str) -> Option<&LanguageMetadata> {
        self.metadata.iter().find(|m| m.id == lang_id)
    }

    /// Detect language from a file path.
    pub fn detect_from_path(&self, path: &Path) -> (String, Option<&LanguageMetadata>) {
        // First try exact filename matches
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            for meta in &self.metadata {
                if meta.filenames.iter().any(|f| f == file_name) {
                    return (meta.id.clone(), Some(meta));
                }
            }
        }

        // Then try extensions
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            for meta in &self.metadata {
                if meta.extensions.iter().any(|e| e == ext) {
                    return (meta.id.clone(), Some(meta));
                }
            }
        }

        // Fallback to plaintext
        ("plaintext".to_string(), self.metadata("plaintext"))
    }

    /// Get or create a highlight configuration for a language.
    pub fn highlight_config(&mut self, lang_id: &str) -> Result<&HighlightConfiguration, SyntaxError> {
        use std::collections::hash_map::Entry;
        // Borrow only the fields we need before the mutable borrow of highlight_configs
        let (query, lang) = {
            let meta = self.metadata.iter().find(|m| m.id == lang_id)
                .ok_or_else(|| SyntaxError::LanguageNotSupported(lang_id.to_string()))?;
            let query = meta.load_highlights_query(&self.runtime)
                .unwrap_or_else(|_| String::new());
            let lang_enum = LanguageId::from_str(lang_id)
                .unwrap_or(LanguageId::PlainText);
            let lang = lang_enum.tree_sitter_language(&self.runtime)
                .ok_or_else(|| SyntaxError::LanguageNotSupported(lang_id.to_string()))?;
            (query, lang)
        };
        match self.highlight_configs.entry(lang_id.to_string()) {
            Entry::Occupied(occ) => Ok(occ.into_mut()),
            Entry::Vacant(vac) => {
                let config = HighlightConfiguration::new(&lang, &query, "", "")
                    .map_err(|e| SyntaxError::QueryError(e.to_string()))?;
                Ok(vac.insert(config))
            }
        }
    }

    /// Get or create a parser for a language.
    pub fn parser(&mut self, lang_id: &str) -> Result<&mut Parser, SyntaxError> {
        use std::collections::hash_map::Entry;
        // Borrow only the runtime before the mutable borrow of parsers
        let lang = {
            let lang_enum = LanguageId::from_str(lang_id)
                .unwrap_or(LanguageId::PlainText);
            lang_enum.tree_sitter_language(&self.runtime)
                .ok_or_else(|| SyntaxError::LanguageNotSupported(lang_id.to_string()))?
        };
        match self.parsers.entry(lang_id.to_string()) {
            Entry::Occupied(occ) => Ok(occ.into_mut()),
            Entry::Vacant(vac) => {
                let mut parser = Parser::new();
                parser.set_language(&lang)
                    .map_err(|e| SyntaxError::ParserError(e.to_string()))?;
                Ok(vac.insert(parser))
            }
        }
    }

    /// Get the Tree-sitter language for a language identifier.
    fn language(&self, lang_id: &str) -> Result<TsLanguage, SyntaxError> {
        let lang_enum = LanguageId::from_str(lang_id)
            .unwrap_or(LanguageId::PlainText);
        lang_enum.tree_sitter_language(&self.runtime)
            .ok_or_else(|| SyntaxError::LanguageNotSupported(lang_id.to_string()))
    }

    /// Check if a language is supported (has a grammar and queries).
    pub fn is_supported(&self, lang_id: &str) -> bool {
        self.metadata(lang_id).map(|m| m.is_supported()).unwrap_or(false)
    }

    /// Get all supported language IDs.
    pub fn supported_languages(&self) -> Vec<String> {
        self.metadata.iter()
            .filter(|m| m.is_supported())
            .map(|m| m.id.clone())
            .collect()
    }
}

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}
