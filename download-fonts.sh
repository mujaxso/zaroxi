#!/usr/bin/env bash

# Script to download JetBrains Mono Nerd Font for Zaroxi Studio
# Run this script from the project root directory

set -euo pipefail

# Configuration
FONT_DIR="apps/desktop/frontend/public/fonts"
NERD_FONTS_REPO="https://github.com/ryanoasis/nerd-fonts/releases/download/v3.3.0"

# Create fonts directory if it doesn't exist
mkdir -p "$FONT_DIR"

echo "Downloading JetBrains Mono Nerd Font to $FONT_DIR..."

# Clean up any existing zip file
rm -f "$FONT_DIR/JetBrainsMono.zip"

# Download the complete font zip
ZIP_FILE="JetBrainsMono.zip"
DOWNLOAD_URL="${NERD_FONTS_REPO}/${ZIP_FILE}"

echo "Downloading from: $DOWNLOAD_URL"
if curl -L -o "$FONT_DIR/$ZIP_FILE" "$DOWNLOAD_URL" --fail --progress-bar; then
    echo "✓ Downloaded JetBrainsMono.zip"
    
    # Check if the zip file is valid and not empty
    if [ ! -s "$FONT_DIR/$ZIP_FILE" ]; then
        echo "Error: Downloaded zip file is empty"
        rm -f "$FONT_DIR/$ZIP_FILE"
        exit 1
    fi
    
    # Create a temporary directory for extraction
    TEMP_DIR=$(mktemp -d)
    echo "Extracting font files..."
    
    # Extract the zip file
    if unzip -q "$FONT_DIR/$ZIP_FILE" -d "$TEMP_DIR" 2>/dev/null; then
        echo "✓ Extraction successful"
        
        # Find all font files
        echo "Looking for font files..."
        find "$TEMP_DIR" -name "*.ttf" -o -name "*.otf" -o -name "*.woff" -o -name "*.woff2" | head -10 | while read -r font_file; do
            filename=$(basename "$font_file")
            echo "  Found: $filename"
        done
        
        # Now, find and copy the specific files we need
        # Look for Regular (not Italic) - try .ttf first
        REGULAR_FILE=$(find "$TEMP_DIR" -name "*.ttf" -type f | grep -i "regular" | grep -v -i "italic" | head -1)
        if [ -n "$REGULAR_FILE" ]; then
            cp "$REGULAR_FILE" "$FONT_DIR/JetBrainsMonoNerdFont-Regular.ttf"
            echo "✓ Copied Regular variant (.ttf)"
        else
            echo "✗ Could not find Regular variant"
        fi
        
        # Look for Bold (not Italic)
        BOLD_FILE=$(find "$TEMP_DIR" -name "*.ttf" -type f | grep -i "bold" | grep -v -i "italic" | head -1)
        if [ -n "$BOLD_FILE" ]; then
            cp "$BOLD_FILE" "$FONT_DIR/JetBrainsMonoNerdFont-Bold.ttf"
            echo "✓ Copied Bold variant (.ttf)"
        else
            echo "✗ Could not find Bold variant"
        fi
        
        # Look for Italic (not Bold)
        ITALIC_FILE=$(find "$TEMP_DIR" -name "*.ttf" -type f | grep -i "italic" | grep -v -i "bold" | head -1)
        if [ -n "$ITALIC_FILE" ]; then
            cp "$ITALIC_FILE" "$FONT_DIR/JetBrainsMonoNerdFont-Italic.ttf"
            echo "✓ Copied Italic variant (.ttf)"
        else
            echo "✗ Could not find Italic variant"
        fi
        
        # Look for Bold Italic
        BOLD_ITALIC_FILE=$(find "$TEMP_DIR" -name "*.ttf" -type f | grep -i "bold" | grep -i "italic" | head -1)
        if [ -n "$BOLD_ITALIC_FILE" ]; then
            cp "$BOLD_ITALIC_FILE" "$FONT_DIR/JetBrainsMonoNerdFont-BoldItalic.ttf"
            echo "✓ Copied Bold Italic variant (.ttf)"
        else
            echo "✗ Could not find Bold Italic variant"
        fi
        
        # Clean up temporary directory
        rm -rf "$TEMP_DIR"
    else
        echo "Error: Failed to extract zip file"
        echo "The zip file might be corrupted or in an unexpected format"
        rm -f "$FONT_DIR/$ZIP_FILE"
        exit 1
    fi
    
    # Clean up the zip file
    rm -f "$FONT_DIR/$ZIP_FILE"
    
    # Verify we have the required files
    echo ""
    echo "Verifying downloaded fonts..."
    REQUIRED_FILES=(
        "JetBrainsMonoNerdFont-Regular.ttf"
        "JetBrainsMonoNerdFont-Bold.ttf"
        "JetBrainsMonoNerdFont-Italic.ttf"
        "JetBrainsMonoNerdFont-BoldItalic.ttf"
    )
    
    all_present=true
    for required_file in "${REQUIRED_FILES[@]}"; do
        if [ -f "$FONT_DIR/$required_file" ]; then
            file_size=$(stat -f%z "$FONT_DIR/$required_file" 2>/dev/null || stat -c%s "$FONT_DIR/$required_file" 2>/dev/null || echo "0")
            if [ "$file_size" -gt 1000 ]; then
                echo "  ✓ $required_file ($((file_size/1024)) KB)"
            else
                echo "  ✗ $required_file (file too small: ${file_size} bytes)"
                all_present=false
            fi
        else
            echo "  ✗ Missing: $required_file"
            all_present=false
        fi
    done
    
    if [ "$all_present" = true ]; then
        echo ""
        echo "✅ Success! All required font files are present."
        echo "Fonts are ready in: $FONT_DIR"
        echo ""
        echo "Note: Downloaded .ttf files. To use .woff2 files instead:"
        echo "1. Install woff2 tools: 'brew install woff2' (macOS) or 'apt-get install woff2' (Ubuntu)"
        echo "2. Convert .ttf to .woff2 using: woff2_compress <filename>.ttf"
        echo "3. Update globals.css to reference .woff2 files"
    else
        echo ""
        echo "⚠️  Warning: Some font files are missing."
        echo "Current contents of fonts directory:"
        ls -la "$FONT_DIR/" 2>/dev/null || echo "  (fonts directory is empty)"
    fi
    
else
    echo "Error: Failed to download JetBrainsMono.zip"
    echo "Possible reasons:"
    echo "1. Network connection issue"
    echo "2. The download URL may have changed"
    echo ""
    echo "You can manually download the font from:"
    echo "https://github.com/ryanoasis/nerd-fonts/releases"
    echo "Look for 'JetBrainsMono.zip' in the latest release"
    exit 1
fi

echo ""
echo "Font download process complete!"
