use iced::{Element, Length, widget::{button, column, container, row, scrollable, text, text_input}};
use iced::widget::text_input;
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;

pub fn assistant_panel(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    let header = container(
        row![
            text("AI ASSISTANT")
                .size(10)
                .style(iced::theme::Text::Color(style.colors.text_muted)),
            iced::widget::horizontal_space(),
            button(
                text("⋯").size(12)
            )
            .on_press(Message::PromptInputChanged("AI options".to_string()))
            .padding([2, 6])
            .style(iced::theme::Button::Secondary)
        ]
        .align_items(iced::Alignment::Center)
    )
    .padding([8, 12])
    .width(Length::Fill);
    
    let welcome_card = container(
        column![
            row![
                text("🤖").size(16),
                text("Neote AI").size(14)
                    .style(iced::theme::Text::Color(style.colors.text_primary)),
            ]
            .spacing(6)
            .align_items(iced::Alignment::Center),
            text("Ask questions about your code, get explanations, refactor suggestions, and more.")
                .size(12)
                .style(iced::theme::Text::Color(style.colors.text_secondary)),
        ]
        .spacing(8)
        .padding(16)
    )
    .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
        container::Appearance {
            background: Some(style.colors.elevated_panel_background.into()),
            border: iced::Border {
                color: style.colors.accent,
                width: 1.0,
                radius: crate::ui::common::RADIUS_MD.into(),
            },
            ..Default::default()
        }
    })));
    
    let quick_actions = container(
        column![
            text("Quick Actions").size(11)
                .style(iced::theme::Text::Color(style.colors.text_muted)),
            column![
                button("Explain this file")
                    .on_press(Message::PromptInputChanged("Explain the current file".to_string()))
                    .padding([8, 10])
                    .width(Length::Fill)
                    .style(iced::theme::Button::Secondary),
                button("Refactor selection")
                    .on_press(Message::PromptInputChanged("Refactor the selected code".to_string()))
                    .padding([8, 10])
                    .width(Length::Fill)
                    .style(iced::theme::Button::Secondary),
                button("Find bugs")
                    .on_press(Message::PromptInputChanged("Find potential bugs in this code".to_string()))
                    .padding([8, 10])
                    .width(Length::Fill)
                    .style(iced::theme::Button::Secondary),
            ]
            .spacing(4),
        ]
        .spacing(8)
        .padding(12)
    );
    
    let input_area = container(
        column![
            text_input("Ask Neote AI...", &app.prompt_input)
                .on_input(Message::PromptInputChanged)
                .padding([12, 14])
                .width(Length::Fill)
                .style(iced::theme::TextInput::Custom(Box::new(move |_theme| {
                    text_input::Appearance {
                        background: style.colors.input_background.into(),
                        border: iced::Border {
                            color: style.colors.border,
                            width: 1.0,
                            radius: crate::ui::common::RADIUS_SM.into(),
                        },
                        icon_color: style.colors.text_muted,
                    }
                }))),
            row![
                button("Send")
                    .on_press(Message::SendPrompt)
                    .padding([12, 18])
                    .style(iced::theme::Button::Primary),
            ]
            .spacing(8)
        ]
        .spacing(12)
    )
    .padding(16)
    .width(Length::Fill);
    
    container(
        column![
            header,
            scrollable(
                column![
                    welcome_card,
                    quick_actions,
                ]
                .spacing(16)
                .padding([0, 16])
            )
            .height(Length::Fill),
            input_area,
        ]
        .height(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(move |_theme| {
        container::Appearance {
            background: Some(style.colors.panel_background.into()),
            border: iced::Border {
                color: style.colors.border,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    })))
    .into()
}
