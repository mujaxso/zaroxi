//! Editor typography settings model.
//!
//! Defines the structure and validation rules for editor font settings,
//! including font family, size, line height, ligature support, and icon capabilities.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Available monospace font families optimized for coding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontFamily {
    JetBrainsMono,
    FiraCode,
    CascadiaCode,
    Iosevka,
    SourceCodePro,
    // Nerd Font variants
    JetBrainsMonoNerd,
    FiraCodeNerd,
    CascadiaCodeNerd,
    IosevkaNerd,
}

impl fmt::Display for FontFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontFamily::JetBrainsMono => write!(f, "JetBrains Mono"),
            FontFamily::FiraCode => write!(f, "Fira Code"),
            FontFamily::CascadiaCode => write!(f, "Cascadia Code"),
            FontFamily::Iosevka => write!(f, "Iosevka"),
            FontFamily::SourceCodePro => write!(f, "Source Code Pro"),
            FontFamily::JetBrainsMonoNerd => write!(f, "JetBrains Mono Nerd Font"),
            FontFamily::FiraCodeNerd => write!(f, "Fira Code Nerd Font"),
            FontFamily::CascadiaCodeNerd => write!(f, "Cascadia Code Nerd Font"),
            FontFamily::IosevkaNerd => write!(f, "Iosevka Nerd Font"),
        }
    }
}

impl FontFamily {
    /// Get the CSS/iced font family string for this font.
    pub fn to_family_string(&self) -> &'static str {
        match self {
            FontFamily::JetBrainsMono => "JetBrains Mono",
            FontFamily::FiraCode => "Fira Code",
            FontFamily::CascadiaCode => "Cascadia Code",
            FontFamily::Iosevka => "Iosevka",
            FontFamily::SourceCodePro => "Source Code Pro",
            FontFamily::JetBrainsMonoNerd => "JetBrainsMono Nerd Font",
            FontFamily::FiraCodeNerd => "FiraCode Nerd Font",
            FontFamily::CascadiaCodeNerd => "CascadiaCode Nerd Font",
            FontFamily::IosevkaNerd => "Iosevka Nerd Font",
        }
    }

    /// Get the fallback font stack for this font family.
    pub fn fallback_stack(&self) -> Vec<&'static str> {
        match self {
            FontFamily::JetBrainsMono | FontFamily::JetBrainsMonoNerd => vec![
                "JetBrainsMono Nerd Font",
                "JetBrains Mono",
                "Fira Code",
                "Cascadia Code",
                "monospace",
            ],
            FontFamily::FiraCode | FontFamily::FiraCodeNerd => vec![
                "FiraCode Nerd Font",
                "Fira Code",
                "JetBrains Mono",
                "Cascadia Code",
                "monospace",
            ],
            FontFamily::CascadiaCode | FontFamily::CascadiaCodeNerd => vec![
                "CascadiaCode Nerd Font",
                "Cascadia Code",
                "JetBrains Mono",
                "Fira Code",
                "monospace",
            ],
            FontFamily::Iosevka | FontFamily::IosevkaNerd => vec![
                "Iosevka Nerd Font",
                "Iosevka",
                "JetBrains Mono",
                "Fira Code",
                "monospace",
            ],
            FontFamily::SourceCodePro => vec![
                "Source Code Pro",
                "JetBrains Mono",
                "Fira Code",
                "Cascadia Code",
                "monospace",
            ],
        }
    }

    /// Get the icon fallback stack for this font family.
    /// This is optimized for rendering developer glyphs and icons.
    pub fn icon_fallback_stack(&self) -> Vec<&'static str> {
        let mut stack = Vec::new();
        
        // Always include Nerd Fonts first for icon support
        stack.push("Symbols Nerd Font");
        stack.push("Noto Color Emoji");
        
        // Add the primary font if it's a Nerd Font variant
        match self {
            FontFamily::JetBrainsMonoNerd => stack.push("JetBrainsMono Nerd Font"),
            FontFamily::FiraCodeNerd => stack.push("FiraCode Nerd Font"),
            FontFamily::CascadiaCodeNerd => stack.push("CascadiaCode Nerd Font"),
            FontFamily::IosevkaNerd => stack.push("Iosevka Nerd Font"),
            _ => {}
        }
        
        // Add standard coding fonts
        stack.extend(self.fallback_stack());
        stack
    }

    /// Check if this font family is a Nerd Font variant
    pub fn is_nerd_font(&self) -> bool {
        matches!(
            self,
            FontFamily::JetBrainsMonoNerd |
            FontFamily::FiraCodeNerd |
            FontFamily::CascadiaCodeNerd |
            FontFamily::IosevkaNerd
        )
    }

    /// Get all available font families.
    pub fn all() -> Vec<FontFamily> {
        vec![
            FontFamily::JetBrainsMono,
            FontFamily::FiraCode,
            FontFamily::CascadiaCode,
            FontFamily::Iosevka,
            FontFamily::SourceCodePro,
            FontFamily::JetBrainsMonoNerd,
            FontFamily::FiraCodeNerd,
            FontFamily::CascadiaCodeNerd,
            FontFamily::IosevkaNerd,
        ]
    }

    /// Get all Nerd Font variants
    pub fn nerd_fonts() -> Vec<FontFamily> {
        vec![
            FontFamily::JetBrainsMonoNerd,
            FontFamily::FiraCodeNerd,
            FontFamily::CascadiaCodeNerd,
            FontFamily::IosevkaNerd,
        ]
    }
}

impl Default for FontFamily {
    fn default() -> Self {
        FontFamily::JetBrainsMono
    }
}

/// Icon rendering mode for developer glyphs and UI icons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IconMode {
    /// Use Nerd Font glyphs when available (recommended for developers)
    NerdFonts,
    /// Use Unicode fallback symbols (more compatible)
    Unicode,
    /// Disable icons entirely
    Disabled,
}

impl Default for IconMode {
    fn default() -> Self {
        // Temporarily use Unicode to debug icon rendering
        IconMode::Unicode
    }
}

impl fmt::Display for IconMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IconMode::NerdFonts => write!(f, "Nerd Fonts"),
            IconMode::Unicode => write!(f, "Unicode"),
            IconMode::Disabled => write!(f, "Disabled"),
        }
    }
}

/// Editor typography settings optimized for coding readability with icon support.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorTypographySettings {
    /// Selected font family
    pub font_family: FontFamily,
    /// Font size in pixels
    pub font_size: u16,
    /// Line height multiplier (e.g., 1.5)
    pub line_height: f32,
    /// Whether ligatures are enabled (for fonts that support them)
    pub ligatures_enabled: bool,
    /// Letter spacing in pixels (can be negative or positive)
    pub letter_spacing: f32,
    /// Icon rendering mode
    pub icon_mode: IconMode,
    /// Whether to prefer Nerd Font variants when available
    pub prefer_nerd_fonts: bool,
}

impl Default for EditorTypographySettings {
    fn default() -> Self {
        Self {
            font_family: FontFamily::default(),
            font_size: 14, // Optimal for coding readability
            line_height: 1.6, // Balanced for scanning code
            ligatures_enabled: false, // Off by default for clarity
            letter_spacing: 0.0, // Monospace fonts typically don't need extra spacing
            icon_mode: IconMode::default(),
            prefer_nerd_fonts: true,
        }
    }
}

impl EditorTypographySettings {
    /// Create new settings with validation
    pub fn new(
        font_family: FontFamily,
        font_size: u16,
        line_height: f32,
        ligatures_enabled: bool,
        letter_spacing: f32,
        icon_mode: IconMode,
        prefer_nerd_fonts: bool,
    ) -> Self {
        let font_size = font_size.clamp(10, 24);
        let line_height = line_height.clamp(1.2, 2.0);
        let letter_spacing = letter_spacing.clamp(-0.2, 0.2);
        
        Self {
            font_family,
            font_size,
            line_height,
            ligatures_enabled,
            letter_spacing,
            icon_mode,
            prefer_nerd_fonts,
        }
    }

    /// Increase font size (zoom in)
    pub fn zoom_in(&mut self) {
        self.font_size = (self.font_size + 1).clamp(10, 24);
    }

    /// Decrease font size (zoom out)
    pub fn zoom_out(&mut self) {
        self.font_size = (self.font_size.saturating_sub(1)).clamp(10, 24);
    }

    /// Reset font size to default
    pub fn reset_zoom(&mut self) {
        self.font_size = 14;
    }

    /// Reset all settings to defaults
    pub fn reset_to_defaults(&mut self) {
        *self = Self::default();
    }

    /// Validate settings are within reasonable bounds
    pub fn validate(&mut self) {
        self.font_size = self.font_size.clamp(10, 24);
        self.line_height = self.line_height.clamp(1.2, 2.0);
        self.letter_spacing = self.letter_spacing.clamp(-0.2, 0.2);
    }

    /// Get the effective line height in pixels
    pub fn line_height_pixels(&self) -> f32 {
        self.font_size as f32 * self.line_height
    }

    /// Get the appropriate font stack for text rendering
    pub fn text_font_stack(&self) -> Vec<&'static str> {
        self.font_family.fallback_stack()
    }

    /// Get the appropriate font stack for icon rendering
    pub fn icon_font_stack(&self) -> Vec<&'static str> {
        if self.icon_mode == IconMode::Disabled {
            return vec!["monospace"];
        }
        
        let mut stack = Vec::new();
        
        match self.icon_mode {
            IconMode::NerdFonts => {
                // Use the exact names from font loading in app.rs
                // These must match exactly
                stack.push("Symbols Nerd Font");
                stack.push("Noto Color Emoji");
                stack.push("JetBrainsMono Nerd Font");
                stack.push("FiraCode Nerd Font");
                stack.push("CascadiaCode Nerd Font");
                stack.push("Iosevka Nerd Font");
                
                // Add the selected Nerd Font variant if applicable
                if self.font_family.is_nerd_font() {
                    stack.push(self.font_family.to_family_string());
                }
                
                // Add standard fallbacks
                stack.extend(self.font_family.fallback_stack());
            }
            IconMode::Unicode => {
                // For Unicode mode, prioritize emoji fonts
                stack.push("Noto Color Emoji");
                stack.push("Segoe UI Emoji");
                stack.push("Apple Color Emoji");
                stack.extend(self.font_family.fallback_stack());
            }
            IconMode::Disabled => {
                stack.push("monospace");
            }
        }
        
        // Remove duplicates while preserving order
        let mut deduped = Vec::new();
        for font in stack {
            if !deduped.contains(&font) {
                deduped.push(font);
            }
        }
        
        // Ensure we always have at least one font
        if deduped.is_empty() {
            deduped.push("monospace");
        }
        
        deduped
    }

    /// Check if icons are enabled
    pub fn icons_enabled(&self) -> bool {
        self.icon_mode != IconMode::Disabled
    }
}
