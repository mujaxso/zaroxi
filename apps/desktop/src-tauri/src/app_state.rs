// App state for Zaroxi Desktop
use std::sync::Mutex;
use zaroxi_theme::{ThemeManager, ThemeSettings};

#[derive(Default)]
pub struct AppState {
    theme_manager: Mutex<ThemeManager>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            theme_manager: Mutex::new(ThemeManager::new()),
        }
    }
    
    pub fn get_theme_manager(&self) -> &Mutex<ThemeManager> {
        &self.theme_manager
    }
    
    pub fn update_theme_settings(&self, settings: ThemeSettings) -> Result<(), String> {
        let mut manager = self.theme_manager.lock().map_err(|e| e.to_string())?;
        manager.update_settings(settings);
        Ok(())
    }
}
