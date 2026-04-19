//! Patch engine for Zaroxi

use serde::{Deserialize, Serialize};

/// A patch to apply to a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
    /// The changes to apply
    pub changes: String,
}

impl Patch {
    /// Create a new patch
    pub fn new(changes: String) -> Self {
        Self { changes }
    }
}
