#!/bin/bash
# Setup script to download and install Godot headless for CLI testing

set -e

# Configuration
GODOT_VERSION="${GODOT_VERSION:-4.5.1}"
GODOT_DIR=".godot-bin"
GODOT_BIN="${GODOT_DIR}/godot"

echo "==================================="
echo "Godot Headless Setup for Castagne"
echo "==================================="
echo ""
echo "Version: ${GODOT_VERSION}"
echo "Install directory: ${GODOT_DIR}"
echo ""

# Check if Godot is available system-wide
if command -v godot &> /dev/null; then
    SYSTEM_GODOT_VERSION=$(godot --version 2>&1 | head -n1 | awk '{print $1}' || echo "unknown")
    echo "ℹ Found system-wide Godot installation: ${SYSTEM_GODOT_VERSION}"
    echo ""
    read -p "Use system Godot instead of downloading? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        mkdir -p "${GODOT_DIR}"
        ln -sf "$(which godot)" "${GODOT_BIN}"
        echo "✓ Created symlink to system Godot at: ${GODOT_BIN}"
        exit 0
    fi
fi

# Create installation directory
mkdir -p "${GODOT_DIR}"

# Check if Godot is already installed locally
if [ -f "${GODOT_BIN}" ]; then
    INSTALLED_VERSION=$(${GODOT_BIN} --version 2>&1 | head -n1 | awk '{print $1}' || echo "unknown")
    echo "Found existing Godot installation: ${INSTALLED_VERSION}"
    echo "To reinstall, delete the ${GODOT_DIR} directory and run this script again."
    echo ""
    echo "Godot is ready at: ${GODOT_BIN}"
    exit 0
fi

# Determine platform
OS=$(uname -s)
ARCH=$(uname -m)

echo "Detected OS: ${OS}"
echo "Detected Architecture: ${ARCH}"
echo ""

# Construct download URL based on platform
case "${OS}" in
    Linux)
        if [ "${ARCH}" = "x86_64" ]; then
            GODOT_FILE="Godot_v${GODOT_VERSION}-stable_linux.x86_64"
            DOWNLOAD_URL="https://github.com/godotengine/godot/releases/download/${GODOT_VERSION}-stable/${GODOT_FILE}.zip"
        else
            echo "Error: Unsupported architecture: ${ARCH}"
            exit 1
        fi
        ;;
    Darwin)
        GODOT_FILE="Godot_v${GODOT_VERSION}-stable_macos.universal"
        DOWNLOAD_URL="https://github.com/godotengine/godot/releases/download/${GODOT_VERSION}-stable/${GODOT_FILE}.zip"
        ;;
    *)
        echo "Error: Unsupported OS: ${OS}"
        exit 1
        ;;
esac

echo "Downloading Godot ${GODOT_VERSION}..."
echo "URL: ${DOWNLOAD_URL}"
echo ""

# Download Godot
if command -v wget &> /dev/null; then
    wget -q --show-progress "${DOWNLOAD_URL}" -O "${GODOT_DIR}/godot.zip"
elif command -v curl &> /dev/null; then
    curl -L --progress-bar "${DOWNLOAD_URL}" -o "${GODOT_DIR}/godot.zip"
else
    echo "Error: Neither wget nor curl found. Please install one of them."
    exit 1
fi

echo ""
echo "Extracting Godot..."

# Extract Godot
cd "${GODOT_DIR}"
unzip -q godot.zip

# Find the extracted binary and rename it to 'godot'
case "${OS}" in
    Linux)
        mv "${GODOT_FILE}" godot
        ;;
    Darwin)
        # On macOS, the extracted file is typically Godot.app
        if [ -d "Godot.app" ]; then
            mv Godot.app/Contents/MacOS/Godot godot
            rm -rf Godot.app
        else
            mv "${GODOT_FILE}" godot
        fi
        ;;
esac

# Make it executable
chmod +x godot

# Clean up
rm godot.zip

cd ..

echo ""
echo "✓ Godot ${GODOT_VERSION} installed successfully!"
echo ""
echo "Location: ${GODOT_BIN}"
echo "Version check:"
${GODOT_BIN} --version
echo ""
echo "You can now run tests with: ./scripts/run-tests.sh"
