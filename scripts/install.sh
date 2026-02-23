#!/bin/bash

# agb installation script

set -e

BINARY_NAME="agb"
INSTALL_DIR="$HOME/.local/bin"
TARGET_PATH="$INSTALL_DIR/$BINARY_NAME"

echo "Building $BINARY_NAME in release mode..."
cargo build --release

# Ensure install directory exists
mkdir -p "$INSTALL_DIR"

# Remove existing binary if it exists
if [ -f "$TARGET_PATH" ]; then
  echo "Removing existing binary at $TARGET_PATH"
  rm "$TARGET_PATH"
fi

echo "Installing $BINARY_NAME to $INSTALL_DIR"
cp "target/release/$BINARY_NAME" "$TARGET_PATH"

echo "Successfully installed $BINARY_NAME to $TARGET_PATH"
echo "Make sure $INSTALL_DIR is in your PATH."
