#!/bin/bash
# Run golden master generation using Godot 3
# This script downloads Godot 3 if needed and runs the generation script

set -e

# Configuration
GODOT3_VERSION="${GODOT3_VERSION:-3.5.3}"
GODOT3_DIR=".godot3-bin"
GODOT3_BIN="${GODOT3_DIR}/godot3"

echo "==========================================="
echo "  Golden Master Generation for Castagne"
echo "==========================================="
echo ""

# Check if Godot 3 is already installed
if [ ! -f "${GODOT3_BIN}" ]; then
    echo "Godot 3 not found. Downloading Godot ${GODOT3_VERSION}..."
    echo ""

    # Create installation directory
    mkdir -p "${GODOT3_DIR}"

    # Determine platform
    OS=$(uname -s)
    ARCH=$(uname -m)

    echo "Detected OS: ${OS}"
    echo "Detected Architecture: ${ARCH}"
    echo ""

    # Construct download URL for Godot 3
    case "${OS}" in
        Linux)
            if [ "${ARCH}" = "x86_64" ]; then
                GODOT_FILE="Godot_v${GODOT3_VERSION}-stable_linux_headless.64"
                DOWNLOAD_URL="https://github.com/godotengine/godot/releases/download/${GODOT3_VERSION}-stable/${GODOT_FILE}.zip"
            else
                echo "Error: Unsupported architecture: ${ARCH}"
                exit 1
            fi
            ;;
        Darwin)
            GODOT_FILE="Godot_v${GODOT3_VERSION}-stable_osx.universal"
            DOWNLOAD_URL="https://github.com/godotengine/godot/releases/download/${GODOT3_VERSION}-stable/${GODOT_FILE}.zip"
            ;;
        *)
            echo "Error: Unsupported OS: ${OS}"
            exit 1
            ;;
    esac

    echo "Downloading Godot ${GODOT3_VERSION} headless..."
    echo "URL: ${DOWNLOAD_URL}"
    echo ""

    # Download Godot 3
    if command -v wget &> /dev/null; then
        wget -q --show-progress "${DOWNLOAD_URL}" -O "${GODOT3_DIR}/godot3.zip"
    elif command -v curl &> /dev/null; then
        curl -L --progress-bar "${DOWNLOAD_URL}" -o "${GODOT3_DIR}/godot3.zip"
    else
        echo "Error: Neither wget nor curl found. Please install one of them."
        exit 1
    fi

    echo ""
    echo "Extracting Godot 3..."

    # Extract Godot
    cd "${GODOT3_DIR}"
    unzip -q godot3.zip

    # Find the extracted binary and rename it to 'godot3'
    case "${OS}" in
        Linux)
            mv "${GODOT_FILE}" godot3
            ;;
        Darwin)
            if [ -d "Godot.app" ]; then
                mv Godot.app/Contents/MacOS/Godot godot3
                rm -rf Godot.app
            else
                mv "${GODOT_FILE}" godot3
            fi
            ;;
    esac

    # Make it executable
    chmod +x godot3

    # Clean up
    rm godot3.zip

    cd ..

    echo ""
    echo "✓ Godot 3 ${GODOT3_VERSION} installed successfully!"
    echo ""
else
    INSTALLED_VERSION=$(${GODOT3_BIN} --version 2>&1 | head -n1 || echo "unknown")
    echo "Using existing Godot 3: ${INSTALLED_VERSION}"
    echo ""
fi

# Run the golden master generation script
echo "Running golden master generation..."
echo ""

# Temporarily swap project files to use Godot 3 configuration
# Godot 3 always looks for 'project.godot', so we need to swap files
if [ -f "project.godot" ]; then
    mv project.godot project_godot4.godot.bak
fi
cp project3.godot project.godot

# Run the generation script
${GODOT3_BIN} --path . --script scripts/generate_golden_masters.gd

# Save the exit code
EXIT_CODE=$?

# Restore original project file
rm project.godot
if [ -f "project_godot4.godot.bak" ]; then
    mv project_godot4.godot.bak project.godot
fi

# Exit with the saved exit code
exit $EXIT_CODE

echo ""
echo "==========================================="
echo "  Golden Master Generation Complete"
echo "==========================================="
echo ""
echo "Generated files are in: golden_masters/"
echo ""
