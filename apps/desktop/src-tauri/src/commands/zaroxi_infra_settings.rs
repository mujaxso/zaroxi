/// Settings commands for Tauri.

use tauri::command;

#[command]
pub async fn load_settings() -> Result<serde_json::Value, String> {
    // TODO: Implement actual settings loading
    Ok(serde_json::json!({
        "theme": "dark",
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
