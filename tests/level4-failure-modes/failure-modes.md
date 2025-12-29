# Failure Mode Testing

Tests to verify the system handles failures gracefully without data corruption or crashes.

## Test 1: Kill Native Host Mid-Write

### Purpose

Verify that killing the native host during a snapshot operation doesn't corrupt data.

### Steps

1. **Start Snapshot Operation**:
   - Offload a tab (trigger snapshot)
   - Immediately kill native host process

2. **Kill Process**:
   ```powershell
   # Windows
   taskkill /IM tabstash-native.exe /F
   ```
   ```bash
   # Linux
   killall -9 tabstash-native
   ```

3. **Verify State**:
   - Check for partial/corrupt files
   - Check index database
   - Restart native host
   - Verify system recovers

### Expected Result

- **No corrupt files** created
- **No partial snapshots** (atomic writes)
- **Index remains consistent**
- **System recovers** after restart

### Success Criteria

- [ ] No corrupt snapshot files
- [ ] No partial writes
- [ ] Index database intact
- [ ] System recovers cleanly
- [ ] Extension handles error gracefully

---

## Test 2: Corrupt Snapshot File

### Purpose

Verify that restoring from a corrupted snapshot fails gracefully.

### Steps

1. **Create Snapshot**:
   - Offload a tab
   - Note snapshot file location

2. **Corrupt Snapshot**:
   - Open snapshot file in hex editor
   - Modify random bytes
   - Or truncate file

3. **Attempt Restore**:
   - Try to restore the tab
   - Observe behavior

### Expected Result

- **Restore fails** with clear error
- **No crash** or hang
- **Error message** is user-friendly
- **Extension handles** error gracefully

### Success Criteria

- [ ] Restore fails (expected)
- [ ] Error message shown
- [ ] No crash
- [ ] Extension remains functional
- [ ] Can retry or skip

---

## Test 3: Restart Chrome Persistence

### Purpose

Verify that snapshots persist across Chrome restarts.

### Steps

1. **Create Snapshots**:
   - Offload several tabs
   - Verify snapshots exist

2. **Restart Chrome**:
   - Close Chrome completely
   - Wait a few seconds
   - Restart Chrome

3. **Verify Persistence**:
   - Check snapshot files still exist
   - Check index database intact
   - Try to restore a tab

### Expected Result

- **Snapshots persist** after restart
- **Index database** remains valid
- **Restore works** after restart
- **No data loss**

### Success Criteria

- [ ] Snapshots still exist
- [ ] Index database valid
- [ ] Restore works
- [ ] No data loss
- [ ] Extension reconnects

---

## Test 4: Disk Full Scenario

### Purpose

Verify behavior when disk is full.

### Steps

1. **Fill Disk** (carefully):
   - Create large files to fill disk
   - Leave minimal free space

2. **Attempt Snapshot**:
   - Try to offload a tab
   - Observe behavior

### Expected Result

- **Operation fails** gracefully
- **Clear error message**
- **No crash**
- **System remains stable**

### Success Criteria

- [ ] Operation fails (expected)
- [ ] Error message shown
- [ ] No crash
- [ ] Can free space and retry

---

## Test 5: Permission Denied

### Purpose

Verify behavior when native host lacks permissions.

### Steps

1. **Remove Permissions**:
   - Remove write permission from data directory
   - Or remove read permission

2. **Attempt Operation**:
   - Try to create snapshot
   - Or try to restore

### Expected Result

- **Operation fails** with clear error
- **Error message** explains issue
- **No crash**

### Success Criteria

- [ ] Operation fails (expected)
- [ ] Error message clear
- [ ] No crash
- [ ] Can fix permissions and retry

---

## Test 6: Native Host Not Installed

### Purpose

Verify extension handles missing native host gracefully.

### Steps

1. **Uninstall Native Host**:
   - Remove binary
   - Remove manifest

2. **Open Extension**:
   - Open popup
   - Try to use extension

### Expected Result

- **Clear error message**
- **Installation instructions** shown
- **No crash**
- **Helpful guidance**

### Success Criteria

- [ ] Error message shown
- [ ] Installer link provided
- [ ] No crash
- [ ] User can install native host

---

## Test 7: Extension ID Mismatch

### Purpose

Verify security: extension with wrong ID cannot connect.

### Steps

1. **Modify Manifest**:
   - Change extension ID in manifest
   - Use wrong/random ID

2. **Try to Connect**:
   - Extension tries to connect
   - Observe behavior

### Expected Result

- **Connection fails** (security)
- **Clear error** (not silent)
- **No security breach**

### Success Criteria

- [ ] Connection fails (expected)
- [ ] Error shown
- [ ] Security maintained
- [ ] Wrong extension cannot connect

---

## Test Results Template

```
Date: [date]
Tester: [name]

Test 1: Kill Mid-Write
- Result: [PASS/FAIL]
- Notes: [notes]

Test 2: Corrupt Snapshot
- Result: [PASS/FAIL]
- Notes: [notes]

Test 3: Restart Persistence
- Result: [PASS/FAIL]
- Notes: [notes]

Test 4: Disk Full
- Result: [PASS/FAIL]
- Notes: [notes]

Test 5: Permission Denied
- Result: [PASS/FAIL]
- Notes: [notes]

Test 6: Native Host Missing
- Result: [PASS/FAIL]
- Notes: [notes]

Test 7: Extension ID Mismatch
- Result: [PASS/FAIL]
- Notes: [notes]

Overall: [PASS/FAIL]
```

---

## Critical Pass Criteria

**ALL tests must pass**:
- ✅ No data corruption
- ✅ Graceful error handling
- ✅ No crashes
- ✅ System recovers
- ✅ Security maintained

**If any fail → FIX BEFORE PUBLISHING**

