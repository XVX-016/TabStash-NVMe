# Security Policy

## Supported Versions

We provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Security Model

TabStash NVMe is designed with security and privacy as core principles.

### Data Storage

- **What is stored**: Tab snapshots (HTML content, JavaScript state, tab metadata)
- **Where stored**: Local NVMe SSD only
  - Windows: `%LOCALAPPDATA%\TabStash\snapshots`
  - Linux: `~/.local/share/tabstash/snapshots`
- **Encryption**: Data is compressed but not encrypted by default (local storage only)
- **Data retention**: Data persists until manually deleted or uninstalled

### Network Access

- **No network access**: The native host binary has zero network capabilities
- **No remote servers**: All operations are local-only
- **No telemetry**: No data is sent to external servers
- **No analytics**: No usage tracking or analytics collection

### Permissions

The extension requires the following permissions:

- `tabs`: Required to read tab state for offloading
- `storage`: Required to store tab metadata in browser storage
- `scripting`: Required to restore tab state when reloading
- `nativeMessaging`: Required to communicate with native host for NVMe access

### Native Host Security

- **Isolated execution**: Native host runs as separate process
- **No elevated privileges**: Runs with user permissions only
- **Input validation**: All messages from extension are validated
- **Error handling**: Failures are handled gracefully without exposing sensitive data

### Code Review

- **Open source**: Full source code available on GitHub
- **Transparency**: All code changes are publicly visible
- **Auditability**: Security researchers can review the codebase

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **Do NOT** create a public GitHub issue
2. Email security details to: [security@yourdomain.com] (update with your contact)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We will:
- Acknowledge receipt within 48 hours
- Provide initial assessment within 7 days
- Keep you informed of progress
- Credit you in the release notes (if desired)

## Security Best Practices

When using TabStash NVMe:

1. **Keep updated**: Install updates when available
2. **Review permissions**: Only grant necessary permissions
3. **Monitor storage**: Check disk usage periodically
4. **Secure device**: Ensure your system is properly secured
5. **Backup data**: Important tab data should be backed up separately

## Known Limitations

- **No encryption**: Tab snapshots are stored unencrypted (local storage only)
- **No access control**: Anyone with system access can read stored data
- **No automatic cleanup**: Old snapshots must be manually removed

These limitations are acceptable for local-only storage but should be considered for sensitive data.

