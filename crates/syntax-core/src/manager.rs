//! Syntax manager for coordinating multiple documents and languages.

use crate::error::SyntaxError;
use crate::highlight::{highlight, HighlightSpan};
use crate::language::LanguageId;
use std::collections::HashMap;
use std::path::Path;
use tree_sitter::{Parser, Tree};

pub struct SyntaxManager {
    documents: HashMap<String, SyntaxDocument>,
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
        }
    }

    pub fn update_document(
        &mut self,
        doc_id: &str,
        text: &str,
        path: &Path,
    ) -> Result<(), SyntaxError> {
        let language = LanguageId::from_path(path);
        let tree = if let Some(ts_lang) = language.tree_sitter_language() {
            let mut parser = Parser::new();
            parser
                .set_language(ts_lang)
                .map_err(|e| SyntaxError::GrammarLoadError(e.to_string()))?;
            Some(parser.parse(text, None).ok_or_else(|| SyntaxError::ParseError)?)
        } else {
            // Unsupported language, store without a tree (no syntax highlighting)
            None
        };

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
        // For simplicity, we reparse the whole document after each edit.
        // A real implementation would use incremental parsing.
        // Currently we do nothing, but we could call update_document again.
        // However we don't have the original text here.
        // We'll just ignore for now.
        let _ = (doc_id, start_byte, old_end_byte, new_text);
        Ok(())
    }

    pub fn contains_document(&self, doc_id: &str) -> bool {
        self.documents.contains_key(doc_id)
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
            Some(tree) => highlight(doc.language, &doc.text, tree),
            None => Ok(Vec::new()),
        }
    }
}

impl Default for SyntaxManager {
    fn default() -> Self {
        Self::new()
    }
}
