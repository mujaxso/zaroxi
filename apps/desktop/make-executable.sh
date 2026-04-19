#!/bin/bash

echo "Making scripts executable..."

# Make this script executable first
chmod +x "$0"

# Make all other scripts executable
chmod +x run.sh start.sh setup.sh build.sh fix-permissions.sh check-setup.js

echo "Scripts are now executable!"
echo ""
echo "You can now run:"
echo "  ./run.sh      # Start development"
echo "  ./fix-permissions.sh  # Fix permissions for all scripts"
