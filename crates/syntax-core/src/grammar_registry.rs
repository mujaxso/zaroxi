//! Registry of available Tree-sitter grammars and their download/compile instructions.

use std::collections::HashMap;
use std::sync::OnceLock;
use serde::{Deserialize, Serialize};

/// Information needed to download and compile a Tree-sitter grammar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarInfo {
    /// Language identifier (e.g., "markdown", "rust", "python")
    pub language_id: String,
    /// Human-readable name
    pub name: String,
    /// File extensions (without dot)
    pub extensions: Vec<String>,
    /// Exact filenames that trigger this language
    pub filenames: Vec<String>,
    /// GitHub repository URL
    pub repo_url: String,
    /// Repository revision/tag (e.g., "v0.20.0")
    pub revision: String,
    /// Optional subdirectory within the repo (for mono-repos)
    pub subdirectory: Option<String>,
    /// Source files needed for compilation
    pub source_files: Vec<String>,
    /// Query files to copy
    pub query_files: Vec<String>,
    /// Whether this grammar has an external scanner
    pub has_scanner: bool,
    /// Scanner language (C or C++)
    pub scanner_lang: Option<String>,
}

/// Global grammar registry
pub struct GrammarRegistry {
    languages: HashMap<&'static str, GrammarInfo>,
}

impl GrammarRegistry {
    /// Get the global grammar registry
    pub fn global() -> &'static Self {
        static REGISTRY: OnceLock<GrammarRegistry> = OnceLock::new();
        REGISTRY.get_or_init(|| {
            let mut registry = GrammarRegistry {
                languages: HashMap::new(),
            };
            registry.load_defaults();
            registry
        })
    }
    
    fn load_defaults(&mut self) {
        // Rust
        self.add_language(GrammarInfo {
            language_id: "rust".to_string(),
            name: "Rust".to_string(),
            extensions: vec!["rs".to_string()],
            filenames: vec![],
            repo_url: "https://github.com/tree-sitter/tree-sitter-rust".to_string(),
            revision: "v0.24.0".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string(), "src/scanner.c".to_string()],
            query_files: vec!["highlights.scm".to_string(), "injections.scm".to_string(), "locals.scm".to_string()],
            has_scanner: true,
            scanner_lang: Some("c".to_string()),
        });
        
        // TOML
        self.add_language(GrammarInfo {
            language_id: "toml".to_string(),
            name: "TOML".to_string(),
            extensions: vec!["toml".to_string()],
            filenames: vec!["Cargo.toml".to_string(), "rust-toolchain.toml".to_string()],
            repo_url: "https://github.com/tree-sitter/tree-sitter-toml".to_string(),
            revision: "v0.5.0".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string()],
            query_files: vec!["highlights.scm".to_string()],
            has_scanner: false,
            scanner_lang: None,
        });
        
        // Markdown
        self.add_language(GrammarInfo {
            language_id: "markdown".to_string(),
            name: "Markdown".to_string(),
            extensions: vec!["md".to_string(), "markdown".to_string()],
            filenames: vec!["README.md".to_string()],
            repo_url: "https://github.com/tree-sitter/tree-sitter-markdown".to_string(),
            revision: "v0.3.0".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string(), "src/scanner.c".to_string()],
            query_files: vec!["highlights.scm".to_string(), "injections.scm".to_string()],
            has_scanner: true,
            scanner_lang: Some("c".to_string()),
        });
        
        // JavaScript
        self.add_language(GrammarInfo {
            language_id: "javascript".to_string(),
            name: "JavaScript".to_string(),
            extensions: vec!["js".to_string(), "jsx".to_string()],
            filenames: vec![],
            repo_url: "https://github.com/tree-sitter/tree-sitter-javascript".to_string(),
            revision: "v0.20.0".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string(), "src/scanner.c".to_string()],
            query_files: vec!["highlights.scm".to_string(), "injections.scm".to_string(), "locals.scm".to_string()],
            has_scanner: true,
            scanner_lang: Some("c".to_string()),
        });
        
        // Python
        self.add_language(GrammarInfo {
            language_id: "python".to_string(),
            name: "Python".to_string(),
            extensions: vec!["py".to_string()],
            filenames: vec![],
            repo_url: "https://github.com/tree-sitter/tree-sitter-python".to_string(),
            revision: "v0.21.0".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string(), "src/scanner.c".to_string()],
            query_files: vec!["highlights.scm".to_string(), "injections.scm".to_string(), "locals.scm".to_string()],
            has_scanner: true,
            scanner_lang: Some("c".to_string()),
        });
        
        // JSON
        self.add_language(GrammarInfo {
            language_id: "json".to_string(),
            name: "JSON".to_string(),
            extensions: vec!["json".to_string()],
            filenames: vec![],
            repo_url: "https://github.com/tree-sitter/tree-sitter-json".to_string(),
            revision: "v0.20.0".to_string(),
            subdirectory: None,
            source_files: vec!["src/parser.c".to_string()],
            query_files: vec!["highlights.scm".to_string()],
            has_scanner: false,
            scanner_lang: None,
        });
    }
    
    fn add_language(&mut self, info: GrammarInfo) {
        self.languages.insert(Box::leak(info.language_id.clone().into_boxed_str()), info);
    }
    
    /// Get information for a specific language
    pub fn get(&self, language_id: &str) -> Option<&GrammarInfo> {
        self.languages.get(language_id)
    }
    
    /// Check if a language is in the registry
    pub fn contains_language(&self, language_id: &str) -> bool {
        self.languages.contains_key(language_id)
    }
    
    /// Get all language IDs
    pub fn language_ids(&self) -> Vec<&str> {
        self.languages.keys().copied().collect()
    }
    
    /// Get all languages
    pub fn languages(&self) -> &HashMap<&'static str, GrammarInfo> {
        &self.languages
    }
}

/// Get the grammar info for a language, if available
pub fn for_language(language_id: &str) -> Option<GrammarInfo> {
    GrammarRegistry::global().get(language_id).cloned()
}

/// Get all available language IDs
pub fn available_languages() -> Vec<String> {
    GrammarRegistry::global().language_ids().iter().map(|s| s.to_string()).collect()
}
