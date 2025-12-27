# Build TabStash NVMe Windows Installer
# Requires: Inno Setup installed and in PATH
# Usage: .\build-installer.ps1 [-ExtensionId "your-id"]

param(
    [string]$ExtensionId = "EXTENSION_ID_PLACEHOLDER"
)

$ErrorActionPreference = "Stop"

Write-Host "Building TabStash NVMe Windows Installer..." -ForegroundColor Green

# Check if Inno Setup is installed
$isccPath = Get-Command iscc -ErrorAction SilentlyContinue
if (-not $isccPath) {
    Write-Host "ERROR: Inno Setup Compiler (iscc) not found in PATH." -ForegroundColor Red
    Write-Host "Please install Inno Setup from https://jrsoftware.org/isinfo.php" -ForegroundColor Yellow
    Write-Host "Or add Inno Setup bin directory to your PATH." -ForegroundColor Yellow
    exit 1
}

# Check if binary exists
$binaryPath = "tabstash-native\target\release\tabstash-native.exe"
if (-not (Test-Path $binaryPath)) {
    Write-Host "ERROR: Binary not found at: $binaryPath" -ForegroundColor Red
    Write-Host "Please build the binary first:" -ForegroundColor Yellow
    Write-Host "  cd tabstash-native" -ForegroundColor White
    Write-Host "  cargo build --release" -ForegroundColor White
    exit 1
}

# Check if installer script exists
$issPath = "installers\tabstash-installer.iss"
if (-not (Test-Path $issPath)) {
    Write-Host "ERROR: Installer script not found: $issPath" -ForegroundColor Red
    exit 1
}

# Create dist directory if it doesn't exist
$distDir = "dist"
if (-not (Test-Path $distDir)) {
    New-Item -ItemType Directory -Path $distDir | Out-Null
}

# Build installer with extension ID
Write-Host "Building installer with extension ID: $ExtensionId" -ForegroundColor Cyan
Write-Host "Using Inno Setup script: $issPath" -ForegroundColor Cyan

$buildArgs = @(
    "/DExtensionId=$ExtensionId",
    $issPath
)

& iscc $buildArgs

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "Installer built successfully!" -ForegroundColor Green
    Write-Host "Output: dist\tabstash-installer-windows.exe" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Yellow
    Write-Host "1. Test the installer on a clean Windows system" -ForegroundColor White
    Write-Host "2. Verify extension can connect after installation" -ForegroundColor White
    Write-Host "3. Upload to GitHub Releases" -ForegroundColor White
} else {
    Write-Host ""
    Write-Host "ERROR: Installer build failed!" -ForegroundColor Red
    exit 1
}

