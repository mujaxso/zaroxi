//! Workspace service implementation.

use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use anyhow::Result;

/// Workspace service for handling workspace operations.
pub struct WorkspaceService {
    /// Internal state.
    state: Arc<Mutex<WorkspaceServiceState>>,
}

struct WorkspaceServiceState {
    /// Whether the service is running.
    running: bool,
}

impl WorkspaceService {
    /// Create a new workspace service.
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(WorkspaceServiceState { running: false })),
        }
    }

    /// Start the workspace service.
    pub async fn start(&self) -> Result<()> {
        let mut state = self.state.lock().await;
        if state.running {
            return Err(anyhow::anyhow!("Workspace service is already running"));
        }
        state.running = true;
        info!("Workspace service started");
        Ok(())
    }

    /// Stop the workspace service.
    pub async fn stop(&self) -> Result<()> {
        let mut state = self.state.lock().await;
        if !state.running {
            return Err(anyhow::anyhow!("Workspace service is not running"));
        }
        state.running = false;
        info!("Workspace service stopped");
        Ok(())
    }

    /// Check if the service is running.
    pub async fn is_running(&self) -> bool {
        let state = self.state.lock().await;
        state.running
    }

    /// Open a workspace at the given path
    pub async fn open_workspace(&self, path: std::path::PathBuf) -> Result<zaroxi_domain_workspace::workspace::Workspace> {
        use zaroxi_domain_workspace::workspace::Workspace;
        use uuid::Uuid;
        use chrono::Utc;
        
        // Validate path exists
        if !path.exists() {
            return Err(anyhow::anyhow!("Path does not exist: {:?}", path));
        }
        if !path.is_dir() {
            return Err(anyhow::anyhow!("Path is not a directory: {:?}", path));
        }

        let now = Utc::now();
        let workspace = Workspace {
            id: Uuid::new_v4(),
            root_path: path.to_string_lossy().to_string(),
            name: path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("workspace")
                .to_string(),
            is_open: true,
            created_at: now,
            last_accessed_at: now,
        };
        
        info!("Opened workspace: {} at {:?}", workspace.name, workspace.root_path);
        Ok(workspace)
    }
}
