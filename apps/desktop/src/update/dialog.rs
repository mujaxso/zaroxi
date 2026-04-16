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
        /// Open a folder picker dialog with portal support
        pub async fn pick_folder(title: &str) -> Result<PathBuf, FilePickerError> {
            // Log environment for diagnostics
            let wayland = std::env::var("WAYLAND_DISPLAY").is_ok()
                || std::env::var("XDG_SESSION_TYPE").unwrap_or_default() == "wayland";
            let xdg_current_desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
            let hyprland = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
                || xdg_current_desktop.to_lowercase().contains("hyprland");
            
            log::debug!("File picker environment: WAYLAND_DISPLAY={}, XDG_SESSION_TYPE={}, XDG_CURRENT_DESKTOP={}, HYPRLAND={}",
                std::env::var("WAYLAND_DISPLAY").unwrap_or_default(),
                std::env::var("XDG_SESSION_TYPE").unwrap_or_default(),
                xdg_current_desktop,
                hyprland);
            
            // On Hyprland/Wayland, we need to ensure portal integration works
            // rfd should handle this automatically with the xdg-portal feature
            
            // Try async dialog first
            let dialog = rfd::AsyncFileDialog::new()
                .set_title(title);
            
            // Note: Native file dialogs use the system theme, not our application theme
            // This is a limitation of native dialogs on all platforms
            log::debug!("Opening async file picker dialog (uses system theme)");
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
                        log::debug!("Opening synchronous file picker dialog (uses system theme)");
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
                    let wayland = std::env::var("WAYLAND_DISPLAY").is_ok()
                        || std::env::var("XDG_SESSION_TYPE").unwrap_or_default() == "wayland";
                    let hyprland = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
                        || std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().to_lowercase().contains("hyprland");
                    
                    let error_msg = if hyprland && wayland {
                        format!("File picker failed on Hyprland (Wayland). {}. Try manual entry below, or ensure xdg-desktop-portal and xdg-desktop-portal-hyprland are installed. Note: File picker uses system theme.", e)
                    } else if wayland {
                        format!("File picker failed on Wayland. {}. Try manual entry below, or ensure xdg-desktop-portal is running. Note: File picker uses system theme.", e)
                    } else {
                        format!("File picker failed: {}. Try manual entry below. Note: File picker uses system theme.", e)
                    };
                    
                    Message::WorkspaceLoaded(Err(error_msg))
                }
            }
        },
        |result| result,
    )
}
