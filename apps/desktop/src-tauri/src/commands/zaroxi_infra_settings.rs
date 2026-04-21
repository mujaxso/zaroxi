/// Settings commands for Tauri.

use tauri::command;
use zaroxi_theme::{ThemeSettings, ZaroxiTheme};

#[command]
pub async fn load_settings() -> Result<serde_json::Value, String> {
    // TODO: Implement actual settings loading
    Ok(serde_json::json!({
        "theme": "system",
        "editor": {
            "font_size": 14,
            "line_height": 1.5
        }
    }))
}

#[command]
pub async fn save_settings(settings: serde_json::Value) -> Result<(), String> {
    // TODO: Implement actual settings saving
    println!("Saving settings: {:?}", settings);
    Ok(())
}

use std::fs;
use std::path::PathBuf;
use tauri::api::path::app_config_dir;

#[command]
pub async fn load_theme_settings() -> Result<ThemeSettings, String> {
    // Get config directory
    let config_dir = app_config_dir(&tauri::Config::default())
        .ok_or_else(|| "Failed to get config directory".to_string())?;
    
    let theme_path = config_dir.join("theme_settings.json");
    
    if !theme_path.exists() {
        return Ok(ThemeSettings::default());
    }
    
    let content = fs::read_to_string(&theme_path)
        .map_err(|e| format!("Failed to read theme settings: {}", e))?;
    
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse theme settings: {}", e))
}

#[command]
pub async fn save_theme_settings(settings: ThemeSettings) -> Result<(), String> {
    // Get config directory
    let config_dir = app_config_dir(&tauri::Config::default())
        .ok_or_else(|| "Failed to get config directory".to_string())?;
    
    // Create config directory if it doesn't exist
    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;
    
    let theme_path = config_dir.join("theme_settings.json");
    
    let content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize theme settings: {}", e))?;
    
    fs::write(&theme_path, content)
        .map_err(|e| format!("Failed to write theme settings: {}", e))?;
    
    Ok(())
}

#[command]
pub async fn get_current_theme() -> Result<ZaroxiTheme, String> {
    let settings = load_theme_settings().await?;
    Ok(settings.theme_mode)
}

#[command]
pub async fn set_theme(theme: ZaroxiTheme) -> Result<(), String> {
    let settings = ThemeSettings {
        theme_mode: theme,
    };
    save_theme_settings(settings).await
}
