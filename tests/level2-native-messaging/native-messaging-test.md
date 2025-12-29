# Native Messaging Verification Test

Manual test guide for verifying native messaging communication between extension and native host.

## Prerequisites

- Extension loaded in Chrome
- Native host installed
- Native messaging manifest installed
- Chrome DevTools available

## Test 1: Health Check from Extension

### Steps

1. **Open Extension Popup**:
   - Click extension icon
   - Observe status indicator

2. **Expected Result**:
   - Status shows: "Native host connected" (green)
   - No error messages
   - Info section visible

3. **Check Background Script**:
   - Open `chrome://extensions`
   - Find TabStash NVMe
   - Click "Inspect service worker"
   - Check console for errors

### Success Criteria

- [ ] Popup shows "Connected"
- [ ] No console errors
- [ ] Background script shows connection state: "connected"

---

## Test 2: DevTools Native Messaging Inspection

### Steps

1. **Open Service Worker Console**:
   - Go to `chrome://extensions`
   - Find TabStash NVMe
   - Click "Inspect service worker"

2. **Test Connection**:
   ```javascript
   // In service worker console
   const port = chrome.runtime.connectNative('tabstash_native');
   console.log('Port:', port);
   ```

3. **Send Test Message**:
   ```javascript
   port.postMessage({
     id: 'test-1',
     action: 'ping'
   });
   ```

4. **Listen for Response**:
   ```javascript
   port.onMessage.addListener((msg) => {
     console.log('Response:', msg);
   });
   ```

### Expected Result

- Port connects successfully
- Message sent without error
- Response received (if native host responds to ping)
- No framing errors

### Success Criteria

- [ ] Connection established
- [ ] Message sent successfully
- [ ] Response received (or timeout handled)
- [ ] No IPC framing errors

---

## Test 3: IPC Framing Validation

### Steps

1. **Check Message Format**:
   - Native messaging uses length-prefixed JSON
   - Format: `[length][JSON]`
   - Length is 32-bit unsigned integer

2. **Verify Extension Sends Correctly**:
   - Check background script message sending
   - Verify JSON serialization
   - Check length prefix

3. **Verify Native Host Receives**:
   - Check native host logs (if available)
   - Verify message parsing
   - Check error handling

### Success Criteria

- [ ] Messages properly formatted
- [ ] Length prefix correct
- [ ] JSON valid
- [ ] No framing errors

---

## Test 4: Connection Error Handling

### Steps

1. **Kill Native Host Process**:
   ```powershell
   # Windows
   taskkill /IM tabstash-native.exe /F
   ```

2. **Try to Connect**:
   - Open extension popup
   - Or send message from service worker

3. **Expected Result**:
   - Connection fails gracefully
   - Error message shown (not crash)
   - Extension handles error

### Success Criteria

- [ ] Error handled gracefully
- [ ] User sees helpful error
- [ ] No extension crash
- [ ] Reconnection attempted (optional)

---

## Test 5: Extension ID Validation

### Steps

1. **Check Manifest**:
   - Location: `%LOCALAPPDATA%\Google\Chrome\User Data\NativeMessagingHosts\tabstash_native.json`
   - Verify `allowed_origins` contains extension ID

2. **Get Extension ID**:
   - Go to `chrome://extensions`
   - Find extension ID (32-character string)

3. **Verify Match**:
   - Extension ID in manifest matches actual ID
   - Case-sensitive match required

### Success Criteria

- [ ] Manifest exists
- [ ] Extension ID matches
- [ ] Connection works with correct ID
- [ ] Connection fails with wrong ID (security)

---

## Troubleshooting

### Connection Fails

**Check**:
1. Native host installed?
2. Manifest installed?
3. Extension ID matches?
4. Chrome restarted after installation?

**Fix**:
- Reinstall native host
- Verify manifest location
- Check extension ID
- Restart Chrome

### Messages Not Received

**Check**:
1. Native host running?
2. Message format correct?
3. IPC framing correct?
4. Native host processing messages?

**Fix**:
- Check native host process
- Verify message format
- Check native host logs
- Test with simple message

### Framing Errors

**Check**:
1. Length prefix correct?
2. JSON valid?
3. Encoding correct (UTF-8)?

**Fix**:
- Verify message serialization
- Check length calculation
- Ensure UTF-8 encoding

---

## Test Results Template

```
Date: [date]
Tester: [name]
Chrome Version: [version]
Extension Version: [version]
Native Host Version: [version]

Test 1: Health Check
- Result: [PASS/FAIL]
- Notes: [notes]

Test 2: DevTools Inspection
- Result: [PASS/FAIL]
- Notes: [notes]

Test 3: IPC Framing
- Result: [PASS/FAIL]
- Notes: [notes]

Test 4: Error Handling
- Result: [PASS/FAIL]
- Notes: [notes]

Test 5: Extension ID
- Result: [PASS/FAIL]
- Notes: [notes]

Overall: [PASS/FAIL]
```

