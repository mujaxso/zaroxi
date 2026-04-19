//! AI daemon library
//! Long-running service for AI task processing

use tokio::sync::mpsc;
use tracing::{info, error};

// Import from new crate structure
use domain::ai_context::{AiTask, TaskStatus};
use ai::ai_agent;

pub struct AiDaemon {
    agent: ai_agent::AiAgent,
    task_queue: mpsc::Receiver<AiTask>,
    task_results: mpsc::Sender<TaskResult>,
}

#[derive(Debug, Clone)]
pub struct TaskResult {
    pub task_id: String,
    pub output: String,
    pub status: TaskStatus,
}

impl AiDaemon {
    pub fn new() -> Self {
        let agent = ai_agent::AiAgent::new(
            "openai".to_string(),
            "gpt-4".to_string(),
        );
        
        let (_, task_queue) = mpsc::channel(100);
        let (task_results, _) = mpsc::channel(100);
        
        Self {
            agent,
            task_queue,
            task_results,
        }
    }
    
    pub async fn run(mut self) -> Result<(), anyhow::Error> {
        info!("AI daemon running");
        
        // TODO: Implement RPC server using infrastructure::rpc
        // TODO: Process AI tasks from queue
        
        // Process tasks from queue
        while let Some(task) = self.task_queue.recv().await {
            info!("Processing AI task: {}", task.id);
            
            match self.agent.execute_task(task.clone()).await {
                Ok(result) => {
                    info!("AI task completed: {}", result.task_id);
                    
                    // Send result back
                    let task_result = TaskResult {
                        task_id: result.task_id,
                        output: result.output,
                        status: result.status,
                    };
                    
                    if let Err(e) = self.task_results.send(task_result).await {
                        error!("Failed to send task result: {}", e);
                    }
                }
                Err(e) => {
                    error!("AI task failed: {}", e);
                    
                    // Send error result
                    let task_result = TaskResult {
                        task_id: task.id,
                        output: format!("Error: {}", e),
                        status: TaskStatus::Failed(e.to_string()),
                    };
                    
                    if let Err(e) = self.task_results.send(task_result).await {
                        error!("Failed to send error result: {}", e);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    pub async fn submit_task(&self, task: AiTask) -> Result<(), anyhow::Error> {
        // TODO: Implement task submission via channel
        // For now, just log
        info!("Would submit task: {}", task.id);
        Ok(())
    }
}
