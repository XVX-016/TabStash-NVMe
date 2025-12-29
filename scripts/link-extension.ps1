# Link Extension ID to Native Host Manifest
# Usage: .\link-extension.ps1 "your-extension-id-here"
#        .\link-extension.ps1 (will prompt for ID)

param(
    [Parameter(Mandatory=$false)]
    [string]$ExtensionId
)

$ErrorActionPreference = "Stop"

# If extension ID not provided, prompt for it
if (-not $ExtensionId) {
    $ExtensionId = Read-Host "Enter your Extension ID (32 lowercase letters)"
}

Write-Host "Linking Extension ID to Native Host..." -ForegroundColor Green
Write-Host "Extension ID: $ExtensionId" -ForegroundColor Cyan

# Validate extension ID format (basic check)
if ($ExtensionId -notmatch '^[a-z]{32}$') {
    Write-Host "ERROR: Extension ID format is incorrect." -ForegroundColor Red
    Write-Host "Expected format: 32 lowercase letters (e.g., abcdefghijklmnopqrstuvwxyzabcdef)" -ForegroundColor Yellow
    exit 1
}

# Determine Chrome Native Messaging Hosts directory (standard path)
$chromeUserData = Join-Path $env:LOCALAPPDATA "Google\Chrome\User Data\NativeMessagingHosts"
$manifestPath = Join-Path $chromeUserData "tabstash_native.json"

# Check if manifest exists
if (-not (Test-Path $manifestPath)) {
    Write-Host "ERROR: Native host manifest not found at:" -ForegroundColor Red
    Write-Host "  $manifestPath" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Please install the native host first:" -ForegroundColor Yellow
    Write-Host "  Run: .\scripts\install-windows.ps1" -ForegroundColor White
    exit 1
}

# Read manifest
try {
    $manifestContent = Get-Content -Path $manifestPath -Raw | ConvertFrom-Json
} catch {
    Write-Host "ERROR: Failed to parse manifest JSON: $_" -ForegroundColor Red
    Write-Host "Manifest file may be corrupted." -ForegroundColor Yellow
    exit 1
}

# Backup manifest before editing
$backupPath = "$manifestPath.backup"
try {
    Copy-Item -Path $manifestPath -Destination $backupPath -Force
    Write-Host "Created backup: $backupPath" -ForegroundColor Gray
} catch {
    Write-Host "WARNING: Failed to create backup: $_" -ForegroundColor Yellow
}

# Update allowed_origins
$extensionOrigin = "chrome-extension://$ExtensionId/"
if (-not $manifestContent.allowed_origins) {
    $manifestContent | Add-Member -MemberType NoteProperty -Name "allowed_origins" -Value @()
}

# Replace or add extension ID
$existingIndex = -1
for ($i = 0; $i -lt $manifestContent.allowed_origins.Count; $i++) {
    if ($manifestContent.allowed_origins[$i] -like "chrome-extension://*") {
        $existingIndex = $i
        break
    }
}

if ($existingIndex -ge 0) {
    $manifestContent.allowed_origins[$existingIndex] = $extensionOrigin
    Write-Host "Updated existing extension ID in manifest" -ForegroundColor Cyan
} else {
    $manifestContent.allowed_origins += $extensionOrigin
    Write-Host "Added extension ID to manifest" -ForegroundColor Cyan
}

# Write updated manifest
try {
    $manifestContent | ConvertTo-Json -Depth 10 | Set-Content -Path $manifestPath -Encoding UTF8 -NoNewline
    Write-Host "Successfully updated manifest: $manifestPath" -ForegroundColor Green
} catch {
    Write-Host "ERROR: Failed to write manifest: $_" -ForegroundColor Red
    # Restore backup if write failed
    if (Test-Path $backupPath) {
        Write-Host "Restoring backup..." -ForegroundColor Yellow
        Copy-Item -Path $backupPath -Destination $manifestPath -Force
    }
    exit 1
}

Write-Host ""
Write-Host "Extension ID linked successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "1. Close all Chrome windows completely" -ForegroundColor White
Write-Host "2. Reopen Chrome" -ForegroundColor White
Write-Host "3. Open the extension popup to verify connection" -ForegroundColor White
Write-Host ""

