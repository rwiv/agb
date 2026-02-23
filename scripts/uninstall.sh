#!/bin/bash

# agb uninstallation script

set -e

BINARY_NAME="agb"
INSTALL_DIR="$HOME/.local/bin"
TARGET_PATH="$INSTALL_DIR/$BINARY_NAME"

if [ -f "$TARGET_PATH" ]; then
    echo "Uninstalling $BINARY_NAME from $TARGET_PATH..."
    rm "$TARGET_PATH"
    echo "Successfully uninstalled $BINARY_NAME."
else
    echo "$BINARY_NAME is not installed at $TARGET_PATH."
fi
