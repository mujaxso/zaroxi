//! Workspace daemon - long-running service for workspace operations
//! This daemon handles workspace indexing, file watching, and provides
//! workspace-related services to the desktop app via RPC.

use workspace_daemon::WorkspaceDaemon;
use tracing_subscriber;
use tracing;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    tracing::info!("Starting Zaroxi workspace daemon...");
    tracing::info!("Version: {}", env!("CARGO_PKG_VERSION"));
    
    let mut daemon = WorkspaceDaemon::new();
    
    // TODO: Parse command line arguments for workspace paths
    // For now, just run the daemon
    
    tracing::info!("Workspace daemon ready. Press Ctrl+C to stop.");
    
    if let Err(e) = daemon.run().await {
        tracing::error!("Workspace daemon error: {}", e);
        return Err(e);
    }
    
    tracing::info!("Workspace daemon stopped gracefully");
    Ok(())
}
