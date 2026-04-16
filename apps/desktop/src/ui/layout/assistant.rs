use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text},
    Alignment, Color, Element, Length,
};

use crate::message::Message;
use crate::ui::icons::Icon;
use crate::settings::editor::EditorTypographySettings;

pub fn ai_panel<'a>(prompt_input: &'a str) -> Element<'a, Message> {
    // For the standalone ai_panel, we need to create a default theme
    // Since we don't have access to app state here, we'll use the dark theme colors
    let colors = crate::theme::QyzerTheme::Dark.colors();
    let scroll_style = AssistantScrollableStyle { colors };
    
    column![
        // Header - refined with subtle styling
        Element::from(container(
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
        .width(Length::Fill)),
        
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
        
        // Content area with custom scrollbar to match explorer
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
        .height(Length::Fill)
        .style(iced::theme::Scrollable::Custom(Box::new(scroll_style))),
        
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

// Custom scrollable style for assistant panel to match explorer
struct AssistantScrollableStyle {
    colors: crate::theme::SemanticColors,
}

impl iced::widget::scrollable::StyleSheet for AssistantScrollableStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::scrollable::Appearance {
        iced::widget::scrollable::Appearance {
            container: container::Appearance::default(),
            scrollbar: iced::widget::scrollable::Scrollbar {
                background: Some(Color::TRANSPARENT.into()),
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                scroller: iced::widget::scrollable::Scroller {
                    color: Color::from_rgba(
                        self.colors.border.r,
                        self.colors.border.g,
                        self.colors.border.b,
                        0.3,
                    ),
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: 3.0.into(),
                    },
                },
            },
            gap: None,
        }
    }

    fn hovered(&self, style: &Self::Style, _is_mouse_over_scrollbar: bool) -> iced::widget::scrollable::Appearance {
        let mut active = self.active(style);
        active.scrollbar.scroller.color = Color::from_rgba(
            self.colors.accent.r,
            self.colors.accent.g,
            self.colors.accent.b,
            0.7,
        );
        active
    }
}

// Also provide assistant_panel function for compatibility
pub fn assistant_panel(app: &crate::state::App) -> Element<'_, Message> {
    // Get the theme colors
    let colors = app.current_theme.colors();
    
    // Create a modified ai_panel that uses theme colors
    let style = AssistantScrollableStyle { colors };
    
    // We need to modify the ai_panel to use the custom scrollable style
    // For now, we'll use the same implementation but with the custom style
    // Since ai_panel doesn't have access to app, we need to restructure
    // Let's create a helper function that takes colors
    ai_panel_with_theme(&app.prompt_input, style)
}

fn ai_panel_with_theme<'a>(prompt_input: &'a str, scroll_style: AssistantScrollableStyle) -> Element<'a, Message> {
    // Same content as ai_panel but with custom scrollable style
    column![
        // Header - refined with subtle styling
        Element::from(container(
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
        .width(Length::Fill)),
        
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
        
        // Content area with custom scrollbar
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
        .height(Length::Fill)
        .style(iced::theme::Scrollable::Custom(Box::new(scroll_style))),
        
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
