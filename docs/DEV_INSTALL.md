# Developer Mode Installation Guide

This guide explains the Developer Mode installation process for TabStash NVMe, including why it's required and how it works.

## Why Developer Mode?

TabStash NVMe uses Chrome's Native Messaging API to communicate with a native host binary. This requires:

1. **Extension ID Binding**: The native host manifest must explicitly list which extension IDs are allowed to connect. This is a security feature.

2. **No Chrome Web Store**: For early releases, we distribute via GitHub to:
   - Enable faster iteration
   - Avoid premature compliance work
   - Get real-world feedback before store submission
   - Allow power users to use cutting-edge features

3. **Developer Mode Required**: Unpacked extensions (loaded from source) require Developer Mode to be enabled. This is a Chrome security requirement and cannot be bypassed.

## Security Model

### What Developer Mode Means

- **Chrome Warnings**: Chrome will display warnings about "extensions in developer mode can be dangerous"
  - This is **expected behavior** and cannot be removed
  - It's Chrome's way of warning users about unpacked extensions
  - The warning appears because the extension isn't from the Chrome Web Store

### What You Should Know

- **Source Code Access**: You have full access to the extension source code
- **No Auto-Updates**: You must manually update by pulling new code and reloading
- **Trust Required**: You're trusting the code you download from GitHub
- **Native Host**: The native host binary has system-level access (disk I/O, file system)

### Is It Safe?

- **Yes, if you trust the source**: If you trust the GitHub repository, Developer Mode is safe
- **Review the code**: All code is open source - you can review it before installing
- **No network access**: The native host has no network capabilities
- **Local-only**: All data stays on your device

## Extension ID System

### Why Extension IDs Matter

Chrome assigns each extension a unique 32-character ID:
- **Store extensions**: Get a fixed ID that never changes
- **Unpacked extensions**: Get a random ID that changes per machine/installation

The native host manifest must list your specific extension ID to allow connections. This is why we need the linking step.

### How It Works

```
1. You load unpacked extension → Chrome generates random ID
2. Extension ID is shown in popup → You copy it
3. Linking script updates manifest → Adds your ID to allowed_origins
4. Browser restart → Chrome reads updated manifest
5. Connection works → Extension ID matches manifest
```

### Extension ID Format

- **Format**: 32 lowercase letters (e.g., `abcdefghijklmnopqrstuvwxyzabcdef`)
- **Location**: 
  - `chrome://extensions` (under extension name)
  - Extension popup (click extension icon)
- **Validation**: The linking scripts validate the format before updating

## Installation Process Deep Dive

### Step-by-Step Breakdown

1. **Clone/Download Repository**
   - Get the source code from GitHub
   - This gives you the extension code and native host binary

2. **Install Native Host**
   - The installer script:
     - Copies binary to system location
     - Creates data directory
     - Installs manifest file (with placeholder extension ID)
   - Requires administrator/sudo privileges

3. **Load Extension**
   - Developer Mode enables "Load unpacked"
   - Chrome loads extension from `extension/` directory
   - Chrome generates random extension ID

4. **Link Extension ID**
   - Copy your extension ID
   - Run linking script with your ID
   - Script updates manifest `allowed_origins` array
   - Manifest now allows your extension to connect

5. **Restart Browser**
   - **Critical step**: Chrome caches manifest files
   - Full restart (close all windows) forces Chrome to reload manifest
   - Without restart, connection will fail even if ID matches

6. **Verify**
   - Installation Status panel shows all checks
   - Each item must show OK for full functionality

## Troubleshooting

### Extension ID Mismatch

**Symptom**: Popup shows "Extension ID mismatch detected"

**Cause**: Your extension ID doesn't match the manifest

**Fix**:
1. Copy your extension ID from popup
2. Run: `scripts/link-extension.ps1 "your-id"` (Windows) or `scripts/link-extension.sh "your-id"` (Linux)
3. **Restart Chrome completely**

### Native Host Not Found

**Symptom**: "Native host manifest not found"

**Cause**: Native host not installed or manifest in wrong location

**Fix**:
1. Run installer: `scripts/install-windows.ps1` or `sudo scripts/install-linux.sh`
2. Verify manifest exists at standard path
3. Check installer completed successfully

### Connection Timeout

**Symptom**: "Native host did not respond within 5 seconds"

**Cause**: Native host process crashed or not running

**Fix**:
1. Run self-test: `tabstash-native --self-test`
2. Check for error messages
3. Verify binary is executable
4. Try reinstalling native host

### Manifest Parse Errors

**Symptom**: Linking script fails with JSON error

**Cause**: Manifest file corrupted or invalid

**Fix**:
1. Check backup file: `tabstash_native.json.backup`
2. Restore from backup if needed
3. Re-run installer to recreate manifest
4. Then run linking script again

## Advanced Topics

### Multiple Browsers (vNext)

Currently, we support Chrome only. Edge and Brave use similar manifest locations but may require:
- Different manifest paths
- Browser-specific linking scripts

This is planned for a future release.

### Multiple Profiles (vNext)

Chrome supports multiple user profiles. Currently:
- We use the default profile's manifest location
- Custom profile paths are not auto-detected
- If using custom profile, manually update manifest path in scripts

This is planned for a future release.

### Updating the Extension

When you pull new code:
1. **Reload extension** (not reinstall):
   - Go to `chrome://extensions`
   - Click reload icon on extension card
   - Extension ID stays the same
   - No need to re-link

2. **If extension ID changes** (shouldn't happen):
   - Run linking script again with new ID
   - Restart browser

### Updating Native Host

When native host binary is updated:
1. Rebuild: `cd tabstash-native && cargo build --release`
2. Reinstall: Run installer script again
3. Extension ID stays linked (manifest preserved)
4. No browser restart needed (binary update only)

## Screenshots (Placeholders)

### Developer Mode Toggle
*Screenshot: chrome://extensions page showing Developer mode toggle in top-right*

### Load Unpacked Dialog
*Screenshot: File picker dialog showing extension/ directory selection*

### Extension ID Location
*Screenshot: chrome://extensions showing extension ID under extension name*

### Installation Status Panel
*Screenshot: Extension popup showing Installation Status checklist with all OK*

## FAQ

**Q: Can I disable the Developer Mode warning?**  
A: No, this is a Chrome security feature and cannot be disabled.

**Q: Will my extension ID change?**  
A: Only if you uninstall and reinstall the extension. Reloading keeps the same ID.

**Q: Do I need to re-link after updating code?**  
A: No, reloading the extension keeps the same ID. Only reinstall requires re-linking.

**Q: Can I use this with Edge/Brave?**  
A: Not yet - Chrome only for now. Edge/Brave support is planned (vNext).

**Q: What if I have multiple Chrome profiles?**  
A: Currently uses default profile. Custom profile support is planned (vNext).

**Q: Is this safe for production use?**  
A: Yes, if you trust the source code. Review the code before installing.

## Getting Help

- **GitHub Issues**: Report bugs or ask questions
- **README**: Check main README.md for installation steps
- **Self-Test**: Run `tabstash-native --self-test` to diagnose issues
- **Installation Status**: Use extension popup's status panel for diagnostics

