use tauri::AppHandle;
use crate::services::theme_service::ThemeService;

pub fn on_app_ready(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // App is ready, start services
    let theme_service = ThemeService::new(app_handle.clone());
    
    // Apply theme on startup
    tauri::async_runtime::spawn(async move {
        theme_service.apply_theme().await;
    });
    
    Ok(())
}
