use iced::{Element, Length, widget::{column, container, mouse_area}, Color};
use crate::message::Message;
use crate::state::{App, Activity};
use super::style::StyleHelpers;
use crate::theme::SemanticColors;
use crate::ui::icons::Icon;

/// Represents a single activity item in the activity bar
#[derive(Debug, Clone)]
struct ActivityItem {
    id: Activity,
    icon: Icon,
    label: &'static str,
    tooltip: &'static str,
    group: ActivityGroup,
}

/// Group for organizing activity items (top = primary, bottom = utility)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActivityGroup {
    Primary,
    Utility,
}

impl ActivityItem {
    /// Get all available activity items for the activity bar
    fn all() -> Vec<Self> {
        vec![
            ActivityItem {
                id: Activity::explorer(),
                icon: Icon::Folder,
                label: "Explorer",
                tooltip: "File Explorer (Ctrl+Shift+E)",
                group: ActivityGroup::Primary,
            },
            ActivityItem {
                id: Activity::search(),
                icon: Icon::Search,
                label: "Search",
                tooltip: "Search across files (Ctrl+Shift+F)",
                group: ActivityGroup::Primary,
            },
            ActivityItem {
                id: Activity::ai_assistant(),
                icon: Icon::Robot,
                label: "AI Assistant",
                tooltip: "AI Assistant (Ctrl+Shift+A)",
                group: ActivityGroup::Primary,
            },
            ActivityItem {
                id: Activity::source_control(),
                icon: Icon::Git,
                label: "Source Control",
                tooltip: "Git source control (Ctrl+Shift+G)",
                group: ActivityGroup::Primary,
            },
            ActivityItem {
                id: Activity::settings(),
                icon: Icon::Settings,
                label: "Settings",
                tooltip: "Settings (Ctrl+,)",
                group: ActivityGroup::Utility,
            },
        ]
    }
    
    /// Get the message to send when this item is activated
    fn activation_message(&self) -> Message {
        match self.id {
            Activity::Auxiliary(crate::state::AuxiliaryView::AiAssistant) => Message::ToggleAiPanel,
            _ => Message::ActivitySelected(self.id),
        }
    }
}

/// Main activity bar component - modern, compact, VS Code-inspired
pub fn activity_bar(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    
    // Get all activity items
    let items = ActivityItem::all();
    
    // Split items by group
    let primary_items: Vec<_> = items.iter()
        .filter(|item| item.group == ActivityGroup::Primary)
        .collect();
    
    let utility_items: Vec<_> = items.iter()
        .filter(|item| item.group == ActivityGroup::Utility)
        .collect();
    
    // Create activity buttons
    let primary_buttons: Vec<Element<_>> = primary_items
        .iter()
        .map(|item| activity_button(item, app, &style))
        .collect();
    
    let utility_buttons: Vec<Element<_>> = utility_items
        .iter()
        .map(|item| activity_button(item, app, &style))
        .collect();
    
    // Build the activity bar layout with proper spacing
    let content = column![
        // Top spacer
        container(iced::widget::Space::with_height(Length::Fixed(12.0))),
        // Primary activities
        column(primary_buttons)
            .spacing(0)
            .width(Length::Fill),
        // Flexible spacer to push utility items to bottom
        container(iced::widget::Space::with_height(Length::Fill)),
        // Utility activities
        column(utility_buttons)
            .spacing(0)
            .width(Length::Fill),
        // Bottom spacer
        container(iced::widget::Space::with_height(Length::Fixed(12.0))),
    ]
    .width(Length::Fill)
    .height(Length::Fill);
    
    // Apply container styling
    container(content)
        .width(Length::Fixed(48.0)) // Fixed compact width like VS Code
        .height(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(ActivityBarContainerStyle {
            colors: style.colors,
        })))
        .into()
}

/// Render a single activity button with modern styling
fn activity_button<'a>(item: &ActivityItem, app: &App, style: &StyleHelpers) -> Element<'a, Message> {
    let is_active = match item.id {
        Activity::Primary(view) => app.workbench_layout.is_primary_view_active(view),
        Activity::Auxiliary(view) => app.workbench_layout.is_auxiliary_view_active(view),
    };
    let is_hovered = app.workbench_layout.hovered_activity == Some(item.id);
    
    // Determine icon color based on state
    let icon_color = if is_active {
        style.colors.accent
    } else if is_hovered {
        style.colors.text_primary
    } else {
        style.colors.text_muted
    };
    
    // Special handling for AI icon to ensure it's visible
    let icon = if matches!(item.icon, Icon::Robot) {
        // Always use a reliable Unicode character for AI icon
        // Use a different approach to ensure visibility
        iced::widget::text("🤖")
            .size(20)
            .style(iced::theme::Text::Color(icon_color))
            .into()
    } else {
        item.icon.render_with_color(
            &app.editor_typography,
            icon_color,
            Some(20), // Icon size
        )
    };
    
    let button_content = container(icon)
        .width(Length::Fill)
        .height(Length::Fixed(48.0))
        .center_x()
        .center_y();
    
    // Wrap in mouse area for hover and click
    let mouse_area = mouse_area(button_content)
        .on_press(item.activation_message())
        .on_enter(Message::ActivityHovered(Some(item.id)));
    
    // Apply button styling
    container(mouse_area)
        .width(Length::Fill)
        .height(Length::Fixed(48.0))
        .style(iced::theme::Container::Custom(Box::new(ActivityButtonStyle {
            is_active,
            is_hovered,
            colors: style.colors,
        })))
        .into()
}

/// Container style for the activity bar
struct ActivityBarContainerStyle {
    colors: SemanticColors,
}

impl iced::widget::container::StyleSheet for ActivityBarContainerStyle {
    type Style = iced::Theme;
    
    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(self.colors.panel_background.into()),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
}

/// Button style for activity items
struct ActivityButtonStyle {
    is_active: bool,
    is_hovered: bool,
    colors: SemanticColors,
}

impl iced::widget::container::StyleSheet for ActivityButtonStyle {
    type Style = iced::Theme;
    
    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        let mut appearance = iced::widget::container::Appearance {
            background: Some(Color::TRANSPARENT.into()),
            border: iced::Border::default(),
            ..Default::default()
        };
        
        // Active state - left accent indicator (VS Code style)
        if self.is_active {
            // Create a left border only
            appearance.border = iced::Border {
                color: self.colors.accent,
                width: 2.0,
                radius: 0.0.into(),
            };
            // Set background for active state
            appearance.background = Some(self.colors.accent_soft_background.into());
        }
        // Hover state - subtle background
        else if self.is_hovered {
            appearance.background = Some(self.colors.hover_background.into());
        }
        
        appearance
    }
}
