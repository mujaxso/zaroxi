//! Workspace model for Zaroxi

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    /// Unique identifier
    pub id: Uuid,
    /// Workspace name
    pub name: String,
    /// Root path
    pub root_path: String,
}

impl Workspace {
    /// Create a new workspace
    pub fn new(name: String, root_path: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            root_path,
        }
    }
}
