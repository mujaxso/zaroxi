//! Semantic icon system for Neote IDE.
//!
//! Provides a clean, maintainable way to use icons throughout the UI,
//! with support for developer glyphs, Nerd Fonts, and graceful fallbacks.

use iced::widget::text;
use iced::{Color, Element};

use crate::settings::editor::{EditorTypographySettings, IconMode};
use crate::ui::style::StyleHelpers;

/// Semantic icon identifiers for the IDE.
/// These represent UI concepts rather than specific glyphs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    // File system
    File,
    Folder,
    FolderOpen,
    
    // Navigation
    ChevronRight,
    ChevronDown,
    ArrowRight,
    ArrowLeft,
    
    // Actions
    Search,
    Settings,
    Refresh,
    Add,
    Edit,
    Delete,
    Save,
    Close,
    
    // Version control
    Git,
    GitBranch,
    GitCommit,
    GitPullRequest,
    
    // Development
    Terminal,
    Debug,
    Run,
    Build,
    
    // Status
    Success,
    Warning,
    Error,
    Info,
    
    // AI/Assistant
    Robot,
    Sparkles,
    
    // UI elements
    Menu,
    MoreHorizontal,
    MoreVertical,
}

impl Icon {
    /// Get the Unicode fallback character for this icon.
    /// Used when Nerd Fonts are not available.
    pub fn unicode_fallback(&self) -> &'static str {
        match self {
            // File system
            Icon::File => "📄",
            Icon::Folder => "📁",
            Icon::FolderOpen => "📂",
            
            // Navigation
            Icon::ChevronRight => "›",
            Icon::ChevronDown => "⌄",
            Icon::ArrowRight => "→",
            Icon::ArrowLeft => "←",
            
            // Actions
            Icon::Search => "🔍",
            Icon::Settings => "⚙",
            Icon::Refresh => "↻",
            Icon::Add => "+",
            Icon::Edit => "✎",
            Icon::Delete => "🗑",
            Icon::Save => "💾",
            Icon::Close => "×",
            
            // Version control
            Icon::Git => "🔄",
            Icon::GitBranch => "",
            Icon::GitCommit => "●",
            Icon::GitPullRequest => "",
            
            // Development
            Icon::Terminal => ">_",
            Icon::Debug => "🐛",
            Icon::Run => "▶",
            Icon::Build => "🔨",
            
            // Status
            Icon::Success => "✓",
            Icon::Warning => "⚠",
            Icon::Error => "✗",
            Icon::Info => "ℹ",
            
            // AI/Assistant - Use a more reliable emoji
            Icon::Robot => "🤖",       // Unicode robot emoji fallback
            Icon::Sparkles => "✨",
            
            // UI elements
            Icon::Menu => "☰",
            Icon::MoreHorizontal => "⋯",
            Icon::MoreVertical => "⋮",
        }
    }

    /// Get the Nerd Font glyph for this icon.
    /// Returns the appropriate Unicode code point for Nerd Fonts.
    pub fn nerd_font_glyph(&self) -> &'static str {
        match self {
            // File system - using Font Awesome icons which are widely supported
            Icon::File => "",        // nf-fa-file_o
            Icon::Folder => "",      // nf-fa-folder
            Icon::FolderOpen => "",  // nf-fa-folder_open
            
            // Navigation
            Icon::ChevronRight => "", // nf-fa-chevron_right
            Icon::ChevronDown => "",  // nf-fa-chevron_down
            Icon::ArrowRight => "",   // nf-fa-arrow_right
            Icon::ArrowLeft => "",    // nf-fa-arrow_left
            
            // Actions
            Icon::Search => "",       // nf-fa-search
            Icon::Settings => "",     // nf-fa-cog
            Icon::Refresh => "",      // nf-fa-refresh
            Icon::Add => "",          // nf-fa-plus_circle
            Icon::Edit => "",         // nf-fa-edit
            Icon::Delete => "",       // nf-fa-trash_o
            Icon::Save => "",         // nf-fa-save
            Icon::Close => "",        // nf-fa-times
            
            // Version control - using nf-dev icons which are in Symbols Nerd Font
            Icon::Git => "",         // nf-dev-git
            Icon::GitBranch => "",   // nf-dev-git_branch
            Icon::GitCommit => "",   // nf-dev-git_commit
            Icon::GitPullRequest => "", // nf-dev-git_pull_request
            
            // Development
            Icon::Terminal => "",     // nf-dev-terminal
            Icon::Debug => "",       // nf-fa-bug
            Icon::Run => "",         // nf-fa-play
            Icon::Build => "",       // nf-fa-wrench
            
            // Status
            Icon::Success => "",     // nf-fa-check_circle
            Icon::Warning => "",     // nf-fa-exclamation_triangle
            Icon::Error => "",       // nf-fa-times_circle
            Icon::Info => "",        // nf-fa-info_circle
            
            // AI/Assistant - Use a more reliable glyph
            // Try nf-fa-robot (f544) which is more standard
            Icon::Robot => "🤖",       // Use Unicode emoji as fallback for now
            Icon::Sparkles => "✨",    // Unicode fallback
            
            // UI elements
            Icon::Menu => "",        // nf-fa-bars
            Icon::MoreHorizontal => "", // nf-fa-ellipsis_h
            Icon::MoreVertical => "", // nf-fa-ellipsis_v
        }
    }


    /// Render this icon as a text element with appropriate styling.
    pub fn render<'a, Message>(
        &self,
        typography: &EditorTypographySettings,
        style: &StyleHelpers,
        size: Option<u16>,
    ) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let icon_size = size.unwrap_or(typography.font_size);
        
        // For the Robot icon, always use a reliable approach
        if matches!(self, Icon::Robot) {
            // Use a system font that definitely supports emoji
            return text("🤖")
                .size(icon_size)
                .style(iced::theme::Text::Color(style.text_secondary()))
                .into();
        }
        
        // For other icons, use the normal logic
        let (icon_char, font_name) = match typography.icon_mode {
            IconMode::NerdFonts => (self.nerd_font_glyph(), typography.font_family.to_family_string()),
            IconMode::Unicode => (self.unicode_fallback(), typography.font_family.to_family_string()),
            IconMode::Disabled => (" ", typography.font_family.to_family_string()),
        };
        
        let font = iced::Font::with_name(font_name);
        
        text(icon_char)
            .size(icon_size)
            .font(font)
            .style(iced::theme::Text::Color(style.text_secondary()))
            .into()
    }

    /// Render this icon as a text element with custom color.
    pub fn render_with_color<'a, Message>(
        &self,
        typography: &EditorTypographySettings,
        color: Color,
        size: Option<u16>,
    ) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let icon_size = size.unwrap_or(typography.font_size);
        
        // For the Robot icon, always use a reliable approach
        if matches!(self, Icon::Robot) {
            // Use a system font that definitely supports emoji
            return text("🤖")
                .size(icon_size)
                .style(iced::theme::Text::Color(color))
                .into();
        }
        
        // For other icons, use the normal logic
        let (icon_char, font_name) = match typography.icon_mode {
            IconMode::NerdFonts => (self.nerd_font_glyph(), typography.font_family.to_family_string()),
            IconMode::Unicode => (self.unicode_fallback(), typography.font_family.to_family_string()),
            IconMode::Disabled => (" ", typography.font_family.to_family_string()),
        };
        
        let font = iced::Font::with_name(font_name);
        
        text(icon_char)
            .size(icon_size)
            .font(font)
            .style(iced::theme::Text::Color(color))
            .into()
    }
}

/// Helper to create an icon button with consistent styling.
pub fn icon_button<'a, Message>(
    icon: Icon,
    typography: &EditorTypographySettings,
    style: &StyleHelpers,
    on_press: Option<Message>,
    size: Option<u16>,
) -> iced::widget::Button<'a, Message>
where
    Message: Clone + 'a,
{
    let button = iced::widget::button(
        icon.render(typography, style, size)
    );

    if let Some(message) = on_press {
        button.on_press(message)
    } else {
        button
    }
    .style(iced::theme::Button::Text)
    .padding(4)
}
