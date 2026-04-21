//! Theme service for desktop app orchestration
use tauri::{AppHandle, Emitter, Manager};
use zaroxi_theme::ZaroxiTheme;

/// Theme service for desktop-specific theme orchestration
#[derive(Clone)]
pub struct ThemeService {
    app_handle: AppHandle,
}

impl ThemeService {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }
    
    /// Get the current theme mode, considering system preference
    pub fn current_theme_mode(&self) -> ZaroxiTheme {
        // TODO: Load from persisted settings
        ZaroxiTheme::System
    }
    
    /// Check if system prefers dark mode
    pub fn system_prefers_dark(&self) -> bool {
        // Try to get from main window
        if let Some(window) = self.app_handle.get_webview_window("main") {
            match window.theme() {
                Ok(tauri::Theme::Dark) => return true,
                Ok(tauri::Theme::Light) => return false,
                _ => {}
            }
        }
        
        // Fallback to checking system preference via darkreader
        // For now, default to dark for consistency
        true
    }
    
    /// Apply theme to the app
    pub fn apply_theme(&self) {
        let theme_mode = self.current_theme_mode();
        let is_dark = match theme_mode {
            ZaroxiTheme::Dark => true,
            ZaroxiTheme::Light => false,
            ZaroxiTheme::System => self.system_prefers_dark(),
        };
        
        // Emit event to frontend with theme data
        let _ = self.app_handle.emit(
            "theme:changed",
            serde_json::json!({
                "mode": theme_mode,
                "isDark": is_dark,
            }),
        );
    }
    
    /// Set theme mode and persist it
    pub fn set_theme_mode(&self, theme_mode: ZaroxiTheme) -> Result<(), String> {
        // TODO: Persist to settings
        // For now, just emit event
        let is_dark = match theme_mode {
            ZaroxiTheme::Dark => true,
            ZaroxiTheme::Light => false,
            ZaroxiTheme::System => self.system_prefers_dark(),
        };
        
        let _ = self.app_handle.emit(
            "theme:changed",
            serde_json::json!({
                "mode": theme_mode,
                "isDark": is_dark,
            }),
        );
        
        Ok(())
    }
}
