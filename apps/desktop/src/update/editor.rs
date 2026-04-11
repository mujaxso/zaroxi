use crate::message::Message;
use crate::state::App;
use iced::Command;
use editor_core::EditorState;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
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
                if let Some(ref mut editor_state) = app.editor_state {
                    // For simplicity, fall back to full update
                    let current_text = app.text_editor.text();
                    editor_state.document_mut().replace_all(&current_text);
                    app.status_message = "Text updated".to_string();
                    app.is_dirty = editor_state.document().is_dirty();
                }
            }
            Command::none()
        }
        Message::EditorInsertText(text) => {
            if let Some(ref mut editor_state) = app.editor_state {
                if let Err(e) = editor_state.insert_text(&text) {
                    app.status_message = format!("Failed to insert text: {}", e);
                } else {
                    app.is_dirty = editor_state.document().is_dirty();
                    app.status_message = "Text inserted".to_string();
                }
            }
            Command::none()
        }
        Message::EditorDeleteBackward => {
            if let Some(ref mut editor_state) = app.editor_state {
                if let Err(e) = editor_state.delete_backward() {
                    app.status_message = format!("Failed to delete: {}", e);
                } else {
                    app.is_dirty = editor_state.document().is_dirty();
                    app.status_message = "Deleted backward".to_string();
                }
            }
            Command::none()
        }
        Message::EditorDeleteForward => {
            if let Some(ref mut editor_state) = app.editor_state {
                if let Err(e) = editor_state.delete_forward() {
                    app.status_message = format!("Failed to delete: {}", e);
                } else {
                    app.is_dirty = editor_state.document().is_dirty();
                    app.status_message = "Deleted forward".to_string();
                }
            }
            Command::none()
        }
        Message::EditorMoveCursor(movement) => {
            if let Some(ref mut editor_state) = app.editor_state {
                editor_state.move_cursor(movement);
                app.status_message = "Cursor moved".to_string();
            }
            Command::none()
        }
        Message::EditorSetDocument(document) => {
            let editor_state = EditorState::from_document(document);
            app.editor_state = Some(editor_state);
            Command::none()
        }
        Message::EditorUpdateState(editor_state) => {
            app.editor_state = Some(editor_state);
            Command::none()
        }
        Message::KeyPressed(key, _modifiers) => {
            // Handle arrow keys for cursor movement
            match key {
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft) => {
                    if let Some(ref mut editor_state) = app.editor_state {
                        editor_state.move_cursor(editor_core::CursorMovement::Left(1));
                    }
                }
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight) => {
                    if let Some(ref mut editor_state) = app.editor_state {
                        editor_state.move_cursor(editor_core::CursorMovement::Right(1));
                    }
                }
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp) => {
                    if let Some(ref mut editor_state) = app.editor_state {
                        editor_state.move_cursor(editor_core::CursorMovement::Up(1));
                    }
                }
                iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown) => {
                    if let Some(ref mut editor_state) = app.editor_state {
                        editor_state.move_cursor(editor_core::CursorMovement::Down(1));
                    }
                }
                _ => {}
            }
            Command::none()
        }
        _ => Command::none(),
    }
}
