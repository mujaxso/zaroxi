// Common UI primitives and helpers for Zaroxi Studio - designed for both core and extensions
use iced::{Color, widget::{button, container, text}};
use crate::theme::SemanticColors;
use crate::settings::editor::EditorTypographySettings;
use crate::ui::icons::Icon;
use crate::ui::style::StyleHelpers;

/// Common spacing constants for reasonable IDE rhythm
pub const SPACING_XS: f32 = 4.0;
pub const SPACING_SM: f32 = 6.0;
pub const SPACING_MD: f32 = 8.0;
pub const SPACING_LG: f32 = 12.0;
pub const SPACING_XL: f32 = 16.0;

/// Common border radius values - subtle edges
pub const RADIUS_XS: f32 = 2.0;
pub const RADIUS_SM: f32 = 4.0;
pub const RADIUS_MD: f32 = 6.0;
pub const RADIUS_LG: f32 = 8.0;

/// Common sizes for UI elements - reasonable IDE density
pub const ICON_SIZE: f32 = 16.0;
pub const BUTTON_HEIGHT_SM: f32 = 28.0;
pub const BUTTON_HEIGHT_MD: f32 = 32.0;
pub const PANEL_HEADER_HEIGHT: f32 = 36.0;
pub const STATUS_BAR_HEIGHT: f32 = 24.0;
pub const EXPLORER_ROW_HEIGHT: f32 = 28.0;
pub const ACTIVITY_BAR_WIDTH: f32 = 48.0;
pub const TOP_BAR_HEIGHT: f32 = 40.0;

/// Extension-friendly theme access interface
/// This trait allows extensions to access Qyzer Studio's semantic colors and styles
pub trait ThemeConsumer {
    /// Get the full semantic colors palette
    fn colors(&self) -> SemanticColors;
    
    /// Get primary text color
    fn text_primary(&self) -> Color {
        self.colors().text_primary
    }
    
    /// Get secondary text color
    fn text_secondary(&self) -> Color {
        self.colors().text_secondary
    }
    
    /// Get panel background color
    fn background_panel(&self) -> Color {
        self.colors().panel_background
    }
    
    /// Get elevated panel background color
    fn background_elevated(&self) -> Color {
        self.colors().elevated_panel_background
    }
    
    /// Get accent color
    fn accent(&self) -> Color {
        self.colors().accent
    }
}

/// Common button styles for extensions to use - Premium compact design
pub mod buttons {
    use super::*;
    
    /// Primary button style - Accent color, used sparingly
    pub fn primary(colors: &SemanticColors) -> button::Appearance {
        button::Appearance {
            background: Some(colors.accent.into()),
            border: iced::Border {
                color: colors.accent,
                width: 1.0,
                radius: RADIUS_SM.into(),
            },
            text_color: colors.text_on_accent,
            ..Default::default()
        }
    }
    
    /// Secondary button style - Elevated panel background
    pub fn secondary(colors: &SemanticColors) -> button::Appearance {
        button::Appearance {
            background: Some(colors.elevated_panel_background.into()),
            border: iced::Border {
                color: colors.border,
                width: 1.0,
                radius: RADIUS_SM.into(),
            },
            text_color: colors.text_primary,
            ..Default::default()
        }
    }
    
    /// Text button style - Minimal, transparent background
    pub fn text(colors: &SemanticColors) -> button::Appearance {
        button::Appearance {
            background: Some(Color::TRANSPARENT.into()),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            text_color: colors.text_secondary,
            ..Default::default()
        }
    }
    
    /// Compact button style for IDE panels
    pub fn compact(colors: &SemanticColors) -> button::Appearance {
        button::Appearance {
            background: Some(colors.panel_background.into()),
            border: iced::Border {
                color: colors.border,
                width: 1.0,
                radius: RADIUS_XS.into(),
            },
            text_color: colors.text_secondary,
            ..Default::default()
        }
    }
}

/// Common container styles for extensions to use - Premium layered surfaces
pub mod containers {
    use super::*;
    
    /// Panel container style - Main side panel
    pub fn panel(colors: &SemanticColors) -> container::Appearance {
        container::Appearance {
            background: Some(colors.panel_background.into()),
            border: iced::Border {
                color: colors.border,
                width: 1.0,
                radius: RADIUS_SM.into(),
            },
            ..Default::default()
        }
    }
    
    /// Elevated panel container style - Cards, inputs
    pub fn elevated(colors: &SemanticColors) -> container::Appearance {
        container::Appearance {
            background: Some(colors.elevated_panel_background.into()),
            border: iced::Border {
                color: colors.border,
                width: 1.0,
                radius: RADIUS_MD.into(),
            },
            ..Default::default()
        }
    }
    
    /// Card container style - For content blocks, assistant messages
    pub fn card(colors: &SemanticColors) -> container::Appearance {
        container::Appearance {
            background: Some(colors.elevated_panel_background.into()),
            border: iced::Border {
                color: colors.divider,
                width: 1.0,
                radius: RADIUS_MD.into(),
            },
            ..Default::default()
        }
    }
    
    /// Shell container style - Outer app background
    pub fn shell(colors: &SemanticColors) -> container::Appearance {
        container::Appearance {
            background: Some(colors.shell_background.into()),
            border: iced::Border::default(),
            ..Default::default()
        }
    }
    
    /// Editor container style - Main editor surface, cleaner and more focused
    pub fn editor(colors: &SemanticColors) -> container::Appearance {
        container::Appearance {
            background: Some(colors.editor_background.into()),
            border: iced::Border {
                color: colors.border,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
    
    /// Editor content container style - ensures no borders and proper clipping
    pub fn editor_content(colors: &SemanticColors) -> container::Appearance {
        container::Appearance {
            background: Some(colors.editor_background.into()),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
    
    /// Input container style - Text inputs, search
    pub fn input(colors: &SemanticColors) -> container::Appearance {
        container::Appearance {
            background: Some(colors.input_background.into()),
            border: iced::Border {
                color: colors.border,
                width: 1.0,
                radius: RADIUS_SM.into(),
            },
            ..Default::default()
        }
    }
}

/// Common text styles for extensions to use
pub mod texts {
    use super::*;
    
    /// Primary text style
    pub fn primary(colors: &SemanticColors) -> text::Appearance {
        text::Appearance {
            color: Some(colors.text_primary),
        }
    }
    
    /// Secondary text style
    pub fn secondary(colors: &SemanticColors) -> text::Appearance {
        text::Appearance {
            color: Some(colors.text_secondary),
        }
    }
    
    /// Muted text style
    pub fn muted(colors: &SemanticColors) -> text::Appearance {
        text::Appearance {
            color: Some(colors.text_muted),
        }
    }
    
    /// Success text style
    pub fn success(colors: &SemanticColors) -> text::Appearance {
        text::Appearance {
            color: Some(colors.success),
        }
    }
    
    /// Warning text style
    pub fn warning(colors: &SemanticColors) -> text::Appearance {
        text::Appearance {
            color: Some(colors.warning),
        }
    }
    
    /// Error text style
    pub fn error(colors: &SemanticColors) -> text::Appearance {
        text::Appearance {
            color: Some(colors.error),
        }
    }
}

/// Create a centered icon button with consistent styling
pub fn centered_icon_button<'a, Message>(
    icon: Icon,
    typography: &EditorTypographySettings,
    style: &StyleHelpers,
    on_press: Option<Message>,
    size: Option<u16>,
    button_size: Option<f32>,
) -> iced::widget::Button<'a, Message>
where
    Message: Clone + 'a,
{
    use iced::Length;
    
    // Use even smaller default sizes: default icon size is 10px, button size is 20px
    let icon_size = size.unwrap_or(10);
    let button_size_val = button_size.unwrap_or(20.0);
    
    // Create the icon element
    let icon_element = icon.render(typography, style, Some(icon_size));
    
    // Center the icon in a container
    let centered_icon = container(icon_element)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();
    
    let button = iced::widget::button(centered_icon)
        .width(Length::Fixed(button_size_val))
        .height(Length::Fixed(button_size_val))
        .padding(0)
        .style(iced::theme::Button::Text);
    
    if let Some(message) = on_press {
        button.on_press(message)
    } else {
        button
    }
}
