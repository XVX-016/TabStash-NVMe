# Update Extension ID in Installer Scripts
# Usage: .\update-extension-id.ps1 -ExtensionId "your-extension-id-here"

param(
    [Parameter(Mandatory=$true)]
    [string]$ExtensionId
)

$ErrorActionPreference = "Stop"

Write-Host "Updating Extension ID to: $ExtensionId" -ForegroundColor Green

# Validate extension ID format (basic check)
if ($ExtensionId -notmatch '^[a-z]{32}$') {
    Write-Host "WARNING: Extension ID format may be incorrect." -ForegroundColor Yellow
    Write-Host "Expected format: 32 lowercase letters (e.g., abcdefghijklmnopqrstuvwxyzabcdef)" -ForegroundColor Yellow
    $confirm = Read-Host "Continue anyway? (y/N)"
    if ($confirm -ne 'y' -and $confirm -ne 'Y') {
        exit 1
    }
}

$filesUpdated = 0

# Files to update
$filesToUpdate = @(
    @{
        Path = "installers\tabstash-installer.iss"
        Pattern = '#define ExtensionId "EXTENSION_ID_PLACEHOLDER"'
        Replacement = "#define ExtensionId `"$ExtensionId`""
        Description = "Inno Setup installer script"
    },
    @{
        Path = "scripts\install-windows.ps1"
        Pattern = '\[string\]\$ExtensionId = "EXTENSION_ID_PLACEHOLDER"'
        Replacement = "[string]`$ExtensionId = `"$ExtensionId`""
        Description = "Windows PowerShell installer script"
    },
    @{
        Path = "native-host\config\tabstash-native.json.template"
        Pattern = 'chrome-extension://EXTENSION_ID_PLACEHOLDER/'
        Replacement = "chrome-extension://$ExtensionId/"
        Description = "Native messaging manifest template"
    }
)

foreach ($file in $filesToUpdate) {
    $filePath = $file.Path
    
    if (-not (Test-Path $filePath)) {
        Write-Host "WARNING: File not found: $filePath" -ForegroundColor Yellow
        continue
    }
    
    $content = Get-Content -Path $filePath -Raw
    
    if ($content -match [regex]::Escape($file.Pattern)) {
        $newContent = $content -replace [regex]::Escape($file.Pattern), $file.Replacement
        Set-Content -Path $filePath -Value $newContent -NoNewline
        Write-Host "  Updated: $($file.Description)" -ForegroundColor Cyan
        $filesUpdated++
    } else {
        Write-Host "  Skipped: $($file.Description) (pattern not found)" -ForegroundColor Gray
    }
}

Write-Host ""
if ($filesUpdated -gt 0) {
    Write-Host "Successfully updated $filesUpdated file(s)!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Yellow
    Write-Host "1. Rebuild installer: .\scripts\build-installer.ps1" -ForegroundColor White
    Write-Host "2. Update Linux installer script manually if needed" -ForegroundColor White
    Write-Host "3. Test installer with published extension" -ForegroundColor White
    Write-Host "4. Upload updated installer to GitHub Releases" -ForegroundColor White
} else {
    Write-Host "No files were updated. Check extension ID format and file paths." -ForegroundColor Yellow
}

