//! Workspace daemon library
//! Long-running service for workspace indexing and file watching

use std::path::PathBuf;
use tokio::sync::mpsc;
use tracing::{error, info};

pub struct WorkspaceDaemon {
    workspaces: Vec<Workspace>,
}

struct Workspace {
    id: String,
    path: PathBuf,
    // watcher: Option<zaroxi_ops_file::FileWatcher>,
    // domain_workspace: DomainWorkspace,
}

impl WorkspaceDaemon {
    pub fn new() -> Self {
        Self { workspaces: Vec::new() }
    }

    pub async fn run(&self) -> Result<(), anyhow::Error> {
        info!("Workspace daemon running");

        // TODO: Implement RPC server using infrastructure::zaroxi_infra_rpc
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
        let workspace_name =
            path.file_name().and_then(|n| n.to_str()).unwrap_or("Untitled").to_string();

        // TODO: Create domain workspace
        // let domain_workspace = DomainWorkspace::new(workspace_name, path.to_string_lossy().to_string());

        let workspace = Workspace {
            id: id.clone(),
            path: path.clone(),
            // watcher: None,
            // domain_workspace,
        };

        self.workspaces.push(workspace);
        info!("Added workspace: {} at {}", id, path.display());

        // TODO: Start watching the workspace
        // TODO: Index the workspace

        Ok(id)
    }

    pub async fn list_workspaces(&self) -> Vec<String> {
        self.workspaces.iter().map(|w| w.id.clone()).collect()
    }

    pub async fn get_workspace(&self, id: &str) -> Option<&Workspace> {
        self.workspaces.iter().find(|w| w.id == id)
    }
}
