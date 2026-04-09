use crate::message::Message;
use crate::state::{App, Activity, FileLoadingState, FileMetadata};
use file_ops::{FileLoader, WorkspaceLoader};
use iced::Command;
use crate::explorer::actions::ExplorerMessage;

// Helper function to normalize paths for consistent comparison
fn normalize_path(path: &str) -> String {
    use std::path::Path;
    let path = Path::new(path);
    // Remove trailing separators and normalize
    let mut normalized = path.to_string_lossy().to_string();
    // Remove trailing separator if present
    while normalized.ends_with(std::path::MAIN_SEPARATOR) {
        normalized.pop();
    }
    normalized
}

// File size thresholds
const LARGE_FILE_THRESHOLD: u64 = 5 * 1024 * 1024; // 5MB
const VERY_LARGE_FILE_THRESHOLD: u64 = 50 * 1024 * 1024; // 50MB

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::WorkspacePathChanged(path) => {
            app.workspace_path = path;
            Command::none()
        }
        Message::OpenWorkspace => {
            if app.workspace_path.is_empty() {
                app.status_message = "Please enter a workspace path".to_string();
                return Command::none();
            }
            
            let path = app.workspace_path.clone();
            Command::perform(
                async move {
                    match WorkspaceLoader::list_directory(&path) {
                        Ok(entries) => Message::WorkspaceLoaded(Ok(entries)),
                        Err(e) => Message::WorkspaceLoaded(Err(format!("Failed to open workspace: {}", e))),
                    }
                },
                |result| result,
            )
        }
        Message::WorkspaceLoaded(result) => {
            match result {
                Ok(entries) => {
                    app.file_entries = entries.clone();
                    app.status_message = format!("Workspace loaded: {} files", app.file_entries.len());
                    app.error_message = None;
                    
                    // Update explorer state
                    app.explorer_state.set_file_tree(entries);
                    
                    let mut state = app.workspace_state.lock().unwrap();
                    state.set_workspace_root(&app.workspace_path);
                    state.set_file_tree(app.file_entries.clone());
                }
                Err(e) => {
                    app.error_message = Some(e);
                    app.status_message = "Failed to load workspace".to_string();
                }
            }
            Command::none()
        }
        Message::FileSelected(index) => {
            if index < app.file_entries.len() {
                let entry = &app.file_entries[index];
                // Only handle files, not directories
                if !entry.is_dir {
                    let path = entry.path.clone();
                    // Start by loading metadata first
                    app.file_loading_state = FileLoadingState::LoadingMetadata { 
                        path: path.clone() 
                    };
                    app.status_message = format!("Checking {}...", entry.name);
                    app.error_message = None;
                    app.is_file_read_only = false;
                    
                    // Load metadata using the file-ops crate
                    Command::perform(
                        async move {
                            match FileLoader::load_metadata(&path) {
                                Ok(metadata) => Message::FileMetadataLoaded(Ok(FileMetadata {
                                    path: metadata.path,
                                    size: metadata.size,
                                })),
                                Err(e) => Message::FileMetadataLoaded(Err(format!("Failed to load metadata: {}", e))),
                            }
                        },
                        |result| result,
                    )
                } else {
                    // Directories are handled by Message::ToggleDirectory
                    Command::none()
                }
            } else {
                Command::none()
            }
        }
        Message::FileSelectedByPath(path) => {
            // Find the index of the file in file_entries
            let index = app.file_entries.iter().position(|entry| entry.path == path);
            if let Some(index) = index {
                update(app, Message::FileSelected(index))
            } else {
                Command::none()
            }
        }
        Message::FileMetadataLoaded(result) => {
            match result {
                Ok(metadata) => {
                    // Check file size thresholds
                    if metadata.size > VERY_LARGE_FILE_THRESHOLD {
                        app.file_loading_state = FileLoadingState::VeryLargeFileWarning {
                            path: metadata.path.clone(),
                            size: metadata.size,
                        };
                        app.status_message = format!("Very large file detected ({} MB)", metadata.size / (1024 * 1024));
                        // For now, automatically open in read-only mode
                        return Command::perform(
                            async move {
                                // Small delay to show the warning
                                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                                Message::OpenLargeFileReadOnly(metadata.path)
                            },
                            |msg| msg,
                        );
                    } else if metadata.size > LARGE_FILE_THRESHOLD {
                        app.file_loading_state = FileLoadingState::LargeFileWarning {
                            path: metadata.path.clone(),
                            size: metadata.size,
                        };
                        app.status_message = format!("Large file detected ({} KB)", metadata.size / 1024);
                        // Automatically proceed with loading, but show warning
                        return Command::perform(
                            async move {
                                Message::ConfirmOpenLargeFile(metadata.path, metadata.size)
                            },
                            |msg| msg,
                        );
                    } else {
                        // Small file, proceed with normal loading
                        app.file_loading_state = FileLoadingState::LoadingContent {
                            path: metadata.path.clone(),
                            size: metadata.size,
                        };
                        app.status_message = format!("Loading file...");
                        
                        return Command::perform(
                            async move {
                                let path = metadata.path;
                                match FileLoader::load_file(&path) {
                                    Ok((content, buffer)) => {
                                        Message::FileLoaded(Ok((path, content, buffer)))
                                    }
                                    Err(e) => Message::FileLoaded(Err(format!("Failed to load file: {}", e))),
                                }
                            },
                            |result| result,
                        );
                    }
                }
                Err(e) => {
                    app.file_loading_state = FileLoadingState::Idle;
                    app.error_message = Some(e);
                    app.status_message = "Failed to load file metadata".to_string();
                }
            }
            Command::none()
        }
        Message::ConfirmOpenLargeFile(path, size) => {
            app.file_loading_state = FileLoadingState::LoadingContent {
                path: path.clone(),
                size,
            };
            app.status_message = format!("Loading large file...");
            
            Command::perform(
                async move {
                    match FileLoader::load_file(&path) {
                        Ok((content, buffer)) => {
                            Message::FileLoaded(Ok((path, content, buffer)))
                        }
                        Err(e) => Message::FileLoaded(Err(format!("Failed to load file: {}", e))),
                    }
                },
                |result| result,
            )
        }
        Message::OpenLargeFileReadOnly(path) => {
            app.file_loading_state = FileLoadingState::ReadOnlyPreview {
                path: path.clone(),
                size: 0, // We'll get this from metadata
            };
            app.status_message = format!("Opening in read-only mode...");
            app.active_file_path = Some(path.clone());
            app.is_file_too_large_for_editor = true;
            app.is_file_read_only = true;
            
            // For very large files, only load a preview
            Command::perform(
                async move {
                    match FileLoader::load_file_preview(&path, 100 * 1024) {
                        Ok((content, buffer)) => {
                            Message::FileLoaded(Ok((path, content, buffer)))
                        }
                        Err(e) => Message::FileLoaded(Err(format!("Failed to load file preview: {}", e))),
                    }
                },
                |result| result,
            )
        }
        Message::FileLoaded(result) => {
            match result {
                Ok((path, content, buffer)) => {
                    app.active_file_path = Some(path.clone());
                    app.editor_buffer = Some(buffer);
                    app.file_loading_state = FileLoadingState::Idle;
                    
                    let file_size = content.len();
                    
                    // Use the read-only flag to determine how to handle the file
                    if app.is_file_read_only {
                        app.is_file_too_large_for_editor = true;
                        app.status_message = format!(
                            "Very large file opened in read-only preview ({} bytes shown)",
                            file_size
                        );
                        // For read-only preview, show the content directly
                        // Limit the content to prevent crashes
                        let preview_content = if content.len() > 100_000 {
                            &content[..100_000]
                        } else {
                            &content
                        };
                        app.text_editor = iced::widget::text_editor::Content::with_text(&format!(
                            "// Read-only preview (first {} bytes)\n// File is very large\n\n{}",
                            preview_content.len(),
                            preview_content
                        ));
                        // Reset the flag for next time
                        app.is_file_read_only = false;
                    } else {
                        // Check thresholds for normal files
                        if file_size > LARGE_FILE_THRESHOLD as usize {
                            app.status_message = format!(
                                "Large file opened ({} MB) - editing enabled",
                                file_size / (1024 * 1024)
                            );
                            app.is_file_too_large_for_editor = false;
                        } else {
                            app.status_message = format!("File loaded: {} ({} bytes)", path, file_size);
                            app.is_file_too_large_for_editor = false;
                        }
                        
                        // Initialize editor content
                        if let Some(ref buffer) = app.editor_buffer {
                            let text = buffer.text();
                            // For very large files, limit the text to prevent crashes
                            let display_text = if text.len() > 1_000_000 {
                                &text[..1_000_000]
                            } else {
                                &text
                            };
                            app.text_editor = iced::widget::text_editor::Content::with_text(display_text);
                        }
                    }
                    
                    app.error_message = None;
                    app.is_dirty = false;
                    
                    // Update workspace state
                    {
                        let mut state = app.workspace_state.lock().unwrap();
                        state.open_buffer(&path, content);
                    }
                }
                Err(e) => {
                    app.file_loading_state = FileLoadingState::Idle;
                    app.is_file_read_only = false;
                    app.error_message = Some(e);
                    app.status_message = "Failed to load file".to_string();
                }
            }
            Command::none()
        }
        Message::EditorContentChanged(action) => {
            // Don't process edits if the file is too large for editor
            if app.is_file_too_large_for_editor {
                app.status_message = "File is too large - editing disabled".to_string();
                return Command::none();
            }
            
            // First, perform the action on the text editor
            app.text_editor.perform(action.clone());
            
            // Check if this action modifies the text content
            // In Iced, only Action::Edit actually modifies the text
            // All other actions (Scroll, MoveCursor, Select, etc.) are navigation/selection
            let should_update_buffer = match &action {
                iced::widget::text_editor::Action::Edit(_) => true,
                // All other actions don't modify text
                _ => false,
            };
            
            if should_update_buffer {
                if let Some(ref mut buffer) = app.editor_buffer {
                    // For simplicity, fall back to full update
                    let current_text = app.text_editor.text();
                    buffer.replace_all(&current_text);
                    app.status_message = "Text updated".to_string();
                    app.is_dirty = buffer.is_dirty();
                }
            }
            Command::none()
        }
        Message::SaveFile => {
            if let Some(path) = &app.active_file_path {
                if let Some(ref buffer) = app.editor_buffer {
                    let content = buffer.text();
                    let path_clone = path.clone();
                    
                    Command::perform(
                        async move {
                            match std::fs::write(&path_clone, &content) {
                                Ok(_) => Message::FileSaved(Ok(())),
                                Err(e) => Message::FileSaved(Err(format!("Failed to save file: {}", e))),
                            }
                        },
                        |result| result,
                    )
                } else {
                    app.status_message = "No buffer to save".to_string();
                    Command::none()
                }
            } else {
                app.status_message = "No file selected to save".to_string();
                Command::none()
            }
        }
        Message::FileSaved(result) => {
            match result {
                Ok(_) => {
                    if let Some(buffer) = &mut app.editor_buffer {
                        buffer.mark_saved();
                        app.is_dirty = buffer.is_dirty();
                    }
                    app.status_message = "File saved successfully".to_string();
                    app.error_message = None;
                }
                Err(e) => {
                    let error_msg = e.clone();
                    app.error_message = Some(e);
                    app.status_message = format!("Failed to save file: {}", error_msg);
                }
            }
            Command::none()
        }
        Message::RefreshWorkspace => {
            if !app.workspace_path.is_empty() {
                let path = app.workspace_path.clone();
                Command::perform(
                    async move {
                        match WorkspaceLoader::list_directory(&path) {
                            Ok(entries) => Message::WorkspaceLoaded(Ok(entries)),
                            Err(e) => Message::WorkspaceLoaded(Err(format!("Failed to refresh workspace: {}", e))),
                        }
                    },
                    |result| result,
                )
            } else {
                Command::none()
            }
        }
        Message::ToggleAiPanel => {
            app.ai_panel_visible = !app.ai_panel_visible;
            // When toggling the AI panel, also set the active activity to AI
            app.active_activity = Activity::Ai;
            Command::none()
        }
        Message::ActivitySelected(activity) => {
            app.active_activity = activity;
            // If the selected activity is AI, ensure the AI panel is visible
            if activity == Activity::Ai {
                app.ai_panel_visible = true;
            }
            Command::none()
        }
        Message::PromptInputChanged(text) => {
            app.prompt_input = text;
            Command::none()
        }
        Message::SendPrompt => {
            // Placeholder for AI prompt
            app.status_message = "AI feature coming soon".to_string();
            app.prompt_input.clear();
            Command::none()
        }
        Message::KeyPressed(key, modifiers) => {
            match key {
                iced::keyboard::Key::Character(c) if c == "s" && modifiers.control() => {
                    // Ctrl+S to save
                    update(app, Message::SaveFile)
                }
                iced::keyboard::Key::Character(c) if c == "r" && modifiers.control() => {
                    // Ctrl+R to refresh workspace
                    update(app, Message::RefreshWorkspace)
                }
                iced::keyboard::Key::Character(c) if c == "o" && modifiers.control() => {
                    // Ctrl+O to open workspace
                    update(app, Message::OpenWorkspace)
                }
                iced::keyboard::Key::Character(c) if c == "p" && modifiers.control() && modifiers.shift() => {
                    // Ctrl+Shift+P for command palette
                    update(app, Message::ToggleCommandPalette)
                }
                _ => Command::none(),
            }
        }
        Message::ToggleDirectory(path) => {
            // Convert to ExplorerMessage
            update(app, Message::Explorer(ExplorerMessage::ToggleDirectory(std::path::PathBuf::from(path))))
        }
        Message::ToggleCommandPalette => {
            // For now, just show a status message
            app.status_message = "Command palette (Ctrl+Shift+P) - coming soon".to_string();
            Command::none()
        }
        Message::WindowResized(width, height) => {
            app.window_width = width;
            app.window_height = height;
            app.update_layout_mode();
            Command::none()
        }
        Message::Explorer(explorer_msg) => {
            match explorer_msg {
                ExplorerMessage::ToggleDirectory(path) => {
                    app.explorer_state.toggle_directory(path);
                }
                ExplorerMessage::SelectFile(path) => {
                    app.explorer_state.select_file(path.clone());
                    // Convert to string and trigger file loading
                    if let Some(path_str) = path.to_str() {
                        app.active_file_path = Some(path_str.to_string());
                        // Trigger file loading
                        return Command::perform(
                            async move { path_str.to_string() },
                            |path| Message::FileSelectedByPath(path),
                        );
                    }
                }
                ExplorerMessage::Refresh => {
                    // Trigger a workspace refresh
                    if !app.workspace_path.is_empty() {
                        let path = app.workspace_path.clone();
                        return Command::perform(
                            async move {
                                match WorkspaceLoader::list_directory(&path) {
                                    Ok(entries) => Message::WorkspaceLoaded(Ok(entries)),
                                    Err(e) => Message::WorkspaceLoaded(Err(format!("Failed to refresh workspace: {}", e))),
                                }
                            },
                            |result| result,
                        );
                    }
                }
                ExplorerMessage::SetWorkspaceRoot(root) => {
                    app.explorer_state.set_workspace_root(root);
                }
                ExplorerMessage::SetFileTree(entries) => {
                    app.explorer_state.set_file_tree(entries);
                }
            }
            Command::none()
        }
    }
}
