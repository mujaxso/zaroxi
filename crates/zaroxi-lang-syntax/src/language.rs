//! Language identification and grammar loading.

use std::collections::HashMap;
use std::path::Path;
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
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_lowercase();

        // Try to match against dynamic language registry first
        // First check by filename
        if let Some(lang_id) = Self::from_filename_dynamic(&name) {
            return LanguageId::Dynamic(lang_id);
        }

        // Then check by extension
        if let Some(lang_id) = Self::from_extension_dynamic(ext) {
            return LanguageId::Dynamic(lang_id);
        }

        // Fall back to built-in language detection for common cases
        match ext {
            "rs" => return LanguageId::Rust,
            "toml" => return LanguageId::Toml,
            "md" | "markdown" => return LanguageId::Markdown,
            "js" | "jsx" => return LanguageId::Dynamic("javascript"),
            "ts" => return LanguageId::Dynamic("typescript"),
            "tsx" => return LanguageId::Dynamic("tsx"),
            "py" => return LanguageId::Dynamic("python"),
            "json" => return LanguageId::Dynamic("json"),
            "css" => return LanguageId::Dynamic("css"),
            "html" | "htm" => return LanguageId::Dynamic("html"),
            "go" => return LanguageId::Dynamic("go"),
            "java" => return LanguageId::Dynamic("java"),
            "sh" | "bash" => return LanguageId::Dynamic("bash"),
            "c" | "h" => return LanguageId::Dynamic("c"),
            "cpp" | "cc" | "cxx" | "hpp" | "hh" | "hxx" => return LanguageId::Dynamic("cpp"),
            "cs" => return LanguageId::Dynamic("c_sharp"),
            "rb" => return LanguageId::Dynamic("ruby"),
            "lua" => return LanguageId::Dynamic("lua"),
            "yaml" | "yml" => return LanguageId::Dynamic("yaml"),
            "zig" => return LanguageId::Dynamic("zig"),
            "cmake" => return LanguageId::Dynamic("cmake"),
            "ex" | "exs" => return LanguageId::Dynamic("elixir"),
            "nix" => return LanguageId::Dynamic("nix"),
            _ => {}
        }

        // Check for specific filenames
        match name.as_str() {
            "cargo.toml"
            | "rust-toolchain.toml"
            | "clippy.toml"
            | "rustfmt.toml"
            | ".clippy.toml"
            | ".rustfmt.toml"
            | "pyproject.toml"
            | "taplo.toml" => {
                return LanguageId::Toml;
            }
            "dockerfile" => return LanguageId::Dynamic("dockerfile"),
            "cmakelists.txt" => return LanguageId::Dynamic("cmake"),
            "gemfile" | "rakefile" => return LanguageId::Dynamic("ruby"),
            _ => {}
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
        eprintln!("DEBUG: tree_sitter_language called for {:?}", self);
        let result = match self {
            LanguageId::Rust => crate::dynamic_loader::load_language("rust"),
            LanguageId::Toml => crate::dynamic_loader::load_language("toml"),
            LanguageId::Markdown => crate::dynamic_loader::load_language("markdown"),
            LanguageId::PlainText => None,
            LanguageId::Dynamic(id) => crate::dynamic_loader::load_language(id),
        };
        if result.is_none() {
            eprintln!("DEBUG: tree_sitter_language returned None for {:?}", self);
        } else {
            eprintln!("DEBUG: tree_sitter_language returned Some for {:?}", self);
        }
        result
    }
}
