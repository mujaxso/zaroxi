#!/bin/bash

# Setup script for Zaroxi Desktop App
# Run this from the apps/desktop directory

echo "Setting up Zaroxi Desktop App..."

# Check if we're in the right directory
if [ ! -f "package.json" ]; then
    echo "Error: Please run this script from the apps/desktop directory"
    echo "Current directory: $(pwd)"
    exit 1
fi

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
echo "  npm run tauri dev"
echo ""
echo "For frontend-only development:"
echo "  npm run dev"
#!/bin/bash

# Setup script for Zaroxi Desktop App
# Run this from the apps/desktop directory

echo "Setting up Zaroxi Desktop App..."

# Check if we're in the right directory
if [ ! -f "package.json" ]; then
    echo "Error: Please run this script from the apps/desktop directory"
    echo "Current directory: $(pwd)"
    exit 1
fi

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
echo "  npm run tauri dev"
echo ""
echo "For frontend-only development:"
echo "  npm run dev"
