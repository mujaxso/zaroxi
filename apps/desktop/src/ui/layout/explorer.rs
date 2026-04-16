use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text},
    Alignment, Element, Length,
};

use crate::message::Message;
use crate::state::{Activity, App};
use crate::ui::icons::Icon;
use crate::ui::style::StyleHelpers;

// Professional Explorer panel with proper tree rendering
pub fn left_panel_with_expanded<'a>(
    _file_entries: &'a [core_types::workspace::DirectoryEntry],
    active_activity: Activity,
    _expanded_directories: &'a std::collections::HashSet<std::path::PathBuf>,
    _workspace_path: &'a str,
) -> Element<'a, Message> {
    match active_activity {
        Activity::Primary(crate::state::PrimarySidebarView::Explorer) => 
            placeholder_panel("Explorer (use explorer_panel with App)"),
        Activity::Primary(crate::state::PrimarySidebarView::Search) => 
            search_panel(),
        Activity::Primary(crate::state::PrimarySidebarView::SourceControl) => 
            terminal_panel(),
        Activity::Primary(crate::state::PrimarySidebarView::Settings) => 
            settings_panel(),
        _ => placeholder_panel(&format!("{} panel", format!("{:?}", active_activity))),
    }
}

pub fn explorer_panel_with_expanded<'a>(
    _file_entries: &'a [core_types::workspace::DirectoryEntry],
    _expanded_directories: &'a std::collections::HashSet<std::path::PathBuf>,
    _workspace_path: &'a str,
) -> Element<'a, Message> {
    placeholder_panel("Explorer (use explorer_panel with App)")
}


// New implementation that properly renders the tree structure
pub fn explorer_panel_professional(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.current_theme);
    
    // Check if we have a workspace
    let content: Element<'_, Message> = if app.workspace_path.is_empty() {
        // No workspace open state
        container(
            column![
                text("No workspace open")
                    .size(13)
                    .style(iced::theme::Text::Color(style.colors.text_muted)),
                button(
                    container(
                        row![
                            Icon::Folder.render(&app.editor_typography, &style, Some(14)),
                            text("Open Workspace")
                                .size(13)
                                .style(iced::theme::Text::Color(style.colors.text_secondary)),
                        ]
                        .spacing(8)
                        .align_items(Alignment::Center)
                    )
                    .padding([8, 12])
                )
                .on_press(Message::OpenWorkspace)
                .style(iced::theme::Button::Secondary),
            ]
            .spacing(16)
            .align_items(Alignment::Center)
        )
        .center_y()
        .center_x()
        .height(Length::Fill)
        .into()
    } else if app.explorer_state.file_tree.is_empty() {
        // Workspace open but no files
        container(
            column![
                text("No files found")
                    .size(13)
                    .style(iced::theme::Text::Color(style.colors.text_muted)),
                button(
                    container(
                        row![
                            Icon::Refresh.render(&app.editor_typography, &style, Some(14)),
                            text("Refresh")
                                .size(13)
                                .style(iced::theme::Text::Color(style.colors.text_secondary)),
                        ]
                        .spacing(8)
                        .align_items(Alignment::Center)
                    )
                    .padding([8, 12])
                )
                .on_press(Message::RefreshWorkspace)
                .style(iced::theme::Button::Secondary),
            ]
            .spacing(16)
            .align_items(Alignment::Center)
        )
        .center_y()
        .center_x()
        .height(Length::Fill)
        .into()
    } else {
        // Render the file tree properly
        let tree_content = render_explorer_tree(
            &app.explorer_state.file_tree,
            &app.explorer_state,
            &app.editor_typography,
            &style,
            0,
        );
        
        scrollable(
            column(tree_content)
                .spacing(0)
                .padding([4, 0])
        )
        .height(Length::Fill)
        .into()
    };

    // Professional header with clean styling
    let header_colors = style.colors;
    let header = container(
        row![
            text("EXPLORER")
                .size(11)
                .font(iced::Font::with_name("JetBrains Mono"))
                .style(iced::theme::Text::Color(header_colors.text_muted)),
            horizontal_space(),
            button(
                Icon::Refresh.render(&app.editor_typography, &style, Some(14))
            )
            .on_press(Message::RefreshWorkspace)
            .padding(4)
            .style(iced::theme::Button::Text),
        ]
        .align_items(Alignment::Center)
    )
    .padding([12, 16])
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
        container::Appearance {
            background: Some(header_colors.panel_background.into()),
            border: iced::Border {
                color: header_colors.border,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    })));

    // For the divider, also need to capture colors
    let divider_colors = style.colors;
    let divider = container(iced::widget::Space::with_height(1.0))
        .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Color::from_rgba8(
                    divider_colors.border.r as u8,
                    divider_colors.border.g as u8,
                    divider_colors.border.b as u8,
                    0.3,
                ).into()),
                ..Default::default()
            }
        })))
        .width(Length::Fill)
        .height(Length::Fixed(1.0));

    column![
        header,
        // Clean, subtle divider
        divider,
        content,
    ]
    .height(Length::Fill)
    .into()
}

// Custom button style for explorer rows
struct ExplorerRowStyle {
    bg_color: iced::Color,
    is_selected: bool,
    accent_color: iced::Color,
}

impl iced::widget::button::StyleSheet for ExplorerRowStyle {
    type Style = iced::Theme;
    
    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        let mut appearance = iced::widget::button::Appearance::default();
        appearance.background = Some(self.bg_color.into());
        if self.is_selected {
            appearance.border = iced::Border {
                color: self.accent_color,
                width: 1.0,
                radius: 0.0.into(),
            };
        } else {
            appearance.border = iced::Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            };
        }
        appearance
    }
    
    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let mut appearance = self.active(style);
        if !self.is_selected {
            // Add a subtle hover effect
            appearance.background = Some(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.05).into());
        }
        appearance
    }
    
    fn disabled(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        self.active(style)
    }
    
    fn pressed(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let mut appearance = self.active(style);
        // Make it slightly darker when pressed
        if let Some(iced::Background::Color(color)) = appearance.background {
            appearance.background = Some(iced::Color {
                a: color.a * 0.8,
                ..color
            }.into());
        }
        appearance
    }
}

// Recursive function to render the explorer tree with proper hierarchy
fn render_explorer_tree<'a>(
    nodes: &'a [crate::explorer::model::ExplorerNode],
    explorer_state: &'a crate::explorer::state::ExplorerState,
    typography: &'a crate::settings::editor::EditorTypographySettings,
    style: &'a StyleHelpers,
    depth: usize,
) -> Vec<Element<'a, Message>> {
    let mut elements = Vec::new();
    
    for node in nodes {
        let is_expanded = explorer_state.is_expanded(&node.path);
        let is_selected = explorer_state.is_selected(&node.path);
        
        // Determine colors based on state
        let text_color = if is_selected {
            style.colors.text_primary
        } else if node.is_dir {
            style.colors.text_secondary
        } else {
            style.colors.text_primary
        };
        
        let bg_color = if is_selected {
            style.colors.accent_soft_background
        } else {
            iced::Color::TRANSPARENT
        };
        
        // Create the row content
        let row_content = row![
            // Indentation
            container(iced::widget::Space::with_width(Length::Fixed((depth * 16) as f32))),
            // Chevron for directories
            if node.is_dir {
                let chevron_icon = if is_expanded {
                    Icon::ChevronDown
                } else {
                    Icon::ChevronRight
                };
                Element::from(
                    button(
                        chevron_icon.render_with_color(typography, style.colors.text_muted, Some(12))
                    )
                    .on_press(Message::ToggleDirectory(node.path.to_string_lossy().to_string()))
                    .padding(2)
                    .style(iced::theme::Button::Text)
                )
            } else {
                // Spacer for files to align with directories
                Element::from(container(iced::widget::Space::with_width(Length::Fixed(20.0))))
            },
            // Icon
            if node.is_dir {
                if is_expanded {
                    Icon::FolderOpen.render_with_color(typography, style.colors.accent, Some(14))
                } else {
                    Icon::Folder.render_with_color(typography, style.colors.text_secondary, Some(14))
                }
            } else {
                Icon::File.render_with_color(typography, text_color, Some(14))
            },
            // File/folder name
            container(
                text(&node.name)
                    .size(13)
                    .font(iced::Font::with_name("JetBrains Mono"))
                    .style(iced::theme::Text::Color(text_color))
            )
            .padding([0, 8])
            .width(Length::Fill),
        ]
        .align_items(Alignment::Center)
        .height(Length::Fixed(28.0));
        
        // Create a custom button style using a proper struct
        let explorer_row_style = ExplorerRowStyle {
            bg_color,
            is_selected,
            accent_color: style.colors.accent,
        };
        
        // Wrap in a button for the entire row
        let row_button = button(
            container(row_content)
                .width(Length::Fill)
                .height(Length::Fixed(28.0))
                .padding([0, 8])
        )
        .on_press(if node.is_dir {
            Message::ToggleDirectory(node.path.to_string_lossy().to_string())
        } else {
            Message::FileSelectedByPath(node.path.to_string_lossy().to_string())
        })
        .width(Length::Fill)
        .style(iced::theme::Button::Custom(Box::new(explorer_row_style)));
        
        elements.push(row_button.into());
        
        // Render children if expanded
        if node.is_dir && is_expanded && !node.children.is_empty() {
            let child_elements = render_explorer_tree(
                &node.children,
                explorer_state,
                typography,
                style,
                depth + 1,
            );
            elements.extend(child_elements);
        }
    }
    
    elements
}

fn search_panel<'a>() -> Element<'a, Message> {
    super::search::search_panel()
}

fn terminal_panel<'a>() -> Element<'a, Message> {
    super::terminal::terminal_panel()
}

fn settings_panel<'a>() -> Element<'a, Message> {
    super::settings::settings_panel()
}

fn placeholder_panel<'a>(label: &str) -> Element<'a, Message> {
    container(
        text(label)
            .style(iced::theme::Text::Color(iced::Color::from_rgb8(150, 150, 150)))
    )
    .center_y()
    .center_x()
    .width(Length::Fixed(250.0))
    .height(Length::Fill)
    .into()
}

// Provide explorer_panel for compatibility - now uses the professional version
pub fn explorer_panel(app: &crate::state::App) -> Element<'_, Message> {
    explorer_panel_professional(app)
}
