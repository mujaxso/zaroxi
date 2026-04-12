fn main() {
    // Check if font files exist from various possible locations
    let possible_paths = [
        // Relative to build.rs location
        "assets/fonts/JetBrainsMono-Regular.ttf",
        "assets/fonts/JetBrainsMonoNerdFont-Regular.ttf",
        "assets/fonts/FiraCode-Regular.ttf",
        "assets/fonts/FiraCodeNerdFont-Regular.ttf",
        "assets/fonts/NotoColorEmoji.ttf",
        "assets/fonts/SymbolsNerdFont-Regular.ttf",
        "assets/fonts/CascadiaCode-Regular.ttf",
        "assets/fonts/CascadiaCodeNerdFont-Regular.ttf",
        "assets/fonts/Iosevka-Regular.ttf",
        "assets/fonts/IosevkaNerdFont-Regular.ttf",
        // Relative to project root
        "apps/desktop/assets/fonts/JetBrainsMono-Regular.ttf",
        "apps/desktop/assets/fonts/JetBrainsMonoNerdFont-Regular.ttf",
        "apps/desktop/assets/fonts/FiraCode-Regular.ttf",
        "apps/desktop/assets/fonts/FiraCodeNerdFont-Regular.ttf",
        "apps/desktop/assets/fonts/NotoColorEmoji.ttf",
        "apps/desktop/assets/fonts/SymbolsNerdFont-Regular.ttf",
        "apps/desktop/assets/fonts/CascadiaCode-Regular.ttf",
        "apps/desktop/assets/fonts/CascadiaCodeNerdFont-Regular.ttf",
        "apps/desktop/assets/fonts/Iosevka-Regular.ttf",
        "apps/desktop/assets/fonts/IosevkaNerdFont-Regular.ttf",
    ];
    
    let mut jetbrains_found = false;
    let mut firacode_found = false;
    let mut _emoji_found = false;
    let mut symbols_found = false;
    
    for path in &possible_paths {
        if std::path::Path::new(path).exists() {
            if path.contains("JetBrainsMono") {
                jetbrains_found = true;
            }
            if path.contains("FiraCode") {
                firacode_found = true;
            }
            if path.contains("NotoColorEmoji") {
                _emoji_found = true;
            }
            if path.contains("SymbolsNerdFont") {
                symbols_found = true;
            }
        }
    }
    
    // Only print warnings if essential fonts are missing
    // For icons, we need Symbols Nerd Font and at least one coding font
    if !symbols_found || (!jetbrains_found && !firacode_found) {
        println!("cargo:warning=Some font files not found. Icons may not display correctly.");
        println!("cargo:warning=To download fonts, run from apps/desktop directory:");
        println!("cargo:warning=  chmod +x scripts/download-fonts.sh");
        println!("cargo:warning=  ./scripts/download-fonts.sh");
    }

    // Determine if we are statically linking tree‑sitter grammars via Cargo features.
    // If the Rust and/or TOML features are enabled, we don't need external .so files.
    let static_rust = std::env::var("CARGO_FEATURE_RUST").is_ok();
    let static_toml = std::env::var("CARGO_FEATURE_TOML").is_ok();

    // Only warn about missing .so files for languages that aren't statically linked.
    if !static_rust || !static_toml {
        let runtime_root = std::path::Path::new("runtime/treesitter");
        let target = std::env::var("TARGET").unwrap_or_default();
        let os = if target.contains("linux") {
            "linux"
        } else if target.contains("darwin") {
            "macos"
        } else if target.contains("windows") {
            "windows"
        } else {
            // unknown, default to linux
            "linux"
        };
        let arch = if target.contains("x86_64") {
            "x86_64"
        } else if target.contains("aarch64") || target.contains("arm64") {
            "aarch64"
        } else {
            "x86_64"
        };
        let grammar_dir = runtime_root.join("grammars").join(format!("{}-{}", os, arch));
        let rust_grammar = grammar_dir.join("libtree-sitter-rust.so");
        let toml_grammar = grammar_dir.join("libtree-sitter-toml.so");

        let mut missing = Vec::new();
        if !static_rust && !rust_grammar.exists() {
            missing.push("rust");
        }
        if !static_toml && !toml_grammar.exists() {
            missing.push("toml");
        }

        if !missing.is_empty() {
            println!("cargo:warning=Tree‑sitter grammar libraries missing for {} (dynamic linking).", missing.join(", "));
            println!("cargo:warning=Attempting to download via fetch‑grammars.sh…");
            // Try to run the existing fetch script
            let script_path = runtime_root.join("fetch-grammars.sh");
            if script_path.exists() {
                use std::process::Command;
                let status = Command::new("bash")
                    .arg(script_path)
                    .status();
                match status {
                    Ok(exit_status) if exit_status.success() => {
                        println!("cargo:warning=Successfully downloaded grammar libraries.");
                    }
                    _ => {
                        println!("cargo:warning=Failed to run fetch-grammars.sh. Grammars may be missing.");
                    }
                }
            } else {
                println!("cargo:warning=fetch-grammars.sh not found at {:?}. Grammars will be missing.", script_path);
            }
        }
    } else {
        // Static linking enabled for both Rust and TOML – no need for external .so files.
        println!("cargo:warning=Static tree‑sitter linking enabled (rust={}, toml={}).", static_rust, static_toml);
    }
}
