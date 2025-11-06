# Claude AI Assistant Configuration

This document contains configuration and guidelines for AI assistants (like Claude) working on this project.

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
