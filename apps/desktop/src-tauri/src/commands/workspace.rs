use std::path::PathBuf;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tauri::command;
use tauri::{AppHandle, State};

use crate::services::workspace_service::WorkspaceService;

#[derive(Debug, Deserialize)]
pub struct OpenWorkspaceRequest {
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct OpenWorkspaceResponse {
    pub workspace_id: String,
    pub root_path: String,
    pub file_count: usize,
}

#[command]
pub async fn open_workspace(
    request: OpenWorkspaceRequest,
    workspace_service: State<'_, Arc<WorkspaceService>>,
    app_handle: AppHandle,
) -> Result<OpenWorkspaceResponse, String> {
    let path = PathBuf::from(&request.path);
    
    // Validate path exists
    if !path.exists() {
        return Err(format!("Path does not exist: {}", request.path));
    }
    
    // Open workspace using the service
    let workspace = workspace_service.open_workspace(path)
        .await
        .map_err(|e| format!("Failed to open workspace: {}", e))?;
    
    // Convert to DTO
    let response = crate::adapters::workspace_adapter::domain_workspace_to_dto(&workspace);
    
    // Emit event
    let emitter = crate::events::workspace_events::WorkspaceEventEmitter::new(&app_handle);
    if let Err(e) = emitter.emit_workspace_opened(&workspace.id.to_string(), &workspace.root_path) {
        eprintln!("Failed to emit workspace opened event: {}", e);
    }
    
    Ok(response)
}

#[derive(Debug, Deserialize)]
pub struct ListDirectoryRequest {
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct DirectoryEntryDto {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub file_type: Option<String>,
    pub size: Option<u64>,
    pub modified: Option<String>,
}

#[command]
pub async fn list_directory(
    request: ListDirectoryRequest,
    workspace_service: State<'_, Arc<WorkspaceService>>,
) -> Result<Vec<DirectoryEntryDto>, String> {
    let path = PathBuf::from(&request.path);
    
    // Validate path exists
    if !path.exists() {
        return Err(format!("Path does not exist: {}", request.path));
    }
    
    // List directory using the service
    let entries = workspace_service.list_directory(path)
        .await
        .map_err(|e| format!("Failed to list directory: {}", e))?;
    
    // Convert to DTOs
    let dtos = entries.iter()
        .map(crate::adapters::workspace_adapter::file_entry_to_dto)
        .collect();
    
    Ok(dtos)
}

#[derive(Debug, Deserialize)]
pub struct OpenFileRequest {
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct OpenFileResponse {
    pub content: String,
    pub language: Option<String>,
}

#[command]
pub async fn open_file(request: OpenFileRequest) -> Result<OpenFileResponse, String> {
    use std::fs;
    
    let path = PathBuf::from(&request.path);
    
    // Read file content
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    // Determine language from file extension
    let language = path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_string());
    
    Ok(OpenFileResponse {
        content,
        language,
    })
}

#[derive(Debug, Deserialize)]
pub struct SaveFileRequest {
    pub path: String,
    pub content: String,
}

#[command]
pub async fn save_file(request: SaveFileRequest) -> Result<(), String> {
    use std::fs;
    
    let path = PathBuf::from(&request.path);
    
    // Write file content
    fs::write(&path, request.content)
        .map_err(|e| format!("Failed to save file: {}", e))?;
    
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct OpenDialogResponse {
    pub selected_path: Option<String>,
}

#[command]
pub async fn open_file_dialog() -> Result<OpenDialogResponse, String> {
    use rfd::AsyncFileDialog;
    
    // Open a directory picker dialog
    let handle = AsyncFileDialog::new()
        .set_title("Select Workspace Directory")
        .pick_folder()
        .await;
    
    let selected_path = handle.map(|handle| handle.path().to_string_lossy().to_string());
    
    Ok(OpenDialogResponse {
        selected_path,
    })
}
