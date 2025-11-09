# Golden Master Generation Pipeline

This directory contains scripts for generating golden master files from Castagne .casp files using the original Godot 3 parser.

## What are Golden Masters?

Golden masters are reference JSON files containing the "correct" parser output from the original GDScript Castagne parser. The Rust parser's output can then be compared against these golden masters to verify correctness.

## Pipeline Overview

```
┌─────────────────────┐
│  .casp files        │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Godot 3 + Original │
│  Castagne Parser    │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Golden Master JSON │
│  (expected output)  │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Rust Parser Tests  │
│  (compare output)   │
└─────────────────────┘
```

## Setup

### Prerequisites

- Linux or macOS
- `wget` or `curl` (for downloading Godot)
- `unzip`
- Initialized Castagne submodule (`git submodule update --init --recursive`)

### Installation

The `run-golden-master-generation.sh` script will automatically:
1. Download Godot 3.5.3 headless if not already installed (to `.godot3-bin/`)
2. Set up the correct Godot 3 project configuration
3. Run the golden master generation script

## Usage

### Generate Golden Masters

```bash
./scripts/run-golden-master-generation.sh
```

This will:
- Parse all `.casp` files listed in `scripts/generate_golden_masters.gd`
- Generate JSON files in `golden_masters/` directory
- Report success/failure for each file

### Output Format

Golden master JSON files contain:

```json
{
  "metadata": {
    "name": "Character Name",
    "author": "Author",
    "description": "Description",
    "version": "1.0",
    "filepath": "path/to/file.casp"
  },
  "subentities": {
    "EntityName": { ... }
  },
  "variables": {
    "VariableName": {
      "Name": "VariableName",
      "Value": "100",
      "Type": "Int",
      "Subtype": "",
      "Mutability": "Variable"
    }
  },
  "states": {
    "StateName": {
      "Parent": null,
      "Type": "Normal",
      "TransitionFlags": [],
      "Phases": {
        "Init": {
          "instruction_count": 5,
          "instructions": [...]
        }
      }
    }
  },
  "transformed_data": {
    ...
  }
}
```

## Files

- `run-golden-master-generation.sh` - Main script to run golden master generation
- `generate_golden_masters.gd` - GDScript that interfaces with the Castagne parser
- `setup-godot.sh` - Downloads and installs Godot 4 (for comparison tests)

## Technical Details

### Godot 3 vs Godot 4

- The original Castagne parser is written for **Godot 3**
- The project has both `project.godot` (Godot 4) and `project3.godot` (Godot 3)
- The run script temporarily swaps these files so Godot 3 can load properly

### Parser Initialization

The Castagne parser requires:
1. A configured `CastagneConfig` object with module list
2. Autoloaded `Castagne` global (from `CastagneGlobal.gd`)
3. All Castagne modules loaded and initialized

### Current Status

✅ **Working:**
- Godot 3 download and installation
- Project file swapping mechanism
- Castagne module initialization
- Parser loading and execution
- JSON serialization and file generation

⚠️ **Known Issues:**
- Complex files (like Baston-Model.casp) may require additional module configuration
- Test files with incorrect syntax will fail to parse (see Syntax below)

### Castagne Syntax

The correct Castagne variable syntax is:

```
:Variables:
var variableName int() = 100
var myString str() = "hello"
var myBool bool() = true
```

NOT:

```
var Health(Int): 100  # ❌ Wrong - will fail to parse
```

## Troubleshooting

### "Missing parenthesis on variable type" error

Your `.casp` file is using incorrect syntax. Check the Castagne examples in `castagne/examples/` for correct formatting.

### "Can't open project at..." error

The script should handle this automatically, but if you see this error, ensure:
1. `project3.godot` exists and is configured for Godot 3
2. The run script is restoring files properly

### Parser hangs or takes too long

Some complex files may require the full Castagne module system. Try:
1. Simpler test files first
2. Check that the file doesn't have circular skeleton dependencies
3. Increase timeout in the script if needed

## Next Steps

1. Create test `.casp` files with correct Castagne syntax
2. Generate golden masters for common test cases
3. Update Rust parser tests to load and compare against golden masters
4. Implement full comparison logic in `src/test_runner.rs`

## References

- Castagne Documentation: https://github.com/panthavma/castagne
- Godot 3 Documentation: https://docs.godotengine.org/en/3.5/
