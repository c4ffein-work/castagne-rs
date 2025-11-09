# Parser Development - Next Steps

## Summary

After analyzing the codebase, test infrastructure, and comparing with the GDScript reference implementation, here's what needs to be done to complete the Rust parser.

## Current State ✅

The parser has:
- ✅ Basic parsing infrastructure (file loading, line parsing, error reporting)
- ✅ Metadata parsing (:Character: blocks)
- ✅ Specblock parsing (raw key-value pairs)
- ✅ Variable parsing (:Variables: blocks)
- ✅ State parsing (state definitions with phases)
- ✅ Skeleton inheritance (loading parent files)
- ✅ JSON serialization (via serde)
- ✅ Test infrastructure (golden masters, comparison framework)

## What's Missing ❌

### 1. Data Transformation (CRITICAL)

**Problem**: `transformed_data` is empty in parser output

**How it should work** (from GDScript parser line 234-239):
```gdscript
for sbName in moduleSpecblocksMain:
    var sb = moduleSpecblocksMain[sbName]
    var tData = sb.TransformDefinedData(_specblockDefines)
    if(tData != null):
        _transformedData[sbName] = tData
```

**What needs to be done**:
1. Add `transform_defined_data()` method to `CastagneModule` trait
2. Implement this method in each module:
   - `Anims` module
   - `Graphics` module
   - `PhysicsMovement` module
   - `PhysicsSystem` module
   - `AttacksMechanics` module
   - `AttacksThrows` module
   - `AttacksTypes` module
   - `AudioSFX` module
   - `MenuData` module
   - `PhysicsTeching` module
   - `UI` module
3. Call these methods in parser after parsing specblocks
4. Store results in `transformed_data` field

**Implementation steps**:
```rust
// 1. Add to CastagneModule trait
pub trait CastagneModule {
    // ... existing methods ...

    /// Transform specblock data into module-specific structures
    fn transform_defined_data(
        &self,
        specblock_defines: &HashMap<String, ParsedVariable>
    ) -> Option<HashMap<String, Variant>> {
        None  // Default: no transformation
    }
}

// 2. In parser, after parsing specblocks:
fn parse_full_file(&mut self) {
    // ... existing parsing ...
    self.parse_specblocks(0);

    // NEW: Transform data using modules
    for module in &self.modules {
        if let Some(transformed) = module.transform_defined_data(&self.specblock_defines) {
            self.transformed_data.insert(
                module.module_name().to_string(),
                transformed
            );
        }
    }

    // ... continue with variables, states ...
}
```

### 2. Subentity Parsing

Currently stubbed out with:
```rust
subentities: HashMap::new(), // TODO: Implement subentity parsing
```

Needed for characters that spawn sub-entities (projectiles, helpers, etc.)

### 3. Instruction Parsing Improvements

Several instruction types are stubs:
- Integer instructions
- Flag instructions
- String instructions
- Function call instructions
- Advanced instructions
- Parser instructions
- Branch instructions
- Conditional branching

These affect action parsing within states.

### 4. Optimization Pass

Line 348 mentions an optimization step that's not implemented. This is likely for:
- Removing duplicate states
- Optimizing instruction sequences
- Pre-computing constant values

## Testing Requirements

**Problem**: Can't run parser tests without Godot runtime

**Options**:
1. **Install Godot** - Download Godot 4.x with gdextension support
2. **Run in Godot** - Use the test scripts in `scripts/` and `run_parser_tests.gd`
3. **View differences** - See what the Rust parser outputs vs golden masters

**Test files available**:
- `test_character_complete.casp` - Simple test case
- Baston files (not in repo but golden masters exist)

## Recommended Approach

### Phase 1: Get Basic Tests Running

1. Install Godot 4 (if not already available)
2. Run `cargo build` to compile the Rust extension
3. Open project in Godot
4. Run `run_parser_tests.gd` to see current differences
5. Document specific failures

### Phase 2: Implement Data Transformation

1. Add `transform_defined_data()` to `CastagneModule` trait
2. Study one module (e.g., Graphics) from GDScript version
3. Implement transformation for that module in Rust
4. Re-run tests to verify improvement
5. Repeat for other modules

### Phase 3: Fix Remaining Issues

1. Address instruction parsing issues
2. Implement subentity support if needed
3. Add optimization pass if needed
4. Re-run tests until perfect match

## Module Transformation Examples

From golden masters, here's what each module's `transformed_data` contains:

**Graphics**:
```json
{
  "Defines": { "GRAPHICS_Scale": 3000, ... },
  "Spritesheets": {
    "TemporaryStickman": {
      "SpritesX": 16,
      "SpritesY": 4,
      "OriginX": 32,
      "OriginY": 6,
      "PixelSize": 100000
    }
  },
  "Palettes": {
    "0": { "DisplayName": "Blue", ... }
  }
}
```

**Anims**:
```json
{
  "Defines": {
    "ANIM_Movement_Basic_Stand_Loop": 56,
    "ANIM_Movement_Basic_WalkF_Loop": 52,
    ...
  },
  "SpriteAnimations": { ... }
}
```

Each module has its own structure based on what data it needs.

## Reference Files

- **GDScript parser**: `castagne_godot4/engine/CastagneParser.gd` (2279 lines)
- **Rust parser**: `src/parser.rs` (3664 lines)
- **Module trait**: `src/module.rs`
- **Test framework**: `src/test_runner.rs`
- **Golden masters**: `golden_masters/*.json`

## Questions to Resolve

1. Do we have Godot 4 with gdextension support available?
2. Can we access the original Baston .casp files for testing?
3. Should we generate a new golden master for `test_character_complete.casp`?
4. Which module should we implement transformation for first?

## Success Criteria

Parser is complete when:
- ✅ All tests in `run_parser_tests.gd` pass
- ✅ `transformed_data` matches golden masters exactly
- ✅ All metadata, variables, states, and subentities match
- ✅ No differences reported by comparison framework

---

Generated: 2025-11-09
Status: Ready for implementation
Priority: Data transformation (critical path)
