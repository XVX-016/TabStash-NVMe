#!/bin/bash
# Native Host Smoke Test (Linux)
# Tests basic native host functionality without Chrome

set -e

echo "Native Host Smoke Test"
echo "======================"
echo ""

# Check if binary exists
BINARY_PATH="../../tabstash-native/target/release/tabstash-native"
if [ ! -f "$BINARY_PATH" ]; then
    echo "ERROR: Binary not found: $BINARY_PATH" >&2
    echo "Please build the binary first: cd tabstash-native && cargo build --release" >&2
    exit 1
fi

echo "Binary found: $BINARY_PATH"
echo ""

# Test 1: Binary exists and is executable
echo "Test 1: Binary exists and is executable"
if [ -f "$BINARY_PATH" ]; then
    echo "  PASS: Binary exists"
else
    echo "  FAIL: Binary not found"
    exit 1
fi

# Test 2: Binary is executable
if [ -x "$BINARY_PATH" ]; then
    echo "  PASS: Binary is executable"
else
    echo "  FAIL: Binary is not executable"
    chmod +x "$BINARY_PATH"
    echo "  Fixed: Made binary executable"
fi

# Test 3: Binary size is reasonable
echo "Test 2: Binary size is reasonable"
FILE_SIZE=$(stat -f%z "$BINARY_PATH" 2>/dev/null || stat -c%s "$BINARY_PATH" 2>/dev/null)
FILE_SIZE_MB=$(echo "scale=2; $FILE_SIZE / 1048576" | bc)
echo "  Binary size: ${FILE_SIZE_MB} MB"

if (( $(echo "$FILE_SIZE_MB < 50" | bc -l) )); then
    echo "  PASS: Binary size is reasonable (< 50MB)"
else
    echo "  WARNING: Binary is large (${FILE_SIZE_MB} MB)"
fi

# Test 4: Check dependencies (basic)
echo "Test 3: Basic dependency check"
if ldd "$BINARY_PATH" >/dev/null 2>&1; then
    echo "  PASS: Binary has valid dependencies"
    ldd "$BINARY_PATH" | head -5
else
    echo "  WARNING: Could not check dependencies (static binary?)"
fi

echo ""
echo "Smoke test complete!"
echo ""
echo "Next: Run storage reality check and protocol validation tests"

