//! Syntax manager for coordinating multiple documents and languages.

use crate::error::SyntaxError;
use crate::highlight::{highlight, HighlightSpan};
use crate::language::LanguageId;
use std::collections::HashMap;
use std::path::Path;
use tree_sitter::{Parser, Tree};

pub struct SyntaxManager {
    documents: HashMap<String, SyntaxDocument>,
    // Cache parsers per language to avoid recreating them
    parsers: HashMap<LanguageId, Parser>,
}

struct SyntaxDocument {
    text: String,
    language: LanguageId,
    tree: Option<Tree>,
}

impl SyntaxManager {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            parsers: HashMap::new(),
        }
    }

    pub fn update_document(
        &mut self,
        doc_id: &str,
        text: &str,
        path: &Path,
    ) -> Result<(), SyntaxError> {
        eprintln!("DEBUG: update_document for path: {}", path.display());
        let language = LanguageId::from_path(path);
        eprintln!("DEBUG: Language determined: {:?}", language);
        
        // Try to get the language
        let ts_lang = match language.tree_sitter_language() {
            Some(lang) => {
                // Check if the language is compatible with the current tree-sitter version
                // We can't directly check the version, but we can try to create a parser and set the language
                // If it fails, we'll know it's incompatible
                eprintln!("DEBUG: Got tree-sitter language for {:?}", language);
                eprintln!("DEBUG: Language version: {}", lang.version());
                lang
            }
            None => {
                eprintln!("DEBUG: No tree-sitter language available for {:?}", language);
                let doc = SyntaxDocument {
                    text: text.to_string(),
                    language,
                    tree: None,
                };
                self.documents.insert(doc_id.to_string(), doc);
                eprintln!("DEBUG: Document updated without tree");
                return Ok(());
            }
        };

        // Create a new parser for this document
        let mut parser = Parser::new();
        
        // Try to set the language on the parser
        eprintln!("DEBUG: Setting tree-sitter language");
        match parser.set_language(&ts_lang) {
            Ok(_) => {
                // Parse the document
                eprintln!("DEBUG: Parsing document with available language");
                let tree = parser.parse(text, None);
                eprintln!("DEBUG: Parse result: {}", if tree.is_some() { "Some" } else { "None" });

                let doc = SyntaxDocument {
                    text: text.to_string(),
                    language,
                    tree,
                };
                self.documents.insert(doc_id.to_string(), doc);
                eprintln!("DEBUG: Document updated successfully");
                Ok(())
            }
            Err(e) => {
                eprintln!("DEBUG: Failed to set language: {:?}", e);
                eprintln!("DEBUG: Language ABI version may be incompatible with tree-sitter library (using tree-sitter v0.26.8)");
                eprintln!("DEBUG: Try rebuilding the grammar with: cargo run --bin build-grammar -- {}", language.as_str());
                // If setting fails, document will have no tree
                let doc = SyntaxDocument {
                    text: text.to_string(),
                    language,
                    tree: None,
                };
                self.documents.insert(doc_id.to_string(), doc);
                eprintln!("DEBUG: Document updated without tree");
                Ok(())
            }
        }
    }

    pub fn edit_document(
        &mut self,
        doc_id: &str,
        start_byte: usize,
        old_end_byte: usize,
        new_text: &str,
    ) -> Result<(), SyntaxError> {
        // Find the document
        if let Some(doc) = self.documents.get_mut(doc_id) {
            // Apply the edit to the text
            let mut text = doc.text.clone();
            if start_byte <= old_end_byte && old_end_byte <= text.len() {
                text.replace_range(start_byte..old_end_byte, new_text);
                doc.text = text;
                
                // Re-parse the document
                // For now, we'll clear the tree and it will be re-parsed on next highlight
                doc.tree = None;
            }
        }
        Ok(())
    }

    pub fn contains_document(&self, doc_id: &str) -> bool {
        self.documents.contains_key(doc_id)
    }

    pub fn highlight_spans(&self, doc_id: &str) -> Result<Vec<HighlightSpan>, SyntaxError> {
        eprintln!("DEBUG: highlight_spans called for doc_id: {}", doc_id);
        let doc = self
            .documents
            .get(doc_id)
            .ok_or(SyntaxError::DocumentNotFound)?;
        eprintln!("DEBUG: Document language: {:?}", doc.language);
        match &doc.tree {
            Some(tree) => {
                eprintln!("DEBUG: Tree exists, calling highlight");
                // Use the global query cache
                // For now, just use the standard highlight function which uses the query cache
                highlight(doc.language, &doc.text, tree)
            }
            None => {
                eprintln!("DEBUG: No tree, returning empty spans");
                Ok(Vec::new())
            }
        }
    }
    
    /// Initialize dynamic grammars and preload queries
    pub fn initialize_dynamic_grammars(&mut self) {
        use crate::dynamic_loader::preload_available_grammars;
        use crate::query_cache::preload_queries;
        use crate::runtime::Runtime;
        
        // Log runtime path for debugging
        let runtime = Runtime::new();
        eprintln!("DEBUG: Runtime directory: {:?}", runtime.root());
        
        // Preload available grammars
        preload_available_grammars();
        
        // Preload queries
        preload_queries();
    }
}

impl Default for SyntaxManager {
    fn default() -> Self {
        Self::new()
    }
}
