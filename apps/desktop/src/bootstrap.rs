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
    
    // Check for available grammars
    println!("Initializing dynamic grammar system...");
    
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
        println!("Note: Some grammars may require git and SSH keys for GitHub.");
        println!("If you encounter authentication issues, try:");
        println!("  1. Setting up SSH keys for GitHub: https://docs.github.com/en/authentication/connecting-to-github-with-ssh");
        println!("  2. Or using HTTPS with a personal access token");
    }
    
    // Preload available grammars
    dynamic_loader::preload_available_grammars();
    
    // Preload queries
    query_cache::preload_queries();
    
    println!("Dynamic grammar system initialized");
}
