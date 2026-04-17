pub mod topbar;
pub mod activity_rail;
pub mod explorer;
pub mod editor;
pub mod assistant;
pub mod search;
pub mod terminal;
pub mod settings;
pub mod status_bar;

// Wrapper functions that accept &App
use crate::message::Message;
use crate::state::App;
use iced::Element;

pub fn top_bar(app: &App) -> Element<'_, Message> {
    let workspace_path = &app.workspace_path;
    
    topbar::top_bar(workspace_path)
}

pub fn activity_rail(app: &App) -> Element<'_, Message> {
    activity_rail::activity_rail(app)
}

pub fn explorer_panel(app: &App) -> Element<'_, Message> {
    explorer::explorer_panel(app)
}

pub fn editor_panel(app: &App) -> Element<'_, Message> {
    let active_file_path = app.active_file_path.as_ref();
    let text_editor = &app.text_editor;
    let is_dirty = app.is_dirty;
    // We can't access the private document field directly
    // For now, pass None since we don't have access
    let editor_document = None;
    let is_file_too_large_for_editor = app.is_file_too_large_for_editor;
    let file_loading_state = &app.file_loading_state;
    let editor_typography = &app.editor_typography;
    let theme = app.current_theme;
    let line_cache = if !app.syntax_highlight_cache.is_empty() {
        Some(app.syntax_highlight_cache.clone())
    } else {
        None
    };
    
    editor::editor_panel(
        active_file_path,
        text_editor,
        is_dirty,
        editor_document,
        is_file_too_large_for_editor,
        file_loading_state,
        editor_typography,
        theme,
        line_cache,
        &app.tab_manager,
    )
}

pub fn assistant_panel(app: &App) -> Element<'_, Message> {
    assistant::assistant_panel(app)
}

pub fn status_bar(app: &App) -> Element<'_, Message> {
    status_bar::status_bar(app)
}
