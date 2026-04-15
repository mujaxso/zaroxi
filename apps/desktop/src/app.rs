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
                        Err(e) => {
                            eprintln!("Failed to install {} grammar: {}", language_id, e);
                            // Try to create at least an empty query file
                            let runtime = syntax_core::runtime::Runtime::new();
                            let query_dir = runtime.language_dir(language_id).join("queries");
                            let _ = std::fs::create_dir_all(&query_dir);
                            let query_path = query_dir.join("highlights.scm");
                            if !query_path.exists() {
                                let _ = std::fs::write(&query_path, "");
                            }
                        }
                    }
                }
            }
            
            // Also ensure all languages have at least empty query files
            for language_id in registry.language_ids() {
                let runtime = syntax_core::runtime::Runtime::new();
                let query_dir = runtime.language_dir(language_id).join("queries");
                let _ = std::fs::create_dir_all(&query_dir);
                let query_path = query_dir.join("highlights.scm");
                if !query_path.exists() {
                    let _ = std::fs::write(&query_path, "");
                }
            }
            
            // Force reinstall of markdown grammar if it's not working
            if !dynamic_loader::is_grammar_available("markdown") {
                println!("Markdown grammar is not available, reinstalling...");
                match grammar_builder::build_and_install_grammar("markdown") {
                    Ok(_) => println!("Successfully reinstalled markdown grammar"),
                    Err(e) => eprintln!("Failed to reinstall markdown grammar: {}", e),
                }
            }
            
            // Check if markdown queries exist, and if not, try to reinstall the grammar
            {
                let runtime = syntax_core::runtime::Runtime::new();
                let query_dir = runtime.language_dir("markdown").join("queries");
                let query_path = query_dir.join("highlights.scm");
                
                // Always replace the markdown query file with our version
                // because the downloaded one uses nvim-treesitter capture names
                println!("Replacing markdown query file with correct capture names...");
                let _ = std::fs::create_dir_all(&query_dir);
                
                // Create a query file with capture names that match our highlight mapping
                let correct_query = r#"
; Markdown highlighting for tree-sitter-markdown with correct capture names
(atx_heading) @heading
(setext_heading) @heading
(emphasis) @emphasis
(strong_emphasis) @strong
(link) @link
(inline_code_span) @inline_code
(code_fence) @code_fence
(block_quote) @block_quote
(list) @list
(thematic_break) @thematic_break
(paragraph) @paragraph
(fenced_code_block) @code_fence
(code_span) @inline_code
(image) @link
(reference_link) @link
(reference_definition) @link
(footnote_reference) @link
(footnote_definition) @link
(task_list_marker) @operator
(strikethrough) @emphasis
(escape_sequence) @string
(hard_line_break) @operator
(soft_line_break) @paragraph
(table) @table
(table_header) @heading
(table_row) @table
(table_cell) @paragraph
(html_block) @html
(html_inline) @html

; Additional captures that might be present
(heading_content) @heading
(list_marker) @operator
(link_label) @link
(link_title) @string
(url) @string
(email) @string
"#;
                
                if let Err(e) = std::fs::write(&query_path, correct_query) {
                    eprintln!("Failed to write markdown query file: {}", e);
                } else {
                    println!("Created markdown query file with correct capture names");
                }
            }
            
            // Initialize syntax manager after installing grammars
            let mut syntax_manager = app.syntax_manager.lock().unwrap();
            syntax_manager.initialize_dynamic_grammars();
            
            // Test markdown highlighting
            {
                use syntax_core::language::LanguageId;
                use syntax_core::dynamic_loader;
                use tree_sitter::Parser;
                
                // Check if markdown language can be loaded
                if let Some(lang) = dynamic_loader::load_language("markdown") {
                    println!("Markdown language loaded successfully");
                    
                    // Create a test parser
                    let mut parser = Parser::new();
                    if parser.set_language(&lang).is_ok() {
                        println!("Markdown parser configured successfully");
                        
                        // Test parse a simple markdown
                        let test_md = "# Heading\n\nThis is **bold** and *italic*.\n\n`code`";
                        if let Some(tree) = parser.parse(test_md, None) {
                            println!("Markdown test parse successful");
                            
                            // Try to highlight
                            use syntax_core::highlight::highlight;
                            match highlight(LanguageId::Markdown, test_md, &tree) {
                                Ok(spans) => {
                                    println!("Markdown highlighting produced {} spans", spans.len());
                                    for span in spans.iter().take(5) {
                                        println!("  Span [{}, {}]: {:?}", span.start, span.end, span.highlight);
                                    }
                                    
                                    // Also print the actual text for each span
                                    for span in spans.iter().take(5) {
                                        if span.start < test_md.len() && span.end <= test_md.len() {
                                            let text = &test_md[span.start..span.end];
                                            println!("    Text: {:?}", text);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Markdown highlighting error: {:?}", e);
                                }
                            }
                        } else {
                            eprintln!("Markdown test parse failed");
                        }
                    } else {
                        eprintln!("Failed to set markdown language on parser");
                    }
                } else {
                    eprintln!("Markdown language not loaded");
                    
                    // Debug: check if the library file exists
                    use syntax_core::runtime::Runtime;
                    let runtime = Runtime::new();
                    let lib_path = runtime.grammar_library_path("markdown");
                    println!("Markdown library path: {}", lib_path.display());
                    println!("Exists: {}", lib_path.exists());
                    
                    // Check query file
                    let query_dir = runtime.language_dir("markdown").join("queries");
                    let query_path = query_dir.join("highlights.scm");
                    println!("Markdown query path: {}", query_path.display());
                    println!("Exists: {}", query_path.exists());
                    
                    if query_path.exists() {
                        if let Ok(content) = std::fs::read_to_string(&query_path) {
                            println!("Query file size: {} bytes", content.len());
                            println!("First 200 chars: {}", &content[..content.len().min(200)]);
                        }
                    }
                }
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
