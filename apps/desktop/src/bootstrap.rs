#[allow(dead_code)]
pub fn init() {
    // Initialize logging
    // TODO: Set up proper logging
    
    // Initialize dynamic grammar system
    init_dynamic_grammars();
}

fn init_dynamic_grammars() {
    use syntax_core::dynamic_loader;
    use syntax_core::query_cache;
    use syntax_core::grammar_registry;
    use syntax_core::runtime::Runtime;
    
    // Check for available grammars
    println!("Initializing dynamic grammar system...");
    
    // Initialize runtime and fix any nested structure
    let runtime = Runtime::new();
    println!("Runtime directory: {:?}", runtime.root());
    
    if !runtime.exists() {
        println!("Warning: Runtime directory does not exist at {:?}", runtime.root());
        println!("Creating directory structure...");
        let _ = std::fs::create_dir_all(runtime.root());
    }
    
    // Check which grammars are available
    let registry = grammar_registry::GrammarRegistry::global();
    let mut missing = Vec::new();
    
    for language_id in registry.language_ids() {
        if !dynamic_loader::is_grammar_available(language_id) {
            missing.push(language_id);
        }
    }
    
    if !missing.is_empty() {
        println!("Warning: Missing grammar libraries for: {:?}", missing);
        println!("To build missing grammars, run:");
        for lang in &missing {
            println!("  cargo run --bin build-grammar -- {}", lang);
        }
        println!("Or build all with: cargo run --bin download-grammars -- install-all");
        println!("Note: If you encounter authentication issues when cloning repositories:");
        println!("  1. Ensure you have git installed and configured");
        println!("  2. For public repositories, HTTPS should work without authentication");
        println!("  3. If prompted for credentials, try:");
        println!("     - Setting GIT_TERMINAL_PROMPT=0 in your environment");
        println!("     - Or using a GitHub personal access token");
    }
    
    // Preload available grammars
    dynamic_loader::preload_available_grammars();
    
    // Preload queries
    query_cache::preload_queries();
    
    println!("Dynamic grammar system initialized");
}
