# Scope Definition - v0.1.0-devmode

This document explicitly defines what is **in scope** and **out of scope** for v0.1.0-devmode release.

## In Scope

### Distribution
- GitHub repository distribution
- Developer Mode (Load Unpacked) installation
- Manual installation process
- Clear installation documentation

### Platforms
- Windows 10/11
- Linux (Ubuntu/Debian)
- Chrome browser (primary)

### Features
- Extension ID detection and display
- Extension ID linking via scripts
- Installation status panel
- Developer mode detection and banner
- Native host self-test (`--self-test`)
- Version handshake
- Basic error handling with actionable messages

### User Experience
- Installation checklist UI
- Clear error messages with fix steps
- Extension ID copy-to-clipboard
- Status indicators (OK/FAIL/CHECKING)

### Technical
- Native Messaging protocol v1
- IPC-based manifest info retrieval
- Timeout handling (5 seconds)
- Error storage and retrieval

---

## Out of Scope (vNext)

### Distribution
- Chrome Web Store submission
- Auto-updates
- Installer with auto-update check

### Platforms
- macOS support
- Firefox support
- Edge/Brave auto-detection (Chrome paths only)

### Features
- Multiple browser detection
- Custom profile path auto-detection
- Zero-restart extension ID linking
- Advanced profile management
- Multi-profile support

### User Experience
- Silent installation
- One-click install
- In-app update notifications
- Telemetry/analytics

### Technical
- Protocol version 2+
- Network capabilities
- Cloud sync
- Advanced error recovery

---

## Rationale

**Why GitHub + Developer Mode First:**
- Faster iteration cycles
- No store review delays
- Real-world validation before store submission
- Power users comfortable with dev mode

**Why Not Chrome Web Store Yet:**
- Need to validate UX with real users first
- Store submission is paperwork if UX is solid
- Can iterate faster outside store constraints

**Why Not Multi-Browser Yet:**
- Chrome is primary target
- Edge/Brave can be added after validation
- Reduces initial complexity

**Why Not Auto-Updates:**
- Developer Mode extensions don't auto-update by design
- Users pulling from GitHub can update manually
- Store version (vNext) will have auto-updates

---

## Migration Path

**v0.1 → v0.2 (if successful):**
- Add Edge/Brave support
- Improve error messages based on feedback
- Performance optimizations

**v0.1 → Chrome Web Store (when ready):**
- UX friction < 5%
- Failure rate < 5%
- Installer + status panel eliminate support burden
- Then store submission becomes paperwork

---

## Change Control

**To add features to v0.1:**
- Must be critical bug fix
- Must not break existing functionality
- Must not delay release > 1 week

**To remove features from v0.1:**
- Must document in release notes
- Must provide migration path
- Must not break core functionality

---

**Document Version:** 1.0  
**Last Updated:** [Release Date]  
**Status:** FROZEN (no changes without lead approval)

