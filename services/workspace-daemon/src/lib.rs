//! Workspace daemon library
//! Long-running service for workspace indexing and file watching

use std::path::PathBuf;
use tokio::sync::mpsc;
use tracing::{info, error};

// Import from new crate structure
use domain::workspace_model::Workspace as DomainWorkspace;
use operations::file_ops;

pub struct WorkspaceDaemon {
    workspaces: Vec<Workspace>,
}

struct Workspace {
    id: String,
    path: PathBuf,
    watcher: Option<file_ops::FileWatcher>,
    domain_workspace: DomainWorkspace,
}

impl WorkspaceDaemon {
    pub fn new() -> Self {
        Self {
            workspaces: Vec::new(),
        }
    }
    
    pub async fn run(&self) -> Result<(), anyhow::Error> {
        info!("Workspace daemon running");
        
        // TODO: Implement RPC server using infrastructure::rpc
        // TODO: Handle workspace indexing
        // TODO: Watch for file changes
        
        // Keep the daemon running until shutdown signal
        tokio::signal::ctrl_c().await?;
        info!("Received shutdown signal");
        
        Ok(())
    }
    
    pub async fn add_workspace(&mut self, path: PathBuf) -> Result<String, anyhow::Error> {
        let id = uuid::Uuid::new_v4().to_string();
        
        // Create domain workspace
        let workspace_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();
        
        let domain_workspace = DomainWorkspace::new(workspace_name, path.to_string_lossy().to_string());
        
        let workspace = Workspace {
            id: id.clone(),
            path: path.clone(),
            watcher: None,
            domain_workspace,
        };
        
        self.workspaces.push(workspace);
        info!("Added workspace: {} at {}", id, path.display());
        
        // TODO: Start watching the workspace
        // TODO: Index the workspace
        
        Ok(id)
    }
    
    pub async fn list_workspaces(&self) -> Vec<DomainWorkspace> {
        self.workspaces.iter()
            .map(|w| w.domain_workspace.clone())
            .collect()
    }
    
    pub async fn get_workspace(&self, id: &str) -> Option<&DomainWorkspace> {
        self.workspaces.iter()
            .find(|w| w.id == id)
            .map(|w| &w.domain_workspace)
    }
}
