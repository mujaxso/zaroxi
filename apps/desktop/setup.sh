#!/usr/bin/env bash

# Setup script for Zaroxi Desktop App
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

echo "Setting up Zaroxi Desktop App..."

# Check if we found the right directories
if [ ! -f "$DESKTOP_DIR/package.json" ]; then
    echo "Error: Could not find apps/desktop/package.json"
    echo "Make sure you're in the zaroxi repository"
    exit 1
fi

cd "$DESKTOP_DIR"

# Install npm dependencies
echo "Installing npm dependencies..."
npm install

if [ $? -ne 0 ]; then
    echo "npm install failed. Please check your Node.js installation."
    exit 1
fi

echo "Setup complete!"
echo ""
echo "To start the app in development mode:"
echo "  ./run.sh"
echo "  or"
echo "  npm run tauri dev"
echo ""
echo "For frontend-only development:"
echo "  npm run dev"
