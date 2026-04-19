//! AI Agent for Zaroxi
//! 
//! This crate provides AI task orchestration and execution capabilities.

pub mod executor;
pub mod planner;
pub mod tools;
pub mod verify;
pub mod patch;

use serde::{Deserialize, Serialize};

/// AI Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAgentConfig {
    /// Provider name (e.g., "openai", "anthropic", "local")
    pub provider: String,
    /// Model name (e.g., "gpt-4", "claude-3-opus")
    pub model: String,
    /// API key (optional for local models)
    pub api_key: Option<String>,
    /// Maximum tokens per request
    pub max_tokens: usize,
}

/// AI Agent for executing tasks
pub struct AiAgent {
    config: AiAgentConfig,
}

impl AiAgent {
    /// Create a new AI agent with the given configuration
    pub fn new(provider: String, model: String) -> Self {
        Self {
            config: AiAgentConfig {
                provider,
                model,
                api_key: None,
                max_tokens: 4096,
            },
        }
    }
    
    /// Create a new AI agent with full configuration
    pub fn with_config(config: AiAgentConfig) -> Self {
        Self { config }
    }
    
    /// Execute an AI task
    pub async fn execute_task(&self, task: ai_context::AiTask) -> Result<TaskResult, anyhow::Error> {
        // TODO: Implement actual AI task execution
        Ok(TaskResult {
            task_id: task.id,
            output: "Task executed successfully".to_string(),
            status: ai_context::TaskStatus::Completed,
        })
    }
}

/// Result of an AI task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// Task ID
    pub task_id: String,
    /// Output text
    pub output: String,
    /// Task status
    pub status: ai_context::TaskStatus,
}
