#!/usr/bin/env bash
set -e

# Wrapper script to download and install Tree-sitter grammars
# for Qyzer Studio

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR"

echo "Installing Tree-sitter grammars for Qyzer Studio..."
echo ""

# Check for required tools
for cmd in git cc cargo; do
    if ! command -v $cmd &> /dev/null; then
        echo "Error: $cmd is required but not installed."
        exit 1
    fi
done

# Create runtime directory structure if it doesn't exist
RUNTIME_DIR="$PROJECT_ROOT/runtime/treesitter"
mkdir -p "$RUNTIME_DIR/grammars"
mkdir -p "$RUNTIME_DIR/languages"

echo "Runtime directory: $RUNTIME_DIR"

# Build and run the download-grammars tool
cd "$PROJECT_ROOT"

echo "Building download-grammars tool..."
cargo build --bin download-grammars --manifest-path crates/syntax-core/Cargo.toml

echo ""
echo "Available commands:"
echo "  ./download_grammars.sh list              - List available grammars"
echo "  ./download_grammars.sh install <lang>    - Install specific grammar"
echo "  ./download_grammars.sh install-common    - Install common grammars"
echo "  ./download_grammars.sh install-all       - Install all grammars"
echo ""

if [ $# -eq 0 ]; then
    echo "Running: cargo run --bin download-grammars -- install-common"
    cargo run --bin download-grammars --manifest-path crates/syntax-core/Cargo.toml -- install-common
else
    cargo run --bin download-grammars --manifest-path crates/syntax-core/Cargo.toml -- "$@"
fi
