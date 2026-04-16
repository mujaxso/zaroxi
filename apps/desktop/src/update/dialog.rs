use crate::message::Message;
use crate::update::workspace::load_directory_recursive;
use iced::Command;

/// Cross-platform file picker service
mod file_picker {
    use std::path::PathBuf;
    
    #[derive(Debug)]
    pub enum FilePickerError {
        DialogFailed(String),
        UserCancelled,
        UnsupportedPlatform,
    }
    
    impl std::fmt::Display for FilePickerError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                FilePickerError::DialogFailed(msg) => write!(f, "Dialog failed: {}", msg),
                FilePickerError::UserCancelled => write!(f, "User cancelled"),
                FilePickerError::UnsupportedPlatform => write!(f, "Unsupported platform"),
            }
        }
    }
    
    /// Platform-specific file picker implementation
    pub struct FilePicker;
    
    impl FilePicker {
        /// Check if system prefers dark theme
        fn system_prefers_dark_theme() -> bool {
            // Check GTK settings
            if let Ok(theme) = std::env::var("GTK_THEME") {
                if theme.to_lowercase().contains("dark") {
                    return true;
                }
            }
            
            // Check COLORFGBG environment variable (common in terminals)
            // Format is "foreground;background" where 15;0 means white on black (dark)
            if let Ok(colorfg) = std::env::var("COLORFGBG") {
                if colorfg.contains("15;0") || colorfg.contains("15;8") {
                    return true;
                }
            }
            
            // Check QT settings
            if let Ok(qt_style) = std::env::var("QT_STYLE_OVERRIDE") {
                if qt_style.to_lowercase().contains("dark") {
                    return true;
                }
            }
            
            // Check XDG desktop portal color scheme preference
            if let Ok(color_scheme) = std::env::var("COLOR_SCHEME") {
                if color_scheme.to_lowercase().contains("dark") {
                    return true;
                }
            }
            
            // Check GNOME/GTK settings via gsettings
            #[cfg(target_os = "linux")]
            {
                use std::process::Command;
                if let Ok(output) = Command::new("gsettings")
                    .args(["get", "org.gnome.desktop.interface", "color-scheme"])
                    .output()
                {
                    if output.status.success() {
                        let result = String::from_utf8_lossy(&output.stdout);
                        if result.to_lowercase().contains("dark") {
                            return true;
                        }
                    }
                }
                
                // Also check gtk-theme
                if let Ok(output) = Command::new("gsettings")
                    .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
                    .output()
                {
                    if output.status.success() {
                        let result = String::from_utf8_lossy(&output.stdout);
                        if result.to_lowercase().contains("dark") {
                            return true;
                        }
                    }
                }
            }
            
            // Default to false if we can't determine
            false
        }
        
        /// Open a folder picker dialog with portal support
        pub async fn pick_folder(title: &str) -> Result<PathBuf, FilePickerError> {
            // Log environment for diagnostics
            let _wayland = std::env::var("WAYLAND_DISPLAY").is_ok()
                || std::env::var("XDG_SESSION_TYPE").unwrap_or_default() == "wayland";
            let xdg_current_desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
            let hyprland = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
                || xdg_current_desktop.to_lowercase().contains("hyprland");
            
            log::debug!("File picker environment: WAYLAND_DISPLAY={}, XDG_SESSION_TYPE={}, XDG_CURRENT_DESKTOP={}, HYPRLAND={}",
                std::env::var("WAYLAND_DISPLAY").unwrap_or_default(),
                std::env::var("XDG_SESSION_TYPE").unwrap_or_default(),
                xdg_current_desktop,
                hyprland);
            
            // Try to ensure dark theme is used if system prefers it
            if Self::system_prefers_dark_theme() {
                log::debug!("System prefers dark theme, configuring GTK for dark mode");
                
                // Set GTK theme to prefer dark variant
                if std::env::var("GTK_THEME").is_err() {
                    unsafe {
                        std::env::set_var("GTK_THEME", "Adwaita:dark");
                    }
                }
                
                // Also try setting GTK application preference for dark theme
                // This is a GTK-specific setting that applications can use
                unsafe {
                    std::env::set_var("GTK_APPLICATION_PREFER_DARK_THEME", "1");
                }
                
                // Set color scheme for portals
                unsafe {
                    std::env::set_var("COLOR_SCHEME", "dark");
                }
            }
            
            // On Hyprland/Wayland, we need to ensure portal integration works
            // rfd should handle this automatically with the xdg-portal feature
            
            // Try async dialog first
            let dialog = rfd::AsyncFileDialog::new()
                .set_title(title);
            
            // Note: Native file dialogs use the system theme, not our application theme
            // This is a limitation of native dialogs on all platforms
            log::debug!("Opening async file picker dialog (trying to use dark theme)");
            match dialog.pick_folder().await {
                Some(handle) => {
                    let path = handle.path().to_path_buf();
                    log::debug!("File picker succeeded: {:?}", path);
                    Ok(path)
                }
                None => {
                    log::debug!("File picker: user cancelled");
                    Err(FilePickerError::UserCancelled)
                }
            }
        }
        
        /// Try alternative methods if the primary method fails
        pub async fn pick_folder_with_fallback(title: &str) -> Result<PathBuf, FilePickerError> {
            // First try the primary async method
            match Self::pick_folder(title).await {
                Ok(path) => Ok(path),
                Err(FilePickerError::UserCancelled) => Err(FilePickerError::UserCancelled),
                Err(e) => {
                    log::warn!("Async file picker failed: {}. Trying synchronous fallback.", e);
                    
                    // Clone title to own it for the spawn_blocking closure
                    let title_clone = title.to_string();
                    
                    // Try synchronous version as fallback
                    // This might work better in some environments
                    match tokio::task::spawn_blocking(move || {
                        // Apply theme settings for synchronous dialog too
                        if Self::system_prefers_dark_theme() {
                            log::debug!("System prefers dark theme, configuring GTK for dark mode (sync)");
                            
                            if std::env::var("GTK_THEME").is_err() {
                                unsafe {
                                    std::env::set_var("GTK_THEME", "Adwaita:dark");
                                }
                            }
                            
                            unsafe {
                                std::env::set_var("GTK_APPLICATION_PREFER_DARK_THEME", "1");
                                std::env::set_var("COLOR_SCHEME", "dark");
                            }
                        }
                        
                        log::debug!("Opening synchronous file picker dialog (trying to use dark theme)");
                        let dialog = rfd::FileDialog::new()
                            .set_title(&title_clone);
                        dialog.pick_folder()
                    }).await {
                        Ok(Some(path)) => {
                            log::debug!("Synchronous file picker succeeded: {:?}", path);
                            Ok(path)
                        }
                        Ok(None) => {
                            log::debug!("Synchronous file picker: user cancelled");
                            Err(FilePickerError::UserCancelled)
                        }
                        Err(e) => {
                            log::error!("Synchronous file picker task failed: {}", e);
                            Err(FilePickerError::DialogFailed(format!("All picker methods failed: {}", e)))
                        }
                    }
                }
            }
        }
    }
}

pub fn open_workspace_dialog() -> Command<Message> {
    Command::perform(
        async move {
            // On some window managers (especially Wayland compositors),
            // a small delay can help ensure the window is properly focused
            // before opening the dialog
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            
            log::info!("Opening workspace dialog (note: uses system theme)");
            
            // Use our improved file picker service
            match file_picker::FilePicker::pick_folder_with_fallback("Select Workspace Directory - Qyzer Studio").await {
                Ok(path) => {
                    let path_str = path.to_string_lossy().to_string();
                    log::info!("Workspace selected: {}", path_str);
                    
                    let path_clone = path_str.clone();
                    match tokio::task::spawn_blocking(move || load_directory_recursive(&path_clone)).await {
                        Ok(Ok(entries)) => {
                            log::info!("Workspace loaded successfully with {} entries", entries.len());
                            Message::WorkspaceLoaded(Ok((path_str, entries)))
                        }
                        Ok(Err(e)) => {
                            log::error!("Failed to load workspace directory: {}", e);
                            Message::WorkspaceLoaded(Err(format!("Failed to open workspace: {}", e)))
                        }
                        Err(e) => {
                            log::error!("Task failed while loading workspace: {}", e);
                            Message::WorkspaceLoaded(Err(format!("Task failed: {}", e)))
                        }
                    }
                }
                Err(file_picker::FilePickerError::UserCancelled) => {
                    log::info!("User cancelled workspace selection");
                    Message::WorkspaceDialogCancelled
                }
                Err(e) => {
                    log::error!("File picker error: {}", e);
                    
                    // Provide helpful error messages based on environment
                    let _wayland = std::env::var("WAYLAND_DISPLAY").is_ok()
                        || std::env::var("XDG_SESSION_TYPE").unwrap_or_default() == "wayland";
                    let hyprland = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
                        || std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().to_lowercase().contains("hyprland");
                    
                    let error_msg = if hyprland && wayland {
                        format!("File picker failed on Hyprland (Wayland). {}. Try manual entry below, or ensure xdg-desktop-portal and xdg-desktop-portal-hyprland are installed. If dialog appears with wrong theme, check your system's dark mode settings.", e)
                    } else if wayland {
                        format!("File picker failed on Wayland. {}. Try manual entry below, or ensure xdg-desktop-portal is running. If dialog appears with wrong theme, check your system's dark mode settings.", e)
                    } else {
                        format!("File picker failed: {}. Try manual entry below. If dialog appears with wrong theme, check your system's dark mode settings.", e)
                    };
                    
                    Message::WorkspaceLoaded(Err(error_msg))
                }
            }
        },
        |result| result,
    )
}
