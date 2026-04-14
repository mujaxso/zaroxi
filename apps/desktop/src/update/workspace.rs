use crate::message::Message;
use crate::state::{App, FileLoadingState, FileMetadata};
use crate::update::dialog;
use file_ops::{FileLoader, WorkspaceLoader};
use iced::Command;
use editor_core::Document;

// File size thresholds for tiered handling
const SYNTAX_HIGHLIGHT_THRESHOLD: u64 = 2 * 1024 * 1024; // 2MB - disable syntax highlighting above this
const LARGE_FILE_THRESHOLD: u64 = 10 * 1024 * 1024; // 10MB - reduce features but keep editing
const VERY_LARGE_FILE_THRESHOLD: u64 = 100 * 1024 * 1024; // 100MB - read-only mode

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::WorkspacePathChanged(path) => {
            app.workspace_path = path;
            Command::none()
        }
        Message::OpenWorkspace => {
            dialog::open_workspace_dialog()
        }
        Message::WorkspaceLoaded(result) => {
            handle_workspace_loaded(app, result)
        }
        Message::FileSelected(index) => {
            handle_file_selected(app, index)
        }
        Message::FileSelectedByPath(path) => {
            handle_file_selected_by_path(app, path)
        }
        Message::FileMetadataLoaded(result) => {
            handle_file_metadata_loaded(app, result)
        }
        Message::ConfirmOpenLargeFile(path, size) => {
            handle_confirm_open_large_file(app, path, size)
        }
        Message::OpenLargeFileReadOnly(path) => {
            handle_open_large_file_read_only(app, path)
        }
        Message::FileLoaded(result) => {
            handle_file_loaded(app, result)
        }
        Message::SaveFile => {
            handle_save_file(app)
        }
        Message::FileSaved(result) => {
            handle_file_saved(app, result)
        }
        Message::RefreshWorkspace => {
            handle_refresh_workspace(app)
        }
        Message::SubmitManualWorkspacePath(path) => {
            handle_submit_manual_workspace_path(app, path)
        }
        Message::WorkspaceDialogCancelled => {
            app.status_message = "Workspace selection cancelled".to_string();
            Command::none()
        }
        _ => Command::none(),
    }
}

fn handle_workspace_loaded(app: &mut App, result: Result<(String, Vec<core_types::workspace::DirectoryEntry>), String>) -> Command<Message> {
    match result {
        Ok((path, entries)) => {
            // Clear file cache when workspace changes
            app.file_cache.clear();
            
            app.workspace_path = path.clone();
            app.file_entries = entries.clone();
            app.status_message = format!("Workspace loaded: {} files", app.file_entries.len());
            app.error_message = None;
            
            // Update explorer state
            app.explorer_state.set_workspace_root(std::path::PathBuf::from(&app.workspace_path));
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

fn handle_file_selected(app: &mut App, index: usize) -> Command<Message> {
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
            app.is_file_too_large_for_editor = false;
            
            // Clear syntax highlight cache for previous file
            app.syntax_highlight_cache.clear();
            app.syntax_highlight_spans.clear();
            app.syntax_highlight_span_count = 0;
            app.syntax_cache_version += 1;
            // Clear text editor content
            app.text_editor = iced::widget::text_editor::Content::new();
            
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

fn handle_file_selected_by_path(app: &mut App, path: String) -> Command<Message> {
    // Find the index of the file in file_entries
    let index = app.file_entries.iter().position(|entry| entry.path == path);
    if let Some(index) = index {
        handle_file_selected(app, index)
    } else {
        Command::none()
    }
}

fn handle_file_metadata_loaded(app: &mut App, result: Result<FileMetadata, String>) -> Command<Message> {
    match result {
        Ok(metadata) => {
            // Check if file is in cache
            if let Some((cached_content, cached_document)) = app.file_cache.get(&metadata.path) {
                // File is cached, use it immediately
                app.file_loading_state = FileLoadingState::LoadingContent {
                    path: metadata.path.clone(),
                    size: metadata.size,
                };
                app.status_message = format!("Loading from cache...");
                
                // Small delay to show loading state (but much faster than actual loading)
                let path = metadata.path.clone();
                let content = cached_content.clone();
                let document = cached_document.clone();
                
                return Command::perform(
                    async move {
                        // Very short delay to show loading state
                        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                        Message::FileLoaded(Ok((path, content, document)))
                    },
                    |result| result,
                );
            } else {
                // File not in cache, load from disk
                app.file_loading_state = FileLoadingState::LoadingContent {
                    path: metadata.path.clone(),
                    size: metadata.size,
                };
                app.status_message = format!("Loading file...");
                
                return Command::perform(
                    async move {
                        let path = metadata.path;
                        match FileLoader::load_file(&path) {
                            Ok((content, _)) => {
                                // Create a Document from the content
                                let document = Document::from_text_with_path(&content, path.clone());
                                Message::FileLoaded(Ok((path, content, document)))
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

fn handle_confirm_open_large_file(app: &mut App, path: String, size: u64) -> Command<Message> {
    app.file_loading_state = FileLoadingState::LoadingContent {
        path: path.clone(),
        size,
    };
    app.status_message = format!("Loading file...");
    
    Command::perform(
        async move {
            match FileLoader::load_file(&path) {
                Ok((content, _)) => {
                    let document = Document::from_text_with_path(&content, path.clone());
                    Message::FileLoaded(Ok((path, content, document)))
                }
                Err(e) => Message::FileLoaded(Err(format!("Failed to load file: {}", e))),
            }
        },
        |result| result,
    )
}

fn handle_open_large_file_read_only(app: &mut App, path: String) -> Command<Message> {
    app.file_loading_state = FileLoadingState::LoadingContent {
        path: path.clone(),
        size: 0,
    };
    app.status_message = format!("Loading file...");
    app.active_file_path = Some(path.clone());
    app.is_file_too_large_for_editor = true;
    app.is_file_read_only = true;
    
    // For very large files, only load a preview
    Command::perform(
        async move {
            match FileLoader::load_file_preview(&path, 100 * 1024) {
                Ok((content, _)) => {
                    let document = Document::from_text_with_path(&content, path.clone());
                    Message::FileLoaded(Ok((path, content, document)))
                }
                Err(e) => Message::FileLoaded(Err(format!("Failed to load file preview: {}", e))),
            }
        },
        |result| result,
    )
}

fn handle_file_loaded(app: &mut App, result: Result<(String, String, Document), String>) -> Command<Message> {
    use std::time::Instant;
    let start_time = Instant::now();
    match result {
        Ok((path, content, document)) => {
            // Cache the file for faster reopening (limit to 10 files)
            if app.file_cache.len() >= 10 {
                // Remove the first (oldest) entry
                let first_key = app.file_cache.keys().next().cloned();
                if let Some(key) = first_key {
                    app.file_cache.remove(&key);
                }
            }
            app.file_cache.insert(path.clone(), (content.clone(), document.clone()));
            
            app.active_file_path = Some(path.clone());
            app.file_loading_state = FileLoadingState::Idle;
            
            let file_size_bytes = content.len();
            
            // Determine file mode based on size
            let is_very_large = file_size_bytes > VERY_LARGE_FILE_THRESHOLD as usize;
            let is_large = file_size_bytes > LARGE_FILE_THRESHOLD as usize;
            let needs_syntax_highlight = file_size_bytes <= SYNTAX_HIGHLIGHT_THRESHOLD as usize;
            
            // Update syntax highlighting flag and clear cache if disabled
            app.syntax_highlighting_enabled = needs_syntax_highlight;
            if !needs_syntax_highlight {
                app.syntax_highlight_cache.clear();
                app.syntax_highlight_spans.clear();
                app.syntax_highlight_span_count = 0;
                app.syntax_cache_version += 1;
            }
            
            // Handle based on size tier
            if is_very_large {
                // Very large files: read-only mode with limited preview
                app.is_file_read_only = true;
                app.is_file_too_large_for_editor = true;
                app.status_message = format!(
                    "Very large file opened in read-only preview ({} MB total, showing first 100KB)",
                    file_size_bytes / (1024 * 1024)
                );
                
                // Limit preview content
                let preview_content = if content.len() > 100_000 {
                    &content[..100_000]
                } else {
                    &content
                };
                app.text_editor = iced::widget::text_editor::Content::with_text(&format!(
                    "// Read-only preview (first {} bytes)\n// File is very large ({} MB total)\n\n{}",
                    preview_content.len(),
                    file_size_bytes / (1024 * 1024),
                    preview_content
                ));
                // Create editor state from document for consistency
                app.editor_state = Some(editor_core::EditorState::from_document(document));
            } else {
                // Large or normal files: editing enabled
                app.is_file_read_only = false;
                app.is_file_too_large_for_editor = false;
                
                if is_large {
                    app.status_message = format!(
                        "Large file opened ({} MB) - editing enabled, syntax highlighting disabled",
                        file_size_bytes / (1024 * 1024)
                    );
                } else if !needs_syntax_highlight {
                    app.status_message = format!(
                        "File opened ({} MB) - syntax highlighting disabled for performance",
                        file_size_bytes / (1024 * 1024)
                    );
                } else {
                    app.status_message = format!("File loaded: {} ({} bytes)", path, file_size_bytes);
                }
                // Don't set editor state here - it will be set in EditorSetDocument
                // This prevents a flash of unstyled content
            }
            
            app.error_message = None;
            app.is_dirty = false;
            
            // Clone content for workspace state and syntax highlighting
            let content_clone = content.clone();
            
            // Update workspace state - use the content we already have
            {
                let mut state = app.workspace_state.lock().unwrap();
                state.open_buffer(&path, content_clone);
            }
            
            // Start syntax highlighting immediately for normal files
            if needs_syntax_highlight {
                // Update syntax document in the background
                if let Some(path) = &app.active_file_path {
                    let doc_id = path.clone();
                    let text = content.clone();
                    let theme = app.theme;
                    
                    // Update syntax manager and get spans
                    let spans = {
                        let mut syntax_manager = app.syntax_manager.lock().unwrap();
                        
                        // Update the document in syntax manager
                        if let Err(e) = syntax_manager.update_document(&doc_id, &text, std::path::Path::new(path)) {
                            eprintln!("Failed to update syntax document: {}", e);
                            None
                        } else {
                            // Get highlight spans
                            match syntax_manager.highlight_spans(&doc_id) {
                                Ok(spans) => Some(spans),
                                Err(e) => {
                                    eprintln!("Failed to get highlight spans: {}", e);
                                    None
                                }
                            }
                        }
                    };
                    
                    // Build cache outside the lock to avoid holding it
                    if let Some(spans) = spans {
                        app.syntax_highlight_span_count = spans.len();
                        app.syntax_highlight_spans = spans.clone();
                        // Build per‑line cache for the real editor
                        app.syntax_highlight_cache =
                            crate::update::editor::build_line_cache(&text, &spans, theme);
                        app.syntax_cache_version += 1;
                    }
                }
            }
            
            // Send EditorSetDocument to update editor state for non-very-large files
            let elapsed = start_time.elapsed();
            if elapsed.as_millis() > 50 {
                eprintln!("File loading took {}ms (highlighting: {})", elapsed.as_millis(), needs_syntax_highlight);
            }
            
            if is_very_large {
                // For very large files, we've already set up the editor state
                Command::none()
            } else {
                // For normal files, send EditorSetDocument
                let doc = document.clone();
                Command::perform(
                    async move {
                        Message::EditorSetDocument(doc)
                    },
                    |msg| msg,
                )
            }
        }
        Err(e) => {
            app.file_loading_state = FileLoadingState::Idle;
            app.is_file_read_only = false;
            app.is_file_too_large_for_editor = false;
            app.error_message = Some(e);
            app.status_message = "Failed to load file".to_string();
            Command::none()
        }
    }
}

fn handle_save_file(app: &mut App) -> Command<Message> {
    if let Some(path) = &app.active_file_path {
        if let Some(ref editor_state) = app.editor_state {
            let content = editor_state.document().text();
            let path_clone = path.clone();
            let content_clone = content.clone();
            
            Command::perform(
                async move {
                    // Use the file-ops crate to save the file
                    match WorkspaceLoader::save_file(&path_clone, &content_clone) {
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

fn handle_file_saved(app: &mut App, result: Result<(), String>) -> Command<Message> {
    match result {
        Ok(_) => {
            if let Some(editor_state) = &mut app.editor_state {
                editor_state.document_mut().mark_saved();
                app.is_dirty = editor_state.document().is_dirty();
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

pub fn load_directory_recursive(path: &str) -> Result<Vec<core_types::workspace::DirectoryEntry>, String> {
    use std::fs;
    use std::path::Path;
    
    let mut entries = Vec::new();
    
    // Helper function to walk directories recursively
    fn walk_directory(dir_path: &Path, entries: &mut Vec<core_types::workspace::DirectoryEntry>) -> Result<(), String> {
        match fs::read_dir(dir_path) {
            Ok(read_dir) => {
                for entry_result in read_dir {
                    match entry_result {
                        Ok(entry) => {
                            let entry_path = entry.path();
                            let path_str = entry_path.to_string_lossy().to_string();
                            
                            // Get file name
                            let name = entry.file_name().to_string_lossy().to_string();
                            
                            // Check if it's a directory
                            let is_dir = entry_path.is_dir();
                            
                            // Add to entries
                            entries.push(core_types::workspace::DirectoryEntry {
                                path: path_str.clone(),
                                name,
                                is_dir,
                            });
                            
                            // If it's a directory, recursively process it
                            if is_dir {
                                walk_directory(&entry_path, entries)?;
                            }
                        }
                        Err(e) => {
                            // Skip entries we can't read (like permission errors)
                            eprintln!("Warning: Failed to read directory entry in {}: {}", dir_path.display(), e);
                        }
                    }
                }
                Ok(())
            }
            Err(e) => {
                Err(format!("Failed to read directory {}: {}", dir_path.display(), e))
            }
        }
    }
    
    let root_path = Path::new(path);
    walk_directory(root_path, &mut entries)?;
    
    Ok(entries)
}

fn handle_refresh_workspace(app: &mut App) -> Command<Message> {
    if !app.workspace_path.is_empty() {
        let path = app.workspace_path.clone();
        Command::perform(
            async move {
                let path_clone = path.clone();
                match tokio::task::spawn_blocking(move || load_directory_recursive(&path_clone)).await {
                    Ok(Ok(entries)) => Message::WorkspaceLoaded(Ok((path, entries))),
                    Ok(Err(e)) => Message::WorkspaceLoaded(Err(format!("Failed to refresh workspace: {}", e))),
                    Err(e) => Message::WorkspaceLoaded(Err(format!("Task failed: {}", e))),
                }
            },
            |result| result,
        )
    } else {
        Command::none()
    }
}

fn handle_submit_manual_workspace_path(app: &mut App, path: String) -> Command<Message> {
    if path.is_empty() {
        app.status_message = "Please enter a workspace path".to_string();
        Command::none()
    } else {
        // Check if the path exists
        let path_buf = std::path::PathBuf::from(&path);
        if !path_buf.exists() {
            app.error_message = Some(format!("Path does not exist: {}", path));
            app.status_message = "Invalid path".to_string();
            return Command::none();
        }
        if !path_buf.is_dir() {
            app.error_message = Some(format!("Path is not a directory: {}", path));
            app.status_message = "Path must be a directory".to_string();
            return Command::none();
        }
        
        // Load the workspace from the manually entered path
        let path_clone = path.clone();
        Command::perform(
            async move {
                let path_clone2 = path_clone.clone();
                match tokio::task::spawn_blocking(move || load_directory_recursive(&path_clone2)).await {
                    Ok(Ok(entries)) => Message::WorkspaceLoaded(Ok((path_clone, entries))),
                    Ok(Err(e)) => Message::WorkspaceLoaded(Err(format!("Manual workspace load failed: {}", e))),
                    Err(e) => Message::WorkspaceLoaded(Err(format!("Task failed: {}", e))),
                }
            },
            |result| result,
        )
    }
}
