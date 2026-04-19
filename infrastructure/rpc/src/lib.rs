//! RPC framework for Zaroxi

use serde::{Deserialize, Serialize};

/// An RPC request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    /// Method name
    pub method: String,
    /// Parameters
    pub params: serde_json::Value,
}

/// An RPC response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// Result
    pub result: serde_json::Value,
    /// Error, if any
    pub error: Option<String>,
}
