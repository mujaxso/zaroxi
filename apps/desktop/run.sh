#!/usr/bin/env bash

# Simple run script for Zaroxi Desktop App
set -e

echo "Starting Zaroxi Desktop App..."

# Go to desktop directory
cd "$(dirname "$0")"

# Kill any vite or tauri processes using ports 1420 or 1421
echo "Clearing port 1420 and 1421..."
pkill -f "vite" || true
pkill -f "tauri dev" || true

# Wait a bit
sleep 1

# Run the app
npm run tauri dev
