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
    /// 1. `QYZER_STUDIO_RUNTIME` environment variable (for compatibility)
    /// 2. `NEOTE_RUNTIME` environment variable.
    /// 3. A directory `runtime/treesitter` sibling to the current executable.
    /// 4. The current working directory `./runtime/treesitter`.
    /// 5. Bundled resources directory for packaged applications.
    ///
    /// Returns a `Runtime` even if the directory does not exist; operations will
    /// fail later with appropriate errors.
    pub fn new() -> Self {
        let root = Self::locate_root().unwrap_or_else(|| {
            // Fallback to a placeholder path; errors will be reported when trying to load.
            PathBuf::from("./runtime/treesitter")
        });
        let runtime = Self { root };
        
        // Try to fix nested structure if it exists
        let _ = runtime.fix_nested_structure();
        
        runtime
    }

    fn locate_root() -> Option<PathBuf> {
        // 1. ZAROXI_STUDIO_RUNTIME environment variable (new)
        if let Ok(env_path) = env::var("ZAROXI_STUDIO_RUNTIME") {
            let p = PathBuf::from(env_path);
            if p.is_dir() {
                return Some(p);
            }
        }

        // 2. QYZER_STUDIO_RUNTIME environment variable (for backward compatibility)
        if let Ok(env_path) = env::var("QYZER_STUDIO_RUNTIME") {
            let p = PathBuf::from(env_path);
            if p.is_dir() {
                return Some(p);
            }
        }

        // 3. NEOTE_RUNTIME environment variable.
        if let Ok(env_path) = env::var("NEOTE_RUNTIME") {
            let p = PathBuf::from(env_path);
            if p.is_dir() {
                return Some(p);
            }
        }

        // 3. Check for the correct structure: look for runtime/treesitter directory
        // First, try current working directory
        if let Ok(cwd) = env::current_dir() {
            // Check for runtime/treesitter directly
            let candidate = cwd.join("runtime/treesitter");
            if candidate.is_dir() {
                return Some(candidate);
            }
            
            // Check for nested runtime/treesitter/runtime/treesitter (incorrect structure)
            let nested_candidate = candidate.join("runtime/treesitter");
            if nested_candidate.is_dir() {
                // This is the incorrect nested structure, use the parent instead
                eprintln!("WARNING: Found nested runtime directory at {:?}. Using parent directory {:?} instead.", 
                         nested_candidate, candidate);
                return Some(candidate);
            }
            
            // Try to find the runtime directory by walking up
            let mut current = cwd.clone();
            while current.parent().is_some() {
                let candidate = current.join("runtime/treesitter");
                if candidate.is_dir() {
                    return Some(candidate);
                }
                current = current.parent().unwrap().to_path_buf();
            }
        }

        // 4. Sibling to executable (development mode)
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // Try development layout: ../runtime/treesitter
                let candidate = exe_dir.join("../runtime/treesitter");
                if candidate.is_dir() {
                    return Some(candidate);
                }
                
                // Try walking up from executable
                let mut current = exe_dir.to_path_buf();
                while current.parent().is_some() {
                    let candidate = current.join("runtime/treesitter");
                    if candidate.is_dir() {
                        return Some(candidate);
                    }
                    current = current.parent().unwrap().to_path_buf();
                }
            }
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
        let prefix = if cfg!(windows) { "" } else { "lib" };
        let extension = if cfg!(windows) {
            ".dll"
        } else if cfg!(target_os = "macos") {
            ".dylib"
        } else {
            ".so"
        };
        // Some language IDs use underscores but the library uses hyphens
        // For example: "c_sharp" -> "c-sharp" in library name
        let lib_name = match language_id {
            "c_sharp" => "c-sharp",
            _ => language_id,
        };
        let lib_name = format!("{}tree-sitter-{}{}", prefix, lib_name, extension);
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
                "Grammar library not found at {}\nRun: cargo run --bin download_grammars -- install {}",
                library_path.display(),
                language_id
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
    
    /// Fix nested runtime directory structure if found
    pub fn fix_nested_structure(&self) -> std::io::Result<()> {
        let nested_path = self.root.join("runtime/treesitter");
        if nested_path.is_dir() {
            eprintln!("Found nested runtime directory at {:?}. Attempting to fix...", nested_path);
            
            // Move contents from nested to parent
            let grammars_nested = nested_path.join("grammars");
            let languages_nested = nested_path.join("languages");
            
            let grammars_target = self.root.join("grammars");
            let languages_target = self.root.join("languages");
            
            // Move grammars if they exist
            if grammars_nested.exists() {
                if !grammars_target.exists() {
                    std::fs::create_dir_all(&grammars_target)?;
                }
                move_dir_contents(&grammars_nested, &grammars_target)?;
            }
            
            // Move languages if they exist
            if languages_nested.exists() {
                if !languages_target.exists() {
                    std::fs::create_dir_all(&languages_target)?;
                }
                move_dir_contents(&languages_nested, &languages_target)?;
            }
            
            // Try to remove the now-empty nested directory
            let _ = std::fs::remove_dir_all(&nested_path);
            eprintln!("Fixed nested runtime structure.");
        }
        Ok(())
    }
}

/// Helper to move directory contents
fn move_dir_contents(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    if !dst.exists() {
        std::fs::create_dir_all(dst)?;
    }
    
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            move_dir_contents(&src_path, &dst_path)?;
            // Try to remove the now-empty source directory
            let _ = std::fs::remove_dir(&src_path);
        } else {
            std::fs::rename(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
