use std::env;
use std::fs;
use std::path::PathBuf;
/// Settings commands for Tauri.
use tauri::command;
use zaroxi_theme::{ThemeSettings, ZaroxiTheme};

fn get_config_dir() -> Result<PathBuf, String> {
    // Try to get config directory using platform-specific methods
    let mut config_dir = if cfg!(target_os = "windows") {
        env::var_os("APPDATA")
            .map(PathBuf::from)
            .ok_or_else(|| "APPDATA environment variable not found".to_string())?
    } else if cfg!(target_os = "macos") {
        let mut home = env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or_else(|| "HOME environment variable not found".to_string())?;
        home.push("Library");
        home.push("Application Support");
        home
    } else {
        // Linux and other Unix-like systems
        env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                env::var_os("HOME").map(|home| {
                    let mut path = PathBuf::from(home);
                    path.push(".config");
                    path
                })
            })
            .ok_or_else(|| "Could not find config directory".to_string())?
    };

    config_dir.push("zaroxi");
    Ok(config_dir)
}

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

#[command]
pub async fn load_theme_settings() -> Result<ThemeSettings, String> {
    match get_config_dir() {
        Ok(config_dir) => {
            let theme_path = config_dir.join("theme_settings.json");

            if !theme_path.exists() {
                return Ok(ThemeSettings { theme_mode: zaroxi_theme::ZaroxiTheme::Light });
            }

            let content = fs::read_to_string(&theme_path)
                .map_err(|e| format!("Failed to read theme settings: {}", e))?;

            serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse theme settings: {}", e))
        }
        Err(e) => {
            // If we can't get config dir, return default
            // Use println since tracing might not be available
            println!("Failed to get config directory: {}, using default theme", e);
            Ok(ThemeSettings::default())
        }
    }
}

#[command]
pub async fn save_theme_settings(settings: ThemeSettings) -> Result<(), String> {
    match get_config_dir() {
        Ok(config_dir) => {
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
        Err(e) => {
            println!("Failed to get config directory: {}", e);
            // Still return Ok since theme is applied in memory
            Ok(())
        }
    }
}

#[command]
pub async fn get_current_theme() -> Result<ZaroxiTheme, String> {
    let settings = load_theme_settings().await?;
    Ok(settings.theme_mode)
}

#[command]
pub async fn set_theme(theme: ZaroxiTheme) -> Result<(), String> {
    let settings = ThemeSettings { theme_mode: theme };
    save_theme_settings(settings).await
}
