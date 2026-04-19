#!/bin/bash

echo "Fixing script permissions..."

# Make all scripts executable
chmod +x run.sh start.sh setup.sh

# Make check-setup.js executable (if needed)
chmod +x check-setup.js

echo "Permissions fixed!"
echo ""
echo "Now you can run:"
echo "  ./run.sh"
echo "or"
echo "  ./start.sh"
