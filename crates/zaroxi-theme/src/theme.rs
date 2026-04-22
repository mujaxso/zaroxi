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
    /// Dark theme semantic colors - optimized for long coding sessions with clear hierarchy
    pub fn dark() -> Self {
        Self {
            // Background surfaces - clear hierarchy from darkest to brightest
            app_background: Color::from_rgb(0.06, 0.07, 0.09),           // #0f1117 - Darkest: app frame
            shell_background: Color::from_rgb(0.08, 0.09, 0.11),        // #14171c - Shell background
            panel_background: Color::from_rgb(0.10, 0.11, 0.13),        // #1a1c21 - Side panels
            elevated_panel_background: Color::from_rgb(0.12, 0.13, 0.15), // #1f2126 - Elevated panels
            editor_background: Color::from_rgb(0.13, 0.14, 0.16),       // #22242a - Editor: brightest surface
            input_background: Color::from_rgb(0.10, 0.11, 0.13),        // #1a1c21 - Inputs match panels
            status_bar_background: Color::from_rgb(0.08, 0.09, 0.11),   // #14171c - Matches shell
            title_bar_background: Color::from_rgb(0.05, 0.06, 0.08),    // #0d0f14 - Top bar: darkest
            activity_rail_background: Color::from_rgb(0.07, 0.08, 0.10), // #121419 - Distinct from panels
            sidebar_background: Color::from_rgb(0.10, 0.11, 0.13),      // #1a1c21 - Sidebar matches panels
            tab_background: Color::from_rgb(0.10, 0.11, 0.13),          // #1a1c21 - Inactive tabs
            tab_active_background: Color::from_rgb(0.13, 0.14, 0.16),   // #22242a - Active tab matches editor
            assistant_panel_background: Color::from_rgb(0.10, 0.11, 0.13), // #1a1c21 - Right panel matches left
            
            // Text colors - improved contrast and clarity
            text_primary: Color::from_rgb(0.98, 0.98, 0.98),            // #fafafa - Primary text
            text_secondary: Color::from_rgb(0.85, 0.86, 0.88),          // #d9dbdf - Secondary: high contrast
            text_muted: Color::from_rgb(0.65, 0.66, 0.68),              // #a6a8ad - Muted but readable
            text_faint: Color::from_rgb(0.50, 0.51, 0.53),              // #808285 - Faint metadata
            text_on_accent: Color::from_rgb(1.0, 1.0, 1.0),             // #ffffff - Text on accent
            text_on_surface: Color::from_rgb(0.98, 0.98, 0.98),         // #fafafa - Text on surfaces
            text_disabled: Color::from_rgb(0.45, 0.46, 0.48),           // #737578 - Disabled text
            text_link: Color::from_rgb(0.40, 0.65, 1.0),                // #66a5ff - Brighter link
            
            // UI elements
            border: Color::from_rgb(0.20, 0.21, 0.23),                  // #33363b - Main borders
            border_subtle: Color::from_rgba(0.20, 0.21, 0.23, 0.4),     // Subtle borders
            divider: Color::from_rgb(0.20, 0.21, 0.23),                 // #33363b - Dividers
            accent: Color::from_rgb(0.40, 0.65, 1.0),                   // #66a5ff - Accent blue
            accent_hover: Color::from_rgb(0.50, 0.75, 1.0),             // #80bfff - Hover accent
            accent_soft: Color::from_rgba(0.40, 0.65, 1.0, 0.15),       // Soft accent
            accent_soft_background: Color::from_rgba(0.40, 0.65, 1.0, 0.08), // Very soft accent
            
            // States
            hover_background: Color::from_rgba(1.0, 1.0, 1.0, 0.08),    // Hover states
            active_background: Color::from_rgba(1.0, 1.0, 1.0, 0.12),   // Active states
            selected_background: Color::from_rgba(0.40, 0.65, 1.0, 0.20), // Selected with accent
            selected_text_background: Color::from_rgba(0.40, 0.65, 1.0, 0.30), // Text selection
            selected_editor_background: Color::from_rgba(0.40, 0.65, 1.0, 0.25), // Editor selection
            
            // Status colors
            success: Color::from_rgb(0.35, 0.85, 0.55),                 // #59d98f - Success green
            warning: Color::from_rgb(1.0, 0.75, 0.30),                  // #ffbf4d - Warning orange
            error: Color::from_rgb(1.0, 0.45, 0.45),                    // #ff7373 - Error red
            info: Color::from_rgb(0.40, 0.65, 1.0),                     // #66a5ff - Info blue
            
            // Focus
            focus_ring: Color::from_rgba(0.40, 0.65, 1.0, 0.35),        // Focus ring
            
            // Editor specific
            editor_gutter_background: Color::from_rgb(0.10, 0.11, 0.13), // #1a1c21 - Gutter matches panels
            editor_line_highlight: Color::from_rgba(1.0, 1.0, 1.0, 0.04), // Line highlight
            editor_cursor: Color::from_rgb(1.0, 1.0, 1.0),              // #ffffff - Bright cursor
            editor_selection: Color::from_rgba(0.40, 0.65, 1.0, 0.25),  // Editor selection
            editor_find_highlight: Color::from_rgba(1.0, 0.75, 0.30, 0.35), // Find highlight
            
            // Syntax colors
            syntax_keyword: Color::from_rgb(1.0, 0.45, 0.45),           // #ff7373 - Keywords
            syntax_function: Color::from_rgb(0.35, 0.85, 0.55),         // #59d98f - Functions
            syntax_string: Color::from_rgb(1.0, 0.75, 0.30),            // #ffbf4d - Strings
            syntax_comment: Color::from_rgb(0.55, 0.56, 0.58),          // #8c8e94 - Comments
            syntax_type: Color::from_rgb(0.40, 0.65, 1.0),              // #66a5ff - Types
            syntax_variable: Color::from_rgb(0.98, 0.98, 0.98),         // #fafafa - Variables
            syntax_constant: Color::from_rgb(1.0, 0.75, 0.30),          // #ffbf4d - Constants
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
