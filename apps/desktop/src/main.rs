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

use app::App;
use iced::{Application, Settings};

fn main() -> iced::Result {
    // Force X11 backend to avoid Wayland issues
    unsafe {
        std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    }
    
    // Increase memory limits for large files
    // This might help with scrolling crashes
    App::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1400.0, 900.0),
            min_size: Some(iced::Size::new(800.0, 600.0)),
            ..Default::default()
        },
        // Enable antialiasing for better text rendering
        antialiasing: true,
        default_font: iced::font::Font::MONOSPACE,
        default_text_size: iced::Pixels(14.0),
        ..Default::default()
    })
}
