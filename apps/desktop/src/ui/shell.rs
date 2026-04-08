use iced::{Element, Length, widget::container};
use crate::message::Message;
use crate::state::{App, LayoutMode};
use super::{
    activity_bar::activity_bar,
    assistant_panel::assistant_panel,
    editor_panel::editor_panel,
    status_bar::status_bar,
    top_bar::top_bar,
    layout::explorer_panel_with_expanded,
};

/// Main shell that composes all UI components - Premium compact layout
pub fn shell(app: &App) -> Element<'_, Message> {
    // Determine if AI panel should be visible
    let ai_panel_visible = matches!(app.active_activity, crate::state::Activity::Ai) || app.ai_panel_visible;
    
    // Get panel widths based on layout mode
    let (explorer_width, assistant_width) = match app.layout_mode {
        LayoutMode::Wide => (260.0, 300.0),
        LayoutMode::Medium => (220.0, 240.0),
        LayoutMode::Narrow => (180.0, 200.0),
    };
    
    // In narrow mode, we might want to hide the AI panel if it's not the active activity
    let show_ai_panel = ai_panel_visible && 
        (app.layout_mode != LayoutMode::Narrow || app.active_activity == crate::state::Activity::Ai);
    
    // Build panels with responsive sizing
    let top_bar = container(top_bar(app))
        .width(Length::Fill)
        .height(Length::Fixed(crate::ui::common::TOP_BAR_HEIGHT));
    
    let activity_bar = container(activity_bar(app))
        .width(Length::Fixed(crate::ui::common::ACTIVITY_BAR_WIDTH))
        .height(Length::Fill);
    
    // Use the explorer panel with expanded directories support
    let explorer_panel = container(
        explorer_panel_with_expanded(
            &app.file_entries,
            &app.expanded_directories,
            &app.workspace_path,
        )
    )
    .width(Length::Fixed(explorer_width))
    .height(Length::Fill);
    
    let editor_panel = container(editor_panel(app))
        .width(Length::Fill)
        .height(Length::Fill);
    
    // Conditionally include AI panel
    let main_content = if show_ai_panel {
        let assistant_panel = container(assistant_panel(app))
            .width(Length::Fixed(assistant_width))
            .height(Length::Fill);
        
        iced::widget::row![
            activity_bar,
            explorer_panel,
            editor_panel,
            assistant_panel,
        ]
        .height(Length::Fill)
    } else {
        iced::widget::row![
            activity_bar,
            explorer_panel,
            editor_panel,
        ]
        .height(Length::Fill)
    };
    
    let status_bar = container(status_bar(app))
        .width(Length::Fill)
        .height(Length::Fixed(crate::ui::common::STATUS_BAR_HEIGHT));
    
    // Combine everything with subtle spacing
    iced::widget::column![
        top_bar,
        main_content,
        status_bar,
    ]
    .height(Length::Fill)
    .into()
}
