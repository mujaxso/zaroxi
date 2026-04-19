//! AI Context for Zaroxi
//! 
//! This crate provides AI context collection and management.

use serde::{Deserialize, Serialize};

/// An AI task to be executed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiTask {
    /// Task ID
    pub id: String,
    /// Task description
    pub description: String,
    /// Task context
    pub context: String,
    /// Additional parameters
    pub parameters: serde_json::Value,
}

/// Status of an AI task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Task is pending
    Pending,
    /// Task is in progress
    InProgress,
    /// Task completed successfully
    Completed,
    /// Task failed with error message
    Failed(String),
}

impl AiTask {
    /// Create a new AI task
    pub fn new(id: String, description: String, context: String) -> Self {
        Self {
            id,
            description,
            context,
            parameters: serde_json::json!({}),
        }
    }
}
