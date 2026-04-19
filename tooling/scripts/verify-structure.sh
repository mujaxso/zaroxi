#!/bin/bash
set -e

echo "Verifying Zaroxi repository structure..."

# Check required directories exist
required_dirs=(
    "apps/desktop"
    "apps/desktop/frontend"
    "apps/desktop/src-tauri"
    "crates/domain"
    "crates/infrastructure"
    "crates/language"
    "crates/operations"
    "crates/ai"
    "crates/theme"
    "services/ai-daemon"
    "services/workspace-daemon"
    "tooling/config/rust"
    "tooling/scripts"
    "docs/architecture"
    "docs/contributing"
    "tests/integration"
)

missing_dirs=()
for dir in "${required_dirs[@]}"; do
    if [ ! -d "$dir" ]; then
        missing_dirs+=("$dir")
    fi
done

if [ ${#missing_dirs[@]} -gt 0 ]; then
    echo "Missing directories:"
    for dir in "${missing_dirs[@]}"; do
        echo "  - $dir"
    done
    echo ""
    echo "Run ./tooling/scripts/setup-new-structure.sh to create missing directories"
    exit 1
fi

echo "✓ Directory structure looks good"

# Check for required files
required_files=(
    "Cargo.toml"
    "apps/desktop/package.json"
    "apps/desktop/src-tauri/Cargo.toml"
    "crates/theme/Cargo.toml"
    "services/ai-daemon/Cargo.toml"
    "services/workspace-daemon/Cargo.toml"
)

missing_files=()
for file in "${required_files[@]}"; do
    if [ ! -f "$file" ]; then
        missing_files+=("$file")
    fi
done

if [ ${#missing_files[@]} -gt 0 ]; then
    echo "Missing files:"
    for file in "${missing_files[@]}"; do
        echo "  - $file"
    done
    exit 1
fi

echo "✓ Required files exist"

# Try to compile
echo "Checking compilation..."
if cargo check --workspace --quiet 2>/dev/null; then
    echo "✓ Workspace compiles successfully"
else
    echo "✗ Compilation failed. Run 'cargo check --workspace' for details"
    exit 1
fi

echo ""
echo "✅ Repository structure verification passed!"
echo ""
echo "Next steps:"
echo "1. Move old crates to new structure: ./tooling/scripts/migrate-old-crates.sh"
echo "2. Update import paths in Rust code if needed"
echo "3. Run tests: cargo test --workspace"
echo "4. Start desktop app: cd apps/desktop && npm run tauri dev"
