use iced::{Element, Length, Color, widget::{button, column, container, row, scrollable, text, text_input}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;
use crate::theme::SemanticColors;

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
    
    struct WelcomeCardStyle {
        colors: SemanticColors,
    }
    
    impl iced::widget::container::StyleSheet for WelcomeCardStyle {
        type Style = iced::Theme;
        
        fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
            container::Appearance {
                background: Some(self.colors.elevated_panel_background.into()),
                border: iced::Border {
                    color: self.colors.accent,
                    width: 1.0,
                    radius: crate::ui::common::RADIUS_MD.into(),
                },
                ..Default::default()
            }
        }
    }
    
    let welcome_card_style = WelcomeCardStyle {
        colors: style.colors,
    };
    
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
    .style(iced::theme::Container::Custom(Box::new(welcome_card_style)));
    
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
    
    struct AssistantInputStyle {
        colors: SemanticColors,
    }
    
    impl iced::widget::text_input::StyleSheet for AssistantInputStyle {
        type Style = iced::Theme;
        
        fn active(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
            iced::widget::text_input::Appearance {
                background: self.colors.input_background.into(),
                border: iced::Border {
                    color: self.colors.border,
                    width: 1.0,
                    radius: crate::ui::common::RADIUS_SM.into(),
                },
                icon_color: self.colors.text_muted,
            }
        }
        
        fn focused(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
            iced::widget::text_input::Appearance {
                background: self.colors.input_background.into(),
                border: iced::Border {
                    color: self.colors.accent,
                    width: 1.0,
                    radius: crate::ui::common::RADIUS_SM.into(),
                },
                icon_color: self.colors.text_muted,
            }
        }
        
        fn placeholder_color(&self, _style: &Self::Style) -> Color {
            self.colors.text_muted
        }
        
        fn value_color(&self, _style: &Self::Style) -> Color {
            self.colors.text_primary
        }
        
        fn selection_color(&self, _style: &Self::Style) -> Color {
            self.colors.accent_soft_background
        }
        
        fn disabled_color(&self, _style: &Self::Style) -> Color {
            self.colors.text_muted
        }
        
        fn disabled(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
            iced::widget::text_input::Appearance {
                background: self.colors.input_background.into(),
                border: iced::Border {
                    color: self.colors.border,
                    width: 1.0,
                    radius: crate::ui::common::RADIUS_SM.into(),
                },
                icon_color: self.colors.text_muted,
            }
        }
    }
    
    let input_style = AssistantInputStyle {
        colors: style.colors,
    };
    
    let input_area = container(
        column![
            text_input("Ask Neote AI...", &app.prompt_input)
                .on_input(Message::PromptInputChanged)
                .padding([12, 14])
                .width(Length::Fill)
                .style(iced::theme::TextInput::Custom(Box::new(input_style))),
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
    
    struct AssistantPanelContainerStyle {
        colors: SemanticColors,
    }
    
    impl iced::widget::container::StyleSheet for AssistantPanelContainerStyle {
        type Style = iced::Theme;
        
        fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
            container::Appearance {
                background: Some(self.colors.panel_background.into()),
                border: iced::Border {
                    color: self.colors.border,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        }
    }
    
    let container_style = AssistantPanelContainerStyle {
        colors: style.colors,
    };
    
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
    .style(iced::theme::Container::Custom(Box::new(container_style)))
    .into()
}
