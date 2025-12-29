# TabStash NVMe Windows Installer Script
# Requires: Administrator privileges
# Usage: Run as Administrator: .\install-windows.ps1

param(
    [string]$ExtensionId = "",
    [string]$BinaryPath = "$env:ProgramFiles\TabStash\tabstash-native.exe"
)

$ErrorActionPreference = "Stop"

Write-Host "Installing TabStash NVMe Native Host..." -ForegroundColor Green

# Check for admin privileges
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "ERROR: This script requires Administrator privileges." -ForegroundColor Red
    Write-Host "Please run PowerShell as Administrator and try again." -ForegroundColor Yellow
    exit 1
}

# Determine binary location (use script directory if binary exists there, otherwise use Program Files)
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$sourceBinary = Join-Path $scriptDir "tabstash-native.exe"

if (-not (Test-Path $sourceBinary)) {
    Write-Host "ERROR: tabstash-native.exe not found in script directory: $scriptDir" -ForegroundColor Red
    Write-Host "Please ensure the binary is in the same directory as this installer." -ForegroundColor Yellow
    exit 1
}

# Create installation directory
$installDir = Split-Path -Parent $BinaryPath
if (-not (Test-Path $installDir)) {
    Write-Host "Creating installation directory: $installDir" -ForegroundColor Cyan
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

# Copy binary
Write-Host "Installing binary to: $BinaryPath" -ForegroundColor Cyan
Copy-Item -Path $sourceBinary -Destination $BinaryPath -Force

# Create data directory
$dataDir = Join-Path $env:LOCALAPPDATA "TabStash\snapshots"
Write-Host "Creating data directory: $dataDir" -ForegroundColor Cyan
New-Item -ItemType Directory -Path $dataDir -Force | Out-Null

# Determine Chrome Native Messaging Hosts directory
$chromeUserData = Join-Path $env:LOCALAPPDATA "Google\Chrome\User Data\NativeMessagingHosts"
$chromeSystemData = Join-Path $env:ProgramFiles "Google\Chrome\Application\NativeMessagingHosts"

# Try user directory first, fall back to system
$manifestDir = $chromeUserData
if (-not (Test-Path $manifestDir)) {
    $manifestDir = $chromeSystemData
    if (-not (Test-Path $manifestDir)) {
        Write-Host "WARNING: Chrome NativeMessagingHosts directory not found." -ForegroundColor Yellow
        Write-Host "Attempting to create: $chromeUserData" -ForegroundColor Cyan
        New-Item -ItemType Directory -Path $chromeUserData -Force | Out-Null
        $manifestDir = $chromeUserData
    }
}

# Create manifest
$manifestPath = Join-Path $manifestDir "tabstash_native.json"

# If extension ID provided, use it; otherwise use placeholder
if ($ExtensionId -and $ExtensionId -ne "" -and $ExtensionId -ne "EXTENSION_ID_PLACEHOLDER") {
    $allowedOrigins = @("chrome-extension://$ExtensionId/")
    Write-Host "Installing manifest with extension ID: $ExtensionId" -ForegroundColor Cyan
} else {
    $allowedOrigins = @("chrome-extension://EXTENSION_ID_PLACEHOLDER/")
    Write-Host "Installing manifest with placeholder. Link extension ID later using link-extension.ps1" -ForegroundColor Yellow
}

$manifestContent = @{
    name = "tabstash_native"
    description = "Native messaging helper for TabStash NVMe"
    path = $BinaryPath
    type = "stdio"
    allowed_origins = $allowedOrigins
} | ConvertTo-Json

Write-Host "Installing manifest to: $manifestPath" -ForegroundColor Cyan
$manifestContent | Out-File -FilePath $manifestPath -Encoding UTF8 -Force

Write-Host ""
Write-Host "Installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "1. Load the extension in Developer Mode:" -ForegroundColor White
Write-Host "   - Open Chrome: chrome://extensions" -ForegroundColor Gray
Write-Host "   - Enable 'Developer mode' (top-right toggle)" -ForegroundColor Gray
Write-Host "   - Click 'Load unpacked'" -ForegroundColor Gray
Write-Host "   - Select the 'extension' directory from this repository" -ForegroundColor Gray
Write-Host ""
Write-Host "2. Link your Extension ID:" -ForegroundColor White
Write-Host "   - Copy your Extension ID from chrome://extensions or extension popup" -ForegroundColor Gray
Write-Host "   - Run: .\scripts\link-extension.ps1 `"your-extension-id`"" -ForegroundColor Gray
Write-Host ""
Write-Host "3. Restart Chrome completely (close all windows)" -ForegroundColor White
Write-Host ""
Write-Host "4. Verify installation:" -ForegroundColor White
Write-Host "   - Open extension popup" -ForegroundColor Gray
Write-Host "   - Check 'Installation Status' panel - all items should show OK" -ForegroundColor Gray
Write-Host ""
Write-Host "If you need to uninstall, run: .\uninstall-windows.ps1" -ForegroundColor Cyan
Write-Host ""
Write-Host "Note: Extension ID parameter is optional. You can link it later using link-extension.ps1" -ForegroundColor Gray

