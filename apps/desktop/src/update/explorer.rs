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
