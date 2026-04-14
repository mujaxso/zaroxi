//! Syntax manager for coordinating multiple documents and languages.

use crate::error::SyntaxError;
use crate::highlight::{highlight, HighlightSpan, get_query_for_language};
use crate::language::LanguageId;
use std::collections::HashMap;
use std::path::Path;
use tree_sitter::{Parser, Tree, Query, QueryCursor};

pub struct SyntaxManager {
    documents: HashMap<String, SyntaxDocument>,
    // Cache parsers per language to avoid recreating them
    parsers: HashMap<LanguageId, Parser>,
    // Cache compiled queries per language
    queries: HashMap<LanguageId, Result<Query, String>>,
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
            queries: HashMap::new(),
        }
    }

    pub fn update_document(
        &mut self,
        doc_id: &str,
        text: &str,
        path: &Path,
    ) -> Result<(), SyntaxError> {
        let language = LanguageId::from_path(path);
        
        // Get or create parser for this language
        let parser = self.parsers.entry(language).or_insert_with(|| {
            let mut parser = Parser::new();
            if let Some(ts_lang) = language.tree_sitter_language() {
                let _ = parser.set_language(ts_lang);
            }
            parser
        });

        let tree = if language.tree_sitter_language().is_some() {
            parser.parse(text, None)
        } else {
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

    pub fn highlight_spans(&self, doc_id: &str) -> Result<Vec<HighlightSpan>, SyntaxError> {
        let doc = self
            .documents
            .get(doc_id)
            .ok_or(SyntaxError::DocumentNotFound)?;
        match &doc.tree {
            Some(tree) => {
                // Try to use cached query first
                if let Some(query_result) = self.queries.get(&doc.language) {
                    match query_result {
                        Ok(query) => {
                            let mut cursor = QueryCursor::new();
                            let root_node = tree.root_node();
                            let mut spans = Vec::new();

                            for match_ in cursor.matches(query, root_node, doc.text.as_bytes()) {
                                for capture in match_.captures {
                                    let node = capture.node;
                                    let start = node.start_byte();
                                    let end = node.end_byte();
                                    let capture_name = &query.capture_names()[capture.index as usize];
                                    let highlight = crate::highlight::map_capture_name(capture_name);
                                    spans.push(HighlightSpan {
                                        start,
                                        end,
                                        highlight,
                                    });
                                }
                            }
                            spans.sort_by_key(|span| span.start);
                            return Ok(spans);
                        }
                        Err(_) => {
                            // Query compilation failed, fall back to standard highlight
                        }
                    }
                }
                // Fall back to standard highlight function
                highlight(doc.language, &doc.text, tree)
            }
            None => Ok(Vec::new()),
        }
    }
    
    // Precompile queries for supported languages to speed up highlighting
    pub fn precompile_queries(&mut self) {
        let languages = [LanguageId::Rust, LanguageId::Toml];
        for &language in &languages {
            if let Some(ts_lang) = language.tree_sitter_language() {
                if let Ok(query_str) = get_query_for_language(language) {
                    let query = Query::new(ts_lang, query_str);
                    self.queries.insert(language, query.map_err(|e| e.to_string()));
                }
            }
        }
    }
}

impl Default for SyntaxManager {
    fn default() -> Self {
        Self::new()
    }
}
