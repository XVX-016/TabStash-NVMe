#!/bin/bash
# TabStash NVMe Linux Uninstaller Script
# Requires: sudo privileges (for system-wide installation)
# Usage: sudo ./uninstall-linux.sh

set -e

BINARY_NAME="tabstash-native"
INSTALL_DIR="/usr/local/bin"
DATA_DIR="$HOME/.local/share/tabstash/snapshots"

# Determine Chrome Native Messaging Hosts directory
if [ -d "$HOME/.config/google-chrome/NativeMessagingHosts" ]; then
    MANIFEST_DIR="$HOME/.config/google-chrome/NativeMessagingHosts"
    SUDO_REQUIRED=false
elif [ -d "/etc/opt/chrome/native-messaging-hosts" ]; then
    MANIFEST_DIR="/etc/opt/chrome/native-messaging-hosts"
    SUDO_REQUIRED=true
else
    MANIFEST_DIR=""
fi

echo "Uninstalling TabStash NVMe Native Host..."

# Remove binary
if [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
    echo "Removing binary: $INSTALL_DIR/$BINARY_NAME"
    if [ "$SUDO_REQUIRED" = true ]; then
        sudo rm -f "$INSTALL_DIR/$BINARY_NAME"
    else
        rm -f "$INSTALL_DIR/$BINARY_NAME"
    fi
fi

# Remove manifest
if [ -n "$MANIFEST_DIR" ]; then
    MANIFEST_PATH="$MANIFEST_DIR/tabstash_native.json"
    if [ -f "$MANIFEST_PATH" ]; then
        echo "Removing manifest: $MANIFEST_PATH"
        if [ "$SUDO_REQUIRED" = true ]; then
            sudo rm -f "$MANIFEST_PATH"
        else
            rm -f "$MANIFEST_PATH"
        fi
    fi
fi

# Note: Data directory is NOT removed by default (preserves user data)
if [ -d "$DATA_DIR" ]; then
    echo ""
    echo "NOTE: Data directory preserved: $DATA_DIR"
    echo "To remove user data, manually delete this directory."
fi

echo ""
echo "Uninstallation complete!"

