//! Persistence for editor typography and theme settings.
//!
//! Handles saving and loading editor font settings and theme preference to/from disk.

use std::fs;
use std::path::PathBuf;
use crate::settings::editor::EditorTypographySettings;
use crate::theme::QyzerTheme;

const SETTINGS_FILE_NAME: &str = "neote_settings.json";

/// Settings that can be persisted
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AppSettings {
    pub typography: EditorTypographySettings,
    pub theme_preference: QyzerTheme,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            typography: EditorTypographySettings::default(),
            theme_preference: QyzerTheme::System,
        }
    }
}

/// Get the path to the settings file in the user's config directory.
fn settings_path() -> Result<PathBuf, String> {
    let mut path = dirs::config_dir()
        .ok_or_else(|| "Could not find config directory".to_string())?;
    
    path.push(crate::brand::CONFIG_DIR_NAME);
    fs::create_dir_all(&path).map_err(|e| format!("Failed to create config directory: {}", e))?;
    
    path.push(SETTINGS_FILE_NAME);
    Ok(path)
}

/// Save app settings to disk.
pub fn save_settings(typography: &EditorTypographySettings, theme_preference: QyzerTheme) -> Result<(), String> {
    let path = settings_path()?;
    
    let settings = AppSettings {
        typography: typography.clone(),
        theme_preference,
    };
    
    let json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    
    fs::write(&path, json)
        .map_err(|e| format!("Failed to write settings to {}: {}", path.display(), e))?;
    
    Ok(())
}

/// Load app settings from disk.
/// Returns default settings if file doesn't exist or can't be read.
pub fn load_settings() -> Result<(EditorTypographySettings, QyzerTheme), String> {
    let path = settings_path()?;
    
    if !path.exists() {
        return Ok((EditorTypographySettings::default(), QyzerTheme::System));
    }
    
    let json = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read settings from {}: {}", path.display(), e))?;
    
    let settings: AppSettings = serde_json::from_str(&json)
        .map_err(|e| format!("Failed to parse settings JSON: {}", e))?;
    
    // Validate loaded typography settings
    let mut typography = settings.typography;
    typography.validate();
    
    Ok((typography, settings.theme_preference))
}
