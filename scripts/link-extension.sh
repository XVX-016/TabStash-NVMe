#!/bin/bash
# Link Extension ID to Native Host Manifest
# Usage: ./link-extension.sh "your-extension-id-here"
#        ./link-extension.sh (will prompt for ID)

set -e

# Get extension ID from argument or prompt
if [ -z "$1" ]; then
    read -p "Enter your Extension ID (32 lowercase letters): " EXTENSION_ID
else
    EXTENSION_ID="$1"
fi

echo "Linking Extension ID to Native Host..."
echo "Extension ID: $EXTENSION_ID"

# Validate extension ID format (basic check)
if ! echo "$EXTENSION_ID" | grep -qE '^[a-z]{32}$'; then
    echo "ERROR: Extension ID format is incorrect." >&2
    echo "Expected format: 32 lowercase letters (e.g., abcdefghijklmnopqrstuvwxyzabcdef)" >&2
    exit 1
fi

# Determine Chrome Native Messaging Hosts directory (standard path)
if [ -d "$HOME/.config/google-chrome" ]; then
    MANIFEST_DIR="$HOME/.config/google-chrome/NativeMessagingHosts"
elif [ -d "/etc/opt/chrome" ]; then
    MANIFEST_DIR="/etc/opt/chrome/native-messaging-hosts"
else
    echo "ERROR: Chrome installation not found." >&2
    exit 1
fi

MANIFEST_PATH="$MANIFEST_DIR/tabstash_native.json"

# Check if manifest exists
if [ ! -f "$MANIFEST_PATH" ]; then
    echo "ERROR: Native host manifest not found at:" >&2
    echo "  $MANIFEST_PATH" >&2
    echo ""
    echo "Please install the native host first:" >&2
    echo "  Run: sudo ./scripts/install-linux.sh" >&2
    exit 1
fi

# Backup manifest before editing
BACKUP_PATH="${MANIFEST_PATH}.backup"
cp "$MANIFEST_PATH" "$BACKUP_PATH"
echo "Created backup: $BACKUP_PATH"

# Read and update manifest using jq if available, otherwise use sed
EXTENSION_ORIGIN="chrome-extension://${EXTENSION_ID}/"

if command -v jq &> /dev/null; then
    # Use jq for proper JSON handling
    # Read current allowed_origins
    CURRENT_ORIGINS=$(jq -r '.allowed_origins // []' "$MANIFEST_PATH")
    
    # Remove any existing chrome-extension:// entries and add new one
    UPDATED_ORIGINS=$(echo "$CURRENT_ORIGINS" | jq --arg origin "$EXTENSION_ORIGIN" '
        map(select(. | startswith("chrome-extension://") | not)) + [$origin]
    ')
    
    # Update manifest
    jq --argjson origins "$UPDATED_ORIGINS" '.allowed_origins = $origins' "$MANIFEST_PATH" > "${MANIFEST_PATH}.tmp"
    mv "${MANIFEST_PATH}.tmp" "$MANIFEST_PATH"
    
    echo "Updated manifest using jq"
else
    # Fallback to sed (less robust but works without jq)
    # This assumes manifest has allowed_origins array
    if grep -q "chrome-extension://" "$MANIFEST_PATH"; then
        # Replace existing extension ID
        sed -i.bak "s|chrome-extension://[a-z]\{32\}/|$EXTENSION_ORIGIN|g" "$MANIFEST_PATH"
        rm -f "${MANIFEST_PATH}.bak"
        echo "Updated existing extension ID in manifest"
    else
        # Add extension ID (this is a simple approach - may break JSON formatting)
        echo "WARNING: jq not found. Using sed fallback which may not preserve JSON formatting perfectly." >&2
        # Try to add before the closing bracket of allowed_origins
        sed -i.bak "s|\"allowed_origins\":\\s*\\[|\"allowed_origins\": [\"$EXTENSION_ORIGIN\",|" "$MANIFEST_PATH"
        rm -f "${MANIFEST_PATH}.bak"
        echo "Added extension ID to manifest (sed fallback)"
    fi
fi

# Validate JSON
if ! python3 -m json.tool "$MANIFEST_PATH" > /dev/null 2>&1; then
    echo "ERROR: Updated manifest has invalid JSON. Restoring backup..." >&2
    cp "$BACKUP_PATH" "$MANIFEST_PATH"
    exit 1
fi

echo ""
echo "Extension ID linked successfully!"
echo ""
echo "Next steps:"
echo "1. Close all Chrome windows completely"
echo "2. Reopen Chrome"
echo "3. Open the extension popup to verify connection"
echo ""

