//! Command-line tool to download and install Tree-sitter grammars.
//! (Stub until clap dependency is added to the workspace)

fn main() {
    eprintln!("download_grammars binary is disabled because the `clap` crate is not in this crate's dependencies.");
    eprintln!("To enable this tool, add `clap` (with derive feature) to `crates/zaroxi-lang-syntax/Cargo.toml`.");
    std::process::exit(1);
}
