use iced::{Element, Length, Color, widget::{button, container, row, text}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;
use crate::theme::SemanticColors;
use crate::ui::icons::{Icon, icon_button};

pub fn top_bar(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    // Create a custom style sheet for the text input
    struct WorkspaceInputStyle {
        colors: SemanticColors,
    }
    
    impl iced::widget::text_input::StyleSheet for WorkspaceInputStyle {
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
    
    let _input_style = WorkspaceInputStyle {
        colors: style.colors,
    };
    
    // Responsive workspace path display (read-only)
    let is_compact = matches!(app.layout_mode, crate::state::LayoutMode::Medium | crate::state::LayoutMode::Narrow);
    
    // Create a container style for the workspace display
    struct WorkspaceDisplayStyle {
        colors: SemanticColors,
    }
    
    impl iced::widget::container::StyleSheet for WorkspaceDisplayStyle {
        type Style = iced::Theme;
        
        fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
            iced::widget::container::Appearance {
                background: Some(self.colors.input_background.into()),
                border: iced::Border {
                    color: self.colors.border,
                    width: 1.0,
                    radius: crate::ui::common::RADIUS_SM.into(),
                },
                ..Default::default()
            }
        }
    }
    
    let workspace_display_style = WorkspaceDisplayStyle {
        colors: style.colors,
    };
    
    let workspace_display = if app.workspace_path.is_empty() {
        container(
            text(if is_compact { "No workspace open" } else { "No workspace open - click Open to select" })
                .size(if is_compact { 12 } else { 13 })
                .style(iced::theme::Text::Color(style.colors.text_muted))
        )
        .padding(if is_compact { [4, 8] } else { [6, 10] })
        .width(if is_compact { Length::FillPortion(2) } else { Length::FillPortion(3) })
        .style(iced::theme::Container::Custom(Box::new(workspace_display_style)))
    } else {
        container(
            text(&app.workspace_path)
                .size(if is_compact { 12 } else { 13 })
                .style(iced::theme::Text::Color(style.colors.text_primary))
        )
        .padding(if is_compact { [4, 8] } else { [6, 10] })
        .width(if is_compact { Length::FillPortion(2) } else { Length::FillPortion(3) })
        .style(iced::theme::Container::Custom(Box::new(workspace_display_style)))
    };
    
    // Responsive buttons
    let open_button = button(
        row![
            Icon::FolderOpen.render(&app.editor_typography, &style, Some(if is_compact { 12 } else { 13 })),
            text(if is_compact { "Open" } else { "Open Workspace" }).size(if is_compact { 12 } else { 13 })
        ]
        .spacing(4)
        .align_items(iced::Alignment::Center)
    )
    .on_press(Message::OpenWorkspace)
    .padding(if is_compact { [4, 8] } else { [6, 12] })
    .style(iced::theme::Button::Secondary);
    
    let save_button = button(
        text(if is_compact { "Save" } else { "Save" }).size(if is_compact { 12 } else { 13 })
    )
    .on_press(Message::SaveFile)
    .padding(if is_compact { [4, 8] } else { [6, 12] })
    .style(iced::theme::Button::Primary);
    
    struct StatusIndicatorStyle {
        colors: SemanticColors,
    }
    
    impl iced::widget::container::StyleSheet for StatusIndicatorStyle {
        type Style = iced::Theme;
        
        fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
            container::Appearance {
                background: Some(self.colors.elevated_panel_background.into()),
                border: iced::Border {
                    color: self.colors.border,
                    width: 1.0,
                    radius: crate::ui::common::RADIUS_SM.into(),
                },
                ..Default::default()
            }
        }
    }
    
    let status_indicator_style = StatusIndicatorStyle {
        colors: style.colors,
    };
    
    // Responsive status indicator using semantic icons
    let status_indicator = if app.is_dirty {
        container(
            row![
                crate::ui::icons::Icon::Warning.render_with_color(
                    &app.editor_typography,
                    style.colors.warning,
                    Some(if is_compact { 9 } else { 10 }),
                ),
                if !is_compact {
                    let txt: Element<_> = text("Unsaved").size(11).style(iced::theme::Text::Color(style.colors.text_secondary)).into();
                    txt
                } else {
                    let space: Element<_> = iced::widget::Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into();
                    space
                },
            ]
            .spacing(if is_compact { 2 } else { 4 })
            .align_items(iced::Alignment::Center)
        )
        .padding(if is_compact { [2, 6] } else { [4, 8] })
        .style(iced::theme::Container::Custom(Box::new(status_indicator_style)))
    } else {
        container(
            row![
                crate::ui::icons::Icon::Success.render_with_color(
                    &app.editor_typography,
                    style.colors.success,
                    Some(if is_compact { 9 } else { 10 }),
                ),
                if !is_compact {
                    let txt: Element<_> = text("Saved").size(11).style(iced::theme::Text::Color(style.colors.text_muted)).into();
                    txt
                } else {
                    let space: Element<_> = iced::widget::Space::new(Length::Fixed(0.0), Length::Fixed(0.0)).into();
                    space
                },
            ]
            .spacing(if is_compact { 2 } else { 4 })
            .align_items(iced::Alignment::Center)
        )
        .padding(if is_compact { [2, 6] } else { [4, 8] })
        .style(iced::theme::Container::Custom(Box::new(status_indicator_style)))
    };
    
    struct TopBarStyle {
        colors: SemanticColors,
    }
    
    impl iced::widget::container::StyleSheet for TopBarStyle {
        type Style = iced::Theme;
        
        fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
            container::Appearance {
                background: Some(self.colors.shell_background.into()),
                border: iced::Border {
                    color: self.colors.border,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        }
    }
    
    let top_bar_style = TopBarStyle {
        colors: style.colors,
    };
    
    container(
        row![
            // Minimal logo/brand
            container(
                row![
                    text("N").size(18).style(iced::theme::Text::Color(style.colors.accent)),
                    text("eote").size(18).style(iced::theme::Text::Color(style.colors.text_primary)),
                ]
                .spacing(0)
            )
            .padding([0, 12]),
            
            iced::widget::horizontal_space(),
            
            // Workspace controls - compact
            container(
                row![
                    workspace_display,
                    open_button,
                    icon_button(
                        Icon::Refresh,
                        &app.editor_typography,
                        &style,
                        Some(Message::RefreshWorkspace),
                        Some(14),
                    )
                    .padding([6, 10])
                    .style(iced::theme::Button::Secondary),
                ]
                .spacing(6)
                .align_items(iced::Alignment::Center)
            )
            .width(Length::FillPortion(2)),
            
            iced::widget::horizontal_space(),
            
            // Status and save - compact
            container(
                row![
                    status_indicator,
                    save_button,
                ]
                .spacing(6)
                .align_items(iced::Alignment::Center)
            ),
        ]
        .align_items(iced::Alignment::Center)
        .padding([0, 8])
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(top_bar_style)))
    .into()
}
