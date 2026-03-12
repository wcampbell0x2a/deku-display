#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BUILD_DIR="$SCRIPT_DIR/_build"
SPHINX_DIR="$(mktemp -d)"

# Generate RST from example files
cargo run -p deku-display-cli --quiet -- \
    "$SCRIPT_DIR"/examples/*.rs \
    --format rst \
    --output "$SPHINX_DIR/deku_types.rst"

# Scaffold a fresh Sphinx site in temp dir
uv run --with sphinx --with sphinxcontrib-bitfield \
    sphinx-quickstart "$SPHINX_DIR" --quiet --project "deku-display" --author "" --no-sep
echo "extensions = ['sphinxcontrib.bitfield']" >> "$SPHINX_DIR/conf.py"

# Add to toctree
sed -i '/^   :caption: Contents:/a\\n   deku_types' "$SPHINX_DIR/index.rst"

# Build HTML
rm -rf "$BUILD_DIR"
uv run --with sphinx --with sphinxcontrib-bitfield \
    sphinx-build "$SPHINX_DIR" "$BUILD_DIR"

# Cleanup
rm -rf "$SPHINX_DIR"

echo "Done — open $BUILD_DIR/deku_types.html"
