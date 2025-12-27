# TabStash NVMe Windows Uninstaller Script
# Requires: Administrator privileges
# Usage: Run as Administrator: .\uninstall-windows.ps1

$ErrorActionPreference = "Stop"

Write-Host "Uninstalling TabStash NVMe Native Host..." -ForegroundColor Green

# Check for admin privileges
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "ERROR: This script requires Administrator privileges." -ForegroundColor Red
    Write-Host "Please run PowerShell as Administrator and try again." -ForegroundColor Yellow
    exit 1
}

# Remove binary
$binaryPath = "$env:ProgramFiles\TabStash\tabstash-native.exe"
if (Test-Path $binaryPath) {
    Write-Host "Removing binary: $binaryPath" -ForegroundColor Cyan
    Remove-Item -Path $binaryPath -Force
}

# Remove installation directory if empty
$installDir = "$env:ProgramFiles\TabStash"
if (Test-Path $installDir) {
    $items = Get-ChildItem -Path $installDir -Force
    if ($items.Count -eq 0) {
        Write-Host "Removing installation directory: $installDir" -ForegroundColor Cyan
        Remove-Item -Path $installDir -Force
    }
}

# Remove manifest
$chromeUserData = Join-Path $env:LOCALAPPDATA "Google\Chrome\User Data\NativeMessagingHosts"
$chromeSystemData = Join-Path $env:ProgramFiles "Google\Chrome\Application\NativeMessagingHosts"

$manifestPath = Join-Path $chromeUserData "tabstash_native.json"
if (Test-Path $manifestPath) {
    Write-Host "Removing manifest: $manifestPath" -ForegroundColor Cyan
    Remove-Item -Path $manifestPath -Force
}

$manifestPath = Join-Path $chromeSystemData "tabstash_native.json"
if (Test-Path $manifestPath) {
    Write-Host "Removing manifest: $manifestPath" -ForegroundColor Cyan
    Remove-Item -Path $manifestPath -Force
}

# Note: Data directory is NOT removed by default (preserves user data)
$dataDir = Join-Path $env:LOCALAPPDATA "TabStash\snapshots"
if (Test-Path $dataDir) {
    Write-Host ""
    Write-Host "NOTE: Data directory preserved: $dataDir" -ForegroundColor Yellow
    Write-Host "To remove user data, manually delete this directory." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Uninstallation complete!" -ForegroundColor Green

