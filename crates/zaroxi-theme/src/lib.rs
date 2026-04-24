//! Theme and design system for Zaroxi
//! This crate provides color themes, design tokens, and styling utilities

mod colors;
mod manager;
mod theme;

pub use colors::*;
pub use manager::{ThemeManager, ThemeSettings};
pub use theme::{DesignTokens, SemanticColors, ZaroxiTheme};
