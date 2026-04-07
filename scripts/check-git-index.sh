#!/usr/bin/env bash
# Check and fix Git index version

set -e

echo "Checking Git index version..."

# Check if we're in a Git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "Not in a Git repository"
    exit 0
fi

# Try to get index version (first 12 bytes: DIRC + version)
if [ -f .git/index ]; then
    echo "Git index file exists"
    # Check first 4 bytes should be "DIRC" (44 49 52 43 in hex)
    HEADER=$(head -c 4 .git/index | xxd -p)
    if [ "$HEADER" != "44495243" ]; then
        echo "Warning: Git index doesn't start with DIRC"
    fi
    
    # Get version (bytes 5-8)
    VERSION_HEX=$(head -c 8 .git/index | tail -c 4 | xxd -p -u | tr -d '[:space:]')
    # Handle empty or invalid hex string
    if [ -z "$VERSION_HEX" ]; then
        echo "Could not read version from Git index"
        exit 1
    fi
    # Convert hex to decimal using printf (more robust)
    VERSION=$(printf "%d" "0x$VERSION_HEX" 2>/dev/null)
    if [ $? -ne 0 ]; then
        echo "Invalid hex value in Git index: $VERSION_HEX"
        exit 1
    fi
    
    echo "Git index version: $VERSION"
    
    if [ $VERSION -eq 3 ]; then
        echo "Git index is version 3, which may cause issues with some tools"
        echo ""
        echo "To fix this, run:"
        echo "  git update-index --index-version 2"
        echo ""
        echo "Or use the --no-git flag with tools like Aider:"
        echo "  aider --no-git"
    elif [ $VERSION -eq 2 ]; then
        echo "Git index is version 2, which should be compatible with most tools"
    else
        echo "Git index is version $VERSION"
    fi
else
    echo "No .git/index file found"
fi
