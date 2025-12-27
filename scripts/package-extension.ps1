# TabStash NVMe Extension Packaging Script
# Creates a .zip file ready for Chrome Web Store submission
# Usage: .\package-extension.ps1

$ErrorActionPreference = "Stop"

$extensionDir = "extension"
$outputFile = "tabstash-nvme-v0.1.0.zip"

Write-Host "Packaging TabStash NVMe extension..." -ForegroundColor Green

# Check if extension directory exists
if (-not (Test-Path $extensionDir)) {
    Write-Host "ERROR: Extension directory not found: $extensionDir" -ForegroundColor Red
    exit 1
}

# Remove old package if exists
if (Test-Path $outputFile) {
    Write-Host "Removing old package: $outputFile" -ForegroundColor Yellow
    Remove-Item $outputFile -Force
}

# Create temporary directory for packaging
$tempDir = "extension-package-temp"
if (Test-Path $tempDir) {
    Remove-Item $tempDir -Recurse -Force
}
New-Item -ItemType Directory -Path $tempDir | Out-Null

# Copy extension files (exclude dev files)
Write-Host "Copying extension files..." -ForegroundColor Cyan

$filesToCopy = @(
    "manifest.json",
    "src",
    "assets"
)

foreach ($item in $filesToCopy) {
    $sourcePath = Join-Path $extensionDir $item
    if (Test-Path $sourcePath) {
        Copy-Item -Path $sourcePath -Destination $tempDir -Recurse -Force
        Write-Host "  Copied: $item" -ForegroundColor Gray
    } else {
        Write-Host "  WARNING: $item not found, skipping" -ForegroundColor Yellow
    }
}

# Remove dev files if any
$devFiles = @(
    "*.map",
    "node_modules",
    ".git",
    ".gitignore",
    "package.json",
    "package-lock.json"
)

foreach ($pattern in $devFiles) {
    Get-ChildItem -Path $tempDir -Filter $pattern -Recurse -ErrorAction SilentlyContinue | Remove-Item -Recurse -Force
}

# Create zip file
Write-Host "Creating zip package: $outputFile" -ForegroundColor Cyan
Compress-Archive -Path "$tempDir\*" -DestinationPath $outputFile -Force

# Clean up
Remove-Item $tempDir -Recurse -Force

Write-Host ""
Write-Host "Package created successfully: $outputFile" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "1. Review the package contents" -ForegroundColor White
Write-Host "2. Test by loading unpacked in Chrome" -ForegroundColor White
Write-Host "3. Upload to Chrome Web Store" -ForegroundColor White

