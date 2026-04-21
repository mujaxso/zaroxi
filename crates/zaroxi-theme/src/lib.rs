//! Theme and design system for Zaroxi
//! This crate provides color themes, design tokens, and styling utilities

mod colors;
mod theme;
mod manager;

pub use colors::*;
pub use theme::{ZaroxiTheme, SemanticColors, DesignTokens};
pub use manager::{ThemeManager, ThemeSettings};
