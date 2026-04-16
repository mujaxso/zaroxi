use iced::{
    widget::{column, container, row, vertical_rule, Space},
    Element, Length,
};

use crate::state::App;
use crate::message::Message;
use crate::theme::QyzerTheme;
use crate::ui::style::StyleHelpers;
use crate::settings::editor::EditorTypographySettings;

// Import layout modules
use super::layout::topbar::top_bar;
use super::layout::activity_rail::activity_rail;
use super::layout::explorer::left_panel_with_expanded;
use super::layout::editor::editor_panel;
use super::layout::assistant::ai_panel;
use super::layout::status_bar::status_bar;

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
    theme: QyzerTheme,
    editor_typography: &'a EditorTypographySettings,
    syntax_highlight_cache: Option<Vec<Vec<(Range<usize>, Color)>>>,
) -> Element<'a, Message> {
    let style = StyleHelpers::new(theme);
    
    // Top bar
    let top_bar = top_bar(workspace_path, is_dirty);

    // Activity rail - now uses the consolidated implementation from layout/activity_rail.rs
    let activity_rail = activity_rail(app);

    // Main content area
    let ai_panel_widget: Element<_> = if ai_panel_visible {
        container(ai_panel(prompt_input))
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .into()
    } else {
        container(Space::new(Length::Fixed(0.0), Length::Fixed(0.0)))
            .width(Length::Fixed(0.0))
            .into()
    };

    let main_content = row![
        // Activity rail
        activity_rail,
        vertical_rule(1),
        // Left panel (explorer) - flexible width
        container(left_panel_with_expanded(file_entries, app.active_activity, _expanded_directories, workspace_path))
            .width(Length::FillPortion(2))
            .height(Length::Fill),
        vertical_rule(1),
        // Editor area - takes most space
        container(editor_panel(active_file_path, text_editor, is_dirty, editor_document, is_file_too_large_for_editor, file_loading_state, editor_typography, theme, syntax_highlight_cache))
            .width(Length::FillPortion(5))
            .height(Length::Fill),
        // AI panel (conditionally visible) - flexible width
        ai_panel_widget,
    ]
    .height(Length::Fill);

    // Status bar - now uses the consolidated implementation from layout/status_bar.rs
    let status_bar = status_bar(app);

    // Combine everything
    let content = column![
        top_bar,
        iced::widget::horizontal_rule(1),
        main_content,
        iced::widget::horizontal_rule(1),
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

fn top_bar<'a>(workspace_path: &'a str, is_dirty: bool) -> Element<'a, Message> {
    // Use theme colors from the outer scope
    // Since we can't access them directly, we'll use a more IDE-like approach
    // For now, use colors that match IDE conventions
    let status_indicator: Element<_> = if is_dirty {
        row![
            text("●").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 180, 0))),
            text("Unsaved").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(220, 220, 220)))
        ]
        .spacing(4)
        .align_items(Alignment::Center)
        .into()
    } else {
        row![
            text("✓").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(0, 200, 100))),
            text("Saved").size(12).style(iced::theme::Text::Color(iced::Color::from_rgb8(180, 180, 180)))
        ]
        .spacing(4)
        .align_items(Alignment::Center)
        .into()
    };

    row![
        // Logo/brand
        row![
            text("Q").size(20).style(iced::theme::Text::Color(iced::Color::from_rgb8(100, 160, 255))),
            text("yzer").size(20).style(iced::theme::Text::Color(iced::Color::from_rgb8(220, 220, 230))),
        ]
        .spacing(0)
        .align_items(Alignment::Center),
        horizontal_space(),
        // Workspace path display with manual entry option
        if workspace_path.is_empty() {
            // When no workspace is open, show an input field for manual entry
            container(
                column![
                    row![
                        text_input("Enter workspace path manually...", workspace_path)
                            .on_input(Message::WorkspacePathChanged)
                            .on_submit(Message::SubmitManualWorkspacePath(workspace_path.to_string()))
                            .padding([10, 12])
                            .width(Length::Fill)
                            .style(iced::theme::TextInput::Default),
                        button("Open")
                            .on_press(Message::SubmitManualWorkspacePath(workspace_path.to_string()))
                            .padding([10, 14])
                            .style(iced::theme::Button::Secondary),
                    ]
                    .spacing(8)
                    .align_items(Alignment::Center),
                    text("Or use the file picker (uses system theme)")
                        .size(10)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150)))
                ]
                .spacing(4)
            )
            .width(Length::FillPortion(3))
            .into()
        } else {
            // When workspace is open, show it as read-only
            container(
                container(
                    text(workspace_path)
                        .size(14)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(220, 220, 220)))
                )
                .padding([10, 12])
                .width(Length::Fill)
                .style(iced::theme::Container::Box)
            )
            .width(Length::FillPortion(3))
            .style(iced::theme::Container::Box)
        },
        // Buttons
        row![
            button("Open Workspace...")
                .on_press(Message::OpenWorkspace)
                .padding([8, 14])
                .style(iced::theme::Button::Secondary),
            button("Refresh")
                .on_press(Message::RefreshWorkspace)
                .padding([8, 14])
                .style(iced::theme::Button::Secondary),
        ]
        .spacing(8),
        horizontal_space(),
        // Status indicator
        container(status_indicator)
            .padding([6, 12])
            .style(iced::theme::Container::Box),
        // Save button
        button("Save")
            .on_press(Message::SaveFile)
            .padding([10, 18])
            .style(iced::theme::Button::Primary),
    ]
    .padding([12, 20])
    .align_items(Alignment::Center)
    .into()
}







