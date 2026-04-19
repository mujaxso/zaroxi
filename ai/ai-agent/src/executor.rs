//! AI task executor

use crate::AiAgent;

/// Executes AI tasks
pub struct TaskExecutor {
    agent: AiAgent,
}

impl TaskExecutor {
    /// Create a new task executor
    pub fn new(agent: AiAgent) -> Self {
        Self { agent }
    }
    
    /// Execute a task
    pub async fn execute(&self, task: ai_context::AiTask) -> Result<crate::TaskResult, anyhow::Error> {
        self.agent.execute_task(task).await
    }
}
