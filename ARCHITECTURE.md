# Architecture Documentation

## Overview

TabStash NVMe is a browser extension that offloads inactive tab memory to NVMe SSD storage, reducing RAM usage while maintaining fast restore times.

## System Architecture

```
┌─────────────────┐
│ Browser (Chrome)│
│                 │
│  ┌───────────┐  │
│  │ Extension │  │
│  │ (JS)      │  │
│  └─────┬─────┘  │
│        │        │
│        │ Native │
│        │ Messaging│
│        │ (stdio) │
└────────┼────────┘
         │
         ▼
┌─────────────────┐
│ Native Host     │
│ (Rust Binary)   │
│                 │
│  ┌───────────┐  │
│  │ Protocol  │  │
│  │ Handler   │  │
│  └─────┬─────┘  │
│        │        │
│  ┌─────▼─────┐  │
│  │ Engine    │  │
│  │ Context   │  │
│  └─────┬─────┘  │
│        │        │
│  ┌─────▼─────┐  │
│  │ Storage   │  │
│  │ Manager   │  │
│  └─────┬─────┘  │
└────────┼────────┘
         │
         ▼
┌─────────────────┐
│ NVMe SSD        │
│ Storage         │
│                 │
│  ┌───────────┐  │
│  │ Snapshots │  │
│  │ Directory │  │
│  └───────────┘  │
│                 │
│  ┌───────────┐  │
│  │ Index DB  │  │
│  │ (sled)    │  │
│  └───────────┘  │
└─────────────────┘
```

## Component Details

### Browser Extension

**Location**: `extension/`

**Technology**: JavaScript, Chrome Extension API (Manifest V3)

**Responsibilities**:
- Monitor tab activity
- Detect inactive tabs
- Request snapshots from native host
- Restore tabs from snapshots
- Manage extension UI (popup)

**Key Files**:
- `src/background/index.js`: Service worker, native messaging connection
- `src/popup/index.html/js`: User interface, health checks
- `manifest.json`: Extension configuration

### Native Host Binary

**Location**: `tabstash-native/`

**Technology**: Rust, Tokio (async runtime)

**Responsibilities**:
- Receive messages from extension
- Process snapshot requests
- Compress tab data
- Store on NVMe SSD
- Manage snapshot lifecycle
- Handle errors gracefully

**Key Modules**:
- `src/main.rs`: Entry point, message loop
- `src/ipc.rs`: Native messaging I/O
- `src/protocol.rs`: Message format definitions
- `src/engine/`: Request routing and handling
- `src/storage/`: NVMe storage operations

### Storage Engine

**Location**: `nvme-engine/` (library), `tabstash-native/src/storage/` (implementation)

**Technology**: Rust, sled (embedded database), zstd (compression)

**Responsibilities**:
- Efficient NVMe I/O
- Key-value indexing
- Compression/decompression
- Snapshot metadata management

## Data Flow

### Snapshot Creation

1. Extension detects tab is inactive (user-defined threshold)
2. Extension sends `snapshot` request to native host:
   ```json
   {
     "id": "req-123",
     "action": "snapshot",
     "data": {
       "tabId": 42,
       "url": "https://example.com",
       "title": "Example Page"
     }
   }
   ```
3. Native host receives request
4. Native host requests tab content from extension (if needed)
5. Native host compresses tab data
6. Native host stores compressed data on NVMe
7. Native host updates index database
8. Native host responds with success:
   ```json
   {
     "id": "req-123",
     "status": "OK",
     "data": {
       "snapshotId": "snap-456",
       "size": 1024,
       "compressed": true
     }
   }
   ```
9. Extension updates metadata in browser storage
10. Extension unloads tab from memory

### Snapshot Restoration

1. User clicks on offloaded tab
2. Extension sends `restore` request:
   ```json
   {
     "id": "req-789",
     "action": "restore",
     "data": {
       "snapshotId": "snap-456"
     }
   }
   ```
3. Native host retrieves snapshot from NVMe
4. Native host decompresses data
5. Native host sends data to extension
6. Extension restores tab state
7. Tab becomes active again

## Native Messaging Protocol

### Message Format

All messages are JSON, sent over stdio with length prefix (Chrome Native Messaging format).

**Request**:
```json
{
  "id": "unique-request-id",
  "action": "action-name",
  "data": { /* action-specific data */ }
}
```

**Response**:
```json
{
  "id": "unique-request-id",
  "status": "OK" | "ERROR",
  "data": { /* response data */ },
  "error": "error message (if status is ERROR)"
}
```

### Supported Actions

- `ping`: Health check
- `snapshot`: Create tab snapshot
- `restore`: Restore tab from snapshot
- `list`: List all snapshots
- `delete`: Delete a snapshot
- `info`: Get snapshot metadata

See `tabstash-native/src/protocol.rs` for complete protocol specification.

## Storage Format

### Directory Structure

```
snapshots/
├── .index                    # sled database (metadata)
├── snap-abc123.bin.zst      # Compressed snapshot
├── snap-def456.bin.zst
└── ...
```

### Snapshot File Format

- **Format**: Compressed binary (zstd)
- **Content**: Serialized tab state (HTML, JS state, etc.)
- **Naming**: `snap-{uuid}.bin.zst`

### Index Database

- **Technology**: sled (embedded key-value store)
- **Keys**: Snapshot IDs
- **Values**: Metadata (URL, title, timestamp, size, etc.)

## Security Considerations

1. **Local-only**: No network access from native host
2. **Input validation**: All messages validated before processing
3. **Error handling**: Failures don't expose sensitive data
4. **File permissions**: Snapshots stored with user permissions only
5. **No encryption**: Data stored unencrypted (local storage acceptable)

## Performance Characteristics

- **Snapshot creation**: ~100-500ms (depending on tab size)
- **Snapshot restoration**: ~50-200ms (NVMe read + decompression)
- **Storage overhead**: ~30-50% compression ratio (zstd)
- **Memory usage**: Minimal (native host < 10MB)

## Future Enhancements

- Encryption for sensitive data
- Automatic cleanup of old snapshots
- Cross-browser support (Firefox)
- macOS support
- Cloud backup (optional)
- Incremental snapshots
- Snapshot versioning

