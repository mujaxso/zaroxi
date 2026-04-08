use iced::{Element, Length, widget::{button, column, container, row, scrollable, text, text_input}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;

pub fn assistant_panel(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    let header = container(
        row![
            text("AI ASSISTANT")
                .size(11)
                .style(iced::theme::Text::Color(style.colors.text_muted)),
            iced::widget::horizontal_space(),
            button(
                text("⋯").size(14)
            )
            .on_press(Message::PromptInputChanged("AI options".to_string()))
            .padding([4, 8])
            .style(iced::theme::Button::Secondary)
        ]
        .align_items(iced::Alignment::Center)
    )
    .padding([12, 16])
    .width(Length::Fill);
    
    let welcome_card = container(
        column![
            row![
                text("🤖").size(20),
                text("Neote AI").size(16)
                    .style(iced::theme::Text::Color(Color::from_rgb(0.9, 0.9, 1.0))),
            ]
            .spacing(8)
            .align_items(iced::Alignment::Center),
            text("Ask questions about your code, get explanations, refactor suggestions, and more.")
                .size(13)
                .style(iced::theme::Text::Color(Color::from_rgb(0.8, 0.85, 0.9))),
        ]
        .spacing(10)
        .padding(20)
    )
    .style(iced::theme::Container::Custom(Box::new(move |theme| {
        container::Appearance {
            background: Some(iced::Background::Color(match theme {
                iced::Theme::Dark => Color::from_rgb(0.12, 0.12, 0.16),
                _ => Color::from_rgb(0.95, 0.95, 0.98),
            })),
            border: iced::Border {
                color: match theme {
                    iced::Theme::Dark => Color::from_rgb(0.25, 0.45, 0.85),
                    _ => Color::from_rgb(0.6, 0.7, 1.0),
                },
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        }
    })));
    
    let quick_actions = container(
        column![
            text("Quick Actions").size(13)
                .style(iced::theme::Text::Color(style.colors.text_muted)),
            column![
                button("Explain this file")
                    .on_press(Message::PromptInputChanged("Explain the current file".to_string()))
                    .padding([10, 12])
                    .width(Length::Fill)
                    .style(iced::theme::Button::Secondary),
                button("Refactor selection")
                    .on_press(Message::PromptInputChanged("Refactor the selected code".to_string()))
                    .padding([10, 12])
                    .width(Length::Fill)
                    .style(iced::theme::Button::Secondary),
                button("Find bugs")
                    .on_press(Message::PromptInputChanged("Find potential bugs in this code".to_string()))
                    .padding([10, 12])
                    .width(Length::Fill)
                    .style(iced::theme::Button::Secondary),
            ]
            .spacing(6),
        ]
        .spacing(12)
        .padding(16)
    );
    
    let input_area = container(
        column![
            text_input("Ask Neote AI...", &app.prompt_input)
                .on_input(Message::PromptInputChanged)
                .padding([12, 14])
                .width(Length::Fill),
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
    .style(iced::theme::Container::Box)
    .into()
}
