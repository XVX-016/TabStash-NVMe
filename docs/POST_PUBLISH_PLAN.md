# Post-Publish Hardening Plan

This document outlines enhancements to implement after the initial release.

## Auto-Update Infrastructure

### Version Checking

**Goal**: Automatically check for native host updates

**Implementation**:
1. Add version endpoint or GitHub API check
2. Compare installed version with latest release
3. Notify user if update available
4. Provide download link

**Files to Modify**:
- `extension/src/background/index.js`: Add version check logic
- `extension/src/popup/index.html`: Add update notification UI
- `tabstash-native/src/protocol.rs`: Add version to protocol

**GitHub API Endpoint**:
```
GET https://api.github.com/repos/your-repo/releases/latest
```

**Version Storage**:
- Store in native host binary (compiled in)
- Store in extension manifest
- Compare on connection

### Update Notification

**UI Design**:
- Show update badge in extension popup
- Display update notification when update available
- Link to GitHub releases page
- Show changelog for new version

**User Experience**:
- Non-intrusive notification
- Clear update instructions
- Optional: Auto-download installer (future)

## Compatibility Checks

### Protocol Versioning

**Goal**: Handle protocol changes gracefully

**Implementation**:
1. Add protocol version to messages
2. Check version on connection
3. Handle version mismatches
4. Provide migration path

**Protocol Changes**:
```rust
// Add to protocol
pub struct ProtocolVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

// Include in handshake
pub struct Handshake {
    pub version: ProtocolVersion,
    pub capabilities: Vec<String>,
}
```

**Version Mismatch Handling**:
- **Major mismatch**: Incompatible, show error
- **Minor mismatch**: May work, show warning
- **Patch mismatch**: Compatible, continue

### Extension-Native Host Compatibility

**Check on Connection**:
1. Extension sends version in first message
2. Native host responds with its version
3. Compare versions
4. Handle mismatches appropriately

**Error Messages**:
- "Native host version outdated. Please update."
- "Extension version outdated. Please update."
- "Version mismatch. Some features may not work."

## Migration Logic

### Protocol Changes

**Scenario**: Protocol changes between versions

**Solution**:
1. Maintain backward compatibility for N versions
2. Support multiple protocol versions
3. Migrate data if needed
4. Document breaking changes

**Migration Steps**:
1. Detect old format
2. Convert to new format
3. Update metadata
4. Verify migration success

### Data Format Changes

**Scenario**: Snapshot format changes

**Solution**:
1. Detect old format snapshots
2. Migrate to new format on access
3. Or: Migrate all on startup (slower but cleaner)
4. Keep old format reader for compatibility

## Error Reporting (Optional)

### User Opt-In

**Goal**: Collect error reports to improve stability

**Implementation**:
1. Add opt-in toggle in extension popup
2. Collect error logs (no personal data)
3. Send to error reporting service (Sentry, etc.)
4. Anonymize data

**Data Collected**:
- Error messages
- Stack traces
- System info (OS, Chrome version)
- Extension version
- Native host version

**Privacy**:
- Opt-in only
- No personal data
- No browsing history
- Anonymized

### Local Error Logging

**Goal**: Help users debug issues

**Implementation**:
1. Log errors to local file
2. Provide "Export logs" button
3. Users can share logs for support

**Log Location**:
- Windows: `%LOCALAPPDATA%\TabStash\logs\`
- Linux: `~/.local/share/tabstash/logs/`

## Performance Monitoring

### Metrics Collection (Local)

**Goal**: Track performance without external services

**Metrics**:
- Snapshot creation time
- Snapshot restoration time
- Storage usage
- Compression ratio
- Error rates

**Storage**:
- Store in local database
- Display in extension popup (optional)
- Export for analysis

## Future Enhancements

### Snapshot Encryption

**Implementation**:
1. Add encryption option (user choice)
2. Use AES-256 encryption
3. Store key securely (OS keychain)
4. Encrypt on write, decrypt on read

**User Control**:
- Toggle in extension settings
- Per-snapshot encryption option
- Key management UI

### Automatic Cleanup

**Goal**: Prevent disk space issues

**Implementation**:
1. Configurable retention policy
2. Delete snapshots older than X days
3. Delete snapshots exceeding size limit
4. User-configurable thresholds

**Settings**:
- Max age: 30/60/90 days or custom
- Max total size: 1GB/5GB/10GB or custom
- Cleanup schedule: Daily/Weekly/Manual

### Incremental Snapshots

**Goal**: Reduce storage usage

**Implementation**:
1. Store only changes from previous snapshot
2. Reconstruct full snapshot on restore
3. More complex but saves space

**Challenges**:
- More complex implementation
- Higher CPU usage
- Potential for corruption

### Snapshot Versioning

**Goal**: Keep multiple versions of same tab

**Implementation**:
1. Store version history
2. Allow restoring previous versions
3. Configurable version count

## Testing Infrastructure

### Automated Testing

**Goal**: Ensure quality with automated tests

**Tests Needed**:
- Unit tests (Rust)
- Integration tests (native messaging)
- End-to-end tests (full workflow)
- Cross-platform tests

**CI/CD**:
- GitHub Actions
- Test on Windows and Linux
- Build and test on each commit
- Release automation

### Beta Testing

**Goal**: Get feedback before public release

**Implementation**:
1. Create beta release channel
2. Recruit beta testers
3. Collect feedback
4. Iterate before public release

## Documentation Improvements

### User Guide

**Goal**: Comprehensive user documentation

**Sections**:
- Getting started
- Configuration
- Troubleshooting
- Advanced usage
- FAQ

### Developer Guide

**Goal**: Help contributors

**Sections**:
- Architecture overview
- Development setup
- Code structure
- Testing
- Contributing

## Security Enhancements

### Code Signing

**Goal**: Eliminate SmartScreen warnings

**Implementation**:
1. Obtain code signing certificate
2. Sign Windows installer
3. Sign native host binary
4. Update release process

**Cost**: ~$200-400/year for certificate

### Security Audit

**Goal**: Identify and fix vulnerabilities

**Implementation**:
1. Regular security reviews
2. Dependency updates
3. Vulnerability scanning
4. Penetration testing (optional)

## Roadmap

### v0.2.0 (Next Release)
- [ ] Auto-update checking
- [ ] Protocol versioning
- [ ] Local error logging
- [ ] Performance metrics

### v0.3.0
- [ ] Snapshot encryption
- [ ] Automatic cleanup
- [ ] Improved error handling

### v1.0.0
- [ ] Firefox support
- [ ] macOS support
- [ ] Incremental snapshots
- [ ] Full test coverage

