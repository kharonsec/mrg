#!/bin/bash
set -e

# Check for cargo
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo is not installed. Please install Rust: https://rustup.rs/"
    exit 1
fi

BINARY_NAME="mrg"

echo "Building $BINARY_NAME in release mode..."
cargo build --release

# Determine installation directory
INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

cp "target/release/$BINARY_NAME" "$INSTALL_DIR/"

echo "Successfully installed $BINARY_NAME to $INSTALL_DIR"
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "Warning: $INSTALL_DIR is not in your PATH. Add it with: export PATH=\$PATH:$INSTALL_DIR"
fi
