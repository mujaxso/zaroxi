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
        let language = LanguageId::from_path(path);
        
        // Try to get the language
        let ts_lang = match language.tree_sitter_language() {
            Some(lang) => lang,
            None => {
                // If no language is available, store document without a tree
                let doc = SyntaxDocument {
                    text: text.to_string(),
                    language,
                    tree: None,
                };
                self.documents.insert(doc_id.to_string(), doc);
                return Ok(());
            }
        };

        // Try to get or create a parser for this language
        let parser = self.parsers.entry(language).or_insert_with(|| {
            let mut parser = Parser::new();
            // Try to set the language, but don't panic if it fails
            let _ = parser.set_language(&ts_lang);
            parser
        });

        // Parse the document
        let tree = parser.parse(text, None);

        let doc = SyntaxDocument {
            text: text.to_string(),
            language,
            tree,
        };
        self.documents.insert(doc_id.to_string(), doc);
        Ok(())
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
        let doc = self
            .documents
            .get(doc_id)
            .ok_or(SyntaxError::DocumentNotFound)?;
        match &doc.tree {
            Some(tree) => {
                // Use the global query cache
                // For now, just use the standard highlight function which uses the query cache
                highlight(doc.language, &doc.text, tree)
            }
            None => {
                Ok(Vec::new())
            }
        }
    }
    
    /// Initialize dynamic grammars and preload queries
    pub fn initialize_dynamic_grammars(&mut self) {
        use crate::dynamic_loader::preload_available_grammars;
        use crate::query_cache::preload_queries;
        
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
