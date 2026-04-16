use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text},
    Alignment, Color, Element, Length,
};
use std::ops::Range;

use crate::message::Message;
use crate::theme::ZaroxiTheme;
use crate::ui::style::StyleHelpers;
use crate::settings::editor::EditorTypographySettings;

pub fn editor_panel<'a>(
    active_file_path: Option<&'a String>,
    text_editor: &'a iced::widget::text_editor::Content,
    is_dirty: bool,
    editor_document: Option<&'a editor_core::Document>,
    is_file_too_large_for_editor: bool,
    file_loading_state: &'a crate::state::FileLoadingState,
    editor_typography: &'a EditorTypographySettings,
    theme: ZaroxiTheme,
    line_cache: Option<Vec<Vec<(Range<usize>, Color)>>>,
    tab_manager: &'a crate::state::TabManager,
) -> Element<'a, Message> {
    let style = StyleHelpers::new(theme);
    
    // Build tab bar - always show with minimum height
    let tab_bar: Element<Message> = {
        let mut tab_row: iced::widget::Row<'_, Message, iced::Theme, iced::Renderer> = row![].spacing(0);
        
        // Show placeholder when no tabs
        if tab_manager.tabs.is_empty() {
            let placeholder = container(
                text("No files open")
                    .size(12)
                    .style(iced::theme::Text::Color(style.colors.text_muted))
            )
            .padding([8, 16])
            .center_y()
            .width(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
                container::Appearance {
                    background: Some(style.colors.editor_background.into()),
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(0.0),
                    },
                    ..Default::default()
                }
            })));
            tab_row = tab_row.push(placeholder);
        } else {
            for tab in &tab_manager.tabs {
                let is_active = tab.is_active;
                
                // Tab label with dirty indicator
                let label = if tab.is_dirty {
                    format!("● {}", tab.display_name)
                } else {
                    tab.display_name.clone()
                };
                
                let tab_label: iced::widget::Text<'_, iced::Theme, iced::Renderer> = text(label)
                    .size(12)
                    .style(if is_active {
                        iced::theme::Text::Color(style.colors.text_primary)
                    } else {
                        iced::theme::Text::Color(style.colors.text_muted)
                    });
                
                // Create the tab content
                let mut tab_row_content: iced::widget::Row<'_, Message, iced::Theme, iced::Renderer> = 
                    row![].spacing(6).align_items(Alignment::Center);
                tab_row_content = tab_row_content.push(tab_label);
                
                // Add close button only for active tab
                if is_active {
                    let close_button: iced::widget::Button<'_, Message, iced::Theme, iced::Renderer> = button(
                        text("×")
                            .size(12)
                            .style(iced::theme::Text::Color(style.colors.text_muted))
                    )
                    .on_press(Message::CloseTab(tab.id))
                    .style(iced::theme::Button::Text)
                    .padding([0, 4]);
                    tab_row_content = tab_row_content.push(close_button);
                }
                
                // Wrap in a button for clicking to activate
                let tab_button: iced::widget::Button<'_, Message, iced::Theme, iced::Renderer> = button(tab_row_content)
                    .on_press(Message::ActivateTab(tab.id))
                    .style(iced::theme::Button::Text)
                    .padding(0);
                
                let tab_element: iced::widget::Container<'_, Message, iced::Theme, iced::Renderer> = container(tab_button)
                    .padding([8, 16])
                    .width(Length::Shrink)
                    .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
                        if is_active {
                            container::Appearance {
                                background: Some(style.colors.editor_background.into()),
                                border: iced::Border {
                                    // Use editor_background for the bottom border to make it seamless
                                    // The left, top, and right borders will still be visible against panel_background
                                    color: style.colors.editor_background,
                                    width: 1.0,
                                    radius: iced::border::Radius::from(0.0),
                                },
                                ..Default::default()
                            }
                        } else {
                            container::Appearance {
                                background: Some(style.colors.panel_background.into()),
                                border: iced::Border {
                                    color: Color::TRANSPARENT,
                                    width: 0.0,
                                    radius: iced::border::Radius::from(0.0),
                                },
                                ..Default::default()
                            }
                        }
                    })));
                
                tab_row = tab_row.push(tab_element);
            }
        }
        
        let tab_scroll_style = TabScrollableStyle {
            colors: style.colors,
        };
        
        let scrollable_tabs: iced::widget::Scrollable<'_, Message, iced::Theme, iced::Renderer> = scrollable(
            tab_row
        )
        .direction(iced::widget::scrollable::Direction::Horizontal(
            iced::widget::scrollable::Properties::new()
                .width(5)
                .margin(0)
                .scroller_width(5)
        ))
        .style(iced::theme::Scrollable::Custom(Box::new(tab_scroll_style)));
        
        // Always use editor_background for seamless integration with editor area
        let tab_bg_color = style.colors.editor_background;
        
        let tab_bar_container: iced::widget::Container<'_, Message, iced::Theme, iced::Renderer> = container(scrollable_tabs)
            .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
                container::Appearance {
                    background: Some(tab_bg_color.into()),
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::from(0.0),
                    },
                    ..Default::default()
                }
            })))
            .width(Length::Fill)
            .height(Length::Fixed(36.0)) // Fixed height for tab bar
            .padding(0);
        
        tab_bar_container.into()
    };
    
    // Status header - minimal, only shows status indicators
    let header: Element<Message> = {
        let mut indicators: Vec<Element<Message>> = Vec::new();
        
        // Read-only indicator
        if is_file_too_large_for_editor {
            indicators.push(
                Element::from(text("Read-only")
                    .size(11)
                    .style(iced::theme::Text::Color(style.colors.text_muted)))
            );
        }
        
        // Dirty status (already shown in tab, but keep for consistency)
        if !is_file_too_large_for_editor && is_dirty {
            indicators.push(
                Element::from(text("● Unsaved")
                    .size(11)
                    .style(iced::theme::Text::Color(style.colors.warning)))
            );
        }
        
        if indicators.is_empty() {
            // Show empty header with zero height and no border
            Element::from(container(horizontal_space()).height(Length::Fixed(0.0)))
        } else {
            let header_row: iced::widget::Row<'_, Message, iced::Theme, iced::Renderer> = row![
                horizontal_space(),
                row(indicators)
                    .spacing(8)
                    .align_items(Alignment::Center),
            ]
            .align_items(Alignment::Center);
            
            Element::from(container(header_row).padding([2, 16]))
        }
    };

    // Check loading state
    let editor_content = match file_loading_state {
        crate::state::FileLoadingState::LoadingMetadata { path } => {
            container(
                column![
                    text("Checking file size...")
                        .size(16)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 255))),
                    text(format!("Path: {}", path))
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
                ]
                .align_items(Alignment::Center)
                .spacing(16),
            )
            .center_y()
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
        crate::state::FileLoadingState::LoadingContent { path, size } => {
            let size_mb = size / (1024 * 1024);
            let size_kb = size / 1024;
            let size_str = if size_mb > 0 {
                format!("{} MB", size_mb)
            } else {
                format!("{} KB", size_kb)
            };
            container(
                column![
                    text("Loading file...")
                        .size(16)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 255))),
                    text(format!("{} ({})", path, size_str))
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
                    text("This may take a moment for large files")
                        .size(10)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
                ]
                .align_items(Alignment::Center)
                .spacing(16),
            )
            .center_y()
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
        crate::state::FileLoadingState::LargeFileWarning { path, size } => {
            let size_mb = size / (1024 * 1024);
            container(
                column![
                    text("⚠ Large File Detected")
                        .size(16)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 200, 0))),
                    text(format!("{} ({} MB)", path, size_mb))
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(200, 200, 200))),
                    text("Opening in editable mode...")
                        .size(10)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
                ]
                .align_items(Alignment::Center)
                .spacing(16),
            )
            .center_y()
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
        crate::state::FileLoadingState::VeryLargeFileWarning { path, size } => {
            let size_mb = size / (1024 * 1024);
            container(
                column![
                    text("⚠ Very Large File Detected")
                        .size(16)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(255, 100, 100))),
                    text(format!("{} ({} MB)", path, size_mb))
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(200, 200, 200))),
                    text("Opening in read-only preview mode")
                        .size(10)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
                ]
                .align_items(Alignment::Center)
                .spacing(16),
            )
            .center_y()
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
        crate::state::FileLoadingState::ReadOnlyPreview { path, size } => {
            let size_mb = size / (1024 * 1024);
            container(
                column![
                    text("📖 Read-Only Preview")
                        .size(16)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 200, 255))),
                    text(format!("{} ({} MB total)", path, size_mb))
                        .size(12)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(200, 200, 200))),
                    text("Only first 100KB shown. Editing disabled.")
                        .size(10)
                        .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150))),
                ]
                .align_items(Alignment::Center)
                .spacing(16),
            )
            .center_y()
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
        crate::state::FileLoadingState::Idle => {
            // Original logic for when not loading
            if active_file_path.is_some() {
                if is_file_too_large_for_editor {
                    // Show read-only text view for very large files
                    let content = if let Some(document) = editor_document {
                        if document.is_very_large() {
                            // For very large files, show only first 100KB
                            // Use safe slicing with bounds checking
                            let len = document.len_chars();
                            let end = 100_000.min(len);
                            document.slice(0, end).unwrap_or_else(|_| String::new())
                        } else {
                            // For large but not very large files, limit to 500KB
                            let text = document.text();
                            if text.len() > 500_000 {
                                text[..500_000].to_string()
                            } else {
                                text
                            }
                        }
                    } else {
                        String::new()
                    };
                    let warning = if let Some(document) = editor_document {
                        if document.is_very_large() {
                            format!("\n\n--- File truncated ({} MB total, showing first 100KB) ---", 
                                   document.len_chars() / 1_000_000)
                        } else if document.is_large() {
                            format!("\n\n--- File truncated ({} MB total, showing first 500KB) ---",
                                   document.len_chars() / 1_000_000)
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };
                    // Use a scrollable with explicit height
                    scrollable(
                        container(
                            text(content + &warning)
                                .font(iced::Font::MONOSPACE)
                                .size(14)
                        )
                        .padding(16)
                        .width(Length::Fill)
                    )
                    .height(Length::Fill)
                    .into()
                } else {
                    // Get the editor background color from the theme
                    let style = StyleHelpers::new(theme);
                    // Pass the line cache to the editor
                    super::super::editor::editor(text_editor, editor_typography, style.colors.editor_background, style.colors.text_primary, line_cache)
                }
            } else {
                container(
                    column![
                        text("Zaroxi Studio")
                            .size(24)
                            .style(iced::theme::Text::Color(style.colors.accent)),
                        text("AI‑first Code Editor")
                            .size(12)
                            .style(iced::theme::Text::Color(style.colors.text_muted)),
                        container(
                            column![
                                button("Open a file from the explorer")
                                    .style(iced::theme::Button::Secondary)
                                    .padding([6, 12]),
                                button("Ask AI about the workspace")
                                    .style(iced::theme::Button::Secondary)
                                    .padding([6, 12]),
                            ]
                            .spacing(8)
                        )
                        .padding(12),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(12),
                )
                .center_y()
                .center_x()
                .width(Length::Fill)
                .height(Length::Fill)
                .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
                    container::Appearance {
                        background: Some(style.colors.editor_background.into()),
                        border: iced::Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: iced::border::Radius::from(0.0),
                        },
                        ..Default::default()
                    }
                })))
                .into()
            }
        }
    };

    let column: iced::widget::Column<'_, Message, iced::Theme, iced::Renderer> = column![
        tab_bar,
        header,
        editor_content,
    ]
    .height(Length::Fill)
    .padding(0)
    .spacing(0);
    
    container(column)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
            container::Appearance {
                background: Some(style.colors.editor_background.into()),
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::from(0.0),
                },
                ..Default::default()
            }
        })))
        .into()
}
