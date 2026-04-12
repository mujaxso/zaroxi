//! Syntax manager for coordinating multiple documents and languages.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use parking_lot::Mutex;

use crate::document::SyntaxDocument;
use crate::language::{LanguageId, LanguageRegistry};
use crate::snapshot::SyntaxSnapshot;
use crate::SyntaxError;

/// Manages syntax documents and provides syntax snapshots
pub struct SyntaxManager {
    /// Language registry
    registry: LanguageRegistry,
    /// Active syntax documents keyed by document ID
    documents: HashMap<String, SyntaxDocument>,
    /// Parser cache for each language
    parsers: HashMap<LanguageId, Arc<Mutex<tree_sitter::Parser>>>,
}

impl SyntaxManager {
    /// Create a new syntax manager
    pub fn new() -> Self {
        let registry = LanguageRegistry::new();
        let mut manager = Self {
            registry,
            documents: HashMap::new(),
            parsers: HashMap::new(),
        };
        
        // Initialize parsers for supported languages
        manager.initialize_parsers();
        
        manager
    }

    /// Initialize parsers for supported languages
    fn initialize_parsers(&mut self) {
        // Only initialize parsers for languages we actually support
        if let Some(parser) = self.registry.create_parser(LanguageId::Rust) {
            self.parsers.insert(LanguageId::Rust, Arc::new(Mutex::new(parser)));
        }
    }

    /// Create or update a syntax document
    pub fn update_document(
        &mut self,
        doc_id: &str,
        text: &str,
        path: &Path,
    ) -> Result<(), SyntaxError> {
        let (language, config) = self.registry.detect_from_path(path);
        let parser = self.parsers.get(&language).cloned();
        
        let highlight_config = config.map(|c| Arc::new(c.clone()));
        
        let document = SyntaxDocument::new(
            text,
            language,
            highlight_config,
            parser,
        )?;
        
        self.documents.insert(doc_id.to_string(), document);
        
        Ok(())
    }

    /// Apply an edit to a document
    pub fn edit_document(
        &mut self,
        doc_id: &str,
        start_byte: usize,
        old_end_byte: usize,
        new_text: &str,
    ) -> Result<(), SyntaxError> {
        if let Some(doc) = self.documents.get_mut(doc_id) {
            doc.edit(start_byte, old_end_byte, new_text)?;
            doc.reparse_if_needed()?;
        } else {
            return Err(SyntaxError::DocumentNotFound);
        }
        
        Ok(())
    }

    /// Get a syntax snapshot for a document
    pub fn snapshot_for_document(&self, doc_id: &str) -> Result<SyntaxSnapshot, SyntaxError> {
        if let Some(doc) = self.documents.get(doc_id) {
            let highlights = doc.highlight_spans()?;
            let text = doc.text();
            let snapshot = SyntaxSnapshot::new(highlights, &text);
            Ok(snapshot)
        } else {
            Ok(SyntaxSnapshot::default())
        }
    }

    /// Remove a document from the manager
    pub fn remove_document(&mut self, doc_id: &str) {
        self.documents.remove(doc_id);
    }

    /// Get the language for a document
    pub fn document_language(&self, doc_id: &str) -> Option<LanguageId> {
        self.documents.get(doc_id).map(|d| d.language())
    }

    /// Check if a document has syntax support
    pub fn has_syntax_support(&self, doc_id: &str) -> bool {
        self.documents.get(doc_id)
            .map(|doc| self.registry.is_supported(doc.language()))
            .unwrap_or(false)
    }
}

impl Default for SyntaxManager {
    fn default() -> Self {
        Self::new()
    }
}
