#!/bin/bash
# TabStash NVMe Linux Installer Script
# Requires: sudo privileges
# Usage: sudo ./install-linux.sh

set -e

EXTENSION_ID="${1:-EXTENSION_ID_PLACEHOLDER}"
BINARY_NAME="tabstash-native"
INSTALL_DIR="/usr/local/bin"
DATA_DIR="$HOME/.local/share/tabstash/snapshots"

# Determine Chrome Native Messaging Hosts directory
if [ -d "$HOME/.config/google-chrome" ]; then
    MANIFEST_DIR="$HOME/.config/google-chrome/NativeMessagingHosts"
elif [ -d "/etc/opt/chrome" ]; then
    MANIFEST_DIR="/etc/opt/chrome/native-messaging-hosts"
    SUDO_REQUIRED=true
else
    echo "ERROR: Chrome installation not found." >&2
    exit 1
fi

echo "Installing TabStash NVMe Native Host..."

# Check if binary exists in script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_BINARY="$SCRIPT_DIR/../tabstash-native/target/release/$BINARY_NAME"

if [ ! -f "$SOURCE_BINARY" ]; then
    echo "ERROR: Binary not found at: $SOURCE_BINARY" >&2
    echo "Please build the binary first: cd tabstash-native && cargo build --release" >&2
    exit 1
fi

# Copy binary
echo "Installing binary to: $INSTALL_DIR/$BINARY_NAME"
if [ "$SUDO_REQUIRED" = true ]; then
    sudo cp "$SOURCE_BINARY" "$INSTALL_DIR/$BINARY_NAME"
    sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
else
    cp "$SOURCE_BINARY" "$INSTALL_DIR/$BINARY_NAME"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
fi

# Create data directory
echo "Creating data directory: $DATA_DIR"
mkdir -p "$DATA_DIR"

# Create manifest directory if needed
if [ ! -d "$MANIFEST_DIR" ]; then
    echo "Creating manifest directory: $MANIFEST_DIR"
    if [ "$SUDO_REQUIRED" = true ]; then
        sudo mkdir -p "$MANIFEST_DIR"
    else
        mkdir -p "$MANIFEST_DIR"
    fi
fi

# Create manifest
MANIFEST_PATH="$MANIFEST_DIR/tabstash_native.json"
MANIFEST_CONTENT=$(cat <<EOF
{
  "name": "tabstash_native",
  "description": "Native messaging helper for TabStash NVMe",
  "path": "$INSTALL_DIR/$BINARY_NAME",
  "type": "stdio",
  "allowed_origins": [
    "chrome-extension://$EXTENSION_ID/"
  ]
}
EOF
)

echo "Installing manifest to: $MANIFEST_PATH"
if [ "$SUDO_REQUIRED" = true ]; then
    echo "$MANIFEST_CONTENT" | sudo tee "$MANIFEST_PATH" > /dev/null
    sudo chmod 644 "$MANIFEST_PATH"
else
    echo "$MANIFEST_CONTENT" > "$MANIFEST_PATH"
    chmod 644 "$MANIFEST_PATH"
fi

echo ""
echo "Installation complete!"
echo ""
echo "Next steps:"
echo "1. Install the TabStash NVMe extension from Chrome Web Store"
echo "2. Open the extension popup to verify connection"
echo ""
echo "If you need to uninstall, run: sudo ./uninstall-linux.sh"

