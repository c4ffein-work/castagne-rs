# Claude AI Assistant Configuration

This document contains configuration and guidelines for AI assistants (like Claude) working on this project.

## ⚠️ FIRST STEP: Godot 4 Setup (ALWAYS CHECK THIS FIRST!)

**CRITICAL: Before doing ANY work on this project, you MUST verify that Godot 4 is installed and set up.**

### Check if Godot 4 is Available

1. **Check for godot command**:
   ```bash
   which godot
   # or
   godot --version
   ```

2. **Expected output**: Should show Godot 4.x (e.g., "4.2.stable.official")

### If Godot 4 is NOT Installed

You MUST install it before proceeding. This project requires Godot 4 with GDExtension support to:
- Run parser integration tests
- Test the Rust parser against golden masters
- Validate any changes to the parser

**Installation Steps**:

1. **Download Godot 4**:
   ```bash
   # Create directory for Godot
   mkdir -p ~/godot
   cd ~/godot

   # Download Godot 4 (adjust version as needed)
   wget https://github.com/godotengine/godot/releases/download/4.2-stable/Godot_v4.2-stable_linux.x86_64.zip

   # Extract
   unzip Godot_v4.2-stable_linux.x86_64.zip

   # Make executable
   chmod +x Godot_v4.2-stable_linux.x86_64

   # Create symlink for easy access
   sudo ln -s ~/godot/Godot_v4.2-stable_linux.x86_64 /usr/local/bin/godot
   ```

2. **Verify installation**:
   ```bash
   godot --version
   ```

3. **Test with project**:
   ```bash
   # From project root
   godot --headless --script scripts/run_comparison_tests.gd
   ```

### Why This Matters

The Rust parser:
- Uses Godot types (`Variant`, `GString`, `Vector2`, etc.)
- Requires Godot runtime to execute
- Cannot be fully tested with `cargo test` alone
- Needs Godot to run integration tests against golden masters

**Without Godot 4 installed, you cannot:**
- Run parser tests
- Validate parser output
- See actual differences vs golden masters
- Complete any parser-related work

## Network Access Requirements

When working on this project, AI assistants may need to download dependencies and resources. If you encounter problems during dependency installation or downloads, you must authorize the following domains:

### Required Domains

- **crates.io** - Rust package registry (crate downloads)
- **index.crates.io** - Rust package index
- **github.com** - GitHub repository access
- **release-assets.githubusercontent.com** - GitHub release assets (e.g., Godot engine downloads)

### Why These Domains?

- `crates.io` and `index.crates.io`: Required for Cargo to fetch Rust dependencies
- `github.com`: Required for git operations and accessing the repository
- `release-assets.githubusercontent.com`: Required for downloading pre-built binaries like the Godot engine

## Troubleshooting

If you see errors like:
- "Failed to download..."
- "Connection refused..."
- "Network error..."

Check that all the above domains are authorized in your AI assistant's configuration.
