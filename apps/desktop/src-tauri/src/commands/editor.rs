//! Tauri commands that serve the editor front‑end.
//!
//! These commands implement a **plain‑text editor with no syntax highlighting**.
//! Syntax decoration will be introduced in a later layer once the core text
//! engine and viewport system are stable.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::command;
use zaroxi_domain_editor::document_cache::BufferManager;
use zaroxi_domain_editor::FileClass;

/// Global buffer manager instance shared across all commands.
static BUFFER_MANAGER: once_cell::sync::Lazy<Arc<BufferManager>> =
    once_cell::sync::Lazy::new(|| Arc::new(BufferManager::new()));

/// Response for opening a document.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenDocumentResponse {
    pub document_id: String, // currently the canonical path
    pub path: String,
    pub line_count: usize,
    pub char_count: usize,
    pub file_class: String,     // "Normal" / "Medium" / "Large"
    pub is_read_only: bool,
    pub content: String,        // full text for normal/medium, truncated preview for large
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

/// Open a document and return its metadata.
///
/// The returned `content` is the full rope text for normal/medium files, and a
/// truncated preview for large files.  Large files are always read‑only.
#[command]
pub async fn open_document(path: String) -> Result<OpenDocumentResponse, String> {
    let path_buf = std::path::PathBuf::from(&path);

    let cached = BUFFER_MANAGER
        .open_document(&path_buf, &zaroxi_ops_file::FileLoader)
        .await
        .map_err(|e| format!("Failed to open document: {}", e))?;

    let document = &cached.document;
    let file_class = document.file_class();
    let is_read_only = file_class.is_read_only();
    let content_truncated = file_class == FileClass::Large;

    let content: String = if content_truncated {
        document.text().chars().take(TRUNCATE_CHARS).collect()
    } else {
        document.text()
    };

    // For large files we use the metrics of the truncated preview,
    // not the enormous original file.  This prevents huge line‑count
    // values from being sent to the frontend and avoids crashes there.
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

/// Get visible lines for a document.
#[command]
pub async fn get_visible_lines(
    request: VisibleLinesRequest,
) -> Result<VisibleLinesResponse, String> {
    let path = std::path::PathBuf::from(&request.document_id);
    let cached = BUFFER_MANAGER
        .get_cached(&path)
        .await
        .ok_or_else(|| "Document not found in cache".to_string())?;

    let document = &cached.document;
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

/// Apply an edit to a document.
#[command]
pub async fn apply_edit(request: EditRequest) -> Result<(), String> {
    let path = std::path::PathBuf::from(&request.document_id);
    let cached = BUFFER_MANAGER
        .get_cached(&path)
        .await
        .ok_or_else(|| "Document not found in cache".to_string())?;

    // Reject edits for read‑only documents (very large files)
    if cached.document.file_class().is_read_only() {
        return Err("Document is read‑only (very large file)".to_string());
    }

    let mut document = cached.document;

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

    // Update cache with the modified document
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
#[command]
pub async fn get_document_content(document_id: String) -> Result<String, String> {
    let path = std::path::PathBuf::from(&document_id);
    let cached = BUFFER_MANAGER
        .get_cached(&path)
        .await
        .ok_or_else(|| "Document not found in cache".to_string())?;

    Ok(cached.document.text())
}

// ---------------------------------------------------------------------------
// Temporary stub – exists only to satisfy the legacy handler registration.
// Will be replaced with the real syntax‑highlighting command when it is layered.
// ---------------------------------------------------------------------------
#[command]
pub async fn get_styled_spans() -> Result<(), String> {
    Ok(())
}
