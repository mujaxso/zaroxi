#!/usr/bin/env bash
set -e

# This script works in both bash and zsh environments
# Create fonts directory
mkdir -p apps/desktop/assets/fonts
cd apps/desktop/assets/fonts

# Download Nerd Fonts
echo "Downloading Nerd Fonts..."

# JetBrains Mono Nerd Font
curl -L -o "JetBrainsMonoNerdFont-Regular.ttf" \
    "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/JetBrainsMono/Ligatures/Regular/complete/JetBrains%20Mono%20Regular%20Nerd%20Font%20Complete.ttf"

# Fira Code Nerd Font
curl -L -o "FiraCodeNerdFont-Regular.ttf" \
    "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/FiraCode/Regular/complete/Fira%20Code%20Regular%20Nerd%20Font%20Complete.ttf"

# Cascadia Code Nerd Font
curl -L -o "CascadiaCodeNerdFont-Regular.ttf" \
    "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/CascadiaCode/Regular/complete/Caskaydia%20Cove%20Nerd%20Font%20Complete.ttf"

# Iosevka Nerd Font
curl -L -o "IosevkaNerdFont-Regular.ttf" \
    "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/Iosevka/Regular/complete/Iosevka%20Nerd%20Font%20Complete.ttf"

# Symbols Nerd Font (for icons)
curl -L -o "SymbolsNerdFont-Regular.ttf" \
    "https://github.com/ryanoasis/nerd-fonts/raw/master/patched-fonts/NerdFontsSymbolsOnly/complete/Symbols-2048-em%20Nerd%20Font%20Complete.ttf"

# Noto Color Emoji
curl -L -o "NotoColorEmoji.ttf" \
    "https://github.com/googlefonts/noto-emoji/raw/main/fonts/NotoColorEmoji.ttf"

echo "Fonts downloaded successfully!"
