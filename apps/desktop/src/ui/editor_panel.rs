use iced::{Element, Length, Color, widget::{column, container, row, text}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;
use super::editor;
use crate::ui::icons::Icon;

pub fn editor_panel(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    let header = if let Some(active_path) = &app.active_file_path {
        let file_name = active_path.split('/').last().unwrap_or(active_path);
        container(
            row![
                Icon::File.render(&app.editor_typography, &style, Some(9)),
                text(file_name)
                    .size(9)
                    .style(iced::theme::Text::Color(style.colors.text_primary)),
                iced::widget::horizontal_space(),
                if app.is_dirty {
                    Icon::Warning.render_with_color(
                        &app.editor_typography,
                        style.colors.warning,
                        Some(7),
                    )
                } else {
                    Icon::Success.render_with_color(
                        &app.editor_typography,
                        style.colors.success,
                        Some(7),
                    )
                }
            ]
            .spacing(1)  // Even less spacing
            .align_items(iced::Alignment::Center)
        )
        .padding([0, 4])  // No vertical padding
        .width(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
            container::Appearance {
                background: None,  // No background to blend with editor
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        })))
    } else {
        container(
            row![
                Icon::File.render(&app.editor_typography, &style, Some(9)),
                text("No file selected")
                    .size(9)
                    .style(iced::theme::Text::Color(style.colors.text_muted)),
            ]
            .spacing(1)  // Even less spacing
            .align_items(iced::Alignment::Center)
        )
        .padding([0, 4])  // No vertical padding
        .width(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
            container::Appearance {
                background: None,  // No background to blend with editor
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        })))
    };
    
    let editor_content = if let Some(_) = &app.active_file_path {
        // Use the existing editor component with typography settings
        editor::editor(&app.text_editor, &app.editor_typography)
    } else {
        // Welcome screen
        container(
            column![
                text("Neote").size(32)
                    .style(iced::theme::Text::Color(style.colors.text_primary)),
                text("AI‑first IDE").size(16)
                    .style(iced::theme::Text::Color(style.colors.text_secondary)),
                container(iced::widget::horizontal_rule(1)).width(150),
                column![
                    text("Open a file from the explorer")
                        .style(iced::theme::Text::Color(style.colors.text_muted)),
                    text("Ask AI about the workspace")
                        .style(iced::theme::Text::Color(style.colors.text_muted)),
                ]
                .spacing(4)
                .padding(8),
            ]
            .spacing(8)
            .align_items(iced::Alignment::Center)
        )
        .center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    };
    
    // Create a clean, borderless editor area that fills available space
    // The editor should directly fill the area without extra containers
    column![
        header,
        // Editor content should fill all remaining space
        editor_content
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .spacing(0)  // No spacing between header and editor
    .into()
}
