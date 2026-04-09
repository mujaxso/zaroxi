use iced::{Element, Length, widget::{button, column, container, row, scrollable, text, text_input, mouse_area}};
use crate::message::Message;
use crate::state::App;
use super::style::StyleHelpers;
use crate::theme::{SemanticColors, NeoteTheme};
use crate::explorer::actions::ExplorerMessage;
use crate::explorer::state::InlineEditMode;

pub fn explorer_panel<'a>(app: &'a App) -> Element<'a, Message> {
    let style = StyleHelpers::new(app.theme);
    
    let is_compact = matches!(app.layout_mode, crate::state::LayoutMode::Medium | crate::state::LayoutMode::Narrow);
    
    // Get visible rows from explorer state
    let visible_rows = app.explorer_state.visible_rows();
    
    // Header with action buttons
    let header = container(
        row![
            text("EXPLORER")
                .size(if is_compact { 10 } else { 11 })
                .style(iced::theme::Text::Color(style.colors.text_muted)),
            iced::widget::horizontal_space(),
            // Action buttons
            row![
                // New file button
                button(
                    text("📄").size(if is_compact { 12 } else { 13 })
                        .style(iced::theme::Text::Color(style.colors.text_secondary))
                        .font(iced::font::Font::with_name("Noto Color Emoji"))
                )
                .on_press(Message::Explorer(ExplorerMessage::CreateFileRequested))
                .padding(if is_compact { [2, 4] } else { [3, 6] })
                .style(iced::theme::Button::Secondary),
                // New folder button
                button(
                    text("📁").size(if is_compact { 12 } else { 13 })
                        .style(iced::theme::Text::Color(style.colors.text_secondary))
                        .font(iced::font::Font::with_name("Noto Color Emoji"))
                )
                .on_press(Message::Explorer(ExplorerMessage::CreateFolderRequested))
                .padding(if is_compact { [2, 4] } else { [3, 6] })
                .style(iced::theme::Button::Secondary),
                // Refresh button - use emoji refresh symbol
                button(
                    text("🔄").size(if is_compact { 12 } else { 13 })
                        .style(iced::theme::Text::Color(style.colors.text_secondary))
                        .font(iced::font::Font::with_name("Noto Color Emoji"))
                )
                .on_press(Message::Explorer(ExplorerMessage::Refresh))
                .padding(if is_compact { [2, 4] } else { [3, 6] })
                .style(iced::theme::Button::Secondary),
            ]
            .spacing(if is_compact { 2 } else { 4 })
            .align_items(iced::Alignment::Center)
        ]
        .align_items(iced::Alignment::Center)
    )
    .padding(if is_compact { [8, 12] } else { [10, 16] })
    .width(Length::Fill);
    
    let content: Element<_> = if visible_rows.is_empty() {
        container(
            column![
                text("No files in workspace")
                    .size(12)
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
        let mut rows: Vec<Element<_>> = Vec::new();
        
        // Check if we need to insert an inline edit row
        let mut has_inserted_inline = false;
        
        for row in &visible_rows {
            // If this is the target for inline edit, insert the edit row before it
            if let InlineEditMode::Rename { ref target } = app.explorer_state.inline_edit {
                if &row.path == target && !has_inserted_inline {
                    rows.push(inline_edit_row(app, row.depth, row.is_dir));
                    has_inserted_inline = true;
                    continue;
                }
            }
            
            rows.push(explorer_row(row.clone(), app.theme, is_compact));
            
            // If creating inside this directory, insert the create row after it
            if let InlineEditMode::CreateFile { ref parent } | InlineEditMode::CreateFolder { ref parent } = app.explorer_state.inline_edit {
                if &row.path == parent && row.is_dir && row.is_expanded && !has_inserted_inline {
                    rows.push(inline_edit_row(app, row.depth + 1, matches!(app.explorer_state.inline_edit, InlineEditMode::CreateFolder { .. })));
                    has_inserted_inline = true;
                }
            }
        }
        
        // If creating at root and not inserted yet
        if let InlineEditMode::CreateFile { ref parent } | InlineEditMode::CreateFolder { ref parent } = app.explorer_state.inline_edit {
            if parent == &app.explorer_state.workspace_root && !has_inserted_inline {
                rows.push(inline_edit_row(app, 0, matches!(app.explorer_state.inline_edit, InlineEditMode::CreateFolder { .. })));
            }
        }
        
        scrollable(
            column(rows)
                .spacing(0)
                .width(Length::Fill)
        )
        .height(Length::Fill)
        .into()
    };
    
    // Panel container style
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

fn explorer_row(row: crate::explorer::state::VisibleRow, theme: NeoteTheme, is_compact: bool) -> Element<'static, Message> {
    let style = StyleHelpers::new(theme);
    let indent = row.depth * 12;
    
    // Choose icon based on type and state
    let icon = if row.is_dir {
        if row.is_expanded { "📂" } else { "📁" }
    } else {
        "📄"
    };
    
    // Text color
    let text_color = if row.is_selected {
        style.colors.text_on_accent
    } else if row.is_dir {
        style.colors.accent
    } else {
        style.colors.text_secondary
    };
    
    // Icon color - always use a visible color
    let icon_color = if row.is_selected {
        style.colors.text_on_accent
    } else if row.is_dir {
        style.colors.accent
    } else {
        style.colors.text_secondary
    };
    
    // Background color
    let background = if row.is_selected {
        style.colors.accent
    } else {
        iced::Color::TRANSPARENT
    };
    
    // Build row content
    let row_content = if row.is_dir {
        let chevron_icon = if row.is_expanded { "▼" } else { "▶" };
        row![
            // Indentation
            iced::widget::Space::with_width(Length::Fixed(indent as f32)),
            // Chevron for folders
            text(chevron_icon)
                .size(9)
                .style(iced::theme::Text::Color(style.colors.text_muted)),
            iced::widget::Space::with_width(Length::Fixed(4.0)),
            // Icon - with explicit color and emoji font
            // We need to use the emoji font for emoji characters
            text(icon)
                .size(if is_compact { 12 } else { 13 })
                .style(iced::theme::Text::Color(icon_color))
                .font(iced::font::Font::with_name("Noto Color Emoji")),
            // Spacing between icon and name
            iced::widget::Space::with_width(Length::Fixed(6.0)),
            // File/folder name
            text(&row.name)
                .size(if is_compact { 12 } else { 13 })
                .style(iced::theme::Text::Color(text_color)),
            // Action buttons (shown on hover/selection)
            iced::widget::horizontal_space(),
            if row.is_hovered || row.is_selected {
                row![
                    // Rename button - with visible icon
                    button(
                        text("✏").size(10)
                            .style(iced::theme::Text::Color(style.colors.text_secondary))
                            .font(iced::font::Font::with_name("Noto Color Emoji"))
                    )
                    .on_press(Message::Explorer(ExplorerMessage::RenameRequested(row.path.clone())))
                    .padding([2, 4])
                    .style(iced::theme::Button::Secondary),
                    // Delete button - with visible icon
                    button(
                        text("🗑").size(10)
                            .style(iced::theme::Text::Color(style.colors.text_secondary))
                            .font(iced::font::Font::with_name("Noto Color Emoji"))
                    )
                    .on_press(Message::Explorer(ExplorerMessage::DeleteRequested(row.path.clone())))
                    .padding([2, 4])
                    .style(iced::theme::Button::Secondary),
                ]
                .spacing(2)
                .align_items(iced::Alignment::Center)
            } else {
                row![].into()
            }
        ]
        .spacing(0)
        .align_items(iced::Alignment::Center)
    } else {
        row![
            // Indentation
            iced::widget::Space::with_width(Length::Fixed(indent as f32)),
            // Space for missing chevron (files don't have chevrons)
            iced::widget::Space::with_width(Length::Fixed(16.0)),
            // Icon - with explicit color and emoji font
            text(icon)
                .size(if is_compact { 12 } else { 13 })
                .style(iced::theme::Text::Color(icon_color))
                .font(iced::font::Font::with_name("Noto Color Emoji")),
            // Spacing between icon and name
            iced::widget::Space::with_width(Length::Fixed(6.0)),
            // File/folder name
            text(&row.name)
                .size(if is_compact { 12 } else { 13 })
                .style(iced::theme::Text::Color(text_color)),
            // Action buttons (shown on hover/selection)
            iced::widget::horizontal_space(),
            if row.is_hovered || row.is_selected {
                row![
                    // Rename button - with visible icon
                    button(
                        text("✏").size(10)
                            .style(iced::theme::Text::Color(style.colors.text_secondary))
                            .font(iced::font::Font::with_name("Noto Color Emoji"))
                    )
                    .on_press(Message::Explorer(ExplorerMessage::RenameRequested(row.path.clone())))
                    .padding([2, 4])
                    .style(iced::theme::Button::Secondary),
                    // Delete button - with visible icon
                    button(
                        text("🗑").size(10)
                            .style(iced::theme::Text::Color(style.colors.text_secondary))
                            .font(iced::font::Font::with_name("Noto Color Emoji"))
                    )
                    .on_press(Message::Explorer(ExplorerMessage::DeleteRequested(row.path.clone())))
                    .padding([2, 4])
                    .style(iced::theme::Button::Secondary),
                ]
                .spacing(2)
                .align_items(iced::Alignment::Center)
            } else {
                row![].into()
            }
        ]
        .spacing(0)
        .align_items(iced::Alignment::Center)
    };
    
    // Determine message for clicking the row
    let message = if row.is_dir {
        Message::Explorer(ExplorerMessage::ToggleDirectory(row.path.clone()))
    } else {
        Message::Explorer(ExplorerMessage::SelectFile(row.path.clone()))
    };
    
    // Create a custom button style
    struct ExplorerRowStyle {
        background: iced::Color,
        hover_background: iced::Color,
    }
    
    impl iced::widget::button::StyleSheet for ExplorerRowStyle {
        type Style = iced::Theme;
        
        fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
            iced::widget::button::Appearance {
                background: Some(self.background.into()),
                border: iced::Border::default(),
                text_color: iced::Color::WHITE, // Will be overridden by text style
                ..Default::default()
            }
        }
        
        fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
            iced::widget::button::Appearance {
                background: Some(self.hover_background.into()),
                border: iced::Border::default(),
                text_color: iced::Color::WHITE,
                ..Default::default()
            }
        }
    }
    
    let button_style = ExplorerRowStyle {
        background,
        hover_background: if row.is_selected {
            style.colors.accent
        } else {
            // Use a subtle hover color
            style.colors.elevated_panel_background
        },
    };
    
    container(
        mouse_area(
            button(row_content)
                .on_press(message)
                .padding(if is_compact { [4, 8] } else { [6, 12] })
                .width(Length::Fill)
                .height(Length::Fixed(if is_compact { 28.0 } else { 32.0 }))
                .style(iced::theme::Button::Custom(Box::new(button_style)))
        )
        .on_enter(Message::ExplorerHoverChanged(Some(row.path.clone())))
        .on_exit(Message::ExplorerHoverChanged(None))
    )
    .into()
}

fn inline_edit_row(app: &App, depth: usize, is_dir: bool) -> Element<'static, Message> {
    let indent = depth * 12;
    let icon = if is_dir { "📁" } else { "📄" };
    
    // Create a simple style for the icon
    // We'll use a default text color
    let icon_color = iced::Color::from_rgb8(150, 150, 150);
    
    let input = text_input("Name", &app.explorer_state.inline_edit_name)
        .on_input(|name| Message::Explorer(ExplorerMessage::InlineEditNameChanged(name)))
        .on_submit(Message::Explorer(ExplorerMessage::InlineEditConfirmed))
        .padding(if depth == 0 { [6, 8] } else { [4, 6] })
        .size(12);
    
    container(
        row![
            iced::widget::Space::with_width(Length::Fixed(indent as f32)),
            // Space for chevron
            iced::widget::Space::with_width(Length::Fixed(16.0)),
            // Icon with emoji font
            text(icon).size(12)
                .style(iced::theme::Text::Color(icon_color))
                .font(iced::font::Font::with_name("Noto Color Emoji")),
            iced::widget::Space::with_width(Length::Fixed(6.0)),
            input,
        ]
        .align_items(iced::Alignment::Center)
    )
    .padding([4, 8])
    .into()
}
