use crate::message::Message;
use crate::state::App;
use iced::Command;
use editor_core::EditorState;
use std::path::Path;

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::EditorContentChanged(action) => {
            // Don't process edits if the file is too large for editor
            if app.is_file_too_large_for_editor {
                app.status_message = "File is too large - editing disabled".to_string();
                return Command::none();
            }
            
            // Perform the action on the text editor
            app.text_editor.perform(action.clone());
            
            // Only update the document for edit actions to improve performance
            match &action {
                iced::widget::text_editor::Action::Edit(_edit) => {
                    if let Some(ref mut editor_state) = app.editor_state {
                        // For any edit, update the entire document text
                        // This is simpler and ensures consistency
                        let current_text = app.text_editor.text();
                        editor_state.document_mut().replace_all(&current_text);
                        app.is_dirty = editor_state.document().is_dirty();
                        app.status_message = "Text updated".to_string();
                        
                        // Update syntax document
                        if let Some(path) = &app.active_file_path {
                            let doc_id = path.clone();
                            let mut syntax_manager = app.syntax_manager.lock().unwrap();
                            match syntax_manager.update_document(&doc_id, &current_text, Path::new(path)) {
                                Ok(()) => {
                                    // Successfully updated syntax
                                    app.status_message = format!("Syntax highlighting active for {}", doc_id);
                                    // Try to retrieve highlight spans to prove it works
                                    match syntax_manager.highlight_spans(&doc_id) {
                                        Ok(spans) => {
                                            app.syntax_highlight_span_count = spans.len();
                                            app.syntax_highlight_spans = spans.clone();
                                            app.status_message = format!(
                                                "{} highlights for {}",
                                                spans.len(),
                                                doc_id
                                            );
                                        }
                                        Err(_) => {
                                            app.syntax_highlight_span_count = 0;
                                            app.syntax_highlight_spans.clear();
                                    }
                                }
                                Err(e) => {
                                    // Don't show error for unsupported languages
                                    if !matches!(e, syntax_core::SyntaxError::LanguageNotSupported(_)) {
                                        app.status_message = format!("Syntax update failed: {}", e);
                                    }
                                    app.syntax_highlight_span_count = 0;
                                    app.syntax_highlight_spans.clear();
                                }
                            }
                        }
                    }
                }
                _ => {
                    // For non-edit actions, just update status
                    app.status_message = "Cursor moved".to_string();
                }
            }
            Command::none()
        }
        Message::EditorInsertText(text) => {
            if let Some(ref mut editor_state) = app.editor_state {
                match editor_state.insert_text(&text) {
                    Ok((start_byte, old_end_byte, new_text)) => {
                        app.is_dirty = editor_state.document().is_dirty();
                        app.status_message = "Text inserted".to_string();
                        
                        // Update syntax document with incremental edit
                        if let Some(path) = &app.active_file_path {
                            let doc_id = path.clone();
                            let mut syntax_manager = app.syntax_manager.lock().unwrap();
                            if let Err(e) = syntax_manager.edit_document(&doc_id, start_byte, old_end_byte, &new_text) {
                                app.status_message = format!("Syntax edit failed: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        app.status_message = format!("Failed to insert text: {}", e);
                    }
                }
            }
            Command::none()
        }
        Message::EditorDeleteBackward => {
            if let Some(ref mut editor_state) = app.editor_state {
                match editor_state.delete_backward() {
                    Ok((start_byte, old_end_byte, _)) => {
                        app.is_dirty = editor_state.document().is_dirty();
                        app.status_message = "Deleted backward".to_string();
                        
                        // Update syntax document with incremental edit
                        if let Some(path) = &app.active_file_path {
                            let doc_id = path.clone();
                            let mut syntax_manager = app.syntax_manager.lock().unwrap();
                            if let Err(e) = syntax_manager.edit_document(&doc_id, start_byte, old_end_byte, "") {
                                app.status_message = format!("Syntax edit failed: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        app.status_message = format!("Failed to delete: {}", e);
                    }
                }
            }
            Command::none()
        }
        Message::EditorDeleteForward => {
            if let Some(ref mut editor_state) = app.editor_state {
                match editor_state.delete_forward() {
                    Ok((start_byte, old_end_byte, _)) => {
                        app.is_dirty = editor_state.document().is_dirty();
                        app.status_message = "Deleted forward".to_string();
                        
                        // Update syntax document with incremental edit
                        if let Some(path) = &app.active_file_path {
                            let doc_id = path.clone();
                            let mut syntax_manager = app.syntax_manager.lock().unwrap();
                            if let Err(e) = syntax_manager.edit_document(&doc_id, start_byte, old_end_byte, "") {
                                app.status_message = format!("Syntax edit failed: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        app.status_message = format!("Failed to delete: {}", e);
                    }
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
            let char_count = editor_state.document().len_chars();
            
            // Set thresholds for performance
            const MODERATE_THRESHOLD: usize = 500_000; // 500k characters
            const LARGE_THRESHOLD: usize = 1_000_000; // 1M characters
            
            if char_count > LARGE_THRESHOLD {
                app.is_file_too_large_for_editor = true;
                app.text_editor = iced::widget::text_editor::Content::new();
                app.status_message = format!(
                    "File is too large ({} chars) - editing disabled",
                    char_count
                );
            } else if char_count > MODERATE_THRESHOLD {
                // For moderate files, load but show a warning
                app.is_file_too_large_for_editor = false;
                let text = editor_state.document().text();
                app.text_editor = iced::widget::text_editor::Content::with_text(&text);
                app.status_message = format!(
                    "File is moderate size ({} chars) - editing may be slow",
                    char_count
                );
            } else {
                // For small files, load normally
                app.is_file_too_large_for_editor = false;
                let text = editor_state.document().text();
                app.text_editor = iced::widget::text_editor::Content::with_text(&text);
                app.status_message = format!("Loaded file ({} chars)", char_count);
            }
            
            // Initialize syntax document
            if let Some(path) = editor_state.path() {
                let doc_id = path.to_string();
                let mut syntax_manager = app.syntax_manager.lock().unwrap();
                if let Err(e) = syntax_manager.update_document(&doc_id, &editor_state.text(), Path::new(path)) {
                    app.status_message = format!("Syntax init failed: {}", e);
                }
            }
            
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
