# Storage Reality Check

Manual test to verify native host storage functionality.

## Prerequisites

- Native host binary built: `tabstash-native/target/release/tabstash-native.exe`
- Test data directory created

## Test Procedure

### 1. Prepare Test Data

Create a test snapshot request:

```json
{
  "id": "test-1",
  "action": "snapshot",
  "data": {
    "tabId": 123,
    "url": "https://example.com",
    "title": "Test Page",
    "content": "<html><body>Test content</body></html>"
  }
}
```

### 2. Send Snapshot Request

**Windows (PowerShell)**:
```powershell
$request = @{
    id = "test-1"
    action = "snapshot"
    data = @{
        tabId = 123
        url = "https://example.com"
        title = "Test Page"
        content = "<html><body>Test content</body></html>"
    }
} | ConvertTo-Json -Depth 10

# Send via native messaging (requires extension)
# Or test directly with binary
```

**Linux**:
```bash
# Similar approach with jq or direct JSON
```

### 3. Verify Storage

**Check Snapshot File**:
- Location: `%LOCALAPPDATA%\TabStash\snapshots\` (Windows)
- Location: `~/.local/share/tabstash/snapshots/` (Linux)
- Should contain: `snap-*.bin.zst` file

**Check Index Database**:
- Location: `snapshots/.index` (sled database)
- Should contain entry for snapshot

### 4. Verify Compression

**Check File Size**:
- Snapshot file should be smaller than original content
- Compression ratio should be reasonable (30-50% typical)

**Decompress Test**:
```bash
# Windows (if zstd available)
zstd -d snap-*.bin.zst -o snap-*.bin

# Linux
zstd -d snap-*.bin.zst -o snap-*.bin
```

### 5. Verify Restore

Send restore request:

```json
{
  "id": "test-2",
  "action": "restore",
  "data": {
    "snapshotId": "snap-xxx"
  }
}
```

**Expected**:
- Returns original content
- Content matches exactly
- No corruption

## Success Criteria

- [ ] Snapshot file created
- [ ] Index database updated
- [ ] Compression working (file size reduced)
- [ ] Restore returns exact content
- [ ] No data corruption
- [ ] Atomic writes (no partial files)

## Troubleshooting

### Snapshot Not Created

- Check data directory permissions
- Check disk space
- Check binary has write access

### Compression Not Working

- Verify zstd is linked correctly
- Check compression settings
- Verify input data format

### Restore Fails

- Check snapshot file exists
- Verify index entry
- Check file permissions
- Verify decompression

## Notes

- This is a manual test (automation possible but complex)
- Requires native messaging connection (via extension)
- Test on clean system for best results

