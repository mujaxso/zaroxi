use crate::state::App;
use crate::message::Message;
use iced::Element;

pub fn view(app: &App) -> Element<'_, Message> {
    crate::ui::layout::ide_layout(
        &app.workspace_path,
        &app.file_entries,
        app.active_file_path.as_ref(),
        app.is_dirty,
        &app.status_message,
        app.error_message.as_ref(),
        app.active_activity,
        app.ai_panel_visible,
        &app.prompt_input,
        &app.expanded_directories,
        &app.text_editor,
        app.editor_buffer.as_ref(),
        app.is_file_too_large_for_editor,
        &app.file_loading_state,
        app.theme,
    )
}
