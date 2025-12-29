# Quick Verification Checklist

Use this checklist for rapid verification before release. Each item should take < 2 minutes.

## Pre-Flight (5 minutes)

- [ ] `tabstash-native --self-test` passes
- [ ] Extension loads without console errors
- [ ] Popup opens and shows status
- [ ] Extension ID is visible and copyable

## Failure Scenarios (10 minutes)

Test each scenario and verify error is shown in popup (not console-only):

- [ ] **No native host**: Don't install → Should show "Native host installed FAIL"
- [ ] **ID mismatch**: Load extension, don't link → Should show "Extension ID linked FAIL" with command
- [ ] **Not restarted**: Link ID, don't restart → Should show "Health check passed FAIL"
- [ ] **Timeout**: Kill native process → Should show timeout error

## Documentation (5 minutes)

- [ ] README installation steps are copy-pasteable
- [ ] All file paths are clear
- [ ] Troubleshooting covers above scenarios
- [ ] Release notes mention Developer Mode requirement

## Version Check (1 minute)

- [ ] Extension manifest: `0.1.0`
- [ ] Native host Cargo.toml: `0.1.0`
- [ ] Protocol version: `1`
- [ ] All match

---

**Total Time:** ~20 minutes  
**If all pass:** Ready for fresh machine test  
**If any fail:** Fix before proceeding

