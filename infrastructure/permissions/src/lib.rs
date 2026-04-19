//! Permissions system for Zaroxi

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A permission grant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grant {
    /// Unique identifier
    pub id: Uuid,
    /// Resource
    pub resource: String,
    /// Action
    pub action: String,
}

impl Grant {
    /// Create a new grant
    pub fn new(resource: String, action: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            resource,
            action,
        }
    }
}
