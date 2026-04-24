use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Always rerun if build script changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/grammar_registry.rs");

    // Check if we should build grammars
    let should_build_grammars = env::var("CARGO_FEATURE_BUILD_GRAMMARS").is_ok()
        || env::var("QYZER_BUILD_GRAMMARS").is_ok();

    if should_build_grammars {
        build_grammars();
    }

    // Generate grammar manifest for inclusion
    generate_grammar_manifest();

    // Check tree-sitter version compatibility
    println!("cargo:rustc-env=TREE_SITTER_VERSION=0.26.8");
}

fn build_grammars() {
    println!("cargo:warning=Building Tree-sitter grammars...");

    // Determine runtime directory
    let runtime_dir = if let Ok(env_path) = env::var("ZAROXI_STUDIO_RUNTIME") {
        env_path
    } else if let Ok(env_path) = env::var("QYZER_STUDIO_RUNTIME") {
        env_path
    } else if let Ok(env_path) = env::var("NEOTE_RUNTIME") {
        env_path
    } else {
        // Default to project root relative to build script
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        format!("{}/../../../runtime/treesitter", manifest_dir)
    };

    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    let grammar_dir = format!("{}/grammars/{}-{}", runtime_dir, os, arch);

    // Create grammar directory if it doesn't exist
    fs::create_dir_all(&grammar_dir).unwrap_or_else(|e| {
        println!("cargo:warning=Failed to create grammar directory: {}", e);
    });

    // Build core grammars (rust, toml, markdown)
    let core_grammars = vec!["rust", "toml", "markdown"];

    for grammar in core_grammars {
        let lib_name = if cfg!(windows) {
            format!("tree-sitter-{}.dll", grammar)
        } else if cfg!(target_os = "macos") {
            format!("libtree-sitter-{}.dylib", grammar)
        } else {
            format!("libtree-sitter-{}.so", grammar)
        };

        let lib_path = format!("{}/{}", grammar_dir, lib_name);

        // Check if grammar already exists
        if Path::new(&lib_path).exists() {
            println!("cargo:warning={} grammar already exists at {}", grammar, lib_path);
            continue;
        }

        println!("cargo:warning=Building {} grammar...", grammar);

        // Build grammar using external tool
        let status =
            Command::new("cargo").args(["run", "--bin", "build-grammar", "--", grammar]).status();

        match status {
            Ok(exit_status) if exit_status.success() => {
                println!("cargo:warning=Successfully built {} grammar", grammar);
            }
            Ok(_) => {
                println!("cargo:warning=Failed to build {} grammar", grammar);
            }
            Err(e) => {
                println!("cargo:warning=Failed to run build-grammar for {}: {}", grammar, e);
            }
        }
    }
}

fn generate_grammar_manifest() {
    // Create a manifest file that can be included in the binary
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_path = Path::new(&out_dir).join("grammar_manifest.rs");

    let manifest_content = r#"
// Auto-generated grammar manifest
// This file is generated at build time

pub const SUPPORTED_LANGUAGES: &[&str] = &[
    "rust",
    "toml", 
    "markdown",
    "javascript",
    "python",
    "json",
    "html",
    "css",
    "go",
    "c",
    "cpp",
    "java",
    "bash",
    "c_sharp",
    "ruby",
    "typescript",
    "tsx",
    "lua",
    "yaml",
    "zig",
    "cmake",
    "dockerfile",
    "elixir",
    "nix",
];
"#;

    fs::write(&manifest_path, manifest_content).unwrap();
    println!("cargo:rerun-if-changed={}", manifest_path.display());
}
