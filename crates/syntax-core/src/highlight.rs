//! Syntax highlighting using Tree-sitter queries.

use crate::error::SyntaxError;
use crate::language::LanguageId;
use tree_sitter::{Query, QueryCursor, Tree};

/// A highlight span in the document
#[derive(Debug, Clone)]
pub struct HighlightSpan {
    /// Start byte offset
    pub start: usize,
    /// End byte offset
    pub end: usize,
    /// Highlight type (maps to theme token categories)
    pub highlight: Highlight,
}

/// Highlight types (maps to Tree-sitter capture names)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Highlight {
    Comment,
    String,
    Keyword,
    Function,
    Variable,
    Type,
    Constant,
    Attribute,
    Operator,
    Number,
    Property,
    Namespace,
    Plain,
}

/// Highlight a syntax tree for a given language.
pub fn highlight(
    language: LanguageId,
    source: &str,
    tree: &Tree,
) -> Result<Vec<HighlightSpan>, SyntaxError> {
    match language {
        LanguageId::Rust => {
            eprintln!("DEBUG: Highlighting Rust code");
            highlight_with_query(language, source, tree)
        }
        #[cfg(feature = "toml")]
        LanguageId::Toml => highlight_with_query(language, source, tree),
        #[cfg(not(feature = "toml"))]
        LanguageId::Toml => Ok(Vec::new()),
        #[cfg(feature = "markdown")]
        LanguageId::Markdown => highlight_with_query(language, source, tree),
        #[cfg(not(feature = "markdown"))]
        LanguageId::Markdown => Ok(Vec::new()),
        LanguageId::PlainText => Ok(Vec::new()),
        LanguageId::Dynamic(_) => highlight_with_query(language, source, tree),
    }
}

fn highlight_with_query(
    language: LanguageId,
    source: &str,
    tree: &Tree,
) -> Result<Vec<HighlightSpan>, SyntaxError> {
    let query_str = get_query_for_language(language)?;
    let ts_lang = language
        .tree_sitter_language()
        .ok_or_else(|| SyntaxError::LanguageNotSupported(language.as_str().to_string()))?;

    // Try to compile the query, but if it fails, try to fix common issues
    let query_result = Query::new(ts_lang, query_str);
    
    let query = match query_result {
        Ok(q) => q,
        Err(e) => {
            // Log the error for debugging
            eprintln!("DEBUG: Tree-sitter query error for {}: {}", language.as_str(), e);
            
            // Try to create a minimal fallback query
            eprintln!("DEBUG: Creating fallback query for {}", language.as_str());
            let fallback_query = create_fallback_query(language);
            match Query::new(ts_lang, fallback_query) {
                Ok(q) => q,
                Err(e2) => {
                    eprintln!("DEBUG: Fallback query also failed: {}", e2);
                    return Ok(Vec::new());
                }
            }
        }
    };

    let mut cursor = QueryCursor::new();
    let root_node = tree.root_node();
    let mut spans = Vec::new();

    for match_ in cursor.matches(&query, root_node, source.as_bytes()) {
        for capture in match_.captures {
            let node = capture.node;
            let start = node.start_byte();
            let end = node.end_byte();
            let capture_name = &query.capture_names()[capture.index as usize];
            let highlight = map_capture_name(capture_name);
            spans.push(HighlightSpan {
                start,
                end,
                highlight,
            });
        }
    }

    // Sort spans by start position
    spans.sort_by_key(|span| span.start);
    
    Ok(spans)
}

fn create_fallback_query(language: LanguageId) -> &'static str {
    match language {
        LanguageId::Rust => {
            r#"
(comment) @comment
(string_literal) @string
(raw_string_literal) @string
(line_comment) @comment
(block_comment) @comment
(identifier) @variable
(type_identifier) @type
(primitive_type) @type
(field_identifier) @property
"# 
        }
        LanguageId::Toml => {
            r#"
(comment) @comment
(string) @string
(boolean) @constant
(integer) @number
(float) @number
"#
        }
        _ => {
            r#"
(comment) @comment
(string) @string
"#
        }
    }
}

pub fn map_capture_name(name: &str) -> Highlight {
    match name {
        // Programming language captures
        "comment" => Highlight::Comment,
        "string" => Highlight::String,
        "string.escape" => Highlight::String,
        "escape" => Highlight::String,
        "string.special" => Highlight::String,
        "keyword" => Highlight::Keyword,
        "function" | "function.call" | "function.method" => Highlight::Function,
        "function.macro" | "macro" => Highlight::Function, // Macros use function color
        "variable" | "variable.parameter" => Highlight::Variable,
        "variable.builtin" => Highlight::Type, // Built-in variables like 'self' use type color (amber in dark theme)
        "type" | "type.builtin" => Highlight::Type,
        "constant" | "constant.builtin" => Highlight::Constant,
        "attribute" => Highlight::Attribute,
        "operator" => Highlight::Operator,
        "punctuation.bracket" => Highlight::Operator,
        "punctuation.delimiter" => Highlight::Operator,
        "number" => Highlight::Number,
        "boolean" => Highlight::Constant,
        "property" => Highlight::Property,
        "namespace" => Highlight::Namespace,
        "constructor" => Highlight::Type,
        "label" => Highlight::Variable,
        "mutable_specifier" => Highlight::Keyword,
        "lifetime" => Highlight::Type,  // Lifetimes use type color
        
        // Markdown-specific captures (based on tree-sitter-markdown grammar)
        // These may vary between versions
        "heading" => Highlight::Type,
        "heading.1" => Highlight::Type,
        "heading.2" => Highlight::Type,
        "heading.3" => Highlight::Type,
        "heading.4" => Highlight::Type,
        "heading.5" => Highlight::Type,
        "heading.6" => Highlight::Type,
        "atx_heading" => Highlight::Type,
        "setext_heading" => Highlight::Type,
        "emphasis" => Highlight::Comment,
        "strong_emphasis" => Highlight::Keyword,
        "strong" => Highlight::Keyword,
        "link" => Highlight::Variable,
        "link_text" => Highlight::Variable,
        "link_destination" => Highlight::String,
        "link_url" => Highlight::String,
        "link_title" => Highlight::String,
        "inline_code_span" => Highlight::Constant,
        "inline_code" => Highlight::Constant,
        "code_fence" => Highlight::Property,
        "block_quote" => Highlight::Comment,
        "blockquote" => Highlight::Comment,
        "list" => Highlight::Property,
        "list_item" => Highlight::Property,
        "thematic_break" => Highlight::Operator,
        "html_block" => Highlight::Attribute,
        "html_inline" => Highlight::Attribute,
        "table" => Highlight::Property,
        "table_header" => Highlight::Type,
        "table_row" => Highlight::Property,
        "table_cell" => Highlight::Plain,
        _ => Highlight::Plain,
    }
}

pub fn get_query_for_language(language: LanguageId) -> Result<&'static str, SyntaxError> {
    match language {
        LanguageId::Rust => {
            eprintln!("DEBUG: Getting query for Rust");
            // Try to load the query from cache
            if let Some(_) = crate::query_cache::get_query("rust", "highlights") {
                eprintln!("DEBUG: Found Rust query in cache");
            } else {
                eprintln!("DEBUG: Rust query not in cache");
            }
            
            if crate::query_cache::get_query("rust", "highlights").is_some() {
                // Store the query source in a static string using OnceLock
                use std::sync::OnceLock;
                static RUST_QUERY: OnceLock<Option<&'static str>> = OnceLock::new();
                
                let query_str = RUST_QUERY.get_or_init(|| {
                    // Load query text from file
                    let runtime = crate::runtime::Runtime::new();
                    let query_path = runtime.language_dir("rust").join("queries/highlights.scm");
                    eprintln!("DEBUG: Looking for query at: {}", query_path.display());
                    match std::fs::read_to_string(&query_path) {
                        Ok(query_text) => {
                            eprintln!("DEBUG: Found Rust query file");
                            Some(Box::leak(query_text.into_boxed_str()))
                        }
                        Err(e) => {
                            eprintln!("DEBUG: Failed to read Rust query file: {}", e);
                            None
                        }
                    }
                });
                
                match query_str {
                    Some(str) => {
                        eprintln!("DEBUG: Returning Rust query");
                        Ok(*str)
                    }
                    None => {
                        eprintln!("DEBUG: No Rust query available");
                        Err(SyntaxError::LanguageNotSupported(
                            "rust grammar not available".to_string(),
                        ))
                    }
                }
            } else {
                eprintln!("DEBUG: Rust query not in cache, checking file directly");
                // Try to load directly
                let runtime = crate::runtime::Runtime::new();
                let query_path = runtime.language_dir("rust").join("queries/highlights.scm");
                eprintln!("DEBUG: Direct check for query at: {}", query_path.display());
                if query_path.exists() {
                    match std::fs::read_to_string(&query_path) {
                        Ok(query_text) => {
                            let leaked = Box::leak(query_text.into_boxed_str());
                            eprintln!("DEBUG: Loaded Rust query directly");
                            Ok(leaked)
                        }
                        Err(e) => {
                            eprintln!("DEBUG: Failed to read Rust query directly: {}", e);
                            Err(SyntaxError::LanguageNotSupported(
                                format!("rust query read error: {}", e),
                            ))
                        }
                    }
                } else {
                    eprintln!("DEBUG: Rust query file doesn't exist");
                    Err(SyntaxError::LanguageNotSupported(
                        "rust grammar not available".to_string(),
                    ))
                }
            }
        }
        LanguageId::Toml => {
            eprintln!("DEBUG: Getting query for TOML");
            // First try to load from cache/file
            if let Some(_) = crate::query_cache::get_query("toml", "highlights") {
                eprintln!("DEBUG: Found TOML query in cache");
                use std::sync::OnceLock;
                static TOML_QUERY: OnceLock<Option<&'static str>> = OnceLock::new();
                
                let query_str = TOML_QUERY.get_or_init(|| {
                    let runtime = crate::runtime::Runtime::new();
                    let query_path = runtime.language_dir("toml").join("queries/highlights.scm");
                    eprintln!("DEBUG: Looking for TOML query at: {}", query_path.display());
                    match std::fs::read_to_string(&query_path) {
                        Ok(query_text) => {
                            eprintln!("DEBUG: Found TOML query file");
                            Some(Box::leak(query_text.into_boxed_str()))
                        }
                        Err(e) => {
                            eprintln!("DEBUG: Failed to read TOML query file: {}", e);
                            None
                        }
                    }
                });
                
                match query_str {
                    Some(str) => {
                        eprintln!("DEBUG: Returning cached TOML query");
                        Ok(*str)
                    }
                    None => {
                        // Fall back to a default TOML query
                        eprintln!("DEBUG: Using default TOML query");
                        static DEFAULT_TOML_QUERY: &str = r#"
; Default TOML highlights query
(table) @type
(key) @property
(string) @string
(boolean) @constant
(integer) @number
(float) @number
(date) @constant
(time) @constant
(date_time) @constant
(comment) @comment
"#;
                        Ok(DEFAULT_TOML_QUERY)
                    }
                }
            } else {
                // Try to load directly from runtime directory
                let runtime = crate::runtime::Runtime::new();
                let query_path = runtime.language_dir("toml").join("queries/highlights.scm");
                eprintln!("DEBUG: Direct check for TOML query at: {}", query_path.display());
                if query_path.exists() {
                    match std::fs::read_to_string(&query_path) {
                        Ok(query_text) => {
                            let leaked = Box::leak(query_text.into_boxed_str());
                            eprintln!("DEBUG: Loaded TOML query directly");
                            Ok(leaked)
                        }
                        Err(e) => {
                            eprintln!("DEBUG: Failed to read TOML query directly: {}", e);
                            // Fall back to a default TOML query
                            eprintln!("DEBUG: Using default TOML query (fallback)");
                            static DEFAULT_TOML_QUERY: &str = r#"
; Default TOML highlights query
(table) @type
(key) @property
(string) @string
(boolean) @constant
(integer) @number
(float) @number
(date) @constant
(time) @constant
(date_time) @constant
(comment) @comment
"#;
                            Ok(DEFAULT_TOML_QUERY)
                        }
                    }
                } else {
                    // Fall back to a default TOML query
                    eprintln!("DEBUG: Using default TOML query (no file)");
                    static DEFAULT_TOML_QUERY: &str = r#"
; Default TOML highlights query
(table) @type
(key) @property
(string) @string
(boolean) @constant
(integer) @number
(float) @number
(date) @constant
(time) @constant
(date_time) @constant
(comment) @comment
"#;
                    Ok(DEFAULT_TOML_QUERY)
                }
            }
        }
        LanguageId::Markdown => {
            if crate::query_cache::get_query("markdown", "highlights").is_some() {
                use std::sync::OnceLock;
                static MARKDOWN_QUERY: OnceLock<Option<&'static str>> = OnceLock::new();
                
                let query_str = MARKDOWN_QUERY.get_or_init(|| {
                    let runtime = crate::runtime::Runtime::new();
                    let query_path = runtime.language_dir("markdown").join("queries/highlights.scm");
                    match std::fs::read_to_string(&query_path) {
                        Ok(query_text) => Some(Box::leak(query_text.into_boxed_str())),
                        Err(_) => None,
                    }
                });
                
                match query_str {
                    Some(str) => Ok(*str),
                    None => Err(SyntaxError::LanguageNotSupported(
                        "markdown grammar not available".to_string(),
                    )),
                }
            } else {
                Err(SyntaxError::LanguageNotSupported(
                    "markdown grammar not available".to_string(),
                ))
            }
        }
        LanguageId::PlainText => Err(SyntaxError::LanguageNotSupported(
            "plaintext has no syntax queries".to_string(),
        )),
        LanguageId::Dynamic(id) => {
            // Use OnceLock with a Mutex for interior mutability
            use std::sync::{OnceLock, Mutex};
            use std::collections::HashMap;
            
            static DYNAMIC_QUERIES: OnceLock<Mutex<HashMap<String, &'static str>>> = OnceLock::new();
            
            let queries_mutex = DYNAMIC_QUERIES.get_or_init(|| Mutex::new(HashMap::new()));
            
            // Check cache first
            {
                let queries = queries_mutex.lock().unwrap();
                if let Some(query_str) = queries.get(id) {
                    return Ok(*query_str);
                }
            }
            
            // Not in cache, load from file
            let runtime = crate::runtime::Runtime::new();
            let query_path = runtime.language_dir(id).join("queries/highlights.scm");
            
            match std::fs::read_to_string(&query_path) {
                Ok(query_text) => {
                    // Leak the string to make it static
                    let query_str = Box::leak(query_text.into_boxed_str());
                    
                    // Insert into cache
                    let mut queries = queries_mutex.lock().unwrap();
                    queries.insert(id.to_string(), query_str);
                    
                    Ok(query_str)
                }
                Err(_) => {
                    Err(SyntaxError::LanguageNotSupported(
                        format!("{} grammar not available", id),
                    ))
                }
            }
        }
    }
}
