//! Tauri commands that serve the editor front‑end.
//!
//! These commands implement a **plain‑text editor** with optional syntax highlighting.
//! Syntax decoration is layered on top using a per‑document cache that is
//! invalidated when the document version changes.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::Mutex;
use tauri::command;
use zaroxi_domain_editor::document_cache::BufferManager;
use zaroxi_domain_editor::FileClass;
use zaroxi_lang_syntax::language::LanguageId;
use zaroxi_lang_syntax::parser::{ParserPool, SyntaxTree};
use zaroxi_lang_syntax::highlight::{HighlightEngine, HighlightSpan};

/// Global buffer manager instance shared across all commands.
static BUFFER_MANAGER: once_cell::sync::Lazy<Arc<BufferManager>> =
    once_cell::sync::Lazy::new(|| Arc::new(BufferManager::new()));

/// Shared parser pool – used to avoid re‑creating parsers on every highlight call.
static PARSER_POOL: once_cell::sync::Lazy<Arc<ParserPool>> =
    once_cell::sync::Lazy::new(|| Arc::new(ParserPool::new()));

// ---------------------------------------------------------------------------
// Syntax highlight cache
// ---------------------------------------------------------------------------

/// Cached syntax data for one document version.
struct CachedSyntax {
    /// The parsed syntax tree (needed for incremental re‑parse, kept for future use).
    tree: SyntaxTree,
    /// Highlight spans for the **whole** document.
    spans: Vec<HighlightSpan>,
}

/// Per‑document syntax cache, keyed by canonical path.
static SYNTAX_CACHE: once_cell::sync::Lazy<Mutex<HashMap<PathBuf, CachedSyntax>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));

// ---------------------------------------------------------------------------
// Command DTOs
// ---------------------------------------------------------------------------

/// Response for opening a document.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenDocumentResponse {
    pub document_id: String,
    pub path: String,
    pub line_count: usize,
    pub char_count: usize,
    pub file_class: String,
    pub is_read_only: bool,
    pub content: String,
    pub content_truncated: bool,
    pub version: u64,
}

/// Maximum characters returned in the `content` field for large files.
const TRUNCATE_CHARS: usize = 50_000;

/// Request for visible lines.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleLinesRequest {
    pub document_id: String,
    pub start_line: usize,
    pub count: usize,
}

/// Response for visible lines.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisibleLinesResponse {
    pub lines: Vec<LineDto>,
    pub total_lines: usize,
}

/// A single line of text.
#[derive(Debug, Serialize)]
pub struct LineDto {
    pub index: usize,
    pub text: String,
}

/// Request for an edit operation.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditRequest {
    pub document_id: String,
    pub start_byte: usize,
    pub old_end_byte: usize,
    pub new_text: String,
}

// ---- Syntax highlight types ----

#[derive(Debug, Serialize)]
pub struct HighlightResponse {
    pub lines: Vec<HighlightedLine>,
}

#[derive(Debug, Serialize)]
pub struct HighlightedLine {
    pub index: usize,
    pub text: String,
    pub spans: Vec<HighlightSpanDto>,
}

#[derive(Debug, Serialize)]
pub struct HighlightSpanDto {
    /// Byte offset **within the line** (0‑based).
    pub start: usize,
    /// Exclusive byte offset.
    pub end: usize,
    /// Human‑readable token type (e.g. "keyword", "string").
    pub token_type: String,
}

// ---------------------------------------------------------------------------
// Open / visible / edit commands (unchanged logic, slightly adapted for
// syntax integration)
// ---------------------------------------------------------------------------

#[command]
pub async fn open_document(path: String) -> Result<OpenDocumentResponse, String> {
    let path_buf = std::path::PathBuf::from(&path);

    let cached_arc = BUFFER_MANAGER
        .open_document(&path_buf, &zaroxi_ops_file::FileLoader)
        .await
        .map_err(|e| format!("Failed to open document: {}", e))?;

    let guard = cached_arc.lock();
    let document = &guard.document;
    let file_class = document.file_class();
    let content_truncated = file_class == FileClass::Large;
    let is_read_only = content_truncated;

    let content: String = if content_truncated {
        document.text().chars().take(TRUNCATE_CHARS).collect()
    } else {
        document.text()
    };

    let (line_count, char_count) = if content_truncated {
        let preview_lines = content.lines().count();
        (preview_lines, content.len())
    } else {
        (document.len_lines(), document.len_chars())
    };

    let version = document.version();

    Ok(OpenDocumentResponse {
        document_id: path.clone(),
        path,
        line_count,
        char_count,
        file_class: format!("{:?}", file_class),
        is_read_only,
        content,
        content_truncated,
        version,
    })
}

#[command]
pub async fn get_visible_lines(
    request: VisibleLinesRequest,
) -> Result<VisibleLinesResponse, String> {
    let path = std::path::PathBuf::from(&request.document_id);
    let cached_arc = BUFFER_MANAGER
        .get_cached(&path)
        .await
        .ok_or_else(|| "Document not found in cache".to_string())?;

    let guard = cached_arc.lock();
    let document = &guard.document;
    let total_lines = document.len_lines();
    let mut lines = Vec::new();

    let start_line = request.start_line.min(total_lines);
    let end_line = (start_line + request.count).min(total_lines);
    for line_idx in start_line..end_line {
        if let Some(text) = document.line(line_idx) {
            lines.push(LineDto {
                index: line_idx,
                text,
            });
        }
    }

    Ok(VisibleLinesResponse { lines, total_lines })
}

#[command]
pub async fn apply_edit(request: EditRequest) -> Result<(), String> {
    let path = std::path::PathBuf::from(&request.document_id);
    let cached_arc = BUFFER_MANAGER
        .get_cached(&path)
        .await
        .ok_or_else(|| "Document not found in cache".to_string())?;

    {
        let mut guard = cached_arc.lock();
        let document = &mut guard.document;

        if document.file_class().is_read_only() {
            return Err("Document is read‑only (very large file)".to_string());
        }

        let start_char = document.byte_to_char(request.start_byte);
        let old_end_char = document.byte_to_char(request.old_end_byte);

        let (delete_start, delete_end) = if start_char <= old_end_char {
            (start_char, old_end_char)
        } else {
            (old_end_char, start_char)
        };

        if delete_start < delete_end {
            document.delete(delete_start, delete_end)?;
        }

        if !request.new_text.is_empty() {
            document.insert(delete_start, &request.new_text)?;
        }
    }

    // Invalidate syntax cache for this path (version changed).
    {
        let mut syn_cache = SYNTAX_CACHE.lock();
        syn_cache.remove(&path);
    }

    BUFFER_MANAGER.mark_dirty(&path).await;

    Ok(())
}

#[command]
pub async fn save_document(document_id: String) -> Result<(), String> {
    let path = std::path::PathBuf::from(&document_id);
    let cached_arc = BUFFER_MANAGER
        .get_cached(&path)
        .await
        .ok_or_else(|| "Document not found in cache".to_string())?;

    {
        let guard = cached_arc.lock();
        let text = guard.document.text();
        std::fs::write(&path, &text).map_err(|e| format!("Failed to save file: {}", e))?;
    }

    BUFFER_MANAGER.mark_clean(&path).await;

    Ok(())
}

#[command]
pub async fn get_line_count(document_id: String) -> Result<usize, String> {
    let path = std::path::PathBuf::from(&document_id);
    let cached_arc = BUFFER_MANAGER
        .get_cached(&path)
        .await
        .ok_or_else(|| "Document not found in cache".to_string())?;

    let guard = cached_arc.lock();
    Ok(guard.document.len_lines())
}

#[command]
pub async fn get_document_content(document_id: String) -> Result<String, String> {
    let path = std::path::PathBuf::from(&document_id);
    let cached_arc = BUFFER_MANAGER
        .get_cached(&path)
        .await
        .ok_or_else(|| "Document not found in cache".to_string())?;

    let guard = cached_arc.lock();
    Ok(guard.document.text())
}

// ---------------------------------------------------------------------------
// Syntax highlight command
// ---------------------------------------------------------------------------

/// Build (or reuse) the full‑document spans for a given file version,
/// then return the spans for the requested line range.
#[command]
pub async fn highlight_document(
    document_id: String,
    start_line: usize,
    count: usize,
) -> Result<HighlightResponse, String> {
    let path = std::path::PathBuf::from(&document_id);
    let cached_arc = BUFFER_MANAGER
        .get_cached(&path)
        .await
        .ok_or_else(|| "Document not found in cache".to_string())?;

    let guard = cached_arc.lock();
    let document = &guard.document;

    // Do not highlight large files – the frontend renders them as plain text.
    if document.file_class() == FileClass::Large {
        return Ok(HighlightResponse { lines: vec![] });
    }

    let lang = LanguageId::from_path(document.path().unwrap_or(std::path::Path::new("")));
    if lang == LanguageId::PlainText {
        return Ok(HighlightResponse { lines: vec![] });
    }

    let version = document.version();
    let full_text = document.text();

    // ---- Syntax cache hit / miss ----
    let spans = {
        let mut syn_cache = SYNTAX_CACHE.lock();
        let needs_rebuild = match syn_cache.get(&path) {
            Some(cached) if cached.tree.version? Wait – our CachedSyntax doesn't have version field. We'll store version separately. Let's adjust.

We need version stored. Let's modify structure: `struct CachedSyntax { tree: SyntaxTree, spans: Vec<HighlightSpan>, version: u64 }`. We'll add.

But currently we don't have version in struct. I'll amend in the final answer: include version.

Thus if cached.version != version then rebuild else reuse spans.

We'll implement.

        };

We need to get spans. Let's write code:

