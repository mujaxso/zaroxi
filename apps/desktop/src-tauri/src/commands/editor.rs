use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::command;
use zaroxi_domain_editor::document_cache::BufferManager;
use zaroxi_domain_editor::document_syntax_cache::DocumentSyntaxCache;
use zaroxi_domain_editor::LargeFileMode;
use zaroxi_lang_syntax::language::LanguageId;
use zaroxi_ops_file::FileLoader;
use zaroxi_theme::theme::SemanticColors;

/// Global buffer manager instance shared across all commands.
static BUFFER_MANAGER: once_cell::sync::Lazy<Arc<BufferManager>> =
    once_cell::sync::Lazy::new(|| Arc::new(BufferManager::new()));

/// Global document syntax cache shared across all commands.
static SYNTAX_CACHE: once_cell::sync::Lazy<DocumentSyntaxCache> =
    once_cell::sync::Lazy::new(|| DocumentSyntaxCache::new());

/// Response for opening a document.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenDocumentResponse {
    pub document_id: String,
    pub path: String,
    pub line_count: usize,
    pub char_count: usize,
    pub large_file_mode: String,
    pub is_read_only: bool,
    pub content: String,
    /// Indicates whether the returned `content` was truncated (file was large).
    pub content_truncated: bool,
    /// The document version for cache invalidation.
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

/// Open a document and return its metadata.
/// The frontend will request visible lines separately.
#[command]
pub async fn open_document(path: String) -> Result<OpenDocumentResponse, String> {
    let path_buf = std::path::PathBuf::from(&path);

    // Use the buffer manager to open (or retrieve cached) document.
    let cached = BUFFER_MANAGER
        .open_document(&path_buf, &FileLoader)
        .await
        .map_err(|e| format!("Failed to open document: {}", e))?;

    let document = &cached.document;
    let line_count = document.len_lines();
    let char_count = document.len_chars();
    let large_file_mode = document.large_file_mode();
    let is_read_only = large_file_mode.is_read_only();
    let content_truncated = large_file_mode == LargeFileMode::Large
        || large_file_mode == LargeFileMode::VeryLarge;

    // Build the response content (only the first TRUNCATE_CHARS characters for large files)
    let content: String = if content_truncated {
        document.text().chars().take(TRUNCATE_CHARS).collect()
    } else {
        document.text()
    };

    let document_id = uuid::Uuid::new_v4().to_string();
    let version = document.version();

    // Pre-warm syntax cache for this document
    let canonical = path_buf.canonicalize().unwrap_or(path_buf.clone());
    let language = LanguageId::from_path(&canonical);
    let mut state = SYNTAX_CACHE.get_or_create_state(&canonical, language);
    state.ensure_syntax_tree(&document.text(), language);
    SYNTAX_CACHE.update_state(&canonical, state);

    Ok(OpenDocumentResponse {
        document_id,
        path,
        line_count,
        char_count,
        large_file_mode: format!("{:?}", large_file_mode),
        is_read_only,
        content,
        content_truncated,
        version,
    })
}

#[derive(Serialize)]
pub struct StyledSpanResponse {
    /// Character offset (not byte offset) – safe for frontend text slicing.
    pub start: usize,
    /// Character offset (not byte offset).
    pub end: usize,
    pub color: String,
    /// The document version for which these spans are valid.
    pub version: u64,
}


#[tauri::command(rename_all = "camelCase")]
pub async fn get_styled_spans(
    path: String,
    start_line: Option<usize>,
    end_line: Option<usize>,
    _version: Option<u64>,
) -> Result<Vec<StyledSpanResponse>, String> {
    let path_buf = std::path::PathBuf::from(&path);
    let canonical = path_buf
        .canonicalize()
        .map_err(|e| format!("Cannot canonicalize path: {}", e))?;

    // Get the document text and version from the buffer manager
    let cached = BUFFER_MANAGER
        .get_cached(&canonical)
        .await
        .ok_or_else(|| "Document not found in cache".to_string())?;

    let document = &cached.document;
    let current_version = document.version();

    let text = document.text();
    let language = LanguageId::from_path(&canonical);

    // Get or create syntax state from the cache
    let mut state = SYNTAX_CACHE.get_or_create_state(&canonical, language);

    // Use dark theme for now (could be made configurable)
    let colors = SemanticColors::dark();

    let styled_spans = if let (Some(sl), Some(el)) = (start_line, end_line) {
        // Only compute highlights for the visible range (plus small overscan)
        state.styled_spans_for_lines(
            &text,
            current_version,
            &colors,
            sl,
            el,
            |line| document.line_to_char(line),
            |byte| document.byte_to_char(byte),
            document.len_chars(),
            document.len_lines(),
        )
    } else {
        // Full‑document highlights (fallback)
        let highlights = state.get_highlights(&text, current_version);
        let mut spans = zaroxi_lang_syntax::theme_map::apply_theme(highlights, &colors);
        // Convert byte offsets to character offsets
        for span in &mut spans {
            let start_char = document.byte_to_char(span.start);
            let end_char = document.byte_to_char(span.end);
            span.start = start_char;
            span.end = end_char;
        }
        spans
    };

    // Update the cache with the new state
    SYNTAX_CACHE.update_state(&canonical, state);

    let response: Vec<StyledSpanResponse> = styled_spans
        .iter()
        .map(|span| StyledSpanResponse {
            start: span.start,
            end: span.end,
            color: span.color.to_hex(),
            version: current_version,
        })
        .collect();

    Ok(response)
}

/// Get visible lines for a document.
#[command]
pub async fn get_visible_lines(
    request: VisibleLinesRequest,
) -> Result<VisibleLinesResponse, String> {
    // Retrieve the document from the buffer manager using the path stored in the request.
    // For simplicity, we assume the document_id is the path (or we could store a mapping).
    // In a real implementation, we'd maintain a mapping from document_id to path.
    // For now, we'll use the path from the request (the frontend should pass the path).
    let path = std::path::PathBuf::from(&request.document_id);
    let cached = BUFFER_MANAGER
        .get_cached(&path)
        .await
        .ok_or_else(|| "Document not found in cache".to_string())?;

    let document = &cached.document;
    let total_lines = document.len_lines();
    let mut lines = Vec::new();

    // Clamp start_line to valid range
    let start_line = request.start_line.min(total_lines);
    let end_line = (start_line + request.count).min(total_lines);
    for line_idx in start_line..end_line {
        if let Some(text) = document.line(line_idx) {
            lines.push(LineDto { index: line_idx, text: text.to_string() });
        }
    }

    Ok(VisibleLinesResponse { lines, total_lines })
}

/// Apply an edit to a document.
#[command]
pub async fn apply_edit(request: EditRequest) -> Result<(), String> {
    let path = std::path::PathBuf::from(&request.document_id);
    let cached = BUFFER_MANAGER
        .get_cached(&path)
        .await
        .ok_or_else(|| "Document not found in cache".to_string())?;

    // We need mutable access to the document. Since BufferManager returns a clone,
    // we need to get the mutable reference from the cache.
    // For now, we'll use a workaround: we'll get the document from the cache again
    // and modify it in place. This is not ideal but works for the prototype.
    // In a real implementation, we'd have a method on BufferManager to apply edits.
    let mut document = cached.document.clone();

    // Reject edits for read‑only documents (very large files)
    if document.large_file_mode().is_read_only() {
        return Err("Document is read‑only (very large file)".to_string());
    }

    // Convert byte positions to character positions
    let start_char = document.byte_to_char(request.start_byte);
    let old_end_char = document.byte_to_char(request.old_end_byte);

    // Ensure start <= end
    let (delete_start, delete_end) = if start_char <= old_end_char {
        (start_char, old_end_char)
    } else {
        (old_end_char, start_char)
    };

    // Delete old range
    if delete_start < delete_end {
        document.delete(delete_start, delete_end)?;
    }

    // Insert new text at the start position (after deletion, the insertion point is delete_start)
    if !request.new_text.is_empty() {
        document.insert(delete_start, &request.new_text)?;
    }

    // Mark the document as dirty in the cache
    BUFFER_MANAGER.mark_dirty(&path).await;

    Ok(())
}

/// Save a document to disk.
#[command]
pub async fn save_document(document_id: String) -> Result<(), String> {
    let path = std::path::PathBuf::from(&document_id);
    let cached = BUFFER_MANAGER
        .get_cached(&path)
        .await
        .ok_or_else(|| "Document not found in cache".to_string())?;

    let text = cached.document.text();
    std::fs::write(&path, &text).map_err(|e| format!("Failed to save file: {}", e))?;

    // Mark the document as clean in the cache
    BUFFER_MANAGER.mark_clean(&path).await;

    Ok(())
}

/// Get the total line count for a document.
#[command]
pub async fn get_line_count(document_id: String) -> Result<usize, String> {
    let path = std::path::PathBuf::from(&document_id);
    let cached = BUFFER_MANAGER
        .get_cached(&path)
        .await
        .ok_or_else(|| "Document not found in cache".to_string())?;

    Ok(cached.document.len_lines())
}

/// Return the full text content of a document.
#[allow(dead_code)]
#[command]
pub async fn get_document_content(document_id: String) -> Result<String, String> {
    let path = std::path::PathBuf::from(&document_id);
    let cached = BUFFER_MANAGER
        .get_cached(&path)
        .await
        .ok_or_else(|| "Document not found in cache".to_string())?;

    Ok(cached.document.text())
}
