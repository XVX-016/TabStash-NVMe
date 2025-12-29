# Release Notes - v0.1.0-devmode

**Release Date:** [TBD]  
**Release Type:** Developer Mode (GitHub Distribution)  
**Status:** Initial Release

---

## Important: Developer Mode Required

This release **requires Chrome Developer Mode** installation. This is not a Chrome Web Store release.

**What this means:**
- You must enable Developer Mode in Chrome
- You must load the extension from source code
- Chrome will show warnings (this is expected)
- Manual updates required (no auto-updates)

**Why Developer Mode:**
- Faster iteration and feedback
- No store review delays
- Power users comfortable with dev tools
- Real-world validation before store submission

---

## What's New

### Core Features
- **Tab Offloading**: Offload inactive tabs to NVMe SSD for reduced RAM usage
- **Native Messaging**: Secure communication between extension and native host
- **High Performance**: Direct NVMe storage for maximum speed

### Developer Experience
- **Extension ID Linking**: One-command script to link extension to native host
- **Installation Status Panel**: Visual checklist showing installation progress
- **Self-Test Mode**: `tabstash-native --self-test` for standalone verification
- **Clear Error Messages**: Actionable error messages with exact fix steps

### User Interface
- **Status Indicators**: Real-time connection status (OK/FAIL/CHECKING)
- **Extension ID Display**: Copy-to-clipboard extension ID in popup
- **Developer Mode Banner**: Clear indication of dev mode installation
- **Installation Checklist**: 4-item status panel for diagnostics

---

## Installation

See [README.md](../README.md#installation-developer-mode) for complete installation instructions.

**Quick Start:**
1. Clone repository
2. Install native host: `scripts/install-windows.ps1` (Windows) or `sudo scripts/install-linux.sh` (Linux)
3. Load extension in Developer Mode
4. Link extension ID: `scripts/link-extension.ps1 "your-id"`
5. Restart Chrome
6. Verify in extension popup

**Installation Time:** ~10 minutes for technical users

---

## Platform Support

- **Windows 10/11**
- **Linux (Ubuntu/Debian)**
- **Chrome Browser**

**Not Supported (vNext):**
- macOS
- Firefox
- Edge/Brave (Chrome paths only for now)

---

## Known Limitations

### v0.1.0 Limitations
- **Developer Mode Required**: Cannot be installed from Chrome Web Store
- **Manual Updates**: Must pull new code and reload extension
- **Chrome Only**: Edge/Brave support planned for vNext
- **Standard Paths Only**: Custom Chrome profile paths not auto-detected
- **Single Profile**: Multi-profile support planned for vNext

### Expected Behavior
- **Chrome Warnings**: Developer Mode warnings are normal and cannot be disabled
- **Extension ID Changes**: ID changes if you uninstall/reinstall (not on reload)
- **Browser Restart Required**: After linking extension ID, full browser restart needed

---

## Troubleshooting

### Common Issues

**"Native host not available"**
- Run installer: `scripts/install-windows.ps1`
- Verify manifest exists at standard Chrome path
- Check Installation Status panel for specific issue

**"Extension ID mismatch"**
- Copy your extension ID from popup
- Run: `scripts/link-extension.ps1 "your-extension-id"`
- **Restart Chrome completely** (close all windows)

**"Health check failed"**
- Verify native host installed
- Check extension ID is linked
- Ensure browser was restarted after linking
- Run self-test: `tabstash-native --self-test`

**Installation takes too long**
- Follow README exactly (no shortcuts)
- Check Installation Status panel for blockers
- See [Troubleshooting](../README.md#troubleshooting) section

---

## Technical Details

### Protocol Version
- **Protocol Version:** 1
- **Extension Version:** 0.1.0
- **Native Host Version:** 0.1.0

### Architecture
- Browser Extension (Manifest V3)
- Native Host Binary (Rust)
- Native Messaging (stdio IPC)
- NVMe Storage (sled database)

### Security
- Local-only operation (no network)
- No telemetry or data collection
- Open source (full code review)
- Native host requires explicit extension ID binding

---

## What's Next (v0.2)

Planned for next release:
- Edge/Brave browser support
- Custom profile path detection
- Performance optimizations
- Enhanced error recovery

**Chrome Web Store:** Will be considered after v0.1 validation (1-2 weeks of real usage).

---

## Feedback & Support

- **GitHub Issues**: Report bugs or ask questions
- **Documentation**: See [docs/DEV_INSTALL.md](DEV_INSTALL.md) for deep-dive
- **Self-Test**: Run `tabstash-native --self-test` for diagnostics

---

## Changelog

### v0.1.0-devmode (Initial Release)
- Initial GitHub + Developer Mode distribution
- Extension ID detection and linking
- Installation status panel
- Native host self-test
- Version handshake
- Comprehensive error handling
- Windows + Linux support

---

**Thank you for trying TabStash NVMe!**

This is an early release. Your feedback helps shape the future of the project.

