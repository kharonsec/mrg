#!/bin/bash
set -e

REPO="kharonsec/mrg"
VERSION="v0.1.0-beta"
BINARY_NAME="mrg"
RELEASE_ASSET_NAME="mrg"

# Determine installation directory
INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

echo "Downloading $BINARY_NAME $VERSION from GitHub..."
URL="https://github.com/$REPO/releases/download/$VERSION/$RELEASE_ASSET_NAME"

curl -L "$URL" -o "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo "Successfully installed $BINARY_NAME to $INSTALL_DIR"
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "Warning: $INSTALL_DIR is not in your PATH. Add it with: export PATH=\$PATH:$INSTALL_DIR"
fi
