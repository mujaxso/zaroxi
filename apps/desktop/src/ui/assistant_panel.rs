use iced::{Element, Length, Color, widget::{button, column, container, row, scrollable, text, text_input}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;
use crate::theme::SemanticColors;
use crate::ui::icons::{Icon, icon_button};

pub fn assistant_panel(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    struct AssistantScrollableStyle {
        colors: SemanticColors,
    }
    
    impl iced::widget::scrollable::StyleSheet for AssistantScrollableStyle {
        type Style = iced::Theme;
        
        fn active(&self, _style: &Self::Style) -> iced::widget::scrollable::Appearance {
            iced::widget::scrollable::Appearance {
                container: iced::widget::container::Appearance::default(),
                scrollbar: iced::widget::scrollable::Scrollbar {
                    background: None,
                    border: iced::Border::default(),
                    scroller: iced::widget::scrollable::Scroller {
                        color: self.colors.border,
                        border: iced::Border::default(),
                    },
                },
                gap: None,
            }
        }
        
        fn hovered(
            &self,
            _style: &Self::Style,
            is_mouse_over_scrollbar: bool,
        ) -> iced::widget::scrollable::Appearance {
            let scroller_color = if is_mouse_over_scrollbar {
                self.colors.accent
            } else {
                self.colors.border
            };
            
            iced::widget::scrollable::Appearance {
                container: iced::widget::container::Appearance::default(),
                scrollbar: iced::widget::scrollable::Scrollbar {
                    background: None,
                    border: iced::Border::default(),
                    scroller: iced::widget::scrollable::Scroller {
                        color: scroller_color,
                        border: iced::Border::default(),
                    },
                },
                gap: None,
            }
        }
    }
    
    // Adjust content based on layout mode
    let is_compact = matches!(app.layout_mode, crate::state::LayoutMode::Medium | crate::state::LayoutMode::Narrow);
    
    let header = container(
        row![
            text("AI ASSISTANT")
                .size(if is_compact { 9 } else { 10 })
                .style(iced::theme::Text::Color(style.colors.text_muted)),
            iced::widget::horizontal_space(),
            if !is_compact {
                let btn: Element<_> = icon_button(
                    Icon::MoreHorizontal,
                    &app.editor_typography,
                    &style,
                    Some(Message::PromptInputChanged("AI options".to_string())),
                    Some(12),
                )
                .padding([2, 6])
                .into();
                btn
            } else {
                let space: Element<_> = iced::widget::Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into();
                space
            }
        ]
        .align_items(iced::Alignment::Center)
    )
    .padding(if is_compact { [7, 12] } else { [9, 16] })
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
    
    let welcome_card: Element<_> = if !is_compact {
        container(
            column![
                row![
                    Icon::Robot.render_with_color(&app.editor_typography, style.colors.text_primary, Some(16)),
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
            .padding([16, 20])
        )
        .style(iced::theme::Container::Custom(Box::new(welcome_card_style)))
        .into()
    } else {
        // Compact version
        container(
            column![
                row![
                    Icon::Robot.render_with_color(&app.editor_typography, style.colors.text_primary, Some(14)),
                    text("AI").size(12)
                        .style(iced::theme::Text::Color(style.colors.text_primary)),
                ]
                .spacing(4)
                .align_items(iced::Alignment::Center),
            ]
            .spacing(4)
            .padding([12, 16])
        )
        .style(iced::theme::Container::Custom(Box::new(welcome_card_style)))
        .into()
    };
    
    let quick_actions = container(
        column![
            if !is_compact {
                let txt: Element<_> = text("Quick Actions").size(11)
                    .style(iced::theme::Text::Color(style.colors.text_muted))
                    .into();
                txt
            } else {
                let space: Element<_> = iced::widget::Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into();
                space
            },
            column![
                button(if is_compact { "Explain" } else { "Explain this file" })
                    .on_press(Message::PromptInputChanged("Explain the current file".to_string()))
                    .padding(if is_compact { [8, 12] } else { [10, 14] })
                    .width(Length::Fill)
                    .style(iced::theme::Button::Secondary),
                button(if is_compact { "Refactor" } else { "Refactor selection" })
                    .on_press(Message::PromptInputChanged("Refactor the selected code".to_string()))
                    .padding(if is_compact { [8, 12] } else { [10, 14] })
                    .width(Length::Fill)
                    .style(iced::theme::Button::Secondary),
                if !is_compact {
                    let btn: Element<_> = button("Find bugs")
                        .on_press(Message::PromptInputChanged("Find potential bugs in this code".to_string()))
                        .padding([10, 14])
                        .width(Length::Fill)
                        .style(iced::theme::Button::Secondary)
                        .into();
                    btn
                } else {
                    let space: Element<_> = iced::widget::Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into();
                    space
                },
            ]
            .spacing(if is_compact { 4 } else { 6 }),
        ]
        .spacing(if is_compact { 6 } else { 8 })
        .padding(if is_compact { [8, 12] } else { [12, 16] })
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
            text_input(if is_compact { "Ask..." } else { "Ask Neote AI..." }, &app.prompt_input)
                .on_input(Message::PromptInputChanged)
                .padding(if is_compact { [10, 14] } else { [14, 18] })
                .width(Length::Fill)
                .style(iced::theme::TextInput::Custom(Box::new(input_style))),
            row![
                button(if is_compact { "→" } else { "Send" })
                    .on_press(Message::SendPrompt)
                    .padding(if is_compact { [8, 12] } else { [12, 18] })
                    .style(iced::theme::Button::Primary),
            ]
            .spacing(8)
        ]
        .spacing(if is_compact { 8 } else { 12 })
    )
    .padding(if is_compact { [8, 8, 8, 8] } else { [16, 16, 16, 16] })
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
                .spacing(if is_compact { 8 } else { 16 })
                .padding(if is_compact { [0, 16] } else { [0, 24] })
            )
            .height(Length::Fill)
            .style(iced::theme::Scrollable::Custom(Box::new(AssistantScrollableStyle {
                colors: style.colors,
            })))
            .scrollable_properties(
                iced::widget::scrollable::Properties::new()
                    .scroller_width(8)
                    .scrollbar_margin(4)
            ),
            input_area,
        ]
        .height(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding([0, 0, 0, 0]) // Ensure no padding that could hide content
    .style(iced::theme::Container::Custom(Box::new(container_style)))
    .into()
}
