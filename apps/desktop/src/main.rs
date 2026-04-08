mod app;
mod bootstrap;
mod commands;
mod ui;
mod events;
mod message;
mod state;
mod update;
mod view;

use app::App;
use iced::Settings;

fn main() -> iced::Result {
    // Force X11 backend to avoid Wayland issues
    unsafe {
        std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    }
    
    // Increase memory limits for large files
    // This might help with scrolling crashes
    App::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1200.0, 800.0),
            ..Default::default()
        },
        // Enable antialiasing for better text rendering
        antialiasing: true,
        ..Default::default()
    })
}
