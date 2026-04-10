mod app;
mod bootstrap;
mod commands;
mod ui;
mod events;
mod message;
mod state;
mod theme;
mod update;
mod view;
mod explorer;
mod settings;

use app::App;
use iced::{Application, Settings};

fn main() -> iced::Result {
    println!("Starting Neote...");
    
    // Check environment
    println!("DEBUG: WAYLAND_DISPLAY = {:?}", std::env::var("WAYLAND_DISPLAY"));
    println!("DEBUG: XDG_SESSION_TYPE = {:?}", std::env::var("XDG_SESSION_TYPE"));
    println!("DEBUG: WINIT_UNIX_BACKEND = {:?}", std::env::var("WINIT_UNIX_BACKEND"));
    
    // Check if we're on Wayland
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        println!("DEBUG: WAYLAND_DISPLAY is set - using Wayland backend");
    }
    
    // Don't force any backend - let winit choose
    // On Wayland, this should work better
    println!("DEBUG: Not forcing any backend");
    
    // Increase memory limits for large files
    // This might help with scrolling crashes
    let settings = Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1200.0, 800.0),
            min_size: Some(iced::Size::new(400.0, 300.0)),
            visible: true,
            position: iced::window::Position::Centered,
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        // Enable antialiasing for better text rendering
        antialiasing: true,
        // Use JetBrains Mono Nerd Font as the default font for better programming experience with icons
        default_font: iced::font::Font::with_name("JetBrainsMono Nerd Font"),
        default_text_size: iced::Pixels(14.0),
        ..Default::default()
    };
    
    println!("Running App::run...");
    let result = App::run(settings);
    println!("App::run returned: {:?}", result);
    result
}
