//! Cache for compiled Tree-sitter queries loaded from bundled files.

use std::collections::HashMap;
use std::sync::{OnceLock, Mutex};
use tree_sitter::Query;

use crate::runtime::Runtime;
use crate::grammar_registry::GrammarRegistry;

/// Global cache for compiled queries
static QUERY_CACHE: OnceLock<Mutex<HashMap<String, Result<&'static Query, String>>>> = OnceLock::new();

/// Get a compiled query for a language
pub fn get_query(language_id: &str, query_type: &str) -> Option<&'static Query> {
    let cache_key = format!("{}:{}", language_id, query_type);
    let cache = QUERY_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    
    // Check cache first
    {
        let cache_guard = cache.lock().unwrap();
        if let Some(result) = cache_guard.get(&cache_key) {
            return match result {
                Ok(query) => Some(*query),
                Err(_) => None,
            };
        }
    }
    
    // Not in cache, load from file
    let result = load_query_from_file(language_id, query_type);
    
    match result {
        Ok(query) => {
            // Box and leak the query to get a static reference
            let query_ptr = Box::leak(Box::new(query));
            
            // Store in cache
            let mut cache_guard = cache.lock().unwrap();
            cache_guard.insert(cache_key, Ok(query_ptr));
            
            Some(query_ptr)
        }
        Err(e) => {
            // Store error in cache
            let mut cache_guard = cache.lock().unwrap();
            cache_guard.insert(cache_key, Err(e));
            None
        }
    }
}

fn load_query_from_file(language_id: &str, query_type: &str) -> Result<Query, String> {
    // Get the language
    let language = crate::dynamic_loader::load_language(language_id)
        .ok_or_else(|| format!("Language '{}' not loaded", language_id))?;
    
    // Get query file path
    let runtime = Runtime::new();
    let query_dir = runtime.language_dir(language_id).join("queries");
    let query_path = query_dir.join(format!("{}.scm", query_type));
    
    // Read query file
    let query_text = std::fs::read_to_string(&query_path)
        .map_err(|e| format!("Failed to read query file {}: {}", query_path.display(), e))?;
    
    // Compile query
    Query::new(&language, &query_text)
        .map_err(|e| format!("Failed to compile query for {}: {}", language_id, e))
}

/// Preload common queries for all available languages
pub fn preload_queries() {
    let registry = GrammarRegistry::global();
    
    for language_id in registry.language_ids() {
        // Try to load highlights query
        let _ = get_query(language_id, "highlights");
        
        // Try to load injections query if it exists
        let _ = get_query(language_id, "injections");
    }
}

/// Query cache struct (for re-export)
pub struct QueryCache;

impl QueryCache {
    /// Get a query
    pub fn get(language_id: &str, query_type: &str) -> Option<&'static Query> {
        get_query(language_id, query_type)
    }
    
    /// Preload queries
    pub fn preload() {
        preload_queries();
    }
}
