use iced::{Color, Theme};

/// Design system tokens for Neote IDE
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

/// Helper to get current theme colors from app state
pub fn current_colors(theme: NeoteTheme) -> SemanticColors {
    theme.colors()
}

/// Semantic color roles for Neote IDE - Premium dark theme
#[derive(Debug, Clone, Copy)]
pub struct SemanticColors {
    // Background surfaces - Premium dark palette
    pub app_background: Color,           // #161821
    pub shell_background: Color,         // #1B1D27
    pub panel_background: Color,         // #1E2130
    pub elevated_panel_background: Color, // #232637
    pub editor_background: Color,        // #171923
    pub input_background: Color,         // #141722
    pub status_bar_background: Color,    // #1B1D27
    
    // Text colors
    pub text_primary: Color,             // #E6EAF2
    pub text_secondary: Color,           // #B7C0D1
    pub text_muted: Color,               // #8892A6
    pub text_faint: Color,               // #687086
    pub text_on_accent: Color,           // #FFFFFF
    
    // UI elements
    pub border: Color,                   // #2B3040
    pub divider: Color,                  // #2B3040
    pub accent: Color,                   // #4C6FFF
    pub accent_hover: Color,             // #5A7BFF
    pub accent_soft_background: Color,   // rgba(76, 111, 255, 0.16)
    
    // States
    pub hover_background: Color,         // #2A2E42
    pub active_background: Color,        // #2A2E42
    pub selected_background: Color,      // #2D3A73
    
    // Status colors
    pub success: Color,                  // #35C46B
    pub warning: Color,                  // #F0B24B
    pub error: Color,                    // #F05D6C
    pub info: Color,                     // #6EA8FF
    
    // Focus
    pub focus_ring: Color,               // rgba(92, 122, 255, 0.45)
}

/// Theme variants for Neote
#[derive(Debug, Clone, Copy)]
pub enum NeoteTheme {
    Dark,
    Light,
    System,
}

impl NeoteTheme {
    /// Get the semantic colors for this theme
    pub fn colors(&self) -> SemanticColors {
        match self {
            NeoteTheme::Dark => SemanticColors::dark(),
            NeoteTheme::Light => SemanticColors::light(),
            NeoteTheme::System => {
                // For now, default to dark theme
                // In a real implementation, we'd detect system preference
                SemanticColors::dark()
            }
        }
    }
    
    /// Convert to iced::Theme
    pub fn to_iced_theme(&self) -> Theme {
        match self {
            NeoteTheme::Dark => Theme::Dark,
            NeoteTheme::Light => Theme::Light,
            NeoteTheme::System => {
                // Default to dark for system
                Theme::Dark
            }
        }
    }
}

impl SemanticColors {
    /// Premium dark theme semantic colors
    pub fn dark() -> Self {
        Self {
            // Background surfaces
            app_background: Color::from_rgb(0.086, 0.094, 0.129),      // #161821
            shell_background: Color::from_rgb(0.106, 0.114, 0.153),    // #1B1D27
            panel_background: Color::from_rgb(0.118, 0.129, 0.188),    // #1E2130
            elevated_panel_background: Color::from_rgb(0.137, 0.149, 0.216), // #232637
            editor_background: Color::from_rgb(0.125, 0.133, 0.169),   // #20242B - slightly lighter than shell for distinction
            input_background: Color::from_rgb(0.078, 0.090, 0.133),    // #141722
            status_bar_background: Color::from_rgb(0.106, 0.114, 0.153), // #1B1D27
            
            // Text colors
            text_primary: Color::from_rgb(0.902, 0.918, 0.949),        // #E6EAF2
            text_secondary: Color::from_rgb(0.718, 0.753, 0.820),      // #B7C0D1
            text_muted: Color::from_rgb(0.533, 0.573, 0.651),          // #8892A6
            text_faint: Color::from_rgb(0.408, 0.439, 0.525),          // #687086
            text_on_accent: Color::from_rgb(1.0, 1.0, 1.0),            // #FFFFFF
            
            // UI elements
            border: Color::from_rgb(0.169, 0.188, 0.251),              // #2B3040 - subtle but visible
            divider: Color::from_rgb(0.169, 0.188, 0.251),             // #2B3040 - subtle but visible
            accent: Color::from_rgb(0.298, 0.435, 1.0),                // #4C6FFF
            accent_hover: Color::from_rgb(0.353, 0.482, 1.0),          // #5A7BFF
            accent_soft_background: Color::from_rgba(0.298, 0.435, 1.0, 0.16), // rgba(76, 111, 255, 0.16)
            
            // States
            hover_background: Color::from_rgb(0.165, 0.180, 0.259),    // #2A2E42
            active_background: Color::from_rgb(0.165, 0.180, 0.259),   // #2A2E42
            selected_background: Color::from_rgb(0.176, 0.227, 0.451), // #2D3A73
            
            // Status colors
            success: Color::from_rgb(0.208, 0.769, 0.420),             // #35C46B
            warning: Color::from_rgb(0.941, 0.698, 0.294),             // #F0B24B
            error: Color::from_rgb(0.941, 0.365, 0.424),               // #F05D6C
            info: Color::from_rgb(0.431, 0.659, 1.0),                  // #6EA8FF
            
            // Focus
            focus_ring: Color::from_rgba(0.361, 0.478, 1.0, 0.45),     // rgba(92, 122, 255, 0.45)
        }
    }
    
    /// Light theme semantic colors - Keeping for compatibility
    pub fn light() -> Self {
        Self {
            // Background surfaces
            app_background: Color::from_rgb(0.96, 0.96, 0.96),
            shell_background: Color::from_rgb(0.98, 0.98, 0.98),
            panel_background: Color::from_rgb(1.0, 1.0, 1.0),
            elevated_panel_background: Color::from_rgb(1.0, 1.0, 1.0),
            editor_background: Color::from_rgb(1.0, 1.0, 1.0),  // Pure white for better contrast in light mode
            input_background: Color::from_rgb(0.95, 0.95, 0.95),
            status_bar_background: Color::from_rgb(0.94, 0.94, 0.94),
            
            // Text colors
            text_primary: Color::from_rgb(0.10, 0.10, 0.10),
            text_secondary: Color::from_rgb(0.30, 0.30, 0.30),
            text_muted: Color::from_rgb(0.50, 0.50, 0.50),
            text_faint: Color::from_rgb(0.70, 0.70, 0.70),
            text_on_accent: Color::from_rgb(1.0, 1.0, 1.0),
            
            // UI elements
            border: Color::from_rgb(0.80, 0.80, 0.80),
            divider: Color::from_rgb(0.85, 0.85, 0.85),
            accent: Color::from_rgb(0.25, 0.55, 0.95),
            accent_hover: Color::from_rgb(0.35, 0.65, 1.0),
            accent_soft_background: Color::from_rgba(0.25, 0.55, 0.95, 0.16),
            
            // States
            hover_background: Color::from_rgb(0.92, 0.92, 0.94),
            active_background: Color::from_rgb(0.88, 0.88, 0.90),
            selected_background: Color::from_rgb(0.20, 0.50, 0.90),
            
            // Status colors
            success: Color::from_rgb(0.25, 0.80, 0.45),
            warning: Color::from_rgb(0.95, 0.70, 0.25),
            error: Color::from_rgb(0.95, 0.35, 0.35),
            info: Color::from_rgb(0.35, 0.70, 0.95),
            
            // Focus
            focus_ring: Color::from_rgba(0.25, 0.55, 0.95, 0.45),
        }
    }
}
