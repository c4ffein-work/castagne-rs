#!/bin/bash
# Run comparison tests between Castagne (GDScript) and castagne-rs (Rust)

set -e

# Allow override via environment variable
GODOT_BIN="${GODOT_BIN:-.godot-bin/godot}"
TEST_SCRIPT="scripts/run_comparison_tests.gd"

echo "======================================"
echo "Castagne-RS Comparison Test Runner"
echo "======================================"
echo ""

# Check if Godot is installed locally
if [ ! -f "${GODOT_BIN}" ]; then
    # Try to find Godot in PATH
    if command -v godot &> /dev/null; then
        echo "ℹ Using system Godot from PATH"
        GODOT_BIN="godot"
    else
        echo "Error: Godot not found"
        echo ""
        echo "Options:"
        echo "  1. Run the setup script: ./scripts/setup-godot.sh"
        echo "  2. Install Godot system-wide"
        echo "  3. Set GODOT_BIN environment variable to your Godot path"
        echo ""
        exit 1
    fi
fi

echo "✓ Found Godot at: ${GODOT_BIN}"
echo ""

# Build the Rust library
echo "Building castagne-rs Rust library..."
cargo build --release
echo "✓ Build complete"
echo ""

# Check if test script exists
if [ ! -f "${TEST_SCRIPT}" ]; then
    echo "Error: Test script not found at ${TEST_SCRIPT}"
    echo "This should have been created during setup."
    exit 1
fi

echo "Running comparison tests..."
echo ""
echo "----------------------------------------"

# Run the test script with Godot headless
${GODOT_BIN} --headless --script "${TEST_SCRIPT}"

EXIT_CODE=$?

echo "----------------------------------------"
echo ""

if [ ${EXIT_CODE} -eq 0 ]; then
    echo "✓ Tests completed successfully!"
else
    echo "✗ Tests failed with exit code: ${EXIT_CODE}"
    exit ${EXIT_CODE}
fi
