# Intent Verification Test

This test verifies that the extension actually does what it's intended to do:
1. Reduce RAM usage by offloading tabs
2. Restore tabs quickly
3. Never access the network

## Test 1: RAM Usage Reduction

### Prerequisites

- Chrome with many tabs open
- Heavy websites (YouTube, Google Docs, etc.)
- Task Manager or Process Monitor

### Steps

1. **Baseline Measurement**:
   - Open Chrome Task Manager: `Shift+Esc`
   - Note total Chrome memory usage
   - Note memory per tab

2. **Open Heavy Tabs**:
   - Open 5-10 heavy websites
   - Wait for pages to load
   - Note memory usage

3. **Offload Tabs**:
   - Let TabStash detect inactive tabs
   - Or manually trigger offload
   - Wait for snapshots to complete

4. **Measure After Offload**:
   - Check Chrome Task Manager again
   - Compare memory usage

### Expected Result

- **RAM should decrease** after offloading
- Tab processes should disappear or shrink
- Total Chrome memory should be lower

### Success Criteria

- [ ] RAM usage decreases after offload
- [ ] Tab processes unloaded from memory
- [ ] Memory savings are measurable
- [ ] Disk usage increases (snapshots stored)

### Measurement

```
Before: [X] MB
After:  [Y] MB
Savings: [X-Y] MB
```

---

## Test 2: Fast Tab Restoration

### Prerequisites

- Tabs already offloaded
- Stopwatch or timer

### Steps

1. **Measure Restore Time**:
   - Click on offloaded tab
   - Start timer
   - Wait for tab to restore
   - Stop timer

2. **Compare with Normal Reload**:
   - Close tab
   - Reload from URL
   - Measure time

### Expected Result

- **Restore should be fast** (< 300ms on NVMe)
- Should feel instant (no spinner)
- Content should match exactly

### Success Criteria

- [ ] Restore time < 300ms (NVMe)
- [ ] Restore time < 500ms (SATA SSD)
- [ ] No reload spinner
- [ ] Content identical to original
- [ ] Faster than normal page reload

### Measurement

```
Restore Time: [X] ms
Normal Reload: [Y] ms
Speedup: [Y-X] ms
```

---

## Test 3: No Network Activity

### Prerequisites

- Network monitoring tool (Wireshark, netstat, Resource Monitor)
- Extension installed and running
- Native host installed

### Steps

1. **Start Network Monitoring**:
   - Windows: Resource Monitor → Network tab
   - Linux: `sudo netstat -tulpn` or `tcpdump`
   - Or: Wireshark

2. **Perform Operations**:
   - Offload tabs
   - Restore tabs
   - Create snapshots
   - List snapshots

3. **Monitor Network**:
   - Watch for any outbound connections
   - Check for DNS queries
   - Verify no HTTP/HTTPS requests

### Expected Result

- **Zero network activity** from native host
- No outbound connections
- No DNS queries
- No HTTP/HTTPS requests

### Success Criteria

- [ ] No network connections from native host
- [ ] No network connections from extension (except Chrome's own)
- [ ] No DNS queries
- [ ] No HTTP/HTTPS requests
- [ ] All operations are local-only

### Verification

```
Network Connections: 0
DNS Queries: 0
HTTP Requests: 0
HTTPS Requests: 0
```

---

## Test 4: Data Integrity

### Steps

1. **Offload Tab**:
   - Offload a tab with specific content
   - Note the content

2. **Verify Snapshot**:
   - Check snapshot file exists
   - Verify file is not corrupted

3. **Restore Tab**:
   - Restore the tab
   - Compare content with original

### Expected Result

- Content matches exactly
- No data loss
- No corruption

### Success Criteria

- [ ] Content identical
- [ ] No data loss
- [ ] No corruption
- [ ] All state preserved

---

## Test 5: Performance Under Load

### Steps

1. **Offload Many Tabs**:
   - Offload 20+ tabs
   - Measure time
   - Check system resources

2. **Restore Many Tabs**:
   - Restore all tabs
   - Measure time
   - Check system resources

### Expected Result

- System remains responsive
- Operations complete successfully
- No crashes or hangs

### Success Criteria

- [ ] System responsive
- [ ] All operations complete
- [ ] No crashes
- [ ] Reasonable performance

---

## Test Results Template

```
Date: [date]
Tester: [name]
System: [OS, CPU, RAM, Storage]

Test 1: RAM Reduction
- Before: [X] MB
- After: [Y] MB
- Savings: [Z] MB
- Result: [PASS/FAIL]

Test 2: Fast Restoration
- Restore Time: [X] ms
- Normal Reload: [Y] ms
- Result: [PASS/FAIL]

Test 3: No Network
- Connections: [0]
- DNS Queries: [0]
- Result: [PASS/FAIL]

Test 4: Data Integrity
- Content Match: [YES/NO]
- Corruption: [NONE]
- Result: [PASS/FAIL]

Test 5: Performance
- Responsive: [YES/NO]
- Crashes: [NONE]
- Result: [PASS/FAIL]

Overall: [PASS/FAIL]
```

---

## Critical Pass/Fail Criteria

**MUST PASS ALL**:
- ✅ RAM decreases after offload
- ✅ Restore is fast (< 500ms)
- ✅ Zero network activity
- ✅ Data integrity maintained
- ✅ System remains stable

**If any fail → DO NOT PUBLISH**

