use crate::message::Message;
use crate::state::App;
use iced::Command;
use editor_core::EditorState;
use syntax_core::language::LanguageId;
use std::path::Path;
use crate::ui::style::StyleHelpers;
use std::ops::Range;

/// Build a per‑line cache of highlight ranges from the raw highlight spans.
/// This cache is used by the real editor widget to apply syntax colors.
fn build_line_cache(
    text: &str,
    spans: &[syntax_core::HighlightSpan],
    theme: crate::theme::NeoteTheme,
) -> Vec<Vec<(Range<usize>, iced::Color)>> {
    use iced::Color;
    let style = StyleHelpers::new(theme);

    // line start positions (character offsets)
    let mut line_starts = vec![0];
    let mut char_idx = 0;
    for ch in text.chars() {
        if ch == '\n' {
            line_starts.push(char_idx + 1);
        }
        char_idx += 1;
    }
    // sentinel for the line after the last newline
    line_starts.push(char_idx);

    let line_count = line_starts.len() - 1;
    let mut line_cache = vec![Vec::new(); line_count];

    for span in spans {
        // convert byte offsets to character offsets
        let byte_start = span.start;
        let byte_end = span.end;
        // safe because spans are produced from the same text
        let char_start = text[..byte_start].chars().count();
        let char_end = char_start + text[byte_start..byte_end].chars().count();

        // find the line that contains char_start
        let line_idx = match line_starts.binary_search(&char_start) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        let mut remaining_start = char_start;
        let mut remaining_end = char_end;
        let mut current_line = line_idx;
        while remaining_start < remaining_end && current_line < line_count {
            let line_start = line_starts[current_line];
            let line_end = line_starts.get(current_line + 1).copied().unwrap_or(char_idx);
            let seg_start = remaining_start.max(line_start);
            let seg_end = remaining_end.min(line_end);
            if seg_start < seg_end {
                let line_range = (seg_start - line_start)..(seg_end - line_start);
                let color = style.highlight_color(span.highlight);
                line_cache[current_line].push((line_range, color));
            }
            remaining_start = seg_end;
            current_line += 1;
        }
    }
    line_cache
}

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
                            let language = LanguageId::from_path(Path::new(path));
                            match syntax_manager.update_document(&doc_id, &current_text, Path::new(path)) {
                                Ok(()) => {
                                    // Successfully updated syntax
                                    app.status_message = format!("Syntax highlighting active for {} (language: {:?})", doc_id, language);
                                    // Try to retrieve highlight spans to prove it works
                                    match syntax_manager.highlight_spans(&doc_id) {
                                        Ok(spans) => {
                                            app.syntax_highlight_span_count = spans.len();
                                            app.syntax_highlight_spans = spans.clone();
                                            // rebuild the per‑line cache for the editor widget
                                            app.syntax_highlight_cache =
                                                build_line_cache(&current_text, &spans, app.theme);
                                            if spans.is_empty() {
                                                app.status_message = format!(
                                                    "No syntax spans for {} (language: {:?})",
                                                    doc_id, language
                                                );
                                            } else {
                                                app.status_message = format!(
                                                    "{} highlights for {} (language: {:?})",
                                                    spans.len(),
                                                    doc_id,
                                                    language
                                                );
                                            }
                                        }
                                        Err(e) => {
                                            app.syntax_highlight_span_count = 0;
                                            app.syntax_highlight_spans.clear();
                                            app.status_message = format!(
                                                "Highlight error for {}: {}",
                                                doc_id, e
                                            );
                                        }
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
                } else {
                    // Retrieve highlight spans for UI
                    match syntax_manager.highlight_spans(&doc_id) {
                        Ok(spans) => {
                            app.syntax_highlight_span_count = spans.len();
                            app.syntax_highlight_spans = spans.clone();
                            // build per‑line cache for the real editor
                            let text = editor_state.text();
                            app.syntax_highlight_cache =
                                build_line_cache(&text, &spans, app.theme);
                        }
                        Err(_) => {
                            app.syntax_highlight_span_count = 0;
                            app.syntax_highlight_spans.clear();
                            app.syntax_highlight_cache.clear();
                        }
                    }
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
