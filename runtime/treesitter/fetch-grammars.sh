#!/usr/bin/env bash
set -euo pipefail

# Script to fetch Tree‑sitter grammar shared libraries for Neote runtime.
# Supports Linux, macOS, and Windows (if compiled).
# Usage: Called from build.rs with TARGET environment variable set.

RUNTIME_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Determine platform from TARGET (if not set, guess)
TARGET=${TARGET:-}
if [[ -z "$TARGET" ]]; then
    # Guess current system
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)
    if [[ "$OS" == "darwin" ]]; then
        OS="macos"
    fi
    if [[ "$ARCH" == "x86_64" ]]; then
        ARCH="x86_64"
    elif [[ "$ARCH" == "arm64" || "$ARCH" == "aarch64" ]]; then
        ARCH="aarch64"
    else
        ARCH="x86_64"
    fi
    TARGET="${OS}-${ARCH}"
else
    # Convert cargo target triple to our directory naming
    case "$TARGET" in
        *linux*)
            OS="linux"
            ;;
        *darwin*)
            OS="macos"
            ;;
        *windows*)
            OS="windows"
            ;;
        *)
            OS="linux"
            ;;
    esac
    case "$TARGET" in
        *x86_64*)
            ARCH="x86_64"
            ;;
        *aarch64*|*arm64*)
            ARCH="aarch64"
            ;;
        *)
            ARCH="x86_64"
            ;;
    esac
    TARGET="${OS}-${ARCH}"
fi

GRAMMAR_DIR="${RUNTIME_ROOT}/grammars/${TARGET}"
mkdir -p "${GRAMMAR_DIR}"
echo "Target platform: ${TARGET}"
echo "Grammar directory: ${GRAMMAR_DIR}"

# Determine library extension
case "$OS" in
    linux)
        EXT=".so"
        PREFIX="lib"
        ;;
    macos)
        EXT=".dylib"
        PREFIX="lib"
        ;;
    windows)
        EXT=".dll"
        PREFIX=""
        ;;
    *)
        EXT=".so"
        PREFIX="lib"
        ;;
esac

# Map language names to download URLs (release artifacts).
# Format: language_name|url|optional_subpath
# For languages where release asset naming differs, we adjust.
LANGUAGES="
bash|https://github.com/tree-sitter/tree-sitter-bash/releases/download/v0.20.4
c|https://github.com/tree-sitter/tree-sitter-c/releases/download/v0.20.6
cpp|https://github.com/tree-sitter/tree-sitter-cpp/releases/download/v0.20.3
c-sharp|https://github.com/tree-sitter/tree-sitter-c-sharp/releases/download/v0.20.3
css|https://github.com/tree-sitter/tree-sitter-css/releases/download/v0.20.0
go|https://github.com/tree-sitter/tree-sitter-go/releases/download/v0.20.1
html|https://github.com/tree-sitter/tree-sitter-html/releases/download/v0.20.0
java|https://github.com/tree-sitter/tree-sitter-java/releases/download/v0.20.2
javascript|https://github.com/tree-sitter/tree-sitter-javascript/releases/download/v0.20.3
json|https://github.com/tree-sitter/tree-sitter-json/releases/download/v0.20.2
python|https://github.com/tree-sitter/tree-sitter-python/releases/download/v0.20.4
ruby|https://github.com/tree-sitter/tree-sitter-ruby/releases/download/v0.20.3
rust|https://github.com/tree-sitter/tree-sitter-rust/releases/download/v0.20.4
typescript|https://github.com/tree-sitter/tree-sitter-typescript/releases/download/v0.20.3
tsx|https://github.com/tree-sitter/tree-sitter-typescript/releases/download/v0.20.3
lua|https://github.com/tree-sitter-grammars/tree-sitter-lua/releases/download/v0.20.2
toml|https://github.com/tree-sitter-grammars/tree-sitter-toml/releases/download/v0.20.2
yaml|https://github.com/tree-sitter-grammars/tree-sitter-yaml/releases/download/v0.20.0
zig|https://github.com/tree-sitter-grammars/tree-sitter-zig/releases/download/v0.20.0
cmake|https://github.com/uyha/tree-sitter-cmake/releases/download/v0.5.0
dockerfile|https://github.com/camdencheek/tree-sitter-dockerfile/releases/download/v0.5.0
elixir|https://github.com/elixir-lang/tree-sitter-elixir/releases/download/v0.20.2
nix|https://github.com/nix-community/tree-sitter-nix/releases/download/v0.20.2
"

# Special handling for markdown (split parser)
MARKDOWN_URL="https://github.com/tree-sitter-grammars/tree-sitter-markdown/releases/download/v0.20.2"

echo "Fetching grammars..."

for lang_spec in $LANGUAGES; do
    if [[ -z "$lang_spec" ]]; then continue; fi
    IFS='|' read -r lang base_url <<< "$lang_spec"
    lib_name="${PREFIX}tree-sitter-${lang}${EXT}"
    dest="${GRAMMAR_DIR}/${lib_name}"
    # Determine asset name (some differ)
    case "$lang" in
        c-sharp)
            asset_name="libtree-sitter-c_sharp${EXT}"
            ;;
        typescript)
            asset_name="libtree-sitter-typescript${EXT}"
            ;;
        tsx)
            asset_name="libtree-sitter-tsx${EXT}"
            ;;
        *)
            asset_name="$lib_name"
            ;;
    esac
    url="${base_url}/${asset_name}"
    echo "Fetching ${lang}…"
    if curl -Lf "$url" -o "$dest" 2>/dev/null; then
        echo "  → ${lang} OK"
    else
        echo "  → ${lang} download failed, attempting to build from source…"
        # Try to build using tree-sitter CLI if available
        if command -v tree-sitter >/dev/null 2>&1; then
            TS_DIR=$(mktemp -d)
            git clone --depth 1 "$base_url" "$TS_DIR" 2>/dev/null || true
            if [[ -d "$TS_DIR" ]]; then
                (cd "$TS_DIR" && tree-sitter generate 2>/dev/null && tree-sitter build 2>/dev/null) || true
                # The built library location depends on platform
                built_lib="$TS_DIR/${asset_name}"
                if [[ -f "$built_lib" ]]; then
                    cp "$built_lib" "$dest"
                    echo "  → built from source"
                else
                    echo "  → building failed"
                fi
                rm -rf "$TS_DIR"
            fi
        else
            echo "  → tree‑sitter CLI not installed. Skipping."
        fi
    fi
done

# Handle markdown separately (split parser)
echo "Fetching markdown grammar…"
md_lib_name="${PREFIX}tree-sitter-markdown${EXT}"
md_dest="${GRAMMAR_DIR}/${md_lib_name}"
if curl -Lf "${MARKDOWN_URL}/libtree-sitter-markdown${EXT}" -o "$md_dest" 2>/dev/null; then
    echo "  → markdown OK"
else
    echo "  → markdown download failed, skipping."
fi

echo "Grammars have been placed in ${GRAMMAR_DIR}"
ls -la "${GRAMMAR_DIR}/"
