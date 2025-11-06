# Castagne-RS Test Scripts

CLI scripts for setting up and running comparison tests.

## Scripts

### `setup-godot.sh`

Downloads and installs Godot headless for CLI testing.

**Usage:**
```bash
./scripts/setup-godot.sh
```

**Options:**
- `GODOT_VERSION=4.3.0 ./scripts/setup-godot.sh` - Use a specific version (default: 4.5.1)

**Features:**
- Auto-detects OS and architecture
- Checks for existing installations
- Supports system-wide Godot
- Interactive prompts for choices

### `run-tests.sh`

Runs comparison tests between Castagne and castagne-rs.

**Usage:**
```bash
./scripts/run-tests.sh
```

**Options:**
- `GODOT_BIN=/path/to/godot ./scripts/run-tests.sh` - Use custom Godot

**What it does:**
1. Builds the Rust library (release mode)
2. Runs comparison tests via Godot headless
3. Reports results with exit code

### `run_comparison_tests.gd`

GDScript entry point for running tests from Godot CLI.

**Usage:**
```bash
godot --headless --script scripts/run_comparison_tests.gd
```

**Note:** This is called automatically by `run-tests.sh`

## Quick Start

```bash
# First time setup
./scripts/setup-godot.sh

# Run tests
./scripts/run-tests.sh
```

## See Also

- [TESTING_CLI.md](../TESTING_CLI.md) - Complete testing guide
- [TESTING.md](../TESTING.md) - Testing strategies
- [src/test_runner.rs](../src/test_runner.rs) - Test implementation
