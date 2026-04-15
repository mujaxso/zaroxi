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
            Ok((typography, theme_preference)) => {
                app.editor_typography = typography;
                app.theme_preference = theme_preference;
                app.update_current_theme();
            }
            Err(e) => {
                eprintln!("Failed to load settings: {}", e);
                // Continue with defaults
            }
        }
        
        // Initialize dynamic grammar system and auto-install missing grammars
        {
            use syntax_core::dynamic_loader;
            use syntax_core::grammar_registry;
            use syntax_core::grammar_builder;
            
            let registry = grammar_registry::GrammarRegistry::global();
            let mut missing = Vec::new();
            
            // Check which grammars are missing
            for language_id in registry.language_ids() {
                if !dynamic_loader::is_grammar_available(language_id) {
                    missing.push(language_id);
                }
            }
            
            // Auto-install missing grammars
            if !missing.is_empty() {
                println!("Installing {} missing grammars...", missing.len());
                for language_id in &missing {
                    println!("Installing {} grammar...", language_id);
                    match grammar_builder::build_and_install_grammar(language_id) {
                        Ok(_) => println!("Successfully installed {} grammar", language_id),
                        Err(e) => eprintln!("Failed to install {} grammar: {}", language_id, e),
                    }
                }
            }
            
            // Initialize syntax manager after installing grammars
            let mut syntax_manager = app.syntax_manager.lock().unwrap();
            syntax_manager.initialize_dynamic_grammars();
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
        
        for (file, _name) in &font_files {
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
                // Font not found, continue silently
            }
        }
        
        // Always load at least the default system fonts
        // Combine font loading commands with the initial app command
        let mut all_commands = font_commands;
        all_commands.push(command);
        (app, Command::batch(all_commands))
    }

    fn title(&self) -> String {
        String::from(crate::brand::WINDOW_TITLE)
    }

    fn theme(&self) -> iced::Theme {
        self.current_theme.to_iced_theme()
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
