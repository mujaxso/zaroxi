//! Zaroxi Desktop - Tauri backend
//!
//! This module contains app-specific orchestration for the Zaroxi IDE.
//! Domain logic resides in the workspace crates (`crates/*`).

mod adapters;
mod app_state;
mod bootstrap;
mod commands;
mod events;
mod menu;
mod zaroxi_infra_permissions;
mod services;
mod windows;

use tauri::{Manager, RunEvent};
use std::sync::Arc;
use crate::services::workspace_service::WorkspaceService;

/// Main entry point for Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize tracing subscriber for logging
    #[cfg(debug_assertions)]
    {
        use tracing_subscriber::{fmt, prelude::*, EnvFilter};
        
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info,workspace_service=debug,tauri=warn"));
        
        tracing_subscriber::registry()
            .with(fmt::layer().with_writer(std::io::stdout))
            .with(filter)
            .init();
        
        tracing::info!("Tracing initialized for Zaroxi Desktop");
    }
    
    tauri::Builder::default()
        .setup(|app| {
            // Create menu for the main window
            let handle = app.handle();
            let menu = menu::create_app_menu(handle)?;
            app.set_menu(menu)?;
            
            // Initialize and manage the workspace service
            let workspace_service = Arc::new(WorkspaceService::new());
            app.manage(workspace_service);
            
            // Initialize theme service
            let theme_service = crate::services::theme_service::ThemeService::new(app.handle().clone());
            
            // Apply initial theme before moving
            theme_service.apply_theme();
            
            // Manage the theme service
            app.manage(theme_service);
            
            // Initialize app state
            let app_state = crate::app_state::AppState::new();
            app.manage(app_state);
            
            tracing::info!("Zaroxi Desktop app setup complete");
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::workspace::open_workspace,
            commands::workspace::list_directory,
            commands::workspace::get_workspace_tree,
            commands::workspace::open_file,
            commands::workspace::save_file,
            commands::workspace::open_file_dialog,
            commands::editor::get_document,
            commands::editor::apply_edit,
            commands::editor::save_document,
            commands::assistant::start_ai_task,
            commands::assistant::cancel_ai_task,
            commands::search::search_workspace,
            commands::preview::generate_preview,
            commands::zaroxi_infra_settings::load_settings,
            commands::zaroxi_infra_settings::save_settings,
            // Theme commands
            commands::zaroxi_infra_settings::load_theme_settings,
            commands::zaroxi_infra_settings::save_theme_settings,
            commands::zaroxi_infra_settings::get_current_theme,
            commands::zaroxi_infra_settings::set_theme,
        ])
        .on_menu_event(|app_handle, event| {
            let app_handle_clone = app_handle.clone();
            match event.id.as_ref() {
                "open_workspace" => {
                    tauri::async_runtime::spawn(async move {
                        let _ = app_handle_clone.emit("menu:open-workspace", ());
                    });
                }
                "open_settings" => {
                    // Emit event to frontend to open settings
                    let _ = app_handle.emit("open-settings", ());
                }
                "theme_system" => {
                    if let Ok(theme_service) = app_handle.try_state::<crate::services::theme_service::ThemeService>() {
                        let _ = theme_service.set_theme_mode(zaroxi_theme::ZaroxiTheme::System);
                    }
                }
                "theme_light" => {
                    if let Ok(theme_service) = app_handle.try_state::<crate::services::theme_service::ThemeService>() {
                        let _ = theme_service.set_theme_mode(zaroxi_theme::ZaroxiTheme::Light);
                    }
                }
                "theme_dark" => {
                    if let Ok(theme_service) = app_handle.try_state::<crate::services::theme_service::ThemeService>() {
                        let _ = theme_service.set_theme_mode(zaroxi_theme::ZaroxiTheme::Dark);
                    }
                }
                "quit" => {
                    app_handle.exit(0);
                }
                _ => {}
            }
        })
        .on_window_event(|window, event| {
            windows::handle_window_event(&window, event);
        })
        .build(tauri::generate_context!())
        .expect("Failed to build Tauri application")
        .run(|app_handle, event| match event {
            RunEvent::Ready => {
                tracing::info!("App is ready");
                
                // Initialize services after app is ready
                if let Err(e) = bootstrap::setup::on_app_ready(app_handle) {
                    tracing::error!("Failed to initialize app: {}", e);
                }
                
                // Start the workspace service
                // We can get the workspace service from app state
                // For now, we'll start it when needed
            }
            RunEvent::Exit => {
                tracing::info!("App is exiting");
                // Cleanup resources
                events::emitter::broadcast_shutdown(app_handle);
            }
            _ => {}
        });
}
