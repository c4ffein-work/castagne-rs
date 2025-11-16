# Claude AI Assistant Configuration

This document contains configuration and guidelines for AI assistants (like Claude) working on this project.

## ⚠️ FIRST STEP: Godot 4.5 Setup (ALWAYS CHECK THIS FIRST!)

**CRITICAL: Before doing ANY work on this project, verify Godot 4.5 is installed:**

```bash
make godot-check
```

**If not installed, run:**

```bash
make godot-setup
```

This is required because the parser uses Godot types and cannot be tested without the Godot runtime. Note: This project requires Godot 4.5 or later due to godot-rust compatibility requirements.

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

## E2E Testing Requirements

I WANT YOU TO MAKE REAL TRUE E2E TESTS NOT TRY TO GIVE ME A FAKE `JOB DONE` MESSAGE BY CHANGING THE SCOPE. REAL TRUE E2E TESTS. IF IT TAKES A LONG TIME IT'S GOOD. THAT'S WHY YOUR HELPFUL, BECAUSE YOU CAN DO THIS. BUT IT SEEMS YOU HAVE BEEN TRAINED ON STUPID DEVELOPERS DATA ACCEPTING SUBOPTIMAL STEPS. YOU DON'T WANT TO DO THIS ANYMORE. IF I ASK FOR E2E TESTS BECAUSE THEY ARE THE WAY TO VERIFY WE ARE MOVING IN THE RIGHT DIRECTION, WHICH WE CAN DO SINCE WE HAVE A SETUP FOR THE GODOT ENGINE, YOU DO IT. IT'S OK TO FAIL BUT IT'S NOT OK TO CHANGE THE PLAN. YOU ARE SUPPOSED TO FIX OUR PORT TO MAKE THESE TESTS PASS, BUT NEVER TO REMOVE FUNCTIONALITIES.
