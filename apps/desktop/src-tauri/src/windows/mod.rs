use tauri::Window;
use tauri::app::WindowEvent;

pub fn handle_window_event<R: tauri::Runtime>(_window: &tauri::Window<R>, _event: &WindowEvent) {
    // Handle window events here
}
