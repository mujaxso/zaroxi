mod app;
mod bootstrap;
mod commands;
mod ui;
mod events;

use app::App;
use iced::{Application, Settings};

fn main() -> iced::Result {
    // Force X11 backend to avoid Wayland issues
    unsafe {
        std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    }
    
    App::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1200.0, 800.0),
            ..Default::default()
        },
        ..Default::default()
    })
}
