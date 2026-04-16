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
) -> Element<'a, Message> {
    let style = StyleHelpers::new(theme);
    let header: Element<_> = if let Some(path) = active_file_path {
        Element::from(container(
            row![
                // File path
                text(path)
                    .size(13)
                    .style(iced::theme::Text::Color(style.colors.text_primary)),
                horizontal_space(),
                // Status indicators
                {
                    let mut indicators = Vec::new();
                    
                    // Large file warning
                    if let Some(document) = editor_document {
                        if document.is_very_large() {
                            indicators.push(
                                Element::from(text("⚠ Large file")
                                    .size(11)
                                    .style(iced::theme::Text::Color(style.colors.error)))
                            );
                        } else if document.is_large() {
                            indicators.push(
                                Element::from(text("⚠ Large")
                                    .size(11)
                                    .style(iced::theme::Text::Color(style.colors.warning)))
                            );
                        }
                    }
                    
                    // Read-only indicator
                    if is_file_too_large_for_editor {
                        indicators.push(
                            Element::from(text("Read-only")
                                .size(11)
                                .style(iced::theme::Text::Color(iced::Color::from_rgb8(200, 150, 50))))
                        );
                    }
                    
                    // Dirty status
                    if !is_file_too_large_for_editor {
                        let status_text: Element<_> = if is_dirty {
                            Element::from(text("● Unsaved")
                                .size(11)
                                .style(iced::theme::Text::Color(style.colors.warning)))
                        } else {
                            Element::from(text("✓ Saved")
                                .size(11)
                                .style(iced::theme::Text::Color(style.colors.success)))
                        };
                        indicators.push(status_text);
                    }
                    
                    row(indicators)
                        .spacing(8)
                        .align_items(Alignment::Center)
                },
            ]
            .align_items(Alignment::Center)
        )
        .padding([12, 16]))
    } else {
        Element::from(container(
            row![
                text("No file selected")
                    .size(13)
                    .style(iced::theme::Text::Color(style.colors.text_muted)),
                horizontal_space(),
            ]
            .align_items(Alignment::Center)
        )
        .padding([12, 16]))
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
                        text("Zaroxi Studio").size(32).style(iced::theme::Text::Color(style.colors.accent)),
                        text("AI‑first Code Editor").size(16).style(iced::theme::Text::Color(style.colors.text_secondary)),
                        container(iced::widget::horizontal_rule(1)).width(150),
                        column![
                            button("Open a file from the explorer").style(iced::theme::Button::Secondary),
                            button("Ask AI about the workspace").style(iced::theme::Button::Secondary),
                        ]
                        .spacing(8)
                        .padding(16),
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
        }
    };

    column![
        tab_bar,
        iced::widget::horizontal_rule(1),
        header,
        iced::widget::horizontal_rule(1),
        editor_content,
    ]
    .height(Length::Fill)
    .into()
}
