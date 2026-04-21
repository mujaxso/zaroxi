/// Settings commands for Tauri.

use tauri::command;
use zaroxi_theme::{ThemeSettings, ZaroxiTheme};
use std::sync::Mutex;
use tauri::State;

/// App state for theme settings
pub struct ThemeState {
    settings: Mutex<ThemeSettings>,
}

impl ThemeState {
    pub fn new() -> Self {
        Self {
            settings: Mutex::new(ThemeSettings::default()),
        }
    }
}

#[command]
pub async fn load_theme_settings() -> Result<ThemeSettings, String> {
    // TODO: Implement actual settings loading from disk
    // For now, return default (System theme)
    Ok(ThemeSettings::default())
}

#[command]
pub async fn save_theme_settings(settings: ThemeSettings) -> Result<(), String> {
    // TODO: Implement actual settings saving to disk
    println!("Saving theme settings: {:?}", settings);
    Ok(())
}

#[command]
pub async fn get_current_theme() -> Result<ZaroxiTheme, String> {
    // TODO: Load from actual settings
    Ok(ZaroxiTheme::System)
}

#[command]
pub async fn set_theme(theme: ZaroxiTheme) -> Result<(), String> {
    // TODO: Save to settings
    println!("Setting theme to: {:?}", theme);
    Ok(())
}
