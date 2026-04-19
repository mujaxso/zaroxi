#!/usr/bin/env bash

# Zaroxi Desktop - Alternative Start Script
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

echo "Zaroxi Desktop - Alternative Start Script"
echo "=========================================="
echo ""

# Check if we found the right directories
if [ ! -f "$DESKTOP_DIR/package.json" ]; then
    echo "ERROR: Could not find apps/desktop/package.json"
    echo "Make sure you're in the zaroxi repository"
    exit 1
fi

cd "$DESKTOP_DIR"

echo "1. Checking npm dependencies..."
if [ ! -d "node_modules" ]; then
    echo "   Installing npm dependencies..."
    npm install
    if [ $? -ne 0 ]; then
        echo "   ERROR: npm install failed"
        exit 1
    fi
    echo "   ✓ npm dependencies installed"
else
    echo "   ✓ npm dependencies already installed"
fi

echo ""
echo "2. Checking Rust dependencies..."
if [ ! -d "$ZAROXI_ROOT/target" ]; then
    echo "   Building Rust workspace..."
    cd "$ZAROXI_ROOT"
    cargo build --workspace
    if [ $? -ne 0 ]; then
        echo "   ERROR: cargo build failed"
        exit 1
    fi
    cd "$DESKTOP_DIR"
    echo "   ✓ Rust dependencies built"
else
    echo "   ✓ Rust dependencies already built"
fi

echo ""
echo "3. Starting Zaroxi Desktop..."
echo "   Frontend: http://localhost:1420"
echo "   Press Ctrl+C to stop"
echo ""
npm run tauri dev
