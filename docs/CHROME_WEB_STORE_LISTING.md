# Chrome Web Store Listing

## Short Description (132 characters max)

Offload inactive tabs to NVMe SSD for reduced RAM usage and faster browser performance.

## Detailed Description

### TabStash NVMe - High-Performance Tab Offloading

Reduce browser RAM usage by offloading inactive tabs to your NVMe SSD. TabStash NVMe provides lightning-fast tab restoration while freeing up memory for active browsing.

**Key Features:**
- **Fast Performance**: NVMe SSD storage for instant tab restoration
- **Memory Savings**: Free up RAM by offloading inactive tabs
- **Privacy First**: All data stored locally, zero network access
- **Lightweight**: Minimal resource usage, maximum efficiency

**How It Works:**
1. Automatically detects inactive tabs
2. Creates compressed snapshots stored on NVMe SSD
3. Unloads tabs from memory
4. Restores instantly when you need them

**Requirements:**
- Chrome browser (Windows or Linux)
- Native host installation (separate installer required)
- NVMe SSD recommended for optimal performance

**Privacy & Security:**
- Local-only storage (no cloud, no servers)
- Zero data collection or telemetry
- No network access from native host
- Open source code available for review

**Installation:**
1. Install the native host from GitHub Releases
2. Install this extension from Chrome Web Store
3. Open extension popup to verify connection

**Note**: This extension requires a separate native host installation. The native host enables direct NVMe access for optimal performance. Installation instructions are provided in the extension popup and on GitHub.

## Category

**Productivity**

## Language

English (en)

## Permissions Justification

### `tabs` Permission
**Why needed**: To read tab state and create snapshots of inactive tabs for offloading to NVMe storage.

**What we access**: Only tab metadata (URL, title) and content when creating snapshots. We do not access tabs you haven't chosen to offload.

**Data usage**: Tab content is stored locally on your device only. Never transmitted or shared.

### `storage` Permission
**Why needed**: To store extension settings and metadata about offloaded tabs (which tabs are offloaded, snapshot IDs).

**What we store**: Extension preferences and a list of offloaded tab IDs. No personal data or browsing history.

**Data location**: Browser's local storage API (local to your device).

### `scripting` Permission
**Why needed**: To restore tab state when reloading offloaded tabs. Required to inject restoration scripts.

**What we do**: Only injects restoration scripts into tabs you're restoring. Never modifies other tabs.

**Scope**: Only tabs that were previously offloaded by you.

### `nativeMessaging` Permission
**Why needed**: To communicate with the native host binary that manages NVMe storage. This is essential for the extension's core functionality.

**What we do**: Sends tab snapshot requests and receives stored data. All communication is local (stdio pipe).

**Security**: Native host has zero network capabilities. All data stays on your device.

## Privacy Practices

### Single Purpose
TabStash NVMe has a single, clear purpose: offload inactive browser tabs to NVMe storage to reduce RAM usage.

### Data Handling
- **Collection**: We collect zero data. No analytics, no telemetry, no personal information.
- **Storage**: Tab snapshots are stored locally on your device only.
- **Transmission**: No data is transmitted anywhere. The native host has no network capabilities.
- **Sharing**: We share nothing because we collect nothing.

### User Data
- **Location**: All data stored in `%LOCALAPPDATA%\TabStash\snapshots` (Windows) or `~/.local/share/tabstash/snapshots` (Linux)
- **Access**: You have full access to all your data. Files are in standard formats.
- **Deletion**: Delete the snapshots directory anytime to remove all data.
- **Backup**: Simply copy the snapshots directory to backup your data.

### Security
- **Network**: Native host has zero network access
- **Encryption**: Data is compressed but not encrypted (local storage acceptable)
- **Permissions**: Extension only accesses tabs you choose to offload
- **Code**: Open source, fully auditable

## Store Images

### Required Images

1. **Icon** (128x128)
   - Professional icon representing tab management/storage
   - Works on light and dark backgrounds

2. **Small Tile** (440x280)
   - Screenshot of extension popup showing connected status
   - Or: Visual representation of tab offloading concept

3. **Marquee** (920x680)
   - Main promotional image
   - Shows extension in action
   - Highlights key benefits (RAM savings, speed)

### Optional Images

4. **Promotional Images** (1400x560)
   - Additional marketing visuals
   - Feature highlights
   - Use cases

## Promotional Text

**Title**: Reduce RAM Usage with NVMe-Powered Tab Offloading

**Description**: 
TabStash NVMe automatically offloads inactive tabs to your NVMe SSD, freeing up RAM while maintaining instant restore times. Perfect for power users who keep many tabs open.

**Highlights**:
- Automatic tab offloading
- NVMe SSD storage for speed
- Local-only, zero data collection
- Open source and privacy-focused

## Support Information

**Support URL**: https://github.com/your-repo/issues

**Homepage URL**: https://github.com/your-repo

**Privacy Policy URL**: https://github.com/your-repo/blob/main/PRIVACY.md

## Additional Notes for Reviewers

### Native Messaging Usage
This extension uses Chrome's Native Messaging API to communicate with a separate native host binary. This is necessary because:

1. Browser extensions cannot directly access NVMe storage with the required performance
2. The native host enables efficient file I/O operations
3. This architecture is standard for extensions requiring native capabilities

The native host:
- Has zero network capabilities
- Only communicates with this extension (via extension ID whitelist)
- Stores all data locally
- Is open source and available for review

### Installation Process
Users must install the native host separately because:
- Chrome Web Store does not allow binary distribution
- Native binaries require platform-specific installation
- This is standard practice for native messaging extensions

We provide clear installation instructions in:
- Extension popup (when native host not detected)
- GitHub README
- Installation scripts for Windows and Linux

### Security Considerations
- Native host runs with user permissions only (no elevation)
- All communication is local (stdio pipe)
- Input validation on all messages
- No code execution from remote sources
- Open source for transparency

