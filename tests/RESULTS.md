# Test Results Template

Use this template to document test results.

## Test Session

**Date**: [date]
**Tester**: [name]
**System**: [OS, version, hardware]
**Chrome Version**: [version]
**Extension Version**: [version]
**Native Host Version**: [version]

---

## Level 1: Native Host Verification

### Smoke Test
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

### Storage Reality Check
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

### Protocol Validation
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

---

## Level 2: Native Messaging Verification

### Health Check
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

### DevTools Inspection
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

### IPC Framing
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

---

## Level 3: Intent Verification

### RAM Reduction
- [ ] PASS / [ ] FAIL
- **Before**: [X] MB
- **After**: [Y] MB
- **Savings**: [Z] MB
- **Notes**: [notes]

### Fast Restoration
- [ ] PASS / [ ] FAIL
- **Restore Time**: [X] ms
- **Notes**: [notes]

### No Network Activity
- [ ] PASS / [ ] FAIL
- **Connections**: [0]
- **Notes**: [notes]

---

## Level 4: Failure Modes

### Kill Mid-Write
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

### Corrupt Snapshot
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

### Restart Persistence
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

### Disk Full
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

### Permission Denied
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

### Native Host Missing
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

### Extension ID Mismatch
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

---

## Level 5: Reviewer Simulation

### Initial Review
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

### Native Host Verification
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

### Permission Audit
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

### Security Review
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

### Functionality Test
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

### User Experience
- [ ] PASS / [ ] FAIL
- **Notes**: [notes]

---

## Overall Assessment

**Total Tests**: [X]
**Passed**: [Y]
**Failed**: [Z]

**Critical Failures**: [list]
**Non-Critical Issues**: [list]

**Recommendation**: 
- [ ] READY FOR SUBMISSION
- [ ] NEEDS FIXES (list fixes needed)
- [ ] NOT READY (major issues)

**Notes**: [overall notes]

---

## Next Steps

1. [ ] Fix critical failures
2. [ ] Retest fixed items
3. [ ] Address non-critical issues
4. [ ] Final verification
5. [ ] Submit to store

