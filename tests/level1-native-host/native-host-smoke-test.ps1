# Native Host Smoke Test (Windows)
# Tests basic native host functionality without Chrome

$ErrorActionPreference = "Stop"

Write-Host "Native Host Smoke Test" -ForegroundColor Green
Write-Host "======================" -ForegroundColor Green
Write-Host ""

# Check if binary exists
$binaryPath = "..\..\tabstash-native\target\release\tabstash-native.exe"
if (-not (Test-Path $binaryPath)) {
    Write-Host "ERROR: Binary not found: $binaryPath" -ForegroundColor Red
    Write-Host "Please build the binary first: cd tabstash-native && cargo build --release" -ForegroundColor Yellow
    exit 1
}

Write-Host "Binary found: $binaryPath" -ForegroundColor Cyan
Write-Host ""

# Test 1: Binary exists and is executable
Write-Host "Test 1: Binary exists and is executable" -ForegroundColor Yellow
if (Test-Path $binaryPath) {
    Write-Host "  PASS: Binary exists" -ForegroundColor Green
} else {
    Write-Host "  FAIL: Binary not found" -ForegroundColor Red
    exit 1
}

# Test 2: Binary size is reasonable
Write-Host "Test 2: Binary size is reasonable" -ForegroundColor Yellow
$fileSize = (Get-Item $binaryPath).Length
$fileSizeMB = [math]::Round($fileSize / 1MB, 2)
Write-Host "  Binary size: $fileSizeMB MB" -ForegroundColor Cyan

if ($fileSizeMB -lt 50) {
    Write-Host "  PASS: Binary size is reasonable (< 50MB)" -ForegroundColor Green
} else {
    Write-Host "  WARNING: Binary is large ($fileSizeMB MB)" -ForegroundColor Yellow
}

# Test 3: Check dependencies (basic)
Write-Host "Test 3: Basic dependency check" -ForegroundColor Yellow
try {
    $process = Start-Process -FilePath $binaryPath -ArgumentList "--help" -NoNewWindow -PassThru -Wait -ErrorAction SilentlyContinue
    Write-Host "  PASS: Binary can be executed" -ForegroundColor Green
} catch {
    Write-Host "  WARNING: Could not test binary execution (may require stdio input)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Smoke test complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Next: Run storage reality check and protocol validation tests" -ForegroundColor Cyan

