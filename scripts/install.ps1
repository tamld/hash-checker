# ==============================================================================
# hash-checker — Windows PowerShell Standalone Installer (CLI + GUI)
# Modeled on g8s Zero-Trust Release & Checksum Verification Standard
#
# Usage:
#   irm https://raw.githubusercontent.com/tamld/hash-checker/main/scripts/install.ps1 | iex
#   $env:HASH_CHECKER_VERSION="v0.2.1"; .\scripts\install.ps1
# ==============================================================================
$ErrorActionPreference = "Stop"

$Repo = if ($env:HASH_CHECKER_REPO) { $env:HASH_CHECKER_REPO } else { "tamld/hash-checker" }
$Version = if ($env:HASH_CHECKER_VERSION) { $env:HASH_CHECKER_VERSION } else { "latest" }
$InstallDir = if ($env:INSTALL_DIR) { $env:INSTALL_DIR } else { Join-Path $env:LOCALAPPDATA "Programs\hash-checker\bin" }

if ($Version -eq "latest") {
    Write-Host "[hash-checker-install] Resolving latest release from https://github.com/$Repo ..." -ForegroundColor Cyan
    $Release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
    $Version = $Release.tag_name
    if (-not $Version) {
        throw "Could not resolve latest release tag from GitHub API."
    }
}

$VersionNum = $Version.TrimStart("v")
$BaseUrl = "https://github.com/$Repo/releases/download/$Version"
$Archive = "hash-checker_${VersionNum}_windows_amd64.zip"
$ChecksumsUrl = "$BaseUrl/checksums.txt"

$TmpDir = Join-Path ([System.IO.Path]::GetTempPath()) ("hash-checker-install-" + [System.Guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Path $TmpDir -Force | Out-Null

try {
    $ArchivePath = Join-Path $TmpDir $Archive
    $ChecksumsPath = Join-Path $TmpDir "checksums.txt"

    Write-Host "[hash-checker-install] Downloading $Archive ($Version) ..." -ForegroundColor Cyan
    Invoke-WebRequest -Uri "$BaseUrl/$Archive" -OutFile $ArchivePath

    Write-Host "[hash-checker-install] Downloading checksums.txt ..." -ForegroundColor Cyan
    Invoke-WebRequest -Uri $ChecksumsUrl -OutFile $ChecksumsPath

    Write-Host "[hash-checker-install] Verifying SHA-256 checksum (fail-closed) ..." -ForegroundColor Cyan
    $ExpectedLine = Get-Content $ChecksumsPath | Where-Object { $_ -match "\s+\*?$([Regex]::Escape($Archive))$" } | Select-Object -First 1
    if (-not $ExpectedLine) {
        throw "Archive $Archive not found in checksums.txt!"
    }
    $ExpectedSha = ($ExpectedLine -split "\s+")[0].ToLowerInvariant()
    $ActualSha = (Get-FileHash -Path $ArchivePath -Algorithm SHA256).Hash.ToLowerInvariant()

    if ($ExpectedSha -ne $ActualSha) {
        throw "SHA-256 mismatch for $Archive! Expected $ExpectedSha, got $ActualSha"
    }
    Write-Host "[hash-checker-install] SHA-256 verified: $ActualSha" -ForegroundColor Green

    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    Expand-Archive -Path $ArchivePath -DestinationPath $TmpDir -Force

    foreach ($Bin in @("hash-checker.exe", "hash-checker-gui.exe")) {
        $BinPath = Join-Path $TmpDir $Bin
        if (Test-Path $BinPath) {
            Copy-Item -Path $BinPath -Destination (Join-Path $InstallDir $Bin) -Force
            Write-Host "[hash-checker-install] Installed $Bin to $InstallDir\$Bin" -ForegroundColor Green
        }
    }

    & (Join-Path $InstallDir "hash-checker.exe") --version
} finally {
    Remove-Item -Path $TmpDir -Recurse -Force -ErrorAction SilentlyContinue
}
