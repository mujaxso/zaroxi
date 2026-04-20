//! Identifier types for Zaroxi Studio.
//!
//! Defines strongly-typed identifiers for various entities in the system
//! (documents, users, sessions, etc.) to prevent mixing different ID types.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A strongly-typed buffer identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(pub Uuid);

/// A strongly-typed workspace identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkspaceId(pub Uuid);

impl BufferId {
    /// Create a new unique buffer ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl WorkspaceId {
    /// Create a new unique workspace ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for BufferId {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for WorkspaceId {
    fn default() -> Self {
        Self::new()
    }
}
