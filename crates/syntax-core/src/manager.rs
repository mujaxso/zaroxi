//! Syntax manager for coordinating multiple documents and languages.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use parking_lot::Mutex;

use crate::document::SyntaxDocument;
use crate::language::LanguageRegistry;
use crate::snapshot::SyntaxSnapshot;
use crate::SyntaxError;

/// Manages syntax documents and provides syntax snapshots
pub struct SyntaxManager {
    /// Language registry (now includes runtime and metadata)
    registry: LanguageRegistry,
    /// Active syntax documents keyed by document ID
    documents: HashMap<String, SyntaxDocument>,
}

impl SyntaxManager {
    /// Create a new syntax manager
    pub fn new() -> Self {
        Self {
            registry: LanguageRegistry::new(),
            documents: HashMap::new(),
        }
    }

    /// Create or update a syntax document
    pub fn update_document(
        &mut self,
        doc_id: &str,
        text: &str,
        path: &Path,
    ) -> Result<(), SyntaxError> {
        let (language_id, _metadata) = self.registry.detect_from_path(path);
        let parser = {
            // Get or create a parser for the detected language
            let parser = self.registry.parser(&language_id)?;
            // We need to wrap the parser in an Arc<Mutex> for SyntaxDocument.
            // Since the registry returns a mutable reference, we must create a new parser.
            // For simplicity, we create a fresh parser using the same language.
            // A more advanced implementation would share the parser across documents.
            let lang = parser.language().ok_or_else(|| {
                SyntaxError::ParserError("Parser has no language set".to_string())
            })?;
            let mut new_parser = tree_sitter::Parser::new();
            new_parser.set_language(&lang)
                .map_err(|e| SyntaxError::ParserError(e.to_string()))?;
            Some(Arc::new(Mutex::new(new_parser)))
        };

        let highlight_config = self.registry.highlight_config(&language_id)
            .map(|c| Arc::new(c.clone()))
            .ok(); // Gracefully fall back if no highlight config

        // For now we pass None as highlight config if detection fails.
        // SyntaxDocument will fall back to plain text.
        let document = SyntaxDocument::new(
            text,
            crate::language::LanguageId::from_str(&language_id)
                .unwrap_or(crate::language::LanguageId::PlainText),
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
    pub fn document_language(&self, doc_id: &str) -> Option<crate::language::LanguageId> {
        self.documents.get(doc_id).map(|d| d.language())
    }

    /// Check if a document has syntax support
    pub fn has_syntax_support(&self, doc_id: &str) -> bool {
        self.documents.get(doc_id)
            .map(|doc| self.registry.is_supported(doc.language().as_str()))
            .unwrap_or(false)
    }
}

impl Default for SyntaxManager {
    fn default() -> Self {
        Self::new()
    }
}
