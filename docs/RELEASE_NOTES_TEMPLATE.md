# Release Notes Template

## v0.1.0 - Initial Release

### Features

- ✅ Automatic tab offloading to NVMe SSD
- ✅ Fast tab restoration (< 200ms typical)
- ✅ Memory savings for inactive tabs
- ✅ Local-only storage (zero network access)
- ✅ Health check UI in extension popup
- ✅ Windows and Linux support
- ✅ Chrome browser support

### Installation

**Important**: This extension requires a separate native host installation.

1. **Install Native Host:**
   - Windows: Download and run `tabstash-installer-windows.exe` from [Releases](https://github.com/your-repo/releases)
   - Linux: Run `sudo ./scripts/install-linux.sh`

2. **Install Extension:**
   - Install from [Chrome Web Store](https://chrome.google.com/webstore/detail/tabstash-nvme/EXTENSION_ID) (link after approval)
   - Or load unpacked from `extension/` directory for development

3. **Verify:**
   - Open extension popup
   - Check that status shows "Native host connected"

### System Requirements

- Chrome browser (Windows 10/11 or Linux)
- NVMe SSD (recommended for optimal performance)
- Administrator/sudo access for native host installation

### Known Limitations

- Chrome-only (Firefox support planned)
- Windows/Linux only (macOS planned)
- Requires manual native host installation
- No automatic updates for native host (yet)
- No encryption (local storage only)

### Security & Privacy

- ✅ Zero data collection
- ✅ No network access
- ✅ Local-only storage
- ✅ Open source code

See [SECURITY.md](../SECURITY.md) and [PRIVACY.md](../PRIVACY.md) for details.

### Documentation

- [Installation Guide](../README.md#installation)
- [Architecture Overview](../ARCHITECTURE.md)
- [Troubleshooting](../README.md#troubleshooting)

### Support

- **Issues**: [GitHub Issues](https://github.com/your-repo/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-repo/discussions)

### Credits

Built with:
- Rust (native host)
- Chrome Extension API
- sled (embedded database)
- zstd (compression)

---

## Future Releases

### Planned for v0.2.0

- Automatic native host updates
- Snapshot encryption option
- Automatic cleanup of old snapshots
- Performance improvements

### Planned for v1.0.0

- Firefox support
- macOS support
- Incremental snapshots
- Snapshot versioning
- Cloud backup (optional)

