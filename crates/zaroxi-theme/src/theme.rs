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
    /// Dark theme semantic colors - Professional IDE with blueish dark colors for better readability
    pub fn dark() -> Self {
        Self {
            // Background surfaces - Blueish dark theme for better contrast
            app_background: Color::from_rgb(0.09, 0.11, 0.16),           // #171c29 - Base with blue tint
            shell_background: Color::from_rgb(0.14, 0.14, 0.16),         // #242428 - Shell (neutral dark)
            panel_background: Color::from_rgb(0.13, 0.15, 0.20),         // #212632 - Panels
            elevated_panel_background: Color::from_rgb(0.15, 0.17, 0.22), // #262b38 - Elevated
            editor_background: Color::from_rgb(0.10, 0.12, 0.17),        // #1a1f2c - Editor
            input_background: Color::from_rgb(0.13, 0.15, 0.20),         // #212632 - Inputs
            status_bar_background: Color::from_rgb(0.14, 0.14, 0.16),    // #242428 - Neutral dark, not blue
            title_bar_background: Color::from_rgb(0.08, 0.10, 0.15),     // #141a25 - Title bar
            activity_rail_background: Color::from_rgb(0.10, 0.12, 0.17), // #1a1f2c - Activity rail
            sidebar_background: Color::from_rgb(0.13, 0.15, 0.20),       // #212632 - Sidebar
            tab_background: Color::from_rgb(0.13, 0.15, 0.20),           // #212632 - Inactive tabs
            tab_active_background: Color::from_rgb(0.10, 0.12, 0.17),    // #1a1f2c - Active tab matches editor
            assistant_panel_background: Color::from_rgb(0.13, 0.15, 0.20), // #212632 - Right panel
            
            // Text colors - Professional IDE with excellent contrast
            text_primary: Color::from_rgb(0.98, 0.98, 0.98),            // #fafafa - Primary text (brighter)
            text_secondary: Color::from_rgb(0.88, 0.90, 0.94),          // #e0e6f0 - Secondary (higher contrast)
            text_muted: Color::from_rgb(0.75, 0.78, 0.85),              // #bfc7d9 - Muted (still readable)
            text_faint: Color::from_rgb(0.60, 0.63, 0.70),              // #99a1b3 - Faint
            text_on_accent: Color::from_rgb(1.0, 1.0, 1.0),             // #ffffff - Text on accent
            text_on_surface: Color::from_rgb(0.95, 0.96, 0.98),         // #f2f5fa - Text on surfaces
            text_disabled: Color::from_rgb(0.45, 0.48, 0.55),           // #737a8c - Disabled
            text_link: Color::from_rgb(0.40, 0.70, 1.0),                // #66b3ff - Link blue
            
            // UI elements - More neutral/whitish borders
            border: Color::from_rgb(0.45, 0.47, 0.52),                  // #737885 - More neutral, less blue
            border_subtle: Color::from_rgba(0.45, 0.47, 0.52, 0.4),     // Subtle borders
            divider: Color::from_rgb(0.45, 0.47, 0.52),                 // #737885 - Dividers
            accent: Color::from_rgb(0.40, 0.70, 1.0),                   // #66b3ff - Accent blue
            accent_hover: Color::from_rgb(0.50, 0.80, 1.0),             // #80ccff - Hover accent
            accent_soft: Color::from_rgba(0.40, 0.70, 1.0, 0.15),       // Soft accent
            accent_soft_background: Color::from_rgba(0.40, 0.70, 1.0, 0.08), // Very soft accent
            
            // States
            hover_background: Color::from_rgba(1.0, 1.0, 1.0, 0.08),    // Hover states
            active_background: Color::from_rgba(1.0, 1.0, 1.0, 0.12),   // Active states
            selected_background: Color::from_rgba(0.40, 0.70, 1.0, 0.20), // Selected with accent
            selected_text_background: Color::from_rgba(0.40, 0.70, 1.0, 0.25), // Text selection
            selected_editor_background: Color::from_rgba(0.40, 0.70, 1.0, 0.20), // Editor selection
            
            // Status colors
            success: Color::from_rgb(0.40, 0.85, 0.60),                 // #66d999 - Success green
            warning: Color::from_rgb(1.0, 0.75, 0.40),                  // #ffbf66 - Warning orange
            error: Color::from_rgb(1.0, 0.55, 0.60),                    // #ff8c99 - Error red
            info: Color::from_rgb(0.40, 0.70, 1.0),                     // #66b3ff - Info blue
            
            // Focus
            focus_ring: Color::from_rgba(0.40, 0.70, 1.0, 0.30),        // Focus ring
            
            // Editor specific
            editor_gutter_background: Color::from_rgb(0.13, 0.15, 0.20), // #212632 - Gutter matches panels
            editor_line_highlight: Color::from_rgba(1.0, 1.0, 1.0, 0.04), // Line highlight
            editor_cursor: Color::from_rgb(0.95, 0.96, 0.98),           // #f2f5fa - Cursor
            editor_selection: Color::from_rgba(0.40, 0.70, 1.0, 0.25),  // Editor selection
            editor_find_highlight: Color::from_rgba(1.0, 0.75, 0.40, 0.30), // Find highlight
            
            // Syntax colors
            syntax_keyword: Color::from_rgb(1.0, 0.55, 0.60),           // #ff8c99 - Keywords
            syntax_function: Color::from_rgb(0.40, 0.85, 0.60),         // #66d999 - Functions
            syntax_string: Color::from_rgb(1.0, 0.75, 0.40),            // #ffbf66 - Strings
            syntax_comment: Color::from_rgb(0.65, 0.68, 0.75),          // #a6adbf - Comments
            syntax_type: Color::from_rgb(0.40, 0.70, 1.0),              // #66b3ff - Types
            syntax_variable: Color::from_rgb(0.95, 0.96, 0.98),         // #f2f5fa - Variables
            syntax_constant: Color::from_rgb(1.0, 0.75, 0.40),          // #ffbf66 - Constants
        }
    }
    
    /// Light theme semantic colors - Clean and consistent professional IDE
    pub fn light() -> Self {
        Self {
            // Background surfaces - All consistent light gray
            app_background: Color::from_rgb(0.98, 0.98, 0.98),           // #fafafa - Very light gray
            shell_background: Color::from_rgb(0.97, 0.97, 0.97),         // #f7f7f7 - Slightly darker
            panel_background: Color::from_rgb(0.98, 0.98, 0.98),         // #fafafa - Matches editor
            elevated_panel_background: Color::from_rgb(0.98, 0.98, 0.98), // #fafafa - Same
            editor_background: Color::from_rgb(0.98, 0.98, 0.98),        // #fafafa - Professional light gray
            input_background: Color::from_rgb(1.0, 1.0, 1.0),            // #ffffff - Inputs pure white
            status_bar_background: Color::from_rgb(0.97, 0.97, 0.97),    // #f7f7f7 - Status bar
            title_bar_background: Color::from_rgb(0.97, 0.97, 0.97),     // #f7f7f7 - Title bar
            activity_rail_background: Color::from_rgb(0.97, 0.97, 0.97), // #f7f7f7 - Activity rail
            sidebar_background: Color::from_rgb(0.98, 0.98, 0.98),       // #fafafa - Sidebar matches editor
            tab_background: Color::from_rgb(0.98, 0.98, 0.98),           // #fafafa - Inactive tabs
            tab_active_background: Color::from_rgb(0.98, 0.98, 0.98),    // #fafafa - Active tab matches editor
            assistant_panel_background: Color::from_rgb(0.98, 0.98, 0.98), // #fafafa - Right panel
            
            // Text colors - Better contrast with light background
            text_primary: Color::from_rgb(0.15, 0.15, 0.15),            // #262626 - Dark gray for primary
            text_secondary: Color::from_rgb(0.35, 0.35, 0.35),          // #595959 - Secondary
            text_muted: Color::from_rgb(0.55, 0.55, 0.55),              // #8c8c8c - Muted
            text_faint: Color::from_rgb(0.70, 0.70, 0.70),              // #b3b3b3 - Faint
            text_on_accent: Color::from_rgb(1.0, 1.0, 1.0),             // #ffffff - Text on accent
            text_on_surface: Color::from_rgb(0.15, 0.15, 0.15),         // #262626 - Text on surfaces
            text_disabled: Color::from_rgb(0.75, 0.75, 0.75),           // #bfbfbf - Disabled
            text_link: Color::from_rgb(0.10, 0.50, 0.90),               // #1a80e6 - Link blue
            
            // UI elements - Subtle borders
            border: Color::from_rgb(0.85, 0.85, 0.85),                  // #d9d9d9 - Light gray border
            border_subtle: Color::from_rgba(0.85, 0.85, 0.85, 0.5),     // Subtle borders
            divider: Color::from_rgb(0.85, 0.85, 0.85),                 // #d9d9d9 - Dividers
            accent: Color::from_rgb(0.10, 0.50, 0.90),                  // #1a80e6 - Accent blue
            accent_hover: Color::from_rgb(0.08, 0.45, 0.82),            // #1473d1 - Hover accent
            accent_soft: Color::from_rgba(0.10, 0.50, 0.90, 0.12),      // Soft accent
            accent_soft_background: Color::from_rgba(0.10, 0.50, 0.90, 0.06), // Very soft accent
            
            // States
            hover_background: Color::from_rgba(0.0, 0.0, 0.0, 0.04),    // Hover states
            active_background: Color::from_rgba(0.0, 0.0, 0.0, 0.08),   // Active states
            selected_background: Color::from_rgba(0.10, 0.50, 0.90, 0.12), // Selected
            selected_text_background: Color::from_rgba(0.10, 0.50, 0.90, 0.20), // Text selection
            selected_editor_background: Color::from_rgba(0.10, 0.50, 0.90, 0.15), // Editor selection
            
            // Status colors
            success: Color::from_rgb(0.20, 0.70, 0.40),                 // #33b366 - Success green
            warning: Color::from_rgb(0.90, 0.60, 0.10),                 // #e69900 - Warning orange
            error: Color::from_rgb(0.90, 0.30, 0.30),                   // #e64d4d - Error red
            info: Color::from_rgb(0.10, 0.50, 0.90),                    // #1a80e6 - Info blue
            
            // Focus
            focus_ring: Color::from_rgba(0.10, 0.50, 0.90, 0.25),       // Focus ring
            
            // Editor specific
            editor_gutter_background: Color::from_rgb(0.98, 0.98, 0.98), // #fafafa - Gutter matches editor
            editor_line_highlight: Color::from_rgba(0.0, 0.0, 0.0, 0.03), // Slightly darker line highlight
            editor_cursor: Color::from_rgb(0.15, 0.15, 0.15),           // #262626 - Cursor
            editor_selection: Color::from_rgba(0.10, 0.50, 0.90, 0.20), // Editor selection
            editor_find_highlight: Color::from_rgba(0.90, 0.60, 0.10, 0.25), // Find highlight
            
            // Syntax colors
            syntax_keyword: Color::from_rgb(0.90, 0.30, 0.30),          // #e64d4d - Keywords red
            syntax_function: Color::from_rgb(0.20, 0.70, 0.40),         // #33b366 - Functions green
            syntax_string: Color::from_rgb(0.90, 0.60, 0.10),           // #e69900 - Strings orange
            syntax_comment: Color::from_rgb(0.55, 0.55, 0.55),          // #8c8c8c - Comments gray
            syntax_type: Color::from_rgb(0.10, 0.50, 0.90),             // #1a80e6 - Types blue
            syntax_variable: Color::from_rgb(0.15, 0.15, 0.15),         // #262626 - Variables dark gray
            syntax_constant: Color::from_rgb(0.90, 0.60, 0.10),         // #e69900 - Constants orange
        }
    }
}
