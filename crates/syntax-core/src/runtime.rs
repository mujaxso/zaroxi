//! Runtime path resolution for Tree-sitter grammars and queries.

use std::env;
use std::path::PathBuf;

/// Runtime environment for locating Tree-sitter assets.
#[derive(Debug, Clone)]
pub struct Runtime {
    /// Root directory of the Tree-sitter runtime (e.g., .../runtime/treesitter).
    root: PathBuf,
}

impl Runtime {
    /// Attempt to locate the runtime directory.
    ///
    /// Searches in the following order:
    /// 1. `NEOTE_RUNTIME` environment variable.
    /// 2. A directory `runtime/treesitter` sibling to the current executable.
    /// 3. The current working directory `./runtime/treesitter`.
    ///
    /// Returns a `Runtime` even if the directory does not exist; operations will
    /// fail later with appropriate errors.
    pub fn new() -> Self {
        let root = Self::locate_root().unwrap_or_else(|| {
            // Fallback to a placeholder path; errors will be reported when trying to load.
            PathBuf::from("./runtime/treesitter")
        });
        Self { root }
    }

    fn locate_root() -> Option<PathBuf> {
        // 1. Environment variable
        if let Ok(env_path) = env::var("NEOTE_RUNTIME") {
            let p = PathBuf::from(env_path);
            if p.is_dir() {
                return Some(p);
            }
        }

        // 2. Sibling to executable
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let candidate = exe_dir.join("../runtime/treesitter").canonicalize().ok();
                if candidate.as_ref().and_then(|p| p.is_dir().then(|| ())).is_some() {
                    return candidate;
                }
            }
        }

        // 3. Current working directory
        let cwd = env::current_dir().ok()?;
        let candidate = cwd.join("runtime/treesitter");
        if candidate.is_dir() {
            return Some(candidate);
        }

        None
    }

    /// Get the path to the directory containing grammar shared libraries
    /// for the current platform and architecture.
    pub fn grammar_dir(&self) -> PathBuf {
        let target = env::consts::ARCH;
        let os = env::consts::OS;

        // Map OS and architecture to the subdirectory name used in the runtime layout.
        // This matches the directory naming scheme described in the architecture.
        let subdir = format!("{}-{}", os, target);
        self.root.join("grammars").join(subdir)
    }

    /// Get the path to the language metadata and queries directory for a language.
    pub fn language_dir(&self, language_id: &str) -> PathBuf {
        self.root.join("languages").join(language_id)
    }

    /// Construct the full path to a grammar shared library.
    ///
    /// The library filename is expected to follow the pattern
    /// `libtree-sitter-{language}.{ext}` on Unix and `tree-sitter-{language}.dll` on Windows.
    pub fn grammar_library_path(&self, language_id: &str) -> PathBuf {
        let mut lib_name = if cfg!(windows) {
            format!("tree-sitter-{}", language_id)
        } else {
            format!("libtree-sitter-{}", language_id)
        };
        lib_name.push_str(env::consts::DLL_EXTENSION);
        self.grammar_dir().join(lib_name)
    }

    /// Load a Tree-sitter language from a shared library in the runtime directory.
    ///
    /// This uses `libloading` to dynamically load the grammar library and retrieve
    /// the `tree_sitter_{language}` function.
    #[cfg(feature = "dynamic-loading")]
    pub fn load_language(&self, language_id: &str) -> Result<tree_sitter::Language, String> {
        use libloading::{Library, Symbol};
        
        let library_path = self.grammar_library_path(language_id);
        if !library_path.exists() {
            return Err(format!(
                "Grammar library not found at {}",
                library_path.display()
            ));
        }

        // Safety: We're loading a shared library that we expect to be a valid
        // Tree-sitter grammar. The library should export a function named
        // `tree_sitter_{language}`.
        unsafe {
            let lib = Library::new(&library_path)
                .map_err(|e| format!("Failed to load library {}: {}", library_path.display(), e))?;
            
            let symbol_name = format!("tree_sitter_{}", language_id);
            let language_fn: Symbol<unsafe extern "C" fn() -> tree_sitter::Language> = lib
                .get(symbol_name.as_bytes())
                .map_err(|e| format!("Failed to get symbol {}: {}", symbol_name, e))?;
            
            // Call the function to get the language before we forget the library
            let language = language_fn();
            
            // The library must not be unloaded while the language is in use.
            // We leak the library handle to keep it loaded for the lifetime of the program.
            // The language_fn symbol is no longer needed after we've called it.
            std::mem::forget(lib);
            
            Ok(language)
        }
    }

    #[cfg(not(feature = "dynamic-loading"))]
    pub fn load_language(&self, language_id: &str) -> Result<tree_sitter::Language, String> {
        Err(format!(
            "Dynamic loading not enabled (feature 'dynamic-loading' required) for language {}",
            language_id
        ))
    }

    /// Get a reference to the runtime root directory.
    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    /// Check whether the runtime root directory exists.
    pub fn exists(&self) -> bool {
        self.root.is_dir()
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
