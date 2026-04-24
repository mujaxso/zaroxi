use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::command;
use zaroxi_domain_editor::Document;
use zaroxi_domain_editor::LargeFileMode;
use zaroxi_ops_file::FileLoader;
use zaroxi_ops_file::file_loader::FileLoadStrategy;

/// In-memory store for open documents.
static DOCUMENTS: once_cell::sync::Lazy<Mutex<HashMap<String, Document>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));

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
    // 1. Get file size before loading contents (avoids reading huge files fully)
    let metadata =
        std::fs::metadata(&path).map_err(|e| format!("Failed to get file metadata: {}", e))?;
    let size = metadata.len();
    let large_file_mode = LargeFileMode::from_size(size);
    let is_read_only = large_file_mode.is_read_only();
    let content_truncated =
        large_file_mode == LargeFileMode::Large || large_file_mode == LargeFileMode::VeryLarge;

    // 2. Decide loading strategy based on size
    let strategy = if content_truncated {
        // For large / very large files, only read the first TRUNCATE_CHARS bytes
        FileLoadStrategy::Preview(TRUNCATE_CHARS)
    } else {
        FileLoadStrategy::for_size(size)
    };

    let (file_source, _) = FileLoader::load_file_with_strategy(&path, strategy)
        .map_err(|e| format!("Failed to load file: {}", e))?;

    // 3. Build the editor Document (full file, using mmap for large files)
    //    For very large files we memory-map the full file quickly, but we never
    //    copy the entire content into a String on this side.
    let document = match &file_source {
        zaroxi_ops_file::file_loader::FileSource::Memory(text) => {
            Document::from_text_with_path(text, path.clone())
        }
        zaroxi_ops_file::file_loader::FileSource::Mmap(_mmap) => {
            // Reuse the existing mmap via file_source.as_str() to avoid a second mmap.
            Document::from_text_with_path(file_source.as_str(), path.clone())
        }
    };

    // 4. Build the response content (only the first TRUNCATE_CHARS characters)
    let content: String = if content_truncated {
        file_source.as_str().chars().take(TRUNCATE_CHARS).collect()
    } else {
        // small file – safe to copy whole content
        file_source.as_str().to_string()
    };

    let line_count = document.len_lines();
    let char_count = document.len_chars();
    let document_id = uuid::Uuid::new_v4().to_string();

    // Store the document
    let mut docs = DOCUMENTS.lock().map_err(|e| format!("Lock error: {}", e))?;
    docs.insert(document_id.clone(), document);

    Ok(OpenDocumentResponse {
        document_id,
        path,
        line_count,
        char_count,
        large_file_mode: format!("{:?}", large_file_mode),
        is_read_only,
        content,
        content_truncated,
    })
}

/// Get visible lines for a document.
#[command]
pub async fn get_visible_lines(
    request: VisibleLinesRequest,
) -> Result<VisibleLinesResponse, String> {
    let docs = DOCUMENTS.lock().map_err(|e| format!("Lock error: {}", e))?;
    let document =
        docs.get(&request.document_id).ok_or_else(|| "Document not found".to_string())?;

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
    let mut docs = DOCUMENTS.lock().map_err(|e| format!("Lock error: {}", e))?;
    let document =
        docs.get_mut(&request.document_id).ok_or_else(|| "Document not found".to_string())?;

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

    Ok(())
}

/// Save a document to disk.
#[command]
pub async fn save_document(document_id: String) -> Result<(), String> {
    let docs = DOCUMENTS.lock().map_err(|e| format!("Lock error: {}", e))?;
    let document = docs.get(&document_id).ok_or_else(|| "Document not found".to_string())?;

    let path = document.path().ok_or_else(|| "Document has no path".to_string())?;

    let text = document.text();
    std::fs::write(path, &text).map_err(|e| format!("Failed to save file: {}", e))?;

    Ok(())
}

/// Get the total line count for a document.
#[command]
pub async fn get_line_count(document_id: String) -> Result<usize, String> {
    let docs = DOCUMENTS.lock().map_err(|e| format!("Lock error: {}", e))?;
    let document = docs.get(&document_id).ok_or_else(|| "Document not found".to_string())?;
    Ok(document.len_lines())
}

/// Return the full text content of a document.
#[allow(dead_code)]
#[command]
pub async fn get_document_content(document_id: String) -> Result<String, String> {
    let docs = DOCUMENTS.lock().map_err(|e| format!("Lock error: {}", e))?;
    let document = docs.get(&document_id).ok_or_else(|| "Document not found".to_string())?;
    Ok(document.text().to_string())
}
