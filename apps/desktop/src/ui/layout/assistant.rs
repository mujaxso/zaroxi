use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text, text_input},
    Alignment, Element, Length,
};

use crate::message::Message;
use crate::ui::icons::Icon;
use crate::settings::editor::EditorTypographySettings;

pub fn ai_panel<'a>(prompt_input: &'a str) -> Element<'a, Message> {
    column![
        // Header - refined with subtle styling
        container(
            row![
                text("AI ASSISTANT").size(11)
                    .style(iced::theme::Text::Color(iced::Color::from_rgb8(160, 160, 170))),
                horizontal_space(),
                button("⋯")
                    .on_press(Message::PromptInputChanged("AI options".to_string()))
                    .padding([4, 6])
                    .style(iced::theme::Button::Text),
            ]
            .align_items(Alignment::Center)
        )
        .padding([12, 16])
        .width(Length::Fill),
        
        // Subtle divider
        container(iced::widget::Space::with_height(1.0))
            .style(iced::theme::Container::Custom(Box::new(|_theme: &iced::Theme| {
                container::Appearance {
                    background: Some(iced::Color::from_rgb8(60, 65, 85).into()),
                    ..Default::default()
                }
            })))
            .width(Length::Fill)
            .height(Length::Fixed(1.0)),
        
        // Content area
        scrollable(
            column![
                // Welcome card - refined with subtle border
                container(
                    column![
                        row![
                            Icon::Robot.render_with_color(
                                &EditorTypographySettings::default(),
                                iced::Color::from_rgb8(100, 160, 255),
                                Some(18)
                            ),
                            text("Qyzer Studio AI").size(15)
                                .style(iced::theme::Text::Color(iced::Color::from_rgb8(220, 220, 255))),
                        ]
                        .spacing(8)
                        .align_items(Alignment::Center),
                        text("Ask questions about your code, get explanations, refactor suggestions, and more.")
                            .size(13)
                            .style(iced::theme::Text::Color(iced::Color::from_rgb8(180, 190, 210))),
                    ]
                    .spacing(10)
                    .padding(16)
                )
                .style(iced::theme::Container::Custom(Box::new(|_theme: &iced::Theme| {
                    container::Appearance {
                        background: Some(iced::Color::from_rgb8(30, 33, 45).into()),
                        border: iced::Border {
                            color: iced::Color::from_rgb8(60, 65, 85),
                            width: 1.0,
                            radius: 6.0.into(),
                        },
                        ..Default::default()
                    }
                }))),
                
                // Quick actions - refined spacing
                container(
                    column![
                        text("Quick Actions").size(12)
                            .style(iced::theme::Text::Color(iced::Color::from_rgb8(160, 160, 170))),
                        column![
                            button("Explain this file")
                                .on_press(Message::PromptInputChanged("Explain the current file".to_string()))
                                .padding([8, 12])
                                .width(Length::Fill)
                                .style(iced::theme::Button::Secondary),
                            button("Refactor selection")
                                .on_press(Message::PromptInputChanged("Refactor the selected code".to_string()))
                                .padding([8, 12])
                                .width(Length::Fill)
                                .style(iced::theme::Button::Secondary),
                            button("Find bugs")
                                .on_press(Message::PromptInputChanged("Find potential bugs in this code".to_string()))
                                .padding([8, 12])
                                .width(Length::Fill)
                                .style(iced::theme::Button::Secondary),
                            button("Write tests")
                                .on_press(Message::PromptInputChanged("Write unit tests for this code".to_string()))
                                .padding([8, 12])
                                .width(Length::Fill)
                                .style(iced::theme::Button::Secondary),
                        ]
                        .spacing(4),
                    ]
                    .spacing(10)
                    .padding(16)
                ),
                
                // Info note - subtle
                container(
                    text("AI features are in development. This is a preview interface.")
                        .size(11)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 160)))
                )
                .padding(16),
            ]
            .spacing(12)
            .padding([12, 16])
        )
        .height(Length::Fill),
        
        // Input area - refined with better spacing
        container(
            row![
                {
                    // Create text input with explicit type annotation
                    let mut input: iced::widget::TextInput<'_, Message, iced::Theme, iced::Renderer> = 
                        iced::widget::text_input("Ask Qyzer Studio AI...", prompt_input);
                    input = input.on_input(Message::PromptInputChanged);
                    input = input.padding([10, 12]);
                    input = input.width(Length::Fill);
                    input
                },
                button("Send")
                    .on_press(Message::SendPrompt)
                    .padding([10, 16])
                    .style(iced::theme::Button::Primary),
            ]
            .align_items(Alignment::Center)
        )
        .padding([12, 16])
        .width(Length::Fill),
    ]
    .height(Length::Fill)
    .into()
}

// Also provide assistant_panel function for compatibility
pub fn assistant_panel(app: &crate::state::App) -> Element<'_, Message> {
    // Use the same implementation as ai_panel but with app context
    ai_panel(&app.prompt_input)
}
