#!/usr/bin/env bash

# Build script for Zaroxi Desktop
# Can be run from anywhere within the zaroxi repository

set -e

# Find the zaroxi root directory
find_zaroxi_root() {
    local dir="$PWD"
    while [ "$dir" != "/" ]; do
        if [ -f "$dir/Cargo.toml" ] && [ -d "$dir/apps/desktop" ]; then
            echo "$dir"
            return 0
        fi
        dir="$(dirname "$dir")"
    done
    return 1
}

ZAROXI_ROOT="$(find_zaroxi_root 2>/dev/null || echo "$PWD")"
DESKTOP_DIR="$ZAROXI_ROOT/apps/desktop"

echo "Building Zaroxi Desktop..."

# Check if we found the right directories
if [ ! -f "$DESKTOP_DIR/package.json" ]; then
    echo "Error: Could not find apps/desktop/package.json"
    echo "Make sure you're in the zaroxi repository"
    exit 1
fi

echo "1. Building Rust workspace..."
cd "$ZAROXI_ROOT"
cargo build --workspace
if [ $? -ne 0 ]; then
    echo "Failed to build Rust workspace"
    exit 1
fi

echo "2. Building frontend..."
cd "$DESKTOP_DIR"
npm run build
if [ $? -ne 0 ]; then
    echo "Failed to build frontend"
    exit 1
fi

echo "3. Building Tauri application..."
npm run tauri build
if [ $? -ne 0 ]; then
    echo "Failed to build Tauri application"
    exit 1
fi

echo "✅ Build complete!"
echo "The application is ready in src-tauri/target/release/"
