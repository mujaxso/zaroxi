// Re-export the main types from the crate root modules
pub use crate::message::Message;
pub use crate::state::App;
pub use crate::update::update;
pub use crate::view::view;

use iced::{Element, Command};

impl iced::Application for App {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (app, command) = App::new();
        
        // Load custom fonts for icon support
        // We'll try to load multiple fonts to ensure icons are visible
        let mut font_commands = Vec::new();
        
        // Try to load programming fonts from various possible locations
        let possible_font_dirs = [
            "apps/desktop/assets/fonts",
            "assets/fonts",
            "../assets/fonts",
        ];
        
        // Fonts to load in order of preference
        let font_files = [
            ("JetBrainsMono-Regular.ttf", "JetBrains Mono"),
            ("FiraCode-Regular.ttf", "Fira Code"),
            ("NotoColorEmoji.ttf", "Noto Color Emoji"),
        ];
        
        for dir in &possible_font_dirs {
            for (file, name) in &font_files {
                let path = format!("{}/{}", dir, file);
                if std::path::Path::new(&path).exists() {
                    match std::fs::read(&path) {
                        Ok(bytes) => {
                            font_commands.push(
                                iced::font::load(bytes)
                                    .map(|_| Message::FontLoaded)
                                    .map_err(|_| Message::FontLoadFailed)
                            );
                            #[cfg(debug_assertions)]
                            eprintln!("Loaded font: {} from {}", name, path);
                        }
                        Err(e) => {
                            #[cfg(debug_assertions)]
                            eprintln!("Failed to load font {} from {}: {}", name, path, e);
                        }
                    }
                }
            }
        }
        
        // If no fonts found in standard locations, try current directory
        if font_commands.is_empty() {
            for (file, name) in &font_files {
                if std::path::Path::new(file).exists() {
                    match std::fs::read(file) {
                        Ok(bytes) => {
                            font_commands.push(
                                iced::font::load(bytes)
                                    .map(|_| Message::FontLoaded)
                                    .map_err(|_| Message::FontLoadFailed)
                            );
                            #[cfg(debug_assertions)]
                            eprintln!("Loaded font: {} from current directory", name);
                        }
                        Err(e) => {
                            #[cfg(debug_assertions)]
                            eprintln!("Failed to load font {} from current directory: {}", name, e);
                        }
                    }
                }
            }
        }
        
        if font_commands.is_empty() {
            #[cfg(debug_assertions)]
            eprintln!("Warning: No custom fonts loaded. Icons may not display correctly.");
            #[cfg(debug_assertions)]
            eprintln!("Run from apps/desktop directory: ./scripts/download-fonts.sh");
            #[cfg(debug_assertions)]
            eprintln!("This will download JetBrains Mono, Fira Code, and emoji fonts.");
        }
        
        // If no fonts were loaded, we'll just use system fonts
        if font_commands.is_empty() {
            #[cfg(debug_assertions)]
            eprintln!("No custom fonts loaded. Run `scripts/download-fonts.sh` to download required fonts.");
            (app, command)
        } else {
            // Combine font loading commands with the initial app command
            let mut all_commands = font_commands;
            all_commands.push(command);
            (app, Command::batch(all_commands))
        }
    }

    fn title(&self) -> String {
        String::from("Neote")
    }

    fn theme(&self) -> iced::Theme {
        self.theme.to_iced_theme()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        update(self, message)
    }

    fn view(&self) -> Element<'_, Message> {
        view(self)
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        self.subscription()
    }
}
