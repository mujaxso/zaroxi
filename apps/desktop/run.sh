#!/bin/bash

# Run script for Zaroxi Desktop App
# Run this from the apps/desktop directory

echo "Starting Zaroxi Desktop App..."

# Check if we're in the right directory
if [ ! -f "package.json" ]; then
    echo "Error: Please run this script from the apps/desktop directory"
    echo "Current directory: $(pwd)"
    echo "Try: cd apps/desktop"
    exit 1
fi

# Check if node_modules exists
if [ ! -d "node_modules" ]; then
    echo "node_modules not found. Running npm install..."
    npm install
    if [ $? -ne 0 ]; then
        echo "npm install failed"
        exit 1
    fi
fi

# Check if Rust dependencies are built
if [ ! -d "../target" ] && [ ! -d "../../target" ]; then
    echo "Rust dependencies not built. Building..."
    cd ../..
    cargo build --workspace
    if [ $? -ne 0 ]; then
        echo "cargo build failed"
        exit 1
    fi
    cd apps/desktop
fi

echo "Starting development server..."
npm run tauri dev
