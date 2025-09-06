# Quick install script for CPM (Crab Package Manager) (Windows)
# Usage: Invoke-WebRequest -Uri "https://raw.githubusercontent.com/JetCrabCollab/cpm/main/scripts/install.ps1" | Invoke-Expression

$ErrorActionPreference = "Stop"

$Repo = "JetCrabCollab/cpm"
$LatestReleaseUrl = "https://api.github.com/repos/$Repo/releases/latest"

Write-Host "🦀 Installing CPM (Crab Package Manager)..." -ForegroundColor Cyan

# Check dependencies
Write-Host "🔍 Checking dependencies..." -ForegroundColor Yellow

# Check for Rust/Cargo
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Cargo not found. Please install Rust first:" -ForegroundColor Red
    Write-Host "   https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

# Check for Node.js/npm
if (-not (Get-Command npm -ErrorAction SilentlyContinue)) {
    Write-Host "❌ npm not found. Please install Node.js first:" -ForegroundColor Red
    Write-Host "   https://nodejs.org/" -ForegroundColor Yellow
    exit 1
}

# Check for JetCrab
if (-not (Get-Command jetcrab -ErrorAction SilentlyContinue)) {
    Write-Host "⚠️  JetCrab not found. Installing JetCrab..." -ForegroundColor Yellow
    cargo install jetcrab
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Failed to install JetCrab. Please install manually:" -ForegroundColor Red
        Write-Host "   cargo install jetcrab" -ForegroundColor Yellow
        exit 1
    }
}

Write-Host "✅ All dependencies found!" -ForegroundColor Green

# Detect architecture
$Arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
$OS = "windows"
$Ext = "zip"

Write-Host "📡 Fetching latest release info..." -ForegroundColor Yellow

# Get latest release info
$ReleaseInfo = Invoke-RestMethod -Uri $LatestReleaseUrl
$Version = $ReleaseInfo.tag_name
$DownloadUrl = "https://github.com/$Repo/releases/download/$Version/cpm-$OS-$Arch.$Ext"

Write-Host "📦 Downloading CPM $Version for $OS-$Arch..." -ForegroundColor Yellow

# Download and install
$TempDir = [System.IO.Path]::GetTempPath()
$ZipFile = Join-Path $TempDir "cpm.zip"
$ExtractDir = Join-Path $TempDir "cpm"

Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipFile

# Extract
Expand-Archive -Path $ZipFile -DestinationPath $ExtractDir -Force

# Install to Program Files
$InstallDir = "C:\Program Files\CPM"
$BinDir = Join-Path $InstallDir "bin"

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force
}

if (-not (Test-Path $BinDir)) {
    New-Item -ItemType Directory -Path $BinDir -Force
}

Copy-Item -Path (Join-Path $ExtractDir "cpm.exe") -Destination $BinDir -Force

# Add to PATH
$CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
if ($CurrentPath -notlike "*$BinDir*") {
    $NewPath = $CurrentPath + ";" + $BinDir
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "Machine")
    $env:PATH = $env:PATH + ";" + $BinDir
}

# Cleanup
Remove-Item $ZipFile -Force
Remove-Item $ExtractDir -Recurse -Force

Write-Host "✅ CPM (Crab Package Manager) installed successfully!" -ForegroundColor Green
Write-Host "🚀 Run 'cpm --version' to verify installation" -ForegroundColor Green
Write-Host "🔧 Run 'cpm doctor' to check all dependencies" -ForegroundColor Green
Write-Host "⚠️  You may need to restart your terminal for PATH changes to take effect" -ForegroundColor Yellow
