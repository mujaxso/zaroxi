use crate::message::Message;
use crate::state::App;
use crate::explorer::actions::ExplorerMessage;
use crate::explorer::state::InlineEditMode;
use iced::Command;
use file_ops::WorkspaceLoader;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::Explorer(explorer_msg) => {
            handle_explorer_message(app, explorer_msg)
        }
        Message::ExplorerHoverChanged(path) => {
            app.explorer_state.set_hovered_row(path);
            Command::none()
        }
        Message::ToggleDirectory(path) => {
            // Convert to ExplorerMessage
            handle_explorer_message(app, ExplorerMessage::ToggleDirectory(std::path::PathBuf::from(path)))
        }
        Message::KeyPressed(key, _modifiers) => {
            // Handle Escape key for canceling inline editing
            if let iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) = key {
                if !matches!(app.explorer_state.inline_edit, InlineEditMode::None) {
                    app.explorer_state.cancel_inline_edit();
                }
            }
            Command::none()
        }
        _ => Command::none(),
    }
}

fn handle_explorer_message(app: &mut App, explorer_msg: ExplorerMessage) -> Command<Message> {
    match explorer_msg {
        ExplorerMessage::ToggleDirectory(path) => {
            app.explorer_state.toggle_directory(path);
            Command::none()
        }
        ExplorerMessage::SelectFile(path) => {
            app.explorer_state.select_file(path.clone());
            // Convert to string and trigger file loading
            let path_string = path.to_string_lossy().to_string();
            
            // Check if this file already has a tab
            if app.tab_manager.has_tab_for_path(&path_string) {
                // File already has a tab, activate it
                if let Some(existing_tab) = app.tab_manager.find_tab_by_path(&path_string) {
                    let tab_id = existing_tab.id;
                    app.tab_manager.activate_tab(tab_id);
                    app.active_file_path = app.tab_manager.get_active_file_path();
                    
                    // Check if buffer exists - if it does, we don't need to reload from disk
                    if app.editor_buffers.contains_key(&path_string) {
                        // Buffer exists, update editor state from buffer
                        if let Some(buffer) = app.editor_buffers.get(&path_string) {
                            // Update syntax highlighting state
                            app.syntax_highlight_spans = buffer.syntax_highlight_spans.clone();
                            app.syntax_highlight_cache = buffer.syntax_highlight_cache.clone();
                            app.syntax_cache_version = buffer.syntax_cache_version;
                            app.syntax_highlight_span_count = buffer.syntax_highlight_span_count;
                            
                            // Set the editor state
                            app.editor_state = Some(editor_core::EditorState::from_document(buffer.document.clone()));
                            app.is_dirty = buffer.is_dirty;
                            
                            // Update tab dirty state
                            app.tab_manager.set_tab_dirty(tab_id, buffer.is_dirty);
                            
                            // Ensure syntax manager has the document
                            {
                                let mut syntax_manager = app.syntax_manager.lock().unwrap();
                                // Update the document in syntax manager if not present
                                if !syntax_manager.contains_document(&path_string) {
                                    let _ = syntax_manager.update_document(
                                        &path_string,
                                        &buffer.content,
                                        std::path::Path::new(&path_string),
                                    );
                                }
                            }
                        }
                        return Command::none();
                    }
                }
            }
            
            // Create a new tab for this file
            let _tab_id = app.tab_manager.open_or_activate_tab(path_string.clone());
            app.active_file_path = Some(path_string.clone());
            
            // Trigger file loading via workspace module
            Command::perform(
                async move { path_string },
                |path| Message::FileSelectedByPath(path),
            )
        }
        ExplorerMessage::Refresh => {
            // Trigger a workspace refresh
            if !app.workspace_path.is_empty() {
                let path = app.workspace_path.clone();
                Command::perform(
                    async move {
                        match WorkspaceLoader::list_directory(&path) {
                            Ok(entries) => Message::WorkspaceLoaded(Ok((path, entries))),
                            Err(e) => Message::WorkspaceLoaded(Err(format!("Failed to refresh workspace: {}", e))),
                        }
                    },
                    |result| result,
                )
            } else {
                Command::none()
            }
        }
        ExplorerMessage::SetWorkspaceRoot(root) => {
            app.explorer_state.set_workspace_root(root);
            Command::none()
        }
        ExplorerMessage::SetFileTree(entries) => {
            app.explorer_state.set_file_tree(entries);
            Command::none()
        }
        ExplorerMessage::CreateFileRequested => {
            // Determine parent directory: selected folder, or workspace root
            let parent = if let Some(selected) = &app.explorer_state.selected_file {
                // If selected is a file, use its parent
                if let Some(parent_path) = selected.parent() {
                    parent_path.to_path_buf()
                } else {
                    app.explorer_state.workspace_root.clone()
                }
            } else {
                app.explorer_state.workspace_root.clone()
            };
            // Ensure the parent directory is expanded
            if !app.explorer_state.is_expanded(&parent) {
                app.explorer_state.toggle_directory(parent.clone());
            }
            app.explorer_state.start_create_file(parent);
            Command::none()
        }
        ExplorerMessage::CreateFolderRequested => {
            // Determine parent directory: selected folder, or workspace root
            let parent = if let Some(selected) = &app.explorer_state.selected_file {
                // If selected is a file, use its parent
                if let Some(parent_path) = selected.parent() {
                    parent_path.to_path_buf()
                } else {
                    app.explorer_state.workspace_root.clone()
                }
            } else {
                app.explorer_state.workspace_root.clone()
            };
            // Ensure the parent directory is expanded
            if !app.explorer_state.is_expanded(&parent) {
                app.explorer_state.toggle_directory(parent.clone());
            }
            app.explorer_state.start_create_folder(parent);
            Command::none()
        }
        ExplorerMessage::RenameRequested(path) => {
            app.explorer_state.start_rename(path);
            Command::none()
        }
        ExplorerMessage::DeleteRequested(path) => {
            // Use the file-ops crate to delete the item
            let path_str = path.to_string_lossy().to_string();
            Command::perform(
                async move {
                    match WorkspaceLoader::delete_item(&path_str) {
                        Ok(_) => Message::Explorer(ExplorerMessage::Refresh),
                        Err(_e) => {
                            // We need to return a message that can update the app's error state
                            // For now, we'll trigger a refresh anyway
                            Message::Explorer(ExplorerMessage::Refresh)
                        }
                    }
                },
                |msg| msg,
            )
        }
        ExplorerMessage::InlineEditNameChanged(name) => {
            app.explorer_state.set_inline_edit_name(name);
            Command::none()
        }
        ExplorerMessage::InlineEditConfirmed => {
            // Handle the inline edit based on current mode
            match &app.explorer_state.inline_edit {
                InlineEditMode::CreateFile { parent } => {
                    let name = app.explorer_state.inline_edit_name.clone();
                    if !name.is_empty() {
                        let mut full_path = parent.clone();
                        full_path.push(&name);
                        let path_str = full_path.to_string_lossy().to_string();
                        app.explorer_state.cancel_inline_edit();
                        Command::perform(
                            async move {
                                match WorkspaceLoader::create_file(&path_str) {
                                    Ok(_) => Message::Explorer(ExplorerMessage::Refresh),
                                    Err(_) => Message::Explorer(ExplorerMessage::Refresh),
                                }
                            },
                            |msg| msg,
                        )
                    } else {
                        app.explorer_state.cancel_inline_edit();
                        Command::none()
                    }
                }
                InlineEditMode::CreateFolder { parent } => {
                    let name = app.explorer_state.inline_edit_name.clone();
                    if !name.is_empty() {
                        let mut full_path = parent.clone();
                        full_path.push(&name);
                        let path_str = full_path.to_string_lossy().to_string();
                        app.explorer_state.cancel_inline_edit();
                        Command::perform(
                            async move {
                                match WorkspaceLoader::create_directory(&path_str) {
                                    Ok(_) => Message::Explorer(ExplorerMessage::Refresh),
                                    Err(_) => Message::Explorer(ExplorerMessage::Refresh),
                                }
                            },
                            |msg| msg,
                        )
                    } else {
                        app.explorer_state.cancel_inline_edit();
                        Command::none()
                    }
                }
                InlineEditMode::Rename { target } => {
                    let new_name = app.explorer_state.inline_edit_name.clone();
                    if !new_name.is_empty() {
                        let mut new_path = target.parent().unwrap_or(&app.explorer_state.workspace_root).to_path_buf();
                        new_path.push(&new_name);
                        let old_path_str = target.to_string_lossy().to_string();
                        let new_path_str = new_path.to_string_lossy().to_string();
                        app.explorer_state.cancel_inline_edit();
                        Command::perform(
                            async move {
                                match WorkspaceLoader::rename_item(&old_path_str, &new_path_str) {
                                    Ok(_) => Message::Explorer(ExplorerMessage::Refresh),
                                    Err(_) => Message::Explorer(ExplorerMessage::Refresh),
                                }
                            },
                            |msg| msg,
                        )
                    } else {
                        app.explorer_state.cancel_inline_edit();
                        Command::none()
                    }
                }
                InlineEditMode::None => {
                    Command::none()
                }
            }
        }
        ExplorerMessage::InlineEditCancelled => {
            app.explorer_state.cancel_inline_edit();
            Command::none()
        }
        ExplorerMessage::ShowContextMenu(path) => {
            app.explorer_state.set_context_menu(Some(path));
            Command::none()
        }
        ExplorerMessage::HideContextMenu => {
            app.explorer_state.set_context_menu(None);
            Command::none()
        }
    }
}
