//! Command-line tool to download and install Tree-sitter grammars.

use clap::{Parser, Subcommand};
use syntax_core::grammar_registry::GrammarInfo;
use syntax_core::grammar_builder::{build_and_install_grammar, install_missing_grammars, is_grammar_installed};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available grammars
    List,
    /// Install a specific grammar
    Install {
        /// Language identifier (e.g., markdown, rust, python)
        language: String,
    },
    /// Install all grammars
    InstallAll,
    /// Install grammars for common languages
    InstallCommon,
    /// Check if a grammar is installed
    Check {
        /// Language identifier
        language: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::List => {
            println!("Available grammars:");
            for language in GrammarInfo::available_languages() {
                let installed = if is_grammar_installed(&language) {
                    "[installed]"
                } else {
                    "[not installed]"
                };
                println!("  - {} {}", language, installed);
            }
        }
        Commands::Install { language } => {
            println!("Installing grammar for {}...", language);
            match build_and_install_grammar(&language) {
                Ok(()) => println!("Successfully installed {} grammar", language),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::InstallAll => {
            let languages_vec = GrammarInfo::available_languages();
            let languages: Vec<&str> = languages_vec.iter().map(|s| s.as_str()).collect();
            install_missing_grammars(&languages);
        }
        Commands::InstallCommon => {
            let common = vec!["markdown", "rust", "toml", "javascript", "python", "json", "html", "css"];
            install_missing_grammars(&common);
        }
        Commands::Check { language } => {
            if is_grammar_installed(&language) {
                println!("Grammar for {} is installed", language);
            } else {
                println!("Grammar for {} is NOT installed", language);
                println!("Install it with: cargo run --bin download-grammars -- install {}", language);
            }
        }
    }
    
    Ok(())
}
