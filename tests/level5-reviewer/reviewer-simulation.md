# Reviewer Simulation Test Plan

Complete test plan simulating what a Chrome Web Store reviewer would do.

## Reviewer Persona

- **Knowledge**: Familiar with Chrome extensions, may not know your specific use case
- **Time**: Limited, wants to verify quickly
- **Focus**: Security, privacy, functionality, user experience
- **Tools**: Chrome browser, basic system tools

---

## Phase 1: Initial Review

### 1.1 Store Listing Review

**Check**:
- [ ] Description is clear
- [ ] Permissions are justified
- [ ] Privacy practices are complete
- [ ] Screenshots are accurate
- [ ] No misleading claims

**Expected**: All items clear and accurate

---

### 1.2 Extension Installation

**Steps**:
1. Install extension from store (Unlisted)
2. Verify extension loads
3. Check for errors

**Expected**:
- Extension installs successfully
- No errors in console
- Extension icon appears

---

### 1.3 First Impression

**Steps**:
1. Open extension popup
2. Observe UI
3. Check for obvious issues

**Expected**:
- Popup loads
- UI is professional
- No blank screens
- Clear status indicators

---

## Phase 2: Native Host Verification

### 2.1 Native Host Missing Scenario

**Steps**:
1. **Do NOT install native host** (simulate new user)
2. Open extension popup
3. Observe behavior

**Expected**:
- ✅ Clear error message: "Native host not available"
- ✅ Installer download link shown
- ✅ Installation instructions provided
- ✅ No crash or blank screen
- ✅ Helpful, not technical

**If this fails → REJECTION RISK**

---

### 2.2 Native Host Installation

**Steps**:
1. Download installer from GitHub Releases
2. Run installer
3. Verify installation

**Expected**:
- Installer works
- Binary installed correctly
- Manifest installed
- No errors

---

### 2.3 Connection Verification

**Steps**:
1. Restart Chrome
2. Open extension popup
3. Verify connection

**Expected**:
- Status shows "Connected"
- No errors
- Extension functional

---

## Phase 3: Permission Audit

### 3.1 Permission Justification

**Check Each Permission**:

**`tabs`**:
- [ ] Justification provided
- [ ] Used for stated purpose only
- [ ] Not used for unrelated tasks

**`storage`**:
- [ ] Justification provided
- [ ] Used for extension data only
- [ ] Not used for tracking

**`scripting`**:
- [ ] Justification provided
- [ ] Used for tab restoration only
- [ ] Not used for injection attacks

**`nativeMessaging`**:
- [ ] Justification provided
- [ ] Native host purpose explained
- [ ] Security model documented

**Expected**: All permissions justified and used appropriately

---

### 3.2 Permission Scope

**Check**:
- [ ] No overbroad permissions
- [ ] Permissions match functionality
- [ ] No hidden behavior

**Expected**: Permissions are minimal and necessary

---

## Phase 4: Security Review

### 4.1 Network Activity

**Steps**:
1. Monitor network activity
2. Use extension
3. Check for outbound connections

**Expected**:
- ✅ **ZERO network activity** from native host
- ✅ No data exfiltration
- ✅ No remote servers
- ✅ Local-only operation

**If network activity detected → REJECTION**

---

### 4.2 Data Storage

**Check**:
- [ ] Data stored locally only
- [ ] No cloud uploads
- [ ] No external APIs
- [ ] Privacy policy matches behavior

**Expected**: All data local, no external transmission

---

### 4.3 Native Host Security

**Check**:
- [ ] Native host has no network code
- [ ] Native host runs with user permissions
- [ ] No elevation required
- [ ] Input validation present

**Expected**: Native host is secure and isolated

---

## Phase 5: Functionality Test

### 5.1 Basic Functionality

**Steps**:
1. Open several tabs
2. Let extension offload tabs
3. Restore tabs
4. Verify content

**Expected**:
- Tabs offload successfully
- Tabs restore correctly
- Content matches
- No data loss

---

### 5.2 Error Handling

**Steps**:
1. Test with native host missing
2. Test with corrupted data
3. Test with disk full
4. Observe error messages

**Expected**:
- Errors handled gracefully
- Clear error messages
- No crashes
- User can recover

---

## Phase 6: User Experience

### 6.1 First-Run Experience

**Check**:
- [ ] Clear installation instructions
- [ ] Helpful error messages
- [ ] Easy to get started
- [ ] No confusion

**Expected**: Smooth first-run experience

---

### 6.2 Documentation

**Check**:
- [ ] README is clear
- [ ] Installation instructions work
- [ ] Troubleshooting available
- [ ] Support channels clear

**Expected**: Good documentation

---

## Reviewer Decision Matrix

### Auto-Approve Criteria

- ✅ All security checks pass
- ✅ Permissions justified
- ✅ Native host installs easily
- ✅ First-run UX is clear
- ✅ Functionality works
- ✅ No network activity
- ✅ Privacy policy accurate

### Rejection Risks

- ❌ Native host missing → blank screen
- ❌ Network activity detected
- ❌ Overbroad permissions
- ❌ Poor error handling
- ❌ Misleading description
- ❌ Security concerns

---

## Test Execution

### Simulate Reviewer

1. **Fresh Install**:
   - Clean Chrome profile
   - No native host installed
   - Install extension from store

2. **Follow Test Plan**:
   - Execute each phase
   - Document results
   - Note any issues

3. **Fix Issues**:
   - Address any failures
   - Retest
   - Verify fixes

---

## Test Results

```
Reviewer Simulation Results
===========================

Date: [date]
Tester: [name]
Chrome Version: [version]

Phase 1: Initial Review
- Store Listing: [PASS/FAIL]
- Installation: [PASS/FAIL]
- First Impression: [PASS/FAIL]

Phase 2: Native Host
- Missing Scenario: [PASS/FAIL]
- Installation: [PASS/FAIL]
- Connection: [PASS/FAIL]

Phase 3: Permissions
- Justification: [PASS/FAIL]
- Scope: [PASS/FAIL]

Phase 4: Security
- Network Activity: [PASS/FAIL]
- Data Storage: [PASS/FAIL]
- Native Host: [PASS/FAIL]

Phase 5: Functionality
- Basic: [PASS/FAIL]
- Error Handling: [PASS/FAIL]

Phase 6: UX
- First-Run: [PASS/FAIL]
- Documentation: [PASS/FAIL]

Overall: [APPROVE/REJECT]
Issues Found: [list]
```

---

## Critical Pass Criteria

**MUST PASS**:
- ✅ Native host missing → clear error + installer link
- ✅ Zero network activity
- ✅ All permissions justified
- ✅ Functionality works
- ✅ Error handling graceful
- ✅ Security model sound

**If any fail → DO NOT SUBMIT**

