use crate::message::Message;
use file_ops::WorkspaceLoader;
use iced::Command;
use rfd::AsyncFileDialog;

// Try synchronous dialog as fallback
async fn try_sync_dialog() -> Result<String, String> {
    use rfd::FileDialog;
    
    // Run in a blocking task to avoid freezing the UI
    match tokio::task::spawn_blocking(|| {
        let dialog = FileDialog::new()
            .set_title("Select Workspace Directory - Neote");
        
        dialog.pick_folder()
    }).await {
        Ok(Some(path)) => {
            let path_str = path.to_string_lossy().to_string();
            Ok(path_str)
        }
        Ok(None) => {
            Err("User cancelled".to_string())
        }
        Err(e) => {
            Err(format!("Task failed: {}", e))
        }
    }
}

pub fn open_workspace_dialog() -> Command<Message> {
    Command::perform(
        async move {
            // Add a longer delay to ensure the window is properly focused
            // This can help with Wayland focus issues
            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            
            // Try the async dialog first
            let dialog = AsyncFileDialog::new()
                .set_title("Select Workspace Directory - Neote");
            
            match dialog.pick_folder().await {
                Some(handle) => {
                    let path = handle.path().to_string_lossy().to_string();
                    match WorkspaceLoader::list_directory(&path) {
                        Ok(entries) => Message::WorkspaceLoaded(Ok((path, entries))),
                        Err(e) => Message::WorkspaceLoaded(Err(format!("Failed to open workspace: {}", e))),
                    }
                }
                None => {
                    // If async dialog fails, try synchronous dialog as fallback
                    // Add another delay before trying the fallback
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    
                    match try_sync_dialog().await {
                        Ok(path) => {
                            match WorkspaceLoader::list_directory(&path) {
                                Ok(entries) => Message::WorkspaceLoaded(Ok((path, entries))),
                                Err(e) => Message::WorkspaceLoaded(Err(format!("Failed to open workspace: {}", e))),
                            }
                        }
                        Err(_) => {
                            // Check if we're in a Nix environment
                            let nix_env = std::env::var("NIX_PROFILES").is_ok() 
                                || std::env::var("IN_NIX_SHELL").is_ok()
                                || std::path::Path::new("/nix/store").exists();
                            
                            // Check if we're in a Wayland environment
                            let wayland = std::env::var("WAYLAND_DISPLAY").is_ok()
                                || std::env::var("XDG_SESSION_TYPE").unwrap_or_default() == "wayland";
                            
                            // Check if we're in a Hyprland environment
                            let hyprland = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
                                || std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().to_lowercase().contains("hyprland");
                            
                            let error_msg = if hyprland && wayland {
                                "File picker failed in Hyprland. Try manual entry below."
                            } else if nix_env && wayland {
                                "File picker failed in Nix+Wayland. Try manual entry below."
                            } else if nix_env {
                                "File picker failed in Nix environment. Try manual entry below."
                            } else if wayland {
                                "File picker failed in Wayland. Try manual entry below."
                            } else {
                                "File picker failed. Try manual entry below."
                            };
                            Message::WorkspaceLoaded(Err(error_msg.to_string()))
                        }
                    }
                }
            }
        },
        |result| result,
    )
}
