use std::path::PathBuf;
use std::sync::Arc;
use anyhow::Result;
use tokio::sync::Mutex;
use tracing::info;

/// App-specific workspace service that orchestrates domain workspace logic
pub struct WorkspaceService {
    domain_service: Arc<zaroxi_service_workspace::service::WorkspaceService>,
    // App-specific state (e.g., active workspace)
    active_workspace: Mutex<Option<zaroxi_domain_workspace::workspace::Workspace>>,
}

impl WorkspaceService {
    pub fn new() -> Self {
        Self {
            domain_service: Arc::new(zaroxi_service_workspace::service::WorkspaceService::new()),
            active_workspace: Mutex::new(None),
        }
    }

    /// Open a workspace at the given path
    pub async fn open_workspace(&self, path: PathBuf) -> Result<zaroxi_domain_workspace::workspace::Workspace> {
        info!("Opening workspace at path: {:?}", path);
        
        // Delegate to the domain service
        let workspace = self.domain_service.open_workspace(path).await?;
        
        // Update app state
        let mut active = self.active_workspace.lock().await;
        *active = Some(workspace.clone());
        
        info!("Workspace opened: {} ({})", workspace.name, workspace.id);
        Ok(workspace)
    }

    /// List directory contents using domain logic when available
    pub async fn list_directory(&self, path: PathBuf) -> Result<Vec<FileEntry>> {
        use std::fs;
        
        // Validate path exists
        if !path.exists() {
            return Err(anyhow::anyhow!("Path does not exist: {:?}", path));
        }
        if !path.is_dir() {
            return Err(anyhow::anyhow!("Path is not a directory: {:?}", path));
        }

        let mut entries = Vec::new();
        
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;
            
            // Determine file type using domain logic when available
            let file_type = if metadata.is_dir() {
                None
            } else {
                path.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|s| s.to_string())
            };
            
            entries.push(FileEntry {
                path: path.to_string_lossy().to_string(),
                name: path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string(),
                is_dir: metadata.is_dir(),
                file_type,
                size: if metadata.is_dir() { None } else { Some(metadata.len()) },
                modified: metadata.modified().ok(),
            });
        }
        
        // Sort: directories first, then files
        entries.sort_by(|a, b| {
            if a.is_dir && !b.is_dir {
                std::cmp::Ordering::Less
            } else if !a.is_dir && b.is_dir {
                std::cmp::Ordering::Greater
            } else {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });
        
        Ok(entries)
    }

    /// Get active workspace
    pub async fn get_active_workspace(&self) -> Option<zaroxi_domain_workspace::workspace::Workspace> {
        let active = self.active_workspace.lock().await;
        active.clone()
    }
}

/// File entry for directory listing with app-specific metadata
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub file_type: Option<String>,
    pub size: Option<u64>,
    pub modified: Option<std::time::SystemTime>,
}
