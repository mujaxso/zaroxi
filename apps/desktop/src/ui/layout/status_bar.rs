use iced::{Element, Length, widget::{container, row, text}};
use crate::message::Message;
use crate::state::App;
use super::super::style::StyleHelpers;
use crate::theme::SemanticColors;
use crate::ui::icons::Icon;

pub fn status_bar(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.current_theme);
    
    let left_status = if let Some(error) = &app.error_message {
        row![
            Icon::Error.render_with_color(
                &app.editor_typography,
                style.colors.error,
                Some(11),
            ),
            text(error)
                .size(11)
                .font(iced::Font::with_name("JetBrains Mono"))
                .style(iced::theme::Text::Color(style.colors.error)),
        ]
        .spacing(6)
        .align_items(iced::Alignment::Center)
    } else {
        row![
            Icon::Success.render_with_color(
                &app.editor_typography,
                style.colors.success,
                Some(11),
            ),
            text(&app.status_message)
                .size(11)
                .font(iced::Font::with_name("JetBrains Mono"))
                .style(iced::theme::Text::Color(style.colors.text_secondary)),
        ]
        .spacing(6)
        .align_items(iced::Alignment::Center)
    };
    
    let center_status = if let Some(active_path) = &app.active_file_path {
        let file_name = active_path.split('/').last().unwrap_or(active_path);
        row![
            Icon::File.render(&app.editor_typography, &style, Some(11)),
            text(file_name)
                .size(11)
                .font(iced::Font::with_name("JetBrains Mono"))
                .style(iced::theme::Text::Color(style.colors.text_secondary)),
        ]
        .spacing(6)
        .align_items(iced::Alignment::Center)
    } else {
        row![
            text("No file open")
                .size(11)
                .font(iced::Font::with_name("JetBrains Mono"))
                .style(iced::theme::Text::Color(style.colors.text_muted)),
        ]
        .align_items(iced::Alignment::Center)
    };
    
    let right_status = row![
        // Saved/unsaved status indicator
        if app.is_dirty {
            row![
                Icon::Warning.render_with_color(
                    &app.editor_typography,
                    style.colors.warning,
                    Some(11),
                ),
                text("Unsaved")
                    .size(11)
                    .font(iced::Font::with_name("JetBrains Mono"))
                    .style(iced::theme::Text::Color(style.colors.warning)),
            ]
            .spacing(4)
            .align_items(iced::Alignment::Center)
        } else {
            row![
                Icon::Success.render_with_color(
                    &app.editor_typography,
                    style.colors.success,
                    Some(11),
                ),
                text("Saved")
                    .size(11)
                    .font(iced::Font::with_name("JetBrains Mono"))
                    .style(iced::theme::Text::Color(style.colors.success)),
            ]
            .spacing(4)
            .align_items(iced::Alignment::Center)
        },
        text(format!("{} files", app.file_entries.len()))
            .size(11)
            .font(iced::Font::with_name("JetBrains Mono"))
            .style(iced::theme::Text::Color(style.colors.text_muted)),
        text("Ln 1, Col 1")
            .size(11)
            .font(iced::Font::with_name("JetBrains Mono"))
            .style(iced::theme::Text::Color(style.colors.text_muted)),
    ]
    .spacing(12)
    .align_items(iced::Alignment::Center);
    
    struct StatusBarStyle {
        colors: SemanticColors,
    }
    
    impl iced::widget::container::StyleSheet for StatusBarStyle {
        type Style = iced::Theme;
        
        fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
            container::Appearance {
                background: Some(self.colors.status_bar_background.into()),
                border: iced::Border {
                    color: iced::Color::from_rgba(
                        self.colors.border.r,
                        self.colors.border.g,
                        self.colors.border.b,
                        0.2
                    ),
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        }
    }
    
    let status_bar_style = StatusBarStyle {
        colors: style.colors,
    };
    
    container(
        row![
            container(left_status).padding([0, 12]),
            iced::widget::horizontal_space(),
            container(center_status),
            iced::widget::horizontal_space(),
            container(right_status).padding([0, 12]),
        ]
        .align_items(iced::Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fixed(28.0))
    .style(iced::theme::Container::Custom(Box::new(status_bar_style)))
    .into()
}
