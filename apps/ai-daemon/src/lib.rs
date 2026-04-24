//! AI daemon library
//! Long-running service for AI task processing

use tokio::sync::mpsc;
use tracing::{error, info};

pub struct AiDaemon {
    // TODO: Add actual AI agent
    task_queue: mpsc::Receiver<()>,
    task_results: mpsc::Sender<()>,
}

impl AiDaemon {
    pub fn new() -> Self {
        let (_, task_queue) = mpsc::channel(100);
        let (task_results, _) = mpsc::channel(100);

        Self { task_queue, task_results }
    }

    pub async fn run(mut self) -> Result<(), anyhow::Error> {
        info!("AI daemon running");

        // TODO: Implement RPC server using infrastructure::zaroxi_infra_rpc
        // TODO: Process AI tasks from queue

        // Keep the daemon running until shutdown signal
        tokio::signal::ctrl_c().await?;
        info!("Received shutdown signal");

        Ok(())
    }
}
