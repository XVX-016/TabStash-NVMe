# Verification Test Suite

This directory contains comprehensive tests for TabStash NVMe covering all 5 levels of the verification ladder.

## Test Structure

```
tests/
├── level1-native-host/          # Native host verification
├── level2-native-messaging/     # Native messaging verification
├── level3-intent/               # Intent verification
├── level4-failure-modes/        # Failure mode testing
└── level5-reviewer/             # Reviewer simulation
```

## Running Tests

### Level 1: Native Host Verification
```bash
# Windows
.\tests\level1-native-host\native-host-smoke-test.ps1

# Linux
./tests/level1-native-host/native-host-smoke-test.sh
```

### Level 2: Native Messaging Verification
See `tests/level2-native-messaging/native-messaging-test.md` for manual test guide.

### Level 3: Intent Verification
See `tests/level3-intent/intent-verification.md` for manual test checklist.

### Level 4: Failure Modes
See `tests/level4-failure-modes/failure-modes.md` for test procedures.

### Level 5: Reviewer Simulation
See `tests/level5-reviewer/reviewer-simulation.md` for complete test plan.

## Test Results

Document test results in:
- `tests/RESULTS.md` (template provided)

## Prerequisites

- Built native host binary: `tabstash-native/target/release/tabstash-native.exe`
- Extension loaded in Chrome (unpacked or from store)
- Native host installed (for some tests)
- Chrome browser
- Administrator/sudo access (for installation tests)

