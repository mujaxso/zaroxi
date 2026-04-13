use iced::{Color, widget::{button, container, text_input}};
use crate::theme::{current_colors, NeoteTheme, SemanticColors};
use super::common;
use syntax_core::Highlight;

/// Get current theme colors from app state
pub fn colors(theme: NeoteTheme) -> SemanticColors {
    current_colors(theme)
}

/// Style helpers for UI components - designed for both core and extensions
#[derive(Debug, Clone, Copy)]
pub struct StyleHelpers {
    pub colors: SemanticColors,
    pub tokens: crate::theme::DesignTokens,
}

impl StyleHelpers {
    pub fn new(theme: NeoteTheme) -> Self {
        Self {
            colors: colors(theme),
            tokens: crate::theme::DesignTokens::default(),
        }
    }
    
    /// Get semantic colors for extension use
    pub fn semantic_colors(&self) -> SemanticColors {
        self.colors
    }
    
    /// Implement ThemeConsumer trait for extensions
    pub fn as_theme_consumer(&self) -> StyleHelpers {
        *self
    }
}

impl common::ThemeConsumer for StyleHelpers {
    fn colors(&self) -> SemanticColors {
        self.colors
    }
}

// Panel container styles
impl StyleHelpers {
    /// Panel container style
    pub fn panel_container(&self) -> container::Appearance {
        common::containers::panel(&self.colors)
    }
    
    /// Elevated panel container style
    pub fn elevated_container(&self) -> container::Appearance {
        common::containers::elevated(&self.colors)
    }
    
    /// Shell container style
    pub fn shell_container(&self) -> container::Appearance {
        common::containers::shell(&self.colors)
    }
    
    /// Editor container style - more premium and focused
    pub fn editor_container(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.editor_background.into()),
            border: iced::Border {
                color: self.colors.border,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }

    /// Editor viewport style - for the actual code area
    pub fn editor_viewport(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.editor_background.into()),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
    
    /// Editor content container style - ensures no borders and proper clipping
    pub fn editor_content_container(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.editor_background.into()),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
    
    /// Input container style
    pub fn input_container(&self) -> container::Appearance {
        common::containers::input(&self.colors)
    }
    
    /// Status bar container style
    pub fn status_bar_container(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.status_bar_background.into()),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            text_color: None,
            shadow: Default::default(),
        }
    }
    
    /// Card container style
    pub fn card_container(&self) -> container::Appearance {
        common::containers::card(&self.colors)
    }
}

// Button styles
impl StyleHelpers {
    /// Primary button style
    pub fn primary_button(&self) -> button::Appearance {
        common::buttons::primary(&self.colors)
    }
    
    /// Secondary button style
    pub fn secondary_button(&self) -> button::Appearance {
        common::buttons::secondary(&self.colors)
    }
    
    /// Text button style
    pub fn text_button(&self) -> button::Appearance {
        common::buttons::text(&self.colors)
    }
}

// Text input styles
impl StyleHelpers {
    /// Text input style
    pub fn text_input(&self) -> text_input::Appearance {
        text_input::Appearance {
            background: self.colors.input_background.into(),
            border: iced::Border {
                color: self.colors.border,
                width: 1.0,
                radius: self.tokens.radius_sm.into(),
            },
            icon_color: self.colors.text_muted,
        }
    }
    
    /// Text editor style
    pub fn text_editor(&self) -> iced::widget::text_editor::Appearance {
        iced::widget::text_editor::Appearance {
            background: self.colors.editor_background.into(),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
        }
    }
}

// Text colors
impl StyleHelpers {
    /// Primary text color
    pub fn text_primary(&self) -> iced::Color {
        self.colors.text_primary
    }
    
    /// Secondary text color
    pub fn text_secondary(&self) -> iced::Color {
        self.colors.text_secondary
    }
    
    /// Muted text color
    pub fn text_muted(&self) -> iced::Color {
        self.colors.text_muted
    }
    
    /// Success text color
    pub fn text_success(&self) -> iced::Color {
        self.colors.success
    }
    
    /// Warning text color
    pub fn text_warning(&self) -> iced::Color {
        self.colors.warning
    }
    
    /// Error text color
    pub fn text_error(&self) -> iced::Color {
        self.colors.error
    }
    
    /// Text on accent color
    pub fn text_on_accent(&self) -> iced::Color {
        self.colors.text_on_accent
    }

    /// Map a syntax highlight token to a semantic color.
    pub fn highlight_color(&self, highlight: Highlight) -> iced::Color {
        match highlight {
            Highlight::Comment => Color::from_rgb(0.5, 0.5, 0.5),       // grey
            Highlight::String => Color::from_rgb(0.4, 1.0, 0.4),        // bright green
            Highlight::Keyword => Color::from_rgb(0.9, 0.2, 0.2),       // red
            Highlight::Function => Color::from_rgb(0.0, 0.6, 0.9),      // blue
            Highlight::Variable => Color::from_rgb(0.8, 0.8, 0.8),      // light grey
            Highlight::Type => Color::from_rgb(0.7, 0.0, 0.8),          // purple
            Highlight::Constant => Color::from_rgb(0.9, 0.5, 0.0),      // orange
            Highlight::Attribute => Color::from_rgb(0.0, 0.8, 0.8),     // cyan
            Highlight::Operator => Color::from_rgb(0.9, 0.9, 0.0),      // yellow
            Highlight::Number => Color::from_rgb(0.9, 0.6, 0.2),        // orange
            Highlight::Property => Color::from_rgb(0.0, 0.8, 0.5),      // teal
            Highlight::Namespace => Color::from_rgb(0.6, 0.2, 0.9),     // violet
            Highlight::Plain => self.colors.text_primary,               // Use theme primary text
        }
    }
}
