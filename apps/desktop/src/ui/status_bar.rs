use iced::{Element, Length, widget::{container, row, text}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;
use crate::theme::SemanticColors;
use crate::ui::icons::Icon;

pub fn status_bar(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    let left_status = if let Some(error) = &app.error_message {
        row![
            Icon::Error.render_with_color(
                &app.editor_typography,
                style.colors.error,
                Some(10),
            ),
            text(error).size(10).style(iced::theme::Text::Color(style.colors.error)),
        ]
        .spacing(4)
        .align_items(iced::Alignment::Center)
    } else {
        row![
            Icon::Success.render_with_color(
                &app.editor_typography,
                style.colors.success,
                Some(10),
            ),
            text(&app.status_message).size(10).style(iced::theme::Text::Color(style.colors.text_secondary)),
        ]
        .spacing(4)
        .align_items(iced::Alignment::Center)
    };
    
    let center_status = if let Some(active_path) = &app.active_file_path {
        let file_name = active_path.split('/').last().unwrap_or(active_path);
        row![
            Icon::File.render(&app.editor_typography, &style, Some(10)),
            text(file_name).size(10).style(iced::theme::Text::Color(style.colors.text_secondary)),
        ]
        .spacing(4)
        .align_items(iced::Alignment::Center)
    } else {
        row![
            text("No file open").size(10).style(iced::theme::Text::Color(style.colors.text_muted)),
        ]
        .align_items(iced::Alignment::Center)
    };
    
    let right_status = row![
        text(format!("{} files", app.file_entries.len())).size(10).style(iced::theme::Text::Color(style.colors.text_muted)),
        text("Ln 1, Col 1").size(10).style(iced::theme::Text::Color(style.colors.text_muted)),
    ]
    .spacing(6)
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
                    color: self.colors.border,
                    width: 0.0,
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
            container(left_status).padding([0, 6]),
            iced::widget::horizontal_space(),
            container(center_status),
            iced::widget::horizontal_space(),
            container(right_status).padding([0, 6]),
        ]
        .align_items(iced::Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(status_bar_style)))
    .into()
}
