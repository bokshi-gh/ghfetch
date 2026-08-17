$ErrorActionPreference = "Stop"

$Repo = "bokshi-gh/ghfetch"

if ($env:GHFETCH_VERSION) {
    $Version = $env:GHFETCH_VERSION
} else {
    $Version = "latest"
}

$InstallDir = Join-Path $env:USERPROFILE ".ghfetch\bin"

if ($Version -eq "latest") {
    $ReleaseUrl = "https://github.com/$Repo/releases/latest/download"
} else {
    $ReleaseUrl = "https://github.com/$Repo/releases/download/$Version"
}

$Architecture = $env:PROCESSOR_ARCHITECTURE

switch ($Architecture) {
    "AMD64" {
        $Asset = "ghfetch-windows-x86_64.zip"
    }

    "ARM64" {
        $Asset = "ghfetch-windows-aarch64.zip"
    }

    default {
        throw "Unsupported architecture: $Architecture"
    }
}

$TempDir = Join-Path $env:TEMP "ghfetch-install"

Remove-Item `
    $TempDir `
    -Recurse `
    -Force `
    -ErrorAction SilentlyContinue

New-Item `
    -ItemType Directory `
    -Path $TempDir `
    -Force | Out-Null

$Archive = Join-Path $TempDir $Asset

Write-Host "Installing ghfetch..."
Write-Host "  Architecture: $Architecture"
Write-Host "  Version:      $Version"
Write-Host ""

Write-Host "Downloading $Asset..."

Invoke-WebRequest `
    -Uri "$ReleaseUrl/$Asset" `
    -OutFile $Archive

Write-Host "Extracting..."

Expand-Archive `
    -Path $Archive `
    -DestinationPath $TempDir `
    -Force

New-Item `
    -ItemType Directory `
    -Path $InstallDir `
    -Force | Out-Null

Copy-Item `
    (Join-Path $TempDir "ghfetch.exe") `
    (Join-Path $InstallDir "ghfetch.exe") `
    -Force

Write-Host ""
Write-Host "ghfetch installed successfully."
Write-Host ""
Write-Host "Binary:"
Write-Host "  $InstallDir\ghfetch.exe"
