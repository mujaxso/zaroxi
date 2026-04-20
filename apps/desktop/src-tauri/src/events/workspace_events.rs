//! Workspace event definitions for Tauri events

use serde::Serialize;
use tauri::Emitter;

/// Workspace events that can be emitted to the frontend
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum WorkspaceEvent {
    /// Emitted when a workspace is opened
    WorkspaceOpened {
        workspace_id: String,
        root_path: String,
    },
    /// Emitted when a workspace is closed
    #[allow(dead_code)]
    WorkspaceClosed {
        workspace_id: String,
    },
    /// Emitted when directory contents change
    #[allow(dead_code)]
    DirectoryChanged {
        path: String,
    },
    /// Emitted when a file is created/modified/deleted
    #[allow(dead_code)]
    FileSystemChanged {
        path: String,
        change_type: FileSystemChangeType,
    },
}

/// Type of file system change
#[derive(Debug, Clone, Serialize)]
pub enum FileSystemChangeType {
    #[allow(dead_code)]
    Created,
    #[allow(dead_code)]
    Modified,
    #[allow(dead_code)]
    Deleted,
    #[allow(dead_code)]
    Renamed { old_path: String },
}

/// Helper to emit workspace events
pub struct WorkspaceEventEmitter<'a> {
    app_handle: &'a tauri::AppHandle,
}

impl<'a> WorkspaceEventEmitter<'a> {
    pub fn new(app_handle: &'a tauri::AppHandle) -> Self {
        Self { app_handle }
    }

    pub fn emit_workspace_opened(&self, workspace_id: &str, root_path: &str) -> tauri::Result<()> {
        self.app_handle.emit(
            "workspace:opened",
            WorkspaceEvent::WorkspaceOpened {
                workspace_id: workspace_id.to_string(),
                root_path: root_path.to_string(),
            },
        )
    }

    #[allow(dead_code)]
    pub fn emit_directory_changed(&self, path: &str) -> tauri::Result<()> {
        self.app_handle.emit(
            "workspace:directory_changed",
            WorkspaceEvent::DirectoryChanged {
                path: path.to_string(),
            },
        )
    }
}
