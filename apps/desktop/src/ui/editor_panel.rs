use iced::{Element, Length, widget::{column, container, row, text}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;
use super::editor;

pub fn editor_panel(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    let header = if let Some(active_path) = &app.active_file_path {
        let file_name = active_path.split('/').last().unwrap_or(active_path);
        container(
            row![
                text("📄").size(14),
                text(file_name)
                    .size(13)
                    .style(iced::theme::Text::Color(style.colors.text_primary)),
                iced::widget::horizontal_space(),
                if app.is_dirty {
                    text("● Unsaved").size(11)
                        .style(iced::theme::Text::Color(style.colors.warning))
                } else {
                    text("✓ Saved").size(11)
                        .style(iced::theme::Text::Color(style.colors.success))
                }
            ]
            .spacing(8)
            .align_items(iced::Alignment::Center)
        )
        .padding([12, 16])
        .width(Length::Fill)
    } else {
        container(
            text("No file selected")
                .style(iced::theme::Text::Color(style.colors.text_muted))
        )
        .padding([12, 16])
        .width(Length::Fill)
    };
    
    let editor_content = if let Some(_) = &app.active_file_path {
        // Use the existing editor component
        editor::editor(&app.text_editor)
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
                .spacing(8)
                .padding(16),
            ]
            .spacing(16)
            .align_items(iced::Alignment::Center)
        )
        .center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    };
    
    container(
        column![
            header,
            container(editor_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(iced::theme::Container::Custom(Box::new(move |theme| {
                    container::Appearance {
                        background: Some(iced::Background::Color(match theme {
                            iced::Theme::Dark => Color::from_rgb(0.08, 0.08, 0.10),
                            _ => Color::from_rgb(1.0, 1.0, 1.0),
                        })),
                        border: iced::Border::default(),
                        ..Default::default()
                    }
                }))),
        ]
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(move |theme| {
        container::Appearance {
            background: Some(iced::Background::Color(match theme {
                iced::Theme::Dark => Color::from_rgb(0.10, 0.10, 0.12),
                _ => Color::from_rgb(0.98, 0.98, 0.98),
            })),
            border: iced::Border {
                color: match theme {
                    iced::Theme::Dark => Color::from_rgb(0.2, 0.2, 0.25),
                    _ => Color::from_rgb(0.8, 0.8, 0.8),
                },
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    })))
    .into()
}
