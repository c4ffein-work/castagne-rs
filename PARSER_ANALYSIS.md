# Parser Analysis and Next Steps

## Current Status

Based on code analysis and documentation review, here's what we know:

### ✅ What's Implemented

1. **Basic Structure** - All core data structures with JSON serialization
   - `ParsedCharacter`, `ParsedState`, `ParsedVariable`, `ParsedAction`
   - Metadata structures with proper serialization
   
2. **File Loading** - Can load and read .casp files
   - Line-by-line parsing
   - File path tracking
   - Error reporting

3. **Core Parsing Functions**
   - `parse_metadata()` - Character metadata (:Character: block)
   - `parse_specblocks()` - Specblock definitions
   - `parse_variables()` - Variable and constant definitions  
   - `parse_states()` - State definitions with phases

4. **Skeleton Inheritance** - Can load and merge parent files
   - Recursive parsing of skeleton files
   - Proper metadata inheritance

### ⚠️ What's Incomplete or Missing

From the TODO comments in `src/parser.rs`:

1. **Line 240**: `parse_full_file` with `stop_after_specblocks=true` not implemented
2. **Line 285**: Subentity parsing not implemented
3. **Line 286**: Data transformation not implemented (critical!)
4. **Line 348**: Optimization step not implemented
5. **Lines 923-968**: Instruction execution/parsing functions mostly stubs:
   - `standard_parse_function()` - TODO parse function arguments
   - Integer instructions - TODO
   - Flag instructions - TODO
   - String instructions - TODO
   - Function call instructions - TODO
   - Advanced instructions - TODO
   - Parser instructions - TODO
   - Branch instructions - TODO
   - Conditional branching - TODO

### 🔴 Critical Missing Feature: Data Transformation

The biggest gap is **transformed_data**. Looking at the golden masters:

```json
"transformed_data": {
  "Anims": { "Defines": {...}, "SpriteAnimations": {...} },
  "Graphics": { "Defines": {...}, "Spritesheets": {...}, "Palettes": {...} },
  "PhysicsMovement": { "Defines": {...} },
  // ... many more module data sections
}
```

This section contains processed/transformed data from specblocks. The parser currently:
- Parses specblocks into raw key-value pairs ✅
- Does NOT transform them into module-specific structures ❌

## Why Tests Can't Run

The parser uses Godot types extensively:
- `godot::prelude::*`
- `Variant`, `GString`, `Vector2`, `Vector3`
- These require Godot runtime to work

Options:
1. Run tests in Godot (requires Godot installation)
2. Refactor parser to use standard Rust types (major change)
3. Create mock Godot types for testing (complex)

## Next Steps

### Immediate Actions

1. **Install Godot** (if possible) to run integration tests
2. **Generate proper golden master** for `test_character_complete.casp`
3. **Run integration tests** to see actual differences

### Parser Development Priority

1. **High Priority**: Implement data transformation
   - Study how specblocks become transformed_data
   - Implement module-specific transformations
   - This is what makes the parser functional

2. **Medium Priority**: Improve instruction parsing
   - Better argument parsing
   - Handle all instruction types properly

3. **Low Priority**: Optimization and edge cases
   - Performance improvements
   - Error handling refinements

## Questions for User

1. Can we install Godot to run the integration tests?
2. Should we focus on a specific feature first?
3. Do you have access to the original GDScript parser for reference?


## Key Discovery: Data Transformation is Module-Based

Found in `castagne_godot4/engine/CastagneParser.gd` lines 234-239:

```gdscript
# --- Transform defined data
for sbName in moduleSpecblocksMain:
    var sb = moduleSpecblocksMain[sbName]
    var tData = sb.TransformDefinedData(_specblockDefines)
    if(tData != null):
        _transformedData[sbName] = tData
```

### How It Works

1. The parser parses specblocks into raw key-value defines
2. For each module that has specblocks:
   - Call the module's `TransformDefinedData()` method
   - Pass in the raw specblock defines
   - The module transforms/processes the data into its own structure
   - Store the result in `_transformedData[moduleName]`

### Implications for Rust Parser

The Rust parser needs:
1. Access to all the modules (CastagneModule implementations)
2. Each module must implement `transform_defined_data()`
3. Call this for each module during parsing

### Module Examples

From the golden master, these modules transform data:
- `Anims` - Animation data
- `AttacksMechanics` - Attack properties  
- `AttacksThrows` - Throw properties
- `AttacksTypes` - Attack type data
- `AudioSFX` - Sound effects
- `Graphics` - Graphics, spritesheets, palettes
- `MenuData` - Menu data
- `PhysicsMovement` - Movement physics
- `PhysicsSystem` - Physics system  
- `PhysicsTeching` - Tech system
- `UI` - UI data

Each of these has custom transformation logic in its respective module file.

