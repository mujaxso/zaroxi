use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // The tree-sitter-rust crate handles its own linking
    // We don't need to do anything special here for that
    
    // Only run if the markdown feature is enabled
    if env::var("CARGO_FEATURE_MARKDOWN").is_ok() {
        println!("cargo:rerun-if-env-changed=NEOTE_RUNTIME");
        println!("cargo:rerun-if-changed=build.rs");
        
        // Always skip automatic download in build script to avoid git operations
        // Users should use the download-grammars tool instead
        println!("cargo:warning=Skipping automatic grammar download in build script. Use 'cargo run --bin download-grammars -- install markdown' instead.");
        return;
    }
}

fn download_markdown_grammar() {
    println!("cargo:warning=Checking for Tree-sitter Markdown grammar...");
    
    // Determine runtime directory
    let runtime_dir = if let Ok(env_path) = env::var("NEOTE_RUNTIME") {
        env_path
    } else {
        // Default to project root relative to build script
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        format!("{}/../../../runtime/treesitter", manifest_dir)
    };
    
    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    let grammar_dir = format!("{}/grammars/{}-{}", runtime_dir, os, arch);
    let lib_name = if cfg!(windows) {
        "tree-sitter-markdown.dll"
    } else if cfg!(target_os = "macos") {
        "libtree-sitter-markdown.dylib"
    } else {
        "libtree-sitter-markdown.so"
    };
    
    let lib_path = format!("{}/{}", grammar_dir, lib_name);
    
    // Check if grammar already exists
    if Path::new(&lib_path).exists() {
        println!("cargo:warning=Markdown grammar already exists at {}", lib_path);
        return;
    }
    
    println!("cargo:warning=Markdown grammar not found at {}", lib_path);
    println!("cargo:warning=Attempting to download and compile...");
    
    // Create temporary directory
    let temp_dir = env::temp_dir().join("neote-markdown-build");
    if temp_dir.exists() {
        let _ = fs::remove_dir_all(&temp_dir);
    }
    fs::create_dir_all(&temp_dir).unwrap();
    
    // Clone repository
    println!("cargo:warning=Cloning tree-sitter-markdown repository...");
    let status = Command::new("git")
        .args(["clone", "--depth", "1", "https://github.com/tree-sitter/tree-sitter-markdown", temp_dir.to_str().unwrap()])
        .status();
    
    if let Ok(status) = status {
        if status.success() {
            // Try to build with tree-sitter CLI if available
            let has_tree_sitter_cli = Command::new("tree-sitter")
                .arg("--version")
                .output()
                .is_ok();
            
            if has_tree_sitter_cli {
                println!("cargo:warning=Using tree-sitter CLI to build...");
                let _ = Command::new("tree-sitter")
                    .current_dir(&temp_dir)
                    .arg("generate")
                    .status();
                
                let _ = Command::new("tree-sitter")
                    .current_dir(&temp_dir)
                    .arg("build")
                    .status();
                
                // Find the built library
                let built_lib = if cfg!(windows) {
                    temp_dir.join("target/release/tree-sitter-markdown.dll")
                } else if cfg!(target_os = "macos") {
                    temp_dir.join("target/release/libtree-sitter-markdown.dylib")
                } else {
                    temp_dir.join("target/release/libtree-sitter-markdown.so")
                };
                
                if built_lib.exists() {
                    // Create target directory
                    fs::create_dir_all(&grammar_dir).unwrap();
                    fs::copy(&built_lib, &lib_path).unwrap();
                    println!("cargo:warning=Successfully built and installed markdown grammar to {}", lib_path);
                } else {
                    println!("cargo:warning=Could not find built library at {:?}", built_lib);
                }
            } else {
                println!("cargo:warning=tree-sitter CLI not found. Please install it or run 'cargo run --bin download-grammars -- install markdown' manually.");
            }
        } else {
            println!("cargo:warning=Failed to clone repository");
        }
    } else {
        println!("cargo:warning=Git not available. Please install git or download the grammar manually.");
    }
    
    // Clean up
    let _ = fs::remove_dir_all(&temp_dir);
}
