use iced::{Element, Length, widget::{button, column, container, row, scrollable, text}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;
use crate::theme::SemanticColors;

pub fn explorer_panel(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    let header = container(
        row![
            text("EXPLORER")
                .size(10)
                .style(iced::theme::Text::Color(style.colors.text_muted)),
            iced::widget::horizontal_space(),
            button(
                text("⟳").size(12)
            )
            .on_press(Message::RefreshWorkspace)
            .padding([2, 6])
            .style(iced::theme::Button::Secondary)
        ]
        .align_items(iced::Alignment::Center)
    )
    .padding([8, 12])
    .width(Length::Fill);
    
    let content: Element<_> = if app.file_entries.is_empty() {
        container(
            column![
                text("No files in workspace")
                    .style(iced::theme::Text::Color(style.colors.text_muted)),
                button("Open Workspace")
                    .on_press(Message::OpenWorkspace)
                    .padding(8)
                    .style(iced::theme::Button::Secondary)
            ]
            .spacing(12)
            .align_items(iced::Alignment::Center)
        )
        .center_y()
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    } else {
        let entries: Vec<Element<_>> = app.file_entries
            .iter()
            .enumerate()
            .map(|(idx, entry)| {
                let is_selected = app.active_file_path.as_ref() == Some(&entry.path);
                
                let icon = if entry.is_dir { "📁" } else { "📄" };
                let text_color = if is_selected {
                    style.colors.text_on_accent
                } else if entry.is_dir {
                    // Make directories more visible with accent color
                    style.colors.accent
                } else {
                    // Make file text more readable
                    style.colors.text_secondary
                };
                
                let row_content = row![
                    text(icon).size(12),
                    text(&entry.name)
                        .size(12)
                        .style(iced::theme::Text::Color(text_color)),
                ]
                .spacing(6)
                .align_items(iced::Alignment::Center);
                
                let message = if entry.is_dir {
                    Message::ToggleDirectory(entry.path.clone())
                } else {
                    Message::FileSelected(idx)
                };
                
                // Custom button style for better IDE feel
                let button_style = if is_selected {
                    iced::theme::Button::Primary
                } else {
                    // Create a custom style that's more IDE-like
                    let colors = style.colors;
                    struct ExplorerButtonStyle {
                        colors: crate::theme::SemanticColors,
                    }
                    impl iced::widget::button::StyleSheet for ExplorerButtonStyle {
                        type Style = iced::Theme;
                        
                        fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
                            iced::widget::button::Appearance {
                                background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
                                border: iced::Border::default(),
                                text_color: self.colors.text_secondary,
                                ..Default::default()
                            }
                        }
                        
                        fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
                            iced::widget::button::Appearance {
                                background: Some(iced::Background::Color(self.colors.hover_background)),
                                border: iced::Border::default(),
                                text_color: self.colors.text_primary,
                                ..Default::default()
                            }
                        }
                    }
                    iced::theme::Button::Custom(Box::new(ExplorerButtonStyle { colors }))
                };
                
                container(
                    button(row_content)
                        .on_press(message)
                        .padding([4, 8])
                        .width(Length::Fill)
                        .height(Length::Fixed(crate::ui::common::EXPLORER_ROW_HEIGHT))
                        .style(button_style)
                )
                .into()
            })
            .collect();
        
        scrollable(
            column(entries)
                .spacing(0)
                .width(Length::Fill)
        )
        .height(Length::Fill)
        .into()
    };
    
    struct ExplorerPanelContainerStyle {
        colors: SemanticColors,
    }
    
    impl iced::widget::container::StyleSheet for ExplorerPanelContainerStyle {
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
    
    let container_style = ExplorerPanelContainerStyle {
        colors: style.colors,
    };
    
    container(
        column![
            header,
            container(content)
                .height(Length::Fill)
                .width(Length::Fill),
        ]
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(container_style)))
    .into()
}
