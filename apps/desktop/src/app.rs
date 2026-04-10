// Re-export the main types from the crate root modules
pub use crate::message::Message;
pub use crate::state::App;
pub use crate::update::update;
pub use crate::view::view;

use iced::{Element, Command};
use crate::state::WorkbenchLayoutState;

impl iced::Application for App {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (mut app, command) = App::new();
        
        // Initialize workbench layout
        app.workbench_layout = WorkbenchLayoutState::default();
        
        // Load saved typography settings
        match crate::settings::persistence::load_settings() {
            Ok(settings) => {
                app.editor_typography = settings;
            }
            Err(e) => {
                eprintln!("Failed to load typography settings: {}", e);
                // Continue with defaults
            }
        }
        
        // Load custom fonts for icon support
        // We'll try to load multiple fonts to ensure icons are visible
        let mut font_commands = Vec::new();
        
        // Fonts to load in order of preference, prioritizing Nerd Font variants for icons
        let font_files = [
            // Icon fonts first - most important for icon rendering
            ("SymbolsNerdFont-Regular.ttf", "Symbols Nerd Font"),
            ("NotoColorEmoji.ttf", "Noto Color Emoji"),
            // Nerd Font variants for coding with icons
            ("JetBrainsMonoNerdFont-Regular.ttf", "JetBrainsMono Nerd Font"),
            ("FiraCodeNerdFont-Regular.ttf", "FiraCode Nerd Font"),
            ("CascadiaCodeNerdFont-Regular.ttf", "CaskaydiaCove Nerd Font"),
            ("IosevkaNerdFont-Regular.ttf", "Iosevka Nerd Font"),
            // Regular coding fonts as fallback
            ("JetBrainsMono-Regular.ttf", "JetBrains Mono"),
            ("FiraCode-Regular.ttf", "Fira Code"),
            ("CascadiaCode-Regular.ttf", "Cascadia Code"),
            ("Iosevka-Regular.ttf", "Iosevka"),
            ("SourceCodePro-Regular.ttf", "Source Code Pro"),
        ];
        
        // Try to load fonts from assets/fonts directory first
        let assets_font_dir = "apps/desktop/assets/fonts";
        if !std::path::Path::new(assets_font_dir).exists() {
            // Create the directory if it doesn't exist
            let _ = std::fs::create_dir_all(assets_font_dir);
        }
        
        for (file, name) in &font_files {
            // Try multiple locations
            let possible_paths = [
                format!("{}/{}", assets_font_dir, file),
                format!("assets/fonts/{}", file),
                format!("../assets/fonts/{}", file),
                file.to_string(),
            ];
            
            let mut loaded = false;
            for path in &possible_paths {
                if std::path::Path::new(path).exists() {
                    if let Ok(bytes) = std::fs::read(path) {
                        println!("DEBUG: Loading font: {} from {}", name, path);
                        font_commands.push(
                            iced::font::load(bytes)
                                .map(|_| Message::FontLoaded)
                        );
                        loaded = true;
                        break; // Load each font only once
                    }
                }
            }
            if !loaded {
                println!("DEBUG: Could not load font: {}", name);
            }
        }
        
        // Always load at least the default system fonts
        // Combine font loading commands with the initial app command
        let mut all_commands = font_commands;
        all_commands.push(command);
        (app, Command::batch(all_commands))
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
