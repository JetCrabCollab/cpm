#!/bin/bash
set -e

# Quick install script for CPM (Crab Package Manager)
# Usage: curl -sSL https://raw.githubusercontent.com/JetCrabCollab/cpm/main/scripts/install.sh | bash

REPO="JetCrabCollab/cpm"
LATEST_RELEASE_URL="https://api.github.com/repos/$REPO/releases/latest"

echo "🦀 Installing CPM (Crab Package Manager)..."

# Check dependencies
echo "🔍 Checking dependencies..."

# Check for Rust/Cargo
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check for Node.js/npm
if ! command -v npm &> /dev/null; then
    echo "❌ npm not found. Please install Node.js first:"
    echo "   https://nodejs.org/"
    exit 1
fi

# Check for JetCrab
if ! command -v jetcrab &> /dev/null; then
    echo "⚠️  JetCrab not found. Installing JetCrab..."
    cargo install jetcrab
    if [ $? -ne 0 ]; then
        echo "❌ Failed to install JetCrab. Please install manually:"
        echo "   cargo install jetcrab"
        exit 1
    fi
fi

echo "✅ All dependencies found!"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture names
case $ARCH in
    x86_64)
        ARCH="x86_64"
        ;;
    arm64|aarch64)
        ARCH="aarch64"
        ;;
    armv7l)
        ARCH="armv7"
        ;;
    *)
        echo "❌ Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Map OS names
case $OS in
    linux)
        OS="linux"
        EXT="tar.gz"
        ;;
    darwin)
        OS="macos"
        EXT="tar.gz"
        ;;
    *)
        echo "❌ Unsupported OS: $OS"
        exit 1
        ;;
esac

# Get latest release info
echo "📡 Fetching latest release info..."
RELEASE_INFO=$(curl -s $LATEST_RELEASE_URL)
VERSION=$(echo $RELEASE_INFO | grep -o '"tag_name": "[^"]*' | grep -o '[^"]*$')
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/cpm-$OS-$ARCH.$EXT"

echo "📦 Downloading CPM $VERSION for $OS-$ARCH..."

# Download and install
TEMP_DIR=$(mktemp -d)
cd $TEMP_DIR

curl -L -o "cpm.$EXT" "$DOWNLOAD_URL"

if [[ "$EXT" == "tar.gz" ]]; then
    tar -xzf "cpm.$EXT"
else
    unzip "cpm.$EXT"
fi

# Install to /usr/local/bin
sudo mv cpm /usr/local/bin/
sudo chmod +x /usr/local/bin/cpm

# Cleanup
cd /
rm -rf $TEMP_DIR

echo "✅ CPM (Crab Package Manager) installed successfully!"
echo "🚀 Run 'cpm --version' to verify installation"
echo "🔧 Run 'cpm doctor' to check all dependencies"
