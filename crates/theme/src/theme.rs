//! Theme definitions for Zaroxi Studio

use serde::{Deserialize, Serialize};
use crate::colors::Color;

/// Theme variants for Zaroxi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZaroxiTheme {
    /// Dark theme
    Dark,
    /// Light theme
    Light,
    /// Use system preference
    System,
}

impl std::fmt::Display for ZaroxiTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZaroxiTheme::Dark => write!(f, "Dark"),
            ZaroxiTheme::Light => write!(f, "Light"),
            ZaroxiTheme::System => write!(f, "System"),
        }
    }
}

impl ZaroxiTheme {
    /// Get all available theme variants
    pub fn all() -> Vec<Self> {
        vec![ZaroxiTheme::System, ZaroxiTheme::Light, ZaroxiTheme::Dark]
    }
    
    /// Get display name for the theme
    pub fn display_name(&self) -> &'static str {
        match self {
            ZaroxiTheme::System => "System",
            ZaroxiTheme::Light => "Light",
            ZaroxiTheme::Dark => "Dark",
        }
    }
}

/// Design system tokens for Zaroxi Studio
#[derive(Debug, Clone, Copy)]
pub struct DesignTokens {
    // Spacing scale (in pixels)
    pub spacing_xxs: f32,
    pub spacing_xs: f32,
    pub spacing_sm: f32,
    pub spacing_md: f32,
    pub spacing_lg: f32,
    pub spacing_xl: f32,
    pub spacing_xxl: f32,
    
    // Border radius
    pub radius_sm: f32,
    pub radius_md: f32,
    pub radius_lg: f32,
    
    // Border widths
    pub border_width: f32,
    pub border_width_thick: f32,
}

impl Default for DesignTokens {
    fn default() -> Self {
        Self {
            spacing_xxs: 2.0,
            spacing_xs: 4.0,
            spacing_sm: 8.0,
            spacing_md: 12.0,
            spacing_lg: 16.0,
            spacing_xl: 24.0,
            spacing_xxl: 32.0,
            
            radius_sm: 4.0,
            radius_md: 6.0,
            radius_lg: 8.0,
            
            border_width: 1.0,
            border_width_thick: 2.0,
        }
    }
}

/// Semantic color roles for Zaroxi Studio
#[derive(Debug, Clone, Copy)]
pub struct SemanticColors {
    // Background surfaces
    pub app_background: Color,
    pub shell_background: Color,
    pub panel_background: Color,
    pub elevated_panel_background: Color,
    pub editor_background: Color,
    pub input_background: Color,
    pub status_bar_background: Color,
    
    // Text colors
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub text_faint: Color,
    pub text_on_accent: Color,
    
    // UI elements
    pub border: Color,
    pub divider: Color,
    pub accent: Color,
    pub accent_hover: Color,
    pub accent_soft: Color,
    pub accent_soft_background: Color,
    
    // States
    pub hover_background: Color,
    pub active_background: Color,
    pub selected_background: Color,
    
    // Status colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    
    // Focus
    pub focus_ring: Color,
    
    // Syntax highlighting colors
    pub syntax_comment: Color,
    pub syntax_string: Color,
    pub syntax_keyword: Color,
    pub syntax_function: Color,
    pub syntax_type: Color,
    pub syntax_variable: Color,
    pub syntax_constant: Color,
    pub syntax_number: Color,
    pub syntax_operator: Color,
    pub syntax_punctuation: Color,
    pub syntax_attribute: Color,
    pub syntax_macro: Color,
    pub syntax_builtin: Color,
    pub syntax_plain: Color,
}

impl SemanticColors {
    /// Dark theme semantic colors
    pub fn dark() -> Self {
        Self {
            app_background: Color::from_rgb(0.098, 0.110, 0.137),
            shell_background: Color::from_rgb(0.118, 0.129, 0.157),
            panel_background: Color::from_rgb(0.137, 0.149, 0.176),
            elevated_panel_background: Color::from_rgb(0.157, 0.169, 0.196),
            editor_background: Color::from_rgb(0.098, 0.110, 0.137),
            input_background: Color::from_rgb(0.137, 0.149, 0.176),
            status_bar_background: Color::from_rgb(0.118, 0.129, 0.157),
            
            text_primary: Color::from_rgb(0.941, 0.949, 0.961),
            text_secondary: Color::from_rgb(0.784, 0.800, 0.824),
            text_muted: Color::from_rgb(0.627, 0.647, 0.678),
            text_faint: Color::from_rgb(0.471, 0.486, 0.518),
            text_on_accent: Color::from_rgb(1.0, 1.0, 1.0),
            
            border: Color::from_rgb(0.196, 0.208, 0.235),
            divider: Color::from_rgb(0.196, 0.208, 0.235),
            accent: Color::from_rgb(0.329, 0.584, 0.988),
            accent_hover: Color::from_rgb(0.400, 0.639, 1.0),
            accent_soft: Color::from_rgba(0.329, 0.584, 0.988, 0.2),
            accent_soft_background: Color::from_rgba(0.329, 0.584, 0.988, 0.12),
            
            hover_background: Color::from_rgba(1.0, 1.0, 1.0, 0.05),
            active_background: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            selected_background: Color::from_rgba(0.329, 0.584, 0.988, 0.15),
            
            success: Color::from_rgb(0.298, 0.843, 0.596),
            warning: Color::from_rgb(0.988, 0.729, 0.298),
            error: Color::from_rgb(0.988, 0.447, 0.447),
            info: Color::from_rgb(0.329, 0.584, 0.988),
            
            focus_ring: Color::from_rgba(0.329, 0.584, 0.988, 0.25),
            
            syntax_comment: Color::from_rgb(0.627, 0.647, 0.678),
            syntax_string: Color::from_rgb(0.298, 0.843, 0.596),
            syntax_keyword: Color::from_rgb(0.788, 0.435, 0.949),
            syntax_function: Color::from_rgb(0.329, 0.584, 0.988),
            syntax_type: Color::from_rgb(0.988, 0.729, 0.298),
            syntax_variable: Color::from_rgb(0.941, 0.949, 0.961),
            syntax_constant: Color::from_rgb(0.949, 0.518, 0.659),
            syntax_number: Color::from_rgb(0.949, 0.518, 0.659),
            syntax_operator: Color::from_rgb(0.784, 0.800, 0.824),
            syntax_punctuation: Color::from_rgb(0.784, 0.800, 0.824),
            syntax_attribute: Color::from_rgb(0.788, 0.435, 0.949),
            syntax_macro: Color::from_rgb(0.329, 0.584, 0.988),
            syntax_builtin: Color::from_rgb(0.988, 0.729, 0.298),
            syntax_plain: Color::from_rgb(0.941, 0.949, 0.961),
        }
    }
    
    /// Light theme semantic colors
    pub fn light() -> Self {
        Self {
            app_background: Color::from_rgb(0.973, 0.976, 0.980),
            shell_background: Color::from_rgb(0.961, 0.965, 0.969),
            panel_background: Color::from_rgb(0.980, 0.982, 0.984),
            elevated_panel_background: Color::from_rgb(1.0, 1.0, 1.0),
            editor_background: Color::from_rgb(0.992, 0.992, 0.990),
            input_background: Color::from_rgb(1.0, 1.0, 1.0),
            status_bar_background: Color::from_rgb(0.961, 0.965, 0.969),
            
            text_primary: Color::from_rgb(0.18, 0.20, 0.23),
            text_secondary: Color::from_rgb(0.40, 0.43, 0.47),
            text_muted: Color::from_rgb(0.55, 0.58, 0.62),
            text_faint: Color::from_rgb(0.70, 0.72, 0.75),
            text_on_accent: Color::from_rgb(1.0, 1.0, 1.0),
            
            border: Color::from_rgb(0.88, 0.89, 0.91),
            divider: Color::from_rgb(0.88, 0.89, 0.91),
            accent: Color::from_rgb(0.06, 0.53, 0.98),
            accent_hover: Color::from_rgb(0.04, 0.47, 0.88),
            accent_soft: Color::from_rgba(0.06, 0.53, 0.98, 0.12),
            accent_soft_background: Color::from_rgba(0.06, 0.53, 0.98, 0.06),
            
            hover_background: Color::from_rgba(0.0, 0.0, 0.0, 0.04),
            active_background: Color::from_rgba(0.0, 0.0, 0.0, 0.06),
            selected_background: Color::from_rgba(0.06, 0.53, 0.98, 0.1),
            
            success: Color::from_rgb(0.16, 0.65, 0.33),
            warning: Color::from_rgb(0.91, 0.58, 0.07),
            error: Color::from_rgb(0.91, 0.26, 0.26),
            info: Color::from_rgb(0.06, 0.53, 0.98),
            
            focus_ring: Color::from_rgba(0.06, 0.53, 0.98, 0.25),
            
            syntax_comment: Color::from_rgb(0.45, 0.52, 0.60),
            syntax_string: Color::from_rgb(0.75, 0.12, 0.12),
            syntax_keyword: Color::from_rgb(0.62, 0.12, 0.68),
            syntax_function: Color::from_rgb(0.00, 0.40, 0.70),
            syntax_type: Color::from_rgb(0.80, 0.35, 0.00),
            syntax_variable: Color::from_rgb(0.20, 0.22, 0.25),
            syntax_constant: Color::from_rgb(0.62, 0.12, 0.68),
            syntax_number: Color::from_rgb(0.12, 0.58, 0.25),
            syntax_operator: Color::from_rgb(0.40, 0.43, 0.47),
            syntax_punctuation: Color::from_rgb(0.55, 0.58, 0.62),
            syntax_attribute: Color::from_rgb(0.62, 0.12, 0.68),
            syntax_macro: Color::from_rgb(0.80, 0.35, 0.00),
            syntax_builtin: Color::from_rgb(0.00, 0.40, 0.70),
            syntax_plain: Color::from_rgb(0.20, 0.22, 0.25),
        }
    }
}

impl ZaroxiTheme {
    /// Get the semantic colors for this theme
    pub fn colors(&self) -> SemanticColors {
        match self {
            ZaroxiTheme::Dark => SemanticColors::dark(),
            ZaroxiTheme::Light => SemanticColors::light(),
            ZaroxiTheme::System => {
                // For now, default to dark theme
                SemanticColors::dark()
            }
        }
    }
}
