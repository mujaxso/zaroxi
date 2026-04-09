//! Editor font settings UI panel.
//!
//! Provides a clean, IDE-like interface for configuring editor typography.

use iced::{
    Element, Length, widget::{
        button, column, container, horizontal_rule, pick_list, row,
        scrollable, slider, text, toggler,
    },
};
use crate::message::Message;
use crate::state::App;
use crate::settings::editor::FontFamily;
use super::super::style::StyleHelpers;

/// Preview code snippet to demonstrate typography settings
const PREVIEW_CODE: &str = r#"// Preview of your typography settings
fn main() {
    let message = "Hello, world!";
    println!("{}", message);
    
    // Distinguish clearly: 0 vs O, 1 vs l, i vs l
    let zero = 0;
    let capital_o = 'O';
    let one = 1;
    let lowercase_l = 'l';
    let lowercase_i = 'i';
    
    // Common operators
    let result = (a + b) * (c - d) / e % f;
    let comparison = a == b && c != d || e > f;
    let arrow = a => b;
    
    // Braces, brackets, parentheses
    let array = [1, 2, 3];
    let tuple = (1, "two", 3.0);
    let map = HashMap::new();
}"#;

pub fn editor_font_settings_panel(app: &App) -> Element<'_, Message> {
    let style = StyleHelpers::new(app.theme);
    let typography = &app.editor_typography;
    
    // Font family picker
    let font_family_picker = column![
        text("Font Family")
            .size(12)
            .style(iced::theme::Text::Color(style.colors.text_secondary)),
        pick_list(
            FontFamily::all(),
            Some(typography.font_family),
            Message::FontFamilyChanged
        )
        .width(Length::Fixed(200.0))
        .padding(8)
    ]
    .spacing(4);
    
    // Font size control
    let font_size_control = column![
        text("Font Size")
            .size(12)
            .style(iced::theme::Text::Color(style.colors.text_secondary)),
        row![
            button("-")
                .on_press(Message::FontSizeChanged(typography.font_size.saturating_sub(1)))
                .padding([4, 8])
                .style(iced::theme::Button::Secondary),
            text(format!("{} px", typography.font_size))
                .size(12)
                .style(iced::theme::Text::Color(style.colors.text_primary))
                .width(Length::Fixed(60.0))
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            button("+")
                .on_press(Message::FontSizeChanged(typography.font_size.saturating_add(1)))
                .padding([4, 8])
                .style(iced::theme::Button::Secondary),
        ]
        .spacing(8)
        .align_items(iced::Alignment::Center)
    ]
    .spacing(4);
    
    // Line height slider
    let line_height_control = column![
        text(format!("Line Height: {:.1}", typography.line_height))
            .size(12)
            .style(iced::theme::Text::Color(style.colors.text_secondary)),
        slider(1.2..=2.0, typography.line_height, Message::LineHeightChanged)
            .step(0.1)
            .width(Length::Fixed(200.0))
    ]
    .spacing(4);
    
    // Ligatures toggle
    let ligatures_control = row![
        text("Ligatures")
            .size(12)
            .style(iced::theme::Text::Color(style.colors.text_secondary)),
        horizontal_space(),
        toggler(
            None::<String>,
            typography.ligatures_enabled,
            Message::LigaturesToggled
        )
        .width(Length::Shrink)
    ]
    .spacing(8)
    .align_items(iced::Alignment::Center)
    .width(Length::Fill);
    
    // Letter spacing slider (optional)
    let letter_spacing_control = column![
        text(format!("Letter Spacing: {:.1}px", typography.letter_spacing))
            .size(12)
            .style(iced::theme::Text::Color(style.colors.text_secondary)),
        slider(-0.2..=0.2, typography.letter_spacing, Message::LetterSpacingChanged)
            .step(0.05)
            .width(Length::Fixed(200.0))
    ]
    .spacing(4);
    
    // Action buttons
    let action_buttons = row![
        button("Reset to Defaults")
            .on_press(Message::ResetTypographyToDefaults)
            .padding([6, 12])
            .style(iced::theme::Button::Secondary),
        horizontal_space(),
        button("Save Settings")
            .on_press(Message::SaveTypographySettings)
            .padding([6, 12])
            .style(iced::theme::Button::Primary),
    ]
    .spacing(8)
    .width(Length::Fill);
    
    // Preview section
    let preview = container(
        column![
            text("Preview")
                .size(12)
                .style(iced::theme::Text::Color(style.colors.text_secondary)),
            container(
                text(PREVIEW_CODE)
                    .font(iced::Font::with_name(typography.font_family.to_family_string()))
                    .size(typography.font_size as f32)
                    .line_height(typography.line_height)
                    .style(iced::theme::Text::Color(style.colors.text_primary))
            )
            .padding(12)
            .width(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
                container::Appearance {
                    background: Some(style.colors.editor_background.into()),
                    border: iced::Border {
                        color: style.colors.border,
                        width: 1.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                }
            }))),
        ]
        .spacing(8)
    )
    .width(Length::Fill);
    
    // Main content
    let content = scrollable(
        column![
            font_family_picker,
            font_size_control,
            line_height_control,
            letter_spacing_control,
            ligatures_control,
            horizontal_rule(1),
            preview,
            horizontal_rule(1),
            action_buttons,
        ]
        .spacing(16)
        .padding(16)
    )
    .height(Length::Fill);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(move |_theme: &iced::Theme| {
            container::Appearance {
                background: Some(style.colors.panel_background.into()),
                border: iced::Border {
                    color: style.colors.border,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        })))
        .into()
}

// Helper for horizontal space
fn horizontal_space<'a>() -> Element<'a, Message> {
    iced::widget::horizontal_space().into()
}
