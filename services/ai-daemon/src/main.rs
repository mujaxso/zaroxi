//! AI daemon - long-running service for AI task processing
//! This daemon handles AI task execution, model management, and provides
//! AI services to the desktop app via RPC.

use ai_daemon::AiDaemon;
use tracing_subscriber;
use tracing;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    tracing::info!("Starting Zaroxi AI daemon...");
    tracing::info!("Version: {}", env!("CARGO_PKG_VERSION"));
    
    let daemon = AiDaemon::new();
    
    tracing::info!("AI daemon ready. Press Ctrl+C to stop.");
    
    if let Err(e) = daemon.run().await {
        tracing::error!("AI daemon error: {}", e);
        return Err(e);
    }
    
    tracing::info!("AI daemon stopped gracefully");
    Ok(())
}
