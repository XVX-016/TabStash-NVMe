# Privacy Policy

**Last Updated**: 2024

## Overview

TabStash NVMe is committed to protecting your privacy. This policy explains what data we collect, how we use it, and your rights.

## Data Collection

**We collect ZERO data.**

- No personal information
- No browsing history
- No analytics
- No telemetry
- No usage statistics
- No crash reports (unless you explicitly opt-in)

## Data Storage

### What is Stored

TabStash NVMe stores tab snapshots locally on your device:

- HTML content of tabs
- JavaScript state
- Tab metadata (URL, title, etc.)

### Where is it Stored

All data is stored **locally on your device**:

- **Windows**: `%LOCALAPPDATA%\TabStash\snapshots`
- **Linux**: `~/.local/share/tabstash/snapshots`

### Who Has Access

- **You**: Full access to all your data
- **Us**: Zero access (we have no servers, no data collection)
- **Third parties**: None (no data sharing)

## Data Transmission

**No data is transmitted anywhere.**

- No network requests
- No remote servers
- No cloud storage
- No external APIs

The native host binary has no network capabilities whatsoever.

## Permissions Explained

### `tabs` Permission
- **Purpose**: Read tab state to create snapshots
- **Scope**: Only active tabs you choose to offload
- **Data**: Tab content and metadata
- **Storage**: Local only

### `storage` Permission
- **Purpose**: Store extension settings and tab metadata
- **Scope**: Browser extension storage (local)
- **Data**: Extension preferences, tab list
- **Storage**: Browser's local storage API

### `scripting` Permission
- **Purpose**: Restore tab state when reloading
- **Scope**: Only tabs you've offloaded
- **Data**: Tab restoration scripts
- **Storage**: Not applicable (runtime only)

### `nativeMessaging` Permission
- **Purpose**: Communicate with local native host
- **Scope**: Local process communication only
- **Data**: Tab snapshot requests/responses
- **Storage**: Not applicable (in-memory only)

## Third-Party Services

**We use ZERO third-party services.**

- No analytics providers
- No advertising networks
- No cloud services
- No CDNs (extension is self-contained)

## Data Retention

- **Active data**: Stored until you delete it or uninstall
- **No automatic deletion**: You control when data is removed
- **Uninstall**: Data remains on disk (you can manually delete)

## Your Rights

You have complete control:

1. **Access**: All data is in accessible file locations
2. **Delete**: Remove data anytime by deleting the snapshots directory
3. **Export**: Copy files directly (they're in standard formats)
4. **Opt-out**: Simply uninstall the extension

## Children's Privacy

TabStash NVMe is not intended for children under 13. We don't collect any data, so this is not a concern, but we want to be clear.

## Changes to This Policy

If we make changes to this policy:

1. We'll update the "Last Updated" date
2. Significant changes will be noted in release notes
3. Continued use implies acceptance

## Contact

For privacy questions or concerns:

- **GitHub Issues**: [Your repo]/issues (public)
- **Email**: [your-email@domain.com] (update with your contact)

## Compliance

- **GDPR**: Compliant (no data collection, local-only)
- **CCPA**: Compliant (no data collection, local-only)
- **COPPA**: Compliant (no data collection)

Since we collect zero data, privacy regulations are easily satisfied.

## Summary

**TL;DR**: We don't collect, store, transmit, or share any data. Everything is local to your device. You're in complete control.

