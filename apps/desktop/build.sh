#!/bin/bash

echo "Building Zaroxi Desktop..."

# Check if we're in the right directory
if [ ! -f "package.json" ]; then
    echo "Error: Please run this script from the apps/desktop directory"
    echo "Current directory: $(pwd)"
    exit 1
fi

echo "1. Building Rust workspace..."
cd ../..
cargo build --workspace
if [ $? -ne 0 ]; then
    echo "Failed to build Rust workspace"
    exit 1
fi

echo "2. Building frontend..."
cd apps/desktop
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
