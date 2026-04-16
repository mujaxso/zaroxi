use iced::{Color, widget::{button, container, text_input}};
use crate::theme::{current_colors, ZaroxiTheme, SemanticColors};
use super::common;
use syntax_core::Highlight;

/// Get current theme colors from app state
pub fn colors(theme: ZaroxiTheme) -> SemanticColors {
    current_colors(theme)
}

/// Style helpers for UI components - designed for both core and extensions
#[derive(Debug, Clone, Copy)]
pub struct StyleHelpers {
    pub colors: SemanticColors,
    pub tokens: crate::theme::DesignTokens,
}

impl StyleHelpers {
    pub fn new(theme: ZaroxiTheme) -> Self {
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
    /// Panel container style - refined with subtle borders
    pub fn panel_container(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.panel_background.into()),
            border: iced::Border {
                color: iced::Color::from_rgba(
                    self.colors.border.r,
                    self.colors.border.g,
                    self.colors.border.b,
                    0.3
                ),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
    
    /// Elevated panel container style - subtle depth
    pub fn elevated_container(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.elevated_panel_background.into()),
            border: iced::Border {
                color: iced::Color::from_rgba(
                    self.colors.border.r,
                    self.colors.border.g,
                    self.colors.border.b,
                    0.2
                ),
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
    
    /// Shell container style
    pub fn shell_container(&self) -> container::Appearance {
        common::containers::shell(&self.colors)
    }
    
    /// Editor container style - premium and focused, minimal border
    pub fn editor_container(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.editor_background.into()),
            border: iced::Border {
                color: iced::Color::from_rgba(
                    self.colors.border.r,
                    self.colors.border.g,
                    self.colors.border.b,
                    0.2
                ),
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
    
    /// Input container style - refined
    pub fn input_container(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.input_background.into()),
            border: iced::Border {
                color: iced::Color::from_rgba(
                    self.colors.border.r,
                    self.colors.border.g,
                    self.colors.border.b,
                    0.3
                ),
                width: 1.0,
                radius: 2.0.into(),
            },
            ..Default::default()
        }
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
    
    /// Card container style - refined with subtle border
    pub fn card_container(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.elevated_panel_background.into()),
            border: iced::Border {
                color: iced::Color::from_rgba(
                    self.colors.border.r,
                    self.colors.border.g,
                    self.colors.border.b,
                    0.15
                ),
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        }
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

// Tab styles
impl StyleHelpers {
    /// Active tab style
    pub fn active_tab(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.editor_background.into()),
            border: iced::Border {
                color: self.colors.border,
                width: 1.0,
                radius: iced::border::Radius::from(0.0),
            },
            ..Default::default()
        }
    }
    
    /// Inactive tab style
    pub fn inactive_tab(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.panel_background.into()),
            border: iced::Border {
                color: self.colors.border,
                width: 1.0,
                radius: iced::border::Radius::from(0.0),
            },
            ..Default::default()
        }
    }
    
    /// Tab bar style
    pub fn tab_bar(&self) -> container::Appearance {
        container::Appearance {
            background: Some(self.colors.panel_background.into()),
            border: iced::Border {
                color: self.colors.border,
                width: 0.0,
                radius: iced::border::Radius::from(0.0),
            },
            ..Default::default()
        }
    }
    
    /// Close button style for tabs
    pub fn tab_close_button(&self) -> button::Appearance {
        button::Appearance {
            background: Some(Color::TRANSPARENT.into()),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::from(0.0),
            },
            text_color: self.colors.text_muted,
            ..Default::default()
        }
    }
}

// Scrollable styles
impl StyleHelpers {
    /// Create a scrollable style that matches the Explorer scrollbar - subtle and integrated
    pub fn subtle_scrollable_style(&self) -> iced::theme::Scrollable {
        struct SubtleScrollableStyle {
            colors: crate::theme::SemanticColors,
        }
        
        impl iced::widget::scrollable::StyleSheet for SubtleScrollableStyle {
            type Style = iced::Theme;

            fn active(&self, _style: &Self::Style) -> iced::widget::scrollable::Appearance {
                iced::widget::scrollable::Appearance {
                    container: container::Appearance::default(),
                    scrollbar: iced::widget::scrollable::Scrollbar {
                        background: Some(Color::TRANSPARENT.into()),
                        border: iced::Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: 0.0.into(),
                        },
                        scroller: iced::widget::scrollable::Scroller {
                            color: Color::from_rgba(
                                self.colors.border.r,
                                self.colors.border.g,
                                self.colors.border.b,
                                0.3,
                            ),
                            border: iced::Border {
                                color: Color::TRANSPARENT,
                                width: 0.0,
                                radius: 3.0.into(),
                            },
                        },
                    },
                    gap: None,
                }
            }

            fn hovered(&self, style: &Self::Style, _is_mouse_over_scrollbar: bool) -> iced::widget::scrollable::Appearance {
                let mut active = self.active(style);
                active.scrollbar.scroller.color = Color::from_rgba(
                    self.colors.accent.r,
                    self.colors.accent.g,
                    self.colors.accent.b,
                    0.7,
                );
                active
            }
        }
        
        iced::theme::Scrollable::Custom(Box::new(SubtleScrollableStyle {
            colors: self.colors,
        }))
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
            Highlight::Comment => self.colors.syntax_comment,
            Highlight::String => self.colors.syntax_string,
            Highlight::Keyword => self.colors.syntax_keyword,
            Highlight::Function => self.colors.syntax_function,
            Highlight::Variable => self.colors.syntax_variable,
            Highlight::Type => self.colors.syntax_type,
            Highlight::Constant => self.colors.syntax_constant,
            Highlight::Attribute => self.colors.syntax_attribute,
            Highlight::Operator => self.colors.syntax_operator,
            Highlight::Number => self.colors.syntax_number,
            Highlight::Property => self.colors.syntax_string,           // Use string color for properties
            Highlight::Namespace => self.colors.syntax_type,            // Use type color for namespaces
            Highlight::Plain => self.colors.syntax_plain,
        }
    }
}
