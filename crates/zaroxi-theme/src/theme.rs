//! Theme definitions for Zaroxi
//! This module provides zaroxi_theme variants, design tokens, and semantic colors

use serde::{Deserialize, Serialize};
use crate::colors::Color;

/// Theme variants for Zaroxi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZaroxiTheme {
    /// Dark zaroxi_theme
    Dark,
    /// Light zaroxi_theme
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

impl Default for ZaroxiTheme {
    fn default() -> Self {
        ZaroxiTheme::System
    }
}

impl ZaroxiTheme {
    /// Get all available zaroxi_theme variants
    pub fn all() -> Vec<Self> {
        vec![ZaroxiTheme::System, ZaroxiTheme::Light, ZaroxiTheme::Dark]
    }
    
    /// Get display name for the zaroxi_theme
    pub fn display_name(&self) -> &'static str {
        match self {
            ZaroxiTheme::System => "System",
            ZaroxiTheme::Light => "Light",
            ZaroxiTheme::Dark => "Dark",
        }
    }
    
    /// Resolve to concrete theme (Dark or Light) based on system preference if needed
    pub fn resolve(&self, system_is_dark: bool) -> Self {
        match self {
            ZaroxiTheme::Dark => ZaroxiTheme::Dark,
            ZaroxiTheme::Light => ZaroxiTheme::Light,
            ZaroxiTheme::System => {
                if system_is_dark {
                    ZaroxiTheme::Dark
                } else {
                    ZaroxiTheme::Light
                }
            }
        }
    }
    
    /// Get the semantic colors for this zaroxi_theme
    pub fn colors(&self, system_is_dark: bool) -> SemanticColors {
        match self.resolve(system_is_dark) {
            ZaroxiTheme::Dark => SemanticColors::dark(),
            ZaroxiTheme::Light => SemanticColors::light(),
            ZaroxiTheme::System => unreachable!(), // Should be resolved above
        }
    }
}

/// Design system tokens for Zaroxi
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
    
    // Typography
    pub font_size_sm: f32,
    pub font_size_md: f32,
    pub font_size_lg: f32,
    pub font_size_xl: f32,
    pub font_size_xxl: f32,
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
            
            font_size_sm: 12.0,
            font_size_md: 14.0,
            font_size_lg: 16.0,
            font_size_xl: 20.0,
            font_size_xxl: 24.0,
        }
    }
}

/// Semantic color roles for Zaroxi IDE
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SemanticColors {
    // Background surfaces - hierarchy from deepest to highest
    pub app_background: Color,
    pub shell_background: Color,
    pub panel_background: Color,
    pub elevated_panel_background: Color,
    pub editor_background: Color,
    pub input_background: Color,
    pub status_bar_background: Color,
    pub title_bar_background: Color,
    pub activity_rail_background: Color,
    pub sidebar_background: Color,
    pub tab_background: Color,
    pub tab_active_background: Color,
    pub assistant_panel_background: Color,
    
    // Text colors - hierarchy from most prominent to subtle
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub text_faint: Color,
    pub text_on_accent: Color,
    pub text_on_surface: Color,
    pub text_disabled: Color,
    pub text_link: Color,
    
    // UI elements
    pub border: Color,
    pub border_subtle: Color,
    pub divider: Color,
    pub accent: Color,
    pub accent_hover: Color,
    pub accent_soft: Color,
    pub accent_soft_background: Color,
    
    // States
    pub hover_background: Color,
    pub active_background: Color,
    pub selected_background: Color,
    pub selected_text_background: Color,
    pub selected_editor_background: Color,
    
    // Status colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    
    // Focus
    pub focus_ring: Color,
    
    // Editor specific
    pub editor_gutter_background: Color,
    pub editor_line_highlight: Color,
    pub editor_cursor: Color,
    pub editor_selection: Color,
    pub editor_find_highlight: Color,
    
    // Syntax colors (basic set for IDE)
    pub syntax_keyword: Color,
    pub syntax_function: Color,
    pub syntax_string: Color,
    pub syntax_comment: Color,
    pub syntax_type: Color,
    pub syntax_variable: Color,
    pub syntax_constant: Color,
}

impl SemanticColors {
    /// Dark theme semantic colors - optimized for long coding sessions
    pub fn dark() -> Self {
        Self {
            app_background: Color::from_rgb(0.098, 0.110, 0.137),
            shell_background: Color::from_rgb(0.118, 0.129, 0.157),
            panel_background: Color::from_rgb(0.137, 0.149, 0.176),
            elevated_panel_background: Color::from_rgb(0.157, 0.169, 0.196),
            editor_background: Color::from_rgb(0.098, 0.110, 0.137),
            input_background: Color::from_rgb(0.137, 0.149, 0.176),
            status_bar_background: Color::from_rgb(0.118, 0.129, 0.157),
            title_bar_background: Color::from_rgb(0.078, 0.090, 0.117),
            activity_rail_background: Color::from_rgb(0.078, 0.090, 0.117),
            sidebar_background: Color::from_rgb(0.118, 0.129, 0.157),
            tab_background: Color::from_rgb(0.137, 0.149, 0.176),
            tab_active_background: Color::from_rgb(0.098, 0.110, 0.137),
            assistant_panel_background: Color::from_rgb(0.118, 0.129, 0.157),
            
            text_primary: Color::from_rgb(0.941, 0.949, 0.961),
            text_secondary: Color::from_rgb(0.784, 0.800, 0.824),
            text_muted: Color::from_rgb(0.627, 0.647, 0.678),
            text_faint: Color::from_rgb(0.471, 0.486, 0.518),
            text_on_accent: Color::from_rgb(1.0, 1.0, 1.0),
            text_on_surface: Color::from_rgb(0.941, 0.949, 0.961),
            text_disabled: Color::from_rgb(0.471, 0.486, 0.518),
            text_link: Color::from_rgb(0.329, 0.584, 0.988),
            
            border: Color::from_rgb(0.196, 0.208, 0.235),
            border_subtle: Color::from_rgba(0.196, 0.208, 0.235, 0.5),
            divider: Color::from_rgb(0.196, 0.208, 0.235),
            accent: Color::from_rgb(0.329, 0.584, 0.988),
            accent_hover: Color::from_rgb(0.400, 0.639, 1.0),
            accent_soft: Color::from_rgba(0.329, 0.584, 0.988, 0.2),
            accent_soft_background: Color::from_rgba(0.329, 0.584, 0.988, 0.12),
            
            hover_background: Color::from_rgba(1.0, 1.0, 1.0, 0.05),
            active_background: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            selected_background: Color::from_rgba(0.329, 0.584, 0.988, 0.15),
            selected_text_background: Color::from_rgba(0.329, 0.584, 0.988, 0.25),
            selected_editor_background: Color::from_rgba(0.329, 0.584, 0.988, 0.2),
            
            success: Color::from_rgb(0.298, 0.843, 0.596),
            warning: Color::from_rgb(0.988, 0.729, 0.298),
            error: Color::from_rgb(0.988, 0.447, 0.447),
            info: Color::from_rgb(0.329, 0.584, 0.988),
            
            focus_ring: Color::from_rgba(0.329, 0.584, 0.988, 0.25),
            
            editor_gutter_background: Color::from_rgb(0.078, 0.090, 0.117),
            editor_line_highlight: Color::from_rgba(1.0, 1.0, 1.0, 0.03),
            editor_cursor: Color::from_rgb(0.941, 0.949, 0.961),
            editor_selection: Color::from_rgba(0.329, 0.584, 0.988, 0.3),
            editor_find_highlight: Color::from_rgba(0.988, 0.729, 0.298, 0.3),
            
            syntax_keyword: Color::from_rgb(0.988, 0.447, 0.447),
            syntax_function: Color::from_rgb(0.298, 0.843, 0.596),
            syntax_string: Color::from_rgb(0.988, 0.729, 0.298),
            syntax_comment: Color::from_rgb(0.627, 0.647, 0.678),
            syntax_type: Color::from_rgb(0.329, 0.584, 0.988),
            syntax_variable: Color::from_rgb(0.941, 0.949, 0.961),
            syntax_constant: Color::from_rgb(0.988, 0.729, 0.298),
        }
    }
    
    /// Light theme semantic colors - clean but not sterile, optimized for readability
    pub fn light() -> Self {
        Self {
            app_background: Color::from_rgb(0.973, 0.976, 0.980),
            shell_background: Color::from_rgb(0.961, 0.965, 0.969),
            panel_background: Color::from_rgb(0.980, 0.982, 0.984),
            elevated_panel_background: Color::from_rgb(1.0, 1.0, 1.0),
            editor_background: Color::from_rgb(0.992, 0.992, 0.990),
            input_background: Color::from_rgb(1.0, 1.0, 1.0),
            status_bar_background: Color::from_rgb(0.961, 0.965, 0.969),
            title_bar_background: Color::from_rgb(0.941, 0.945, 0.949),
            activity_rail_background: Color::from_rgb(0.941, 0.945, 0.949),
            sidebar_background: Color::from_rgb(0.961, 0.965, 0.969),
            tab_background: Color::from_rgb(0.980, 0.982, 0.984),
            tab_active_background: Color::from_rgb(0.992, 0.992, 0.990),
            assistant_panel_background: Color::from_rgb(0.961, 0.965, 0.969),
            
            text_primary: Color::from_rgb(0.18, 0.20, 0.23),
            text_secondary: Color::from_rgb(0.40, 0.43, 0.47),
            text_muted: Color::from_rgb(0.55, 0.58, 0.62),
            text_faint: Color::from_rgb(0.70, 0.72, 0.75),
            text_on_accent: Color::from_rgb(1.0, 1.0, 1.0),
            text_on_surface: Color::from_rgb(0.18, 0.20, 0.23),
            text_disabled: Color::from_rgb(0.70, 0.72, 0.75),
            text_link: Color::from_rgb(0.06, 0.53, 0.98),
            
            border: Color::from_rgb(0.88, 0.89, 0.91),
            border_subtle: Color::from_rgba(0.88, 0.89, 0.91, 0.5),
            divider: Color::from_rgb(0.88, 0.89, 0.91),
            accent: Color::from_rgb(0.06, 0.53, 0.98),
            accent_hover: Color::from_rgb(0.04, 0.47, 0.88),
            accent_soft: Color::from_rgba(0.06, 0.53, 0.98, 0.12),
            accent_soft_background: Color::from_rgba(0.06, 0.53, 0.98, 0.06),
            
            hover_background: Color::from_rgba(0.0, 0.0, 0.0, 0.04),
            active_background: Color::from_rgba(0.0, 0.0, 0.0, 0.06),
            selected_background: Color::from_rgba(0.06, 0.53, 0.98, 0.1),
            selected_text_background: Color::from_rgba(0.06, 0.53, 0.98, 0.2),
            selected_editor_background: Color::from_rgba(0.06, 0.53, 0.98, 0.15),
            
            success: Color::from_rgb(0.16, 0.65, 0.33),
            warning: Color::from_rgb(0.91, 0.58, 0.07),
            error: Color::from_rgb(0.91, 0.26, 0.26),
            info: Color::from_rgb(0.06, 0.53, 0.98),
            
            focus_ring: Color::from_rgba(0.06, 0.53, 0.98, 0.25),
            
            editor_gutter_background: Color::from_rgb(0.941, 0.945, 0.949),
            editor_line_highlight: Color::from_rgba(0.0, 0.0, 0.0, 0.02),
            editor_cursor: Color::from_rgb(0.18, 0.20, 0.23),
            editor_selection: Color::from_rgba(0.06, 0.53, 0.98, 0.2),
            editor_find_highlight: Color::from_rgba(0.91, 0.58, 0.07, 0.2),
            
            syntax_keyword: Color::from_rgb(0.91, 0.26, 0.26),
            syntax_function: Color::from_rgb(0.16, 0.65, 0.33),
            syntax_string: Color::from_rgb(0.91, 0.58, 0.07),
            syntax_comment: Color::from_rgb(0.55, 0.58, 0.62),
            syntax_type: Color::from_rgb(0.06, 0.53, 0.98),
            syntax_variable: Color::from_rgb(0.18, 0.20, 0.23),
            syntax_constant: Color::from_rgb(0.91, 0.58, 0.07),
        }
    }
}
