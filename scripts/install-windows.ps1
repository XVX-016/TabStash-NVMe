# TabStash NVMe Windows Installer Script
# Requires: Administrator privileges
# Usage: Run as Administrator: .\install-windows.ps1

param(
    [string]$ExtensionId = "EXTENSION_ID_PLACEHOLDER",
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
$manifestContent = @{
    name = "tabstash_native"
    description = "Native messaging helper for TabStash NVMe"
    path = $BinaryPath
    type = "stdio"
    allowed_origins = @(
        "chrome-extension://$ExtensionId/"
    )
} | ConvertTo-Json

Write-Host "Installing manifest to: $manifestPath" -ForegroundColor Cyan
$manifestContent | Out-File -FilePath $manifestPath -Encoding UTF8 -Force

Write-Host ""
Write-Host "Installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "1. Install the TabStash NVMe extension from Chrome Web Store" -ForegroundColor White
Write-Host "2. Open the extension popup to verify connection" -ForegroundColor White
Write-Host ""
Write-Host "If you need to uninstall, run: .\uninstall-windows.ps1" -ForegroundColor Cyan

