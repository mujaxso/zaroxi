use tauri::{WebviewWindow, Window, WindowEvent};

pub fn handle_window_event<R: tauri::Runtime>(_window: &Window<R>, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api: _, .. } => {
            // You can prevent closing here if needed
            // api.prevent_close();
        }
        _ => {}
    }
}

pub fn setup_window<R: tauri::Runtime>(
    window: &WebviewWindow<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Remove native window decorations to use our custom title bar
    // Note: This is already set in tauri.conf.json, but we keep it here for safety
    window.set_decorations(false)?;

    // Make the window background transparent for custom styling
    // This helps with custom title bar integration
    // Note: set_transparent may not be available in all Tauri versions
    // Since we already set transparent: true in tauri.conf.json, we can skip this
    // window.set_transparent(true)?;

    Ok(())
}
