//! Tools available to the AI agent

use serde::{Deserialize, Serialize};

/// A tool that can be used by the AI agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Parameters schema (JSON Schema)
    pub parameters: serde_json::Value,
}

impl Tool {
    /// Create a new tool
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        }
    }
}

/// Registry of available tools
pub struct ToolRegistry {
    tools: Vec<Tool>,
}

impl ToolRegistry {
    /// Create a new tool registry
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }
    
    /// Register a tool
    pub fn register(&mut self, tool: Tool) {
        self.tools.push(tool);
    }
    
    /// Get all registered tools
    pub fn tools(&self) -> &[Tool] {
        &self.tools
    }
}
