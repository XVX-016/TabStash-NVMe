# Release Verification Plan - v0.1.0-devmode

This document defines the verification steps required before releasing v0.1.0-devmode.

## Phase A: Hard Verification (Mandatory)

### A1. Fresh Machine Install Test

**Windows Test Machine:**
- [ ] Fresh Windows 10/11 installation OR clean user profile
- [ ] Chrome installed (default location)
- [ ] No existing TabStash installation
- [ ] No existing Chrome extensions
- [ ] Follow README.md installation steps exactly
- [ ] Document any steps that require "tribal knowledge"
- [ ] Time to complete: Record actual time
- [ ] Success criteria: All 4 installation status items show OK

**Linux Test Machine (if supported):**
- [ ] Fresh Linux installation OR clean user profile
- [ ] Chrome installed
- [ ] No existing TabStash installation
- [ ] Follow README.md installation steps exactly
- [ ] Document any steps that require "tribal knowledge"
- [ ] Time to complete: Record actual time
- [ ] Success criteria: All 4 installation status items show OK

**Failure Criteria:**
- If any step requires information not in README → README must be updated
- If installation takes > 15 minutes → UX issue
- If user gets stuck at any step → blocker

---

### A2. Failure Injection Tests

For each failure scenario, verify:
1. Error is visible in popup (not console-only)
2. Error message is specific and actionable
3. User knows exactly what to do next

| Failure Scenario | Test Method | Expected Popup Result | Expected Error Message Contains |
|-----------------|-------------|----------------------|--------------------------------|
| Native host not installed | Don't run installer | FAIL "Native host installed" | "Run installer: scripts/install-windows.ps1" |
| Extension ID mismatch | Load extension, don't link | FAIL "Extension ID linked" | "Run: scripts/link-extension.ps1 [ID]" |
| Browser not restarted | Link ID, don't restart | FAIL "Health check passed" | "Restart browser completely" |
| Old native binary | Use old binary version | FAIL "Health check passed" | "Version mismatch" or "Protocol version" |
| Native host hangs | Kill native process | FAIL "Health check passed" | "Timeout" or "did not respond" |
| Corrupt manifest | Edit manifest to invalid JSON | FAIL "Native host installed" | "Manifest" or "JSON" error |
| Corrupt storage dir | Delete/rename snapshots dir | FAIL "Health check passed" | Storage error (if detectable) |
| Missing binary | Delete tabstash-native.exe | FAIL "Native host installed" | "Binary not found" or path error |

**Failure Criteria:**
- Silent failure (no error shown) → BLOCKER
- Vague error ("Something went wrong") → BLOCKER
- Console-only error → BLOCKER
- Error without fix steps → BLOCKER

---

### A3. Self-Test Gate

**Before opening Chrome:**

```bash
# Windows
tabstash-native.exe --self-test

# Linux
./tabstash-native --self-test
```

**Verification:**
- [ ] Exit code 0 on success
- [ ] Exit code non-zero on failure
- [ ] Output is human-readable (not binary)
- [ ] Each test clearly labeled
- [ ] Errors are actionable
- [ ] Test completes in < 2 seconds

**Failure Scenarios to Test:**
- [ ] Missing manifest → Test 4 should fail with clear message
- [ ] Invalid manifest JSON → Test 4 should fail with parse error
- [ ] Missing data directory → Test 2 should fail
- [ ] No write permissions → Test 2 should fail

**Failure Criteria:**
- If self-test passes but Chrome fails → Integration bug (BLOCKER)
- If self-test output is unreadable → UX issue
- If exit codes are wrong → Scripting issue

---

## Phase B: Code Review for Silent Failures

### B1. Error Handling Audit

Review all error paths:

**Extension Background Script:**
- [ ] `connectNativeHost()` errors stored in `lastError`
- [ ] `sendToNativeHost()` timeout errors stored
- [ ] `getManifestInfo()` errors caught and propagated
- [ ] `checkIdMatch()` errors returned (not swallowed)
- [ ] Message handler errors don't crash service worker

**Extension Popup:**
- [ ] `performHealthCheck()` errors displayed in UI
- [ ] `loadExtensionId()` failures don't break popup
- [ ] `loadDevModeBanner()` failures don't break popup
- [ ] All async operations have catch blocks
- [ ] Storage API errors handled gracefully

**Native Host:**
- [ ] Manifest read errors return proper error response
- [ ] JSON parse errors caught
- [ ] File I/O errors propagated to extension
- [ ] Protocol errors don't crash native host

### B2. Edge Case Testing

- [ ] Extension ID with special characters (shouldn't happen, but validate)
- [ ] Very long error messages (UI doesn't break)
- [ ] Rapid popup open/close (no race conditions)
- [ ] Multiple Chrome windows (manifest shared correctly)
- [ ] Extension reload during connection (graceful handling)

---

## Phase C: Documentation Verification

### C1. README Completeness

- [ ] Every installation step is explicit
- [ ] No assumptions about user knowledge
- [ ] All file paths are absolute or clearly relative
- [ ] All commands are copy-pasteable
- [ ] Troubleshooting covers all failure scenarios from A2
- [ ] Screenshots or placeholders for complex steps

### C2. Error Message Mapping

Verify every error message in code has:
- [ ] Corresponding README entry
- [ ] Exact fix steps
- [ ] No "contact support" dead ends

---

## Phase D: Release Packaging

### D1. Version Numbers

- [ ] Extension manifest.json: `"version": "0.1.0"`
- [ ] Native host Cargo.toml: `version = "0.1.0"`
- [ ] Protocol version: `1` (documented)
- [ ] All version numbers match

### D2. Git Tag

- [ ] Tag created: `v0.1.0-devmode`
- [ ] Tag message includes release notes
- [ ] Tag points to commit with all features complete

### D3. Release Notes

- [ ] GitHub release created
- [ ] "Developer Mode Required" warning prominent
- [ ] Known limitations listed
- [ ] Installation instructions linked
- [ ] Breaking changes (if any) documented

---

## Phase E: Scope Lock Document

Create `docs/SCOPE_v0.1.md` explicitly marking:

**In Scope:**
- GitHub + Developer Mode installation
- Extension ID linking
- Installation status panel
- Basic error handling
- Windows + Linux support

**Out of Scope (vNext):**
- Chrome Web Store distribution
- Auto-updates
- Multi-browser auto-detection
- Custom profile path detection
- Zero-restart linking

**Rationale:** Prevents scope creep during release.

---

## Go/No-Go Decision Matrix

**GO Criteria (all must pass):**
- Fresh install test completes in < 15 minutes
- All failure injection tests show clear errors
- Self-test works independently
- No silent failures found in code review
- README is complete and accurate
- Version numbers consistent
- Release notes written

**NO-GO Criteria (any one blocks):**
- Silent failure in any scenario
- Vague error message
- README requires tribal knowledge
- Self-test doesn't work
- Installation takes > 20 minutes for technical user

---

## Post-Release Monitoring

**Week 1:**
- Monitor GitHub issues for installation problems
- Track time-to-install from user reports
- Document common failure patterns

**Week 2:**
- If failure rate < 5% → Consider Chrome Web Store
- If failure rate > 10% → Fix blockers before store
- Collect UX feedback on status panel

---

## Sign-Off

**Verification Completed By:** _________________  
**Date:** _________________  
**Go/No-Go Decision:** [ ] GO  [ ] NO-GO  
**Blockers (if NO-GO):** _________________

