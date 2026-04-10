use iced::{Element, Length, widget::{container, text}};
use crate::message::Message;
use crate::state::{App, LayoutMode, PrimarySidebarView, AuxiliaryView};
use super::{
    activity_bar::activity_bar,
    assistant_panel::assistant_panel,
    editor_panel::editor_panel,
    status_bar::status_bar,
    top_bar::top_bar,
    explorer_panel::explorer_panel,
    settings::editor_font_settings_panel,
};

/// Main shell that composes all UI components - Premium compact layout
pub fn shell(app: &App) -> Element<'_, Message> {
    // Get panel widths based on layout mode
    let (explorer_width, assistant_width) = match app.layout_mode {
        LayoutMode::Wide => (260.0, 300.0),
        LayoutMode::Medium => (220.0, 240.0),
        LayoutMode::Narrow => (180.0, 200.0),
    };
    
    // Build panels with responsive sizing
    let top_bar = container(top_bar(app))
        .width(Length::Fill)
        .height(Length::Fixed(crate::ui::common::TOP_BAR_HEIGHT));
    
    let activity_bar = container(activity_bar(app))
        .width(Length::Fixed(crate::ui::common::ACTIVITY_BAR_WIDTH))
        .height(Length::Fill);
    
    // Check if we're in Settings mode - Settings should take over the main editor area
    let is_settings_mode = app.workbench_layout.active_primary_view == PrimarySidebarView::Settings;
    
    // Primary sidebar panel (not shown when in settings mode)
    let primary_sidebar = if app.workbench_layout.primary_sidebar_visible && !is_settings_mode {
        match app.workbench_layout.active_primary_view {
            PrimarySidebarView::Explorer => {
                let explorer_panel = container(explorer_panel(app))
                    .width(Length::Fixed(explorer_width))
                    .height(Length::Fill);
                Some(explorer_panel)
            }
            PrimarySidebarView::Search => {
                // For now, use a placeholder
                let search_panel = container(text("Search Panel"))
                    .width(Length::Fixed(explorer_width))
                    .height(Length::Fill);
                Some(search_panel)
            }
            PrimarySidebarView::SourceControl => {
                // For now, use a placeholder
                let git_panel = container(text("Git Panel"))
                    .width(Length::Fixed(explorer_width))
                    .height(Length::Fill);
                Some(git_panel)
            }
            PrimarySidebarView::Settings => {
                // This shouldn't be reached when is_settings_mode is true
                let settings_panel = container(editor_font_settings_panel(app))
                    .width(Length::Fixed(explorer_width))
                    .height(Length::Fill);
                Some(settings_panel)
            }
        }
    } else {
        None
    };
    
    // Editor panel or Settings panel
    let main_editor_area = if is_settings_mode {
        // When in settings mode, show settings in the main editor area
        container(editor_font_settings_panel(app))
            .width(Length::Fill)
            .height(Length::Fill)
    } else {
        // Normal editor panel
        container(editor_panel(app))
            .width(Length::Fill)
            .height(Length::Fill)
    };
    
    // Auxiliary sidebar (AI Assistant)
    let auxiliary_sidebar = if app.workbench_layout.auxiliary_sidebar_visible && !is_settings_mode {
        match app.workbench_layout.active_auxiliary_view {
            Some(AuxiliaryView::AiAssistant) => {
                let assistant_panel = container(assistant_panel(app))
                    .width(Length::Fixed(assistant_width))
                    .height(Length::Fill);
                Some(assistant_panel)
            }
            None => None,
        }
    } else {
        None
    };
    
    // Build the main content row
    let mut main_content_row = iced::widget::row![
        activity_bar,
    ];
    
    // Only show primary sidebar if not in settings mode
    if !is_settings_mode {
        if let Some(primary) = primary_sidebar {
            main_content_row = main_content_row.push(primary);
        }
    }
    
    main_content_row = main_content_row.push(main_editor_area);
    
    // Only show auxiliary sidebar if not in settings mode
    if !is_settings_mode {
        if let Some(auxiliary) = auxiliary_sidebar {
            main_content_row = main_content_row.push(auxiliary);
        }
    }
    
    let main_content = main_content_row.height(Length::Fill);
    
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
