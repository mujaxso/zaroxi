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
mod permissions;
mod services;
mod windows;

use app_state::AppState;
use tauri::{Manager, RunEvent};

/// Main entry point for Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(bootstrap::setup::init_app)
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::workspace::open_workspace,
            commands::workspace::list_directory,
            commands::workspace::open_file,
            commands::workspace::save_file,
            commands::editor::get_document,
            commands::editor::apply_edit,
            commands::editor::save_document,
            commands::assistant::start_ai_task,
            commands::assistant::cancel_ai_task,
            commands::search::search_workspace,
            commands::preview::generate_preview,
            commands::settings::load_settings,
            commands::settings::save_settings,
        ])
        .menu(menu::create_app_menu())
        .on_window_event(|window, event| {
            windows::handle_window_event(&window, event);
        })
        .build(tauri::generate_context!())
        .expect("Failed to build Tauri application")
        .run(|app_handle, event| match event {
            RunEvent::Ready => {
                // Initialize services after app is ready
                if let Err(e) = bootstrap::setup::on_app_ready(app_handle) {
                    eprintln!("Failed to initialize app: {}", e);
                }
            }
            RunEvent::Exit => {
                // Cleanup resources
                events::emitter::broadcast_shutdown(app_handle);
            }
            _ => {}
        });
}
