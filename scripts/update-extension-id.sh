#!/bin/bash
# Update Extension ID in Installer Scripts
# Usage: ./update-extension-id.sh "your-extension-id-here"

set -e

if [ -z "$1" ]; then
    echo "ERROR: Extension ID required" >&2
    echo "Usage: ./update-extension-id.sh \"your-extension-id-here\"" >&2
    exit 1
fi

EXTENSION_ID="$1"

# Validate extension ID format (basic check)
if ! echo "$EXTENSION_ID" | grep -qE '^[a-z]{32}$'; then
    echo "WARNING: Extension ID format may be incorrect." >&2
    echo "Expected format: 32 lowercase letters (e.g., abcdefghijklmnopqrstuvwxyzabcdef)" >&2
    read -p "Continue anyway? (y/N) " confirm
    if [ "$confirm" != "y" ] && [ "$confirm" != "Y" ]; then
        exit 1
    fi
fi

echo "Updating Extension ID to: $EXTENSION_ID"

FILES_UPDATED=0

# Update Linux installer script
if [ -f "scripts/install-linux.sh" ]; then
    if sed -i.bak "s/EXTENSION_ID_PLACEHOLDER/$EXTENSION_ID/g" scripts/install-linux.sh; then
        rm -f scripts/install-linux.sh.bak
        echo "  Updated: Linux installer script"
        FILES_UPDATED=$((FILES_UPDATED + 1))
    fi
fi

# Update native messaging manifest template
if [ -f "native-host/config/tabstash-native.json.template" ]; then
    if sed -i.bak "s/EXTENSION_ID_PLACEHOLDER/$EXTENSION_ID/g" native-host/config/tabstash-native.json.template; then
        rm -f native-host/config/tabstash-native.json.template.bak
        echo "  Updated: Native messaging manifest template"
        FILES_UPDATED=$((FILES_UPDATED + 1))
    fi
fi

echo ""
if [ $FILES_UPDATED -gt 0 ]; then
    echo "Successfully updated $FILES_UPDATED file(s)!"
    echo ""
    echo "Next steps:"
    echo "1. Test installer with published extension"
    echo "2. Update Windows installer script manually if needed"
    echo "3. Upload updated installer to GitHub Releases"
else
    echo "No files were updated. Check extension ID format and file paths."
fi

