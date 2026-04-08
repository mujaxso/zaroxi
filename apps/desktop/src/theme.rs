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

/// Semantic color roles for Neote IDE
#[derive(Debug, Clone, Copy)]
pub struct SemanticColors {
    // Background surfaces
    pub app_background: Color,
    pub panel_background: Color,
    pub elevated_panel_background: Color,
    pub status_bar_background: Color,
    
    // Text colors
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub text_on_accent: Color,
    
    // UI elements
    pub border: Color,
    pub divider: Color,
    pub accent: Color,
    pub accent_secondary: Color,
    
    // States
    pub hover_background: Color,
    pub active_background: Color,
    pub selected_background: Color,
    
    // Status colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
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
    /// Dark theme semantic colors
    pub fn dark() -> Self {
        Self {
            app_background: Color::from_rgb(0.08, 0.08, 0.10),
            panel_background: Color::from_rgb(0.12, 0.12, 0.14),
            elevated_panel_background: Color::from_rgb(0.16, 0.16, 0.18),
            status_bar_background: Color::from_rgb(0.10, 0.10, 0.12),
            
            text_primary: Color::from_rgb(0.95, 0.95, 0.95),
            text_secondary: Color::from_rgb(0.75, 0.75, 0.75),
            text_muted: Color::from_rgb(0.55, 0.55, 0.55),
            text_on_accent: Color::from_rgb(1.0, 1.0, 1.0),
            
            border: Color::from_rgb(0.25, 0.25, 0.30),
            divider: Color::from_rgb(0.20, 0.20, 0.25),
            accent: Color::from_rgb(0.30, 0.55, 0.95),
            accent_secondary: Color::from_rgb(0.40, 0.65, 1.0),
            
            hover_background: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            active_background: Color::from_rgba(1.0, 1.0, 1.0, 0.12),
            selected_background: Color::from_rgba(0.30, 0.55, 0.95, 0.20),
            
            success: Color::from_rgb(0.30, 0.85, 0.50),
            warning: Color::from_rgb(1.0, 0.75, 0.30),
            error: Color::from_rgb(1.0, 0.40, 0.40),
            info: Color::from_rgb(0.40, 0.75, 1.0),
        }
    }
    
    /// Light theme semantic colors
    pub fn light() -> Self {
        Self {
            app_background: Color::from_rgb(0.98, 0.98, 0.98),
            panel_background: Color::from_rgb(0.96, 0.96, 0.96),
            elevated_panel_background: Color::from_rgb(1.0, 1.0, 1.0),
            status_bar_background: Color::from_rgb(0.94, 0.94, 0.94),
            
            text_primary: Color::from_rgb(0.15, 0.15, 0.15),
            text_secondary: Color::from_rgb(0.35, 0.35, 0.35),
            text_muted: Color::from_rgb(0.55, 0.55, 0.55),
            text_on_accent: Color::from_rgb(1.0, 1.0, 1.0),
            
            border: Color::from_rgb(0.85, 0.85, 0.85),
            divider: Color::from_rgb(0.90, 0.90, 0.90),
            accent: Color::from_rgb(0.20, 0.50, 0.90),
            accent_secondary: Color::from_rgb(0.30, 0.60, 1.0),
            
            hover_background: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
            active_background: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            selected_background: Color::from_rgba(0.20, 0.50, 0.90, 0.15),
            
            success: Color::from_rgb(0.20, 0.75, 0.40),
            warning: Color::from_rgb(0.95, 0.65, 0.20),
            error: Color::from_rgb(0.95, 0.30, 0.30),
            info: Color::from_rgb(0.30, 0.65, 0.95),
        }
    }
}
