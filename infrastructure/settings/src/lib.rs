//! Settings management for Zaroxi

use serde::{Deserialize, Serialize};

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Editor settings
    pub editor: EditorSettings,
    /// AI settings
    pub ai: AiSettings,
}

/// Editor settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    /// Font size
    pub font_size: u32,
    /// Theme
    pub theme: String,
}

/// AI settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSettings {
    /// Provider
    pub provider: String,
    /// Model
    pub model: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            editor: EditorSettings {
                font_size: 14,
                theme: "dark".to_string(),
            },
            ai: AiSettings {
                provider: "openai".to_string(),
                model: "gpt-4".to_string(),
            },
        }
    }
}
