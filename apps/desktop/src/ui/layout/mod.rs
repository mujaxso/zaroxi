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

// Additional imports for ide_layout
use core_types;
use editor_core;
use std::collections::HashSet;
use std::ops::Range;
use iced::Color;
use iced::widget::text_editor;
use iced::{Length, widget::{container, row, column, vertical_rule, Space}};
use iced::widget::horizontal_rule;
use crate::theme::ZaroxiTheme;
use crate::settings::editor::EditorTypographySettings;
use crate::state::FileLoadingState;
use crate::ui::style::StyleHelpers;

pub fn top_bar(app: &App) -> Element<'_, Message> {
    let workspace_path = &app.workspace_path;
    let theme = app.current_theme;
    
    topbar::top_bar(workspace_path, theme)
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

/// Main IDE layout function (consolidated from the old layout.rs)
pub fn ide_layout<'a>(
    app: &'a App,
    workspace_path: &'a str,
    file_entries: &'a [core_types::workspace::DirectoryEntry],
    active_file_path: Option<&'a String>,
    is_dirty: bool,
    status_message: &'a str,
    error_message: Option<&'a String>,
    ai_panel_visible: bool,
    prompt_input: &'a str,
    _expanded_directories: &'a std::collections::HashSet<String>,
    text_editor: &'a iced::widget::text_editor::Content,
    editor_document: Option<&'a editor_core::Document>,
    is_file_too_large_for_editor: bool,
    file_loading_state: &'a FileLoadingState,
    theme: ZaroxiTheme,
    editor_typography: &'a EditorTypographySettings,
    syntax_highlight_cache: Option<Vec<Vec<(Range<usize>, Color)>>>,
) -> Element<'a, Message> {
    let style = StyleHelpers::new(theme);
    
    // Top bar
    let top_bar = topbar::top_bar(workspace_path, theme);

    // Activity rail - now uses the consolidated implementation from layout/activity_rail.rs
    let activity_rail = activity_rail::activity_rail(app);

    // Main content area
    let ai_panel_widget: Element<_> = if ai_panel_visible {
        container(assistant::ai_panel(prompt_input))
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .into()
    } else {
        container(Space::new(Length::Fixed(0.0), Length::Fixed(0.0)))
            .width(Length::Fixed(0.0))
            .into()
    };

    let left_panel = container(explorer::explorer_panel(app))
        .width(Length::FillPortion(2))
        .height(Length::Fill);
    
    let editor_panel = container(editor::editor_panel(
        active_file_path,
        text_editor,
        is_dirty,
        editor_document,
        is_file_too_large_for_editor,
        file_loading_state,
        editor_typography,
        theme,
        syntax_highlight_cache,
        &app.tab_manager,
    ))
    .width(Length::FillPortion(5))
    .height(Length::Fill);

    let main_content = row![
        // Activity rail
        activity_rail,
        vertical_rule(1),
        // Left panel (explorer) - flexible width
        left_panel,
        vertical_rule(1),
        // Editor area - takes most space
        editor_panel,
        // AI panel (conditionally visible) - flexible width
        ai_panel_widget,
    ]
    .height(Length::Fill);

    // Status bar - now uses the consolidated implementation from layout/status_bar.rs
    let status_bar = status_bar::status_bar(app);

    // Combine everything
    let content = column![
        top_bar,
        horizontal_rule(1),
        main_content,
        horizontal_rule(1),
        status_bar,
    ]
    .height(Length::Fill);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
            container::Appearance {
                background: Some(style.colors.app_background.into()),
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        })))
        .into()
}
