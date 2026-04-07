#!/usr/bin/env bash
# Fix Git index version to version 2

set -e

echo "Fixing Git index version..."

# Check if we're in a Git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "Not in a Git repository"
    exit 1
fi

echo "Current Git index version:"
./scripts/check-git-index.sh

echo ""
echo "Attempting to downgrade Git index to version 2..."

if git update-index --index-version 2; then
    echo "Successfully set Git index to version 2"
    echo ""
    echo "New Git index version:"
    ./scripts/check-git-index.sh
else
    echo "Failed to set Git index to version 2"
    echo ""
    echo "You may need to:"
    echo "1. Make sure you have no unstaged changes"
    echo "2. Try: git read-tree HEAD && git update-index --index-version 2"
    echo "3. Or use tools with '--no-git' flag"
    exit 1
fi
