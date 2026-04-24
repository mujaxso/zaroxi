//! Command-line tool to download and install Tree-sitter grammars.

use clap::{Parser, Subcommand};
use zaroxi_lang_syntax::grammar_builder;

#[derive(Parser)]
#[command(name = "download_grammars")]
#[command(about = "Download and install Tree-sitter grammars", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install specific grammars
    Install {
        /// Language IDs to install
        languages: Vec<String>,
    },
    /// List available grammars
    List,
    /// Check which grammars are installed
    Status,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { languages } => {
            if languages.is_empty() {
                eprintln!("Please specify at least one language to install");
                eprintln!("Example: cargo run --bin download_grammars -- install rust typescript");
                return Ok(());
            }

            let language_ids: Vec<&str> = languages.iter().map(|s| s.as_str()).collect();
            let installed = grammar_builder::install_missing_grammars(&language_ids);

            if installed.is_empty() {
                println!("All specified grammars are already installed.");
            } else {
                println!("Successfully installed: {}", installed.join(", "));
            }
        }
        Commands::List => {
            let available = zaroxi_lang_syntax::grammar_registry::available_languages();
            println!("Available grammars:");
            for lang in available {
                println!("  - {}", lang);
            }
        }
        Commands::Status => {
            let available = zaroxi_lang_syntax::grammar_registry::available_languages();
            println!("Grammar installation status:");
            for lang in available {
                let installed = grammar_builder::is_grammar_installed(&lang);
                let status = if installed { "✓" } else { "✗" };
                println!("  {} {}", status, lang);
            }
        }
    }

    Ok(())
}
