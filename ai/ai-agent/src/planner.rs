//! AI task planning and decomposition

use serde::{Deserialize, Serialize};

/// A planned step in an AI task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStep {
    /// Step description
    pub description: String,
    /// Expected outcome
    pub expected_outcome: String,
    /// Whether this step is completed
    pub completed: bool,
}

/// A plan for executing an AI task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPlan {
    /// Task ID
    pub task_id: String,
    /// Steps to complete the task
    pub steps: Vec<TaskStep>,
    /// Current step index
    pub current_step: usize,
}

impl TaskPlan {
    /// Create a new task plan
    pub fn new(task_id: String) -> Self {
        Self {
            task_id,
            steps: Vec::new(),
            current_step: 0,
        }
    }
    
    /// Add a step to the plan
    pub fn add_step(&mut self, description: String, expected_outcome: String) {
        self.steps.push(TaskStep {
            description,
            expected_outcome,
            completed: false,
        });
    }
}
