//! Dynamic loading of Tree-sitter grammars from bundled runtime.

use std::collections::HashMap;
use std::sync::{OnceLock, Mutex};
use tree_sitter;
use libloading::{Library, Symbol};

use crate::runtime::Runtime;
use crate::grammar_registry::GrammarRegistry;

/// Global cache for loaded languages
static LANGUAGE_CACHE: OnceLock<Mutex<HashMap<String, Option<tree_sitter::Language>>>> = OnceLock::new();

/// Load a Tree-sitter language dynamically from the runtime directory
pub fn load_language(language_id: &str) -> Option<tree_sitter::Language> {
    let cache = LANGUAGE_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    
    // Check cache first
    {
        let cache_guard = cache.lock().unwrap();
        if let Some(cached) = cache_guard.get(language_id) {
            return cached.clone();
        }
    }
    
    // Not in cache, try to load
    let result = load_language_impl(language_id);
    
    // Store in cache
    let mut cache_guard = cache.lock().unwrap();
    cache_guard.insert(language_id.to_string(), result.clone());
    
    result
}

fn load_language_impl(language_id: &str) -> Option<tree_sitter::Language> {
    // Check if the language is in the registry
    let registry = GrammarRegistry::global();
    if !registry.contains_language(language_id) {
        eprintln!("DEBUG: Language {} not in registry", language_id);
        return None;
    }
    
    let runtime = Runtime::new();
    
    // Check if the grammar library exists
    let library_path = runtime.grammar_library_path(language_id);
    if !library_path.exists() {
        eprintln!("DEBUG: Library path doesn't exist: {}", library_path.display());
        return None;
    }
    
    println!("DEBUG: Loading language {} from {}", language_id, library_path.display());
    
    // Load the library
    unsafe {
        match Library::new(&library_path) {
            Ok(lib) => {
                // Get the language function
                let symbol_name = format!("tree_sitter_{}", language_id);
                println!("DEBUG: Looking for symbol: {}", symbol_name);
                
                let language_fn: Result<Symbol<unsafe extern "C" fn() -> tree_sitter::Language>, _> = 
                    lib.get(symbol_name.as_bytes());
                
                match language_fn {
                    Ok(func) => {
                        println!("DEBUG: Found symbol for {}", language_id);
                        let language = func();
                        // Leak the library to keep it loaded
                        std::mem::forget(lib);
                        // Print some info about the language
                        println!("DEBUG: Language {} loaded successfully, node count: {}", language_id, language.node_kind_count());
                        // Print node types for debugging
                        if language_id == "markdown" {
                            for i in 0..language.node_kind_count() {
                                let kind = language.node_kind_for_id(i as u16);
                                if let Some(kind) = kind {
                                    println!("DEBUG: Node type {}: {}", i, kind);
                                }
                            }
                        }
                        Some(language)
                    }
                    Err(e) => {
                        eprintln!("DEBUG: Failed to get symbol {}: {}", symbol_name, e);
                        // Try alternative symbol names
                        // For markdown, try tree_sitter_markdown_inline
                        if language_id == "markdown" {
                            println!("DEBUG: Trying alternative symbol for markdown...");
                            let alt_symbol_name = "tree_sitter_markdown_inline";
                            match lib.get::<unsafe extern "C" fn() -> tree_sitter::Language>(alt_symbol_name.as_bytes()) {
                                Ok(func) => {
                                    println!("DEBUG: Found alternative symbol {}", alt_symbol_name);
                                    let language = func();
                                    std::mem::forget(lib);
                                    println!("DEBUG: Language {} loaded via alternative symbol, node count: {}", language_id, language.node_kind_count());
                                    return Some(language);
                                }
                                Err(e) => {
                                    eprintln!("DEBUG: Failed to get alternative symbol {}: {}", alt_symbol_name, e);
                                }
                            }
                            
                            // Try another alternative: tree_sitter_markdown
                            println!("DEBUG: Trying another alternative symbol for markdown...");
                            let alt_symbol_name2 = "tree_sitter_markdown";
                            match lib.get::<unsafe extern "C" fn() -> tree_sitter::Language>(alt_symbol_name2.as_bytes()) {
                                Ok(func) => {
                                    println!("DEBUG: Found alternative symbol {}", alt_symbol_name2);
                                    let language = func();
                                    std::mem::forget(lib);
                                    println!("DEBUG: Language {} loaded via alternative symbol, node count: {}", language_id, language.node_kind_count());
                                    return Some(language);
                                }
                                Err(e) => {
                                    eprintln!("DEBUG: Failed to get alternative symbol {}: {}", alt_symbol_name2, e);
                                }
                            }
                        }
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("DEBUG: Failed to load library {}: {}", library_path.display(), e);
                None
            }
        }
    }
}

/// Preload all available grammars to warm up the cache
pub fn preload_available_grammars() {
    let registry = GrammarRegistry::global();
    for language_id in registry.language_ids() {
        // Try to load each language
        load_language(language_id);
    }
}

/// Check if a grammar is available for loading
pub fn is_grammar_available(language_id: &str) -> bool {
    let runtime = Runtime::new();
    let library_path = runtime.grammar_library_path(language_id);
    library_path.exists()
}

/// Dynamic grammar loader struct (for re-export)
pub struct DynamicGrammarLoader;

impl DynamicGrammarLoader {
    /// Load a language
    pub fn load(language_id: &str) -> Option<tree_sitter::Language> {
        load_language(language_id)
    }
    
    /// Check if a grammar is available
    pub fn is_available(language_id: &str) -> bool {
        is_grammar_available(language_id)
    }
    
    /// Preload all grammars
    pub fn preload_all() {
        preload_available_grammars();
    }
}
