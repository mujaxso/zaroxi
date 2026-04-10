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
    
    // Also try to unset WAYLAND_DISPLAY to force X11
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        println!("DEBUG: WAYLAND_DISPLAY is set, trying to force X11");
    }
    
    // Force X11 explicitly
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    println!("DEBUG: Forced X11 backend");
    
    // Increase memory limits for large files
    // This might help with scrolling crashes
    let settings = Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1400.0, 900.0),
            min_size: Some(iced::Size::new(800.0, 600.0)),
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
