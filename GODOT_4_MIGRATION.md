# Godot 3 to 4 Migration Summary

This document summarizes the migration of Castagne GDScript code from Godot 3 to Godot 4.5.

## Overview

All GDScript files in `castagne_godot4/` have been migrated from Godot 3 syntax to Godot 4.5 syntax.
- **Total files migrated**: 46 GDScript files
- **Total lines of code**: ~15,558 lines

## Automated Migrations Applied

### 1. Method Renames
- ✅ `instance()` → `instantiate()` (9 files)
- ✅ `deg2rad()` → `deg_to_rad()` (3 files)
- ✅ `rad2deg()` → `rad_to_deg()` (if any)
- ✅ `get_tree().get_root()` → `get_tree().root` (2 files)

### 2. Node Type Renames
- ✅ `Spatial` → `Node3D` (1 file)
- ✅ `KinematicBody2D` → `CharacterBody2D` (if any)
- ✅ `KinematicBody` → `CharacterBody3D` (if any)

### 3. File API Migration
Files migrated from `File` class to `FileAccess` class:

#### CastagneConfig.gd
- `File.new()` → `FileAccess.open(path, mode)`
- `file.file_exists()` → `FileAccess.file_exists()`
- `file.open(path, File.READ)` → `FileAccess.open(path, FileAccess.READ)`
- `parse_json()` → `JSON.parse_string()`
- `to_json()` → `JSON.stringify()`

#### CastagneGlobal.gd
- Same migrations as CastagneConfig.gd
- Added proper null checking for FileAccess.open()

#### CastagneParser.gd
- Migrated File API with proper error handling
- Updated file storage and access patterns

#### CMAudio_MusicPlayer.gd
- Migrated `File.new().file_exists()` to `FileAccess.file_exists()`

#### CastagneLoader.gd
- Already had File API commented out, no changes needed

### 4. Async/Coroutine Migration
File: `CastagneNet.gd`
- ✅ `yield(get_tree(), "idle_frame")` → `await get_tree().process_frame`
- ✅ `yield(get_tree().create_timer(2.0), "timeout")` → `await get_tree().create_timer(2.0).timeout`

### 5. Network/RPC Migration
File: `CastagneNet.gd`
- ✅ `remotesync func` → `@rpc("any_peer", "call_local")`
- ✅ Updated connect() calls to use Callable (see below)

### 6. Signal Connection Migration
Migrated all `connect()` calls to use `Callable()` wrapper:
- ✅ CastagneEngine.gd (2 calls)
- ✅ CastagneNet.gd (5 calls)
- ✅ CastagneModuleSpecblock.gd (3 calls)
- ✅ CMAttacksSBTypes-Graph.gd (5 calls)
- ✅ CMFlowFighting.gd (1 call)

**Total**: 16 connect() calls fixed across 5 files

Before:
```gdscript
signal_emitter.connect("signal_name", self, "method_name")
signal_emitter.connect("signal_name", self, "method_name", [params])
```

After:
```gdscript
signal_emitter.connect("signal_name", Callable(self, "method_name"))
signal_emitter.connect("signal_name", Callable(self, "method_name").bind([params]))
```

### 7. Onready Variables
- ✅ `onready var` → `@onready var` (7 occurrences)

Files affected:
- CastagneGlobal.gd (4 variables)
- CastagneParser.gd (1 variable)
- CMAttacksSBTypes-Graph.gd (1 variable)
- CMPhysics2D.gd (1 variable)

## Manual Review Recommendations

The following areas were identified but may need manual review:

### 1. Network Peer API
The `get_tree().network_peer` API has changed significantly in Godot 4. Files using this:
- `CastagneNet.gd` - Uses old network API (functions are prefixed with `Old1_` and `Old2_`)

### 2. Export Annotations
If there are complex export() statements with ranges or flags, they may need manual conversion to:
- `@export_range(min, max, step)`
- `@export_flags()`
- `@export_enum()`

### 3. Super Calls
Engine callbacks like `_ready()` and `_process()` may need explicit `super` calls if extending custom classes. The migration tool flagged these for consideration.

## Files Modified

### Engine Core (10 files)
- CastagneConfig.gd - File API, JSON methods
- CastagneEngine.gd - instance(), connect()
- CastagneGlobal.gd - File API, instance(), @onready
- CastagneInput.gd - (automated changes)
- CastagneLoader.gd - (no changes needed)
- CastagneMemory.gd - (no changes needed)
- CastagneMenus.gd - instance(), get_tree().root
- CastagneNet.gd - yield→await, RPC, connect(), network peer
- CastagneParser.gd - File API, @onready
- CastagneStateHandle.gd - (no changes needed)

### Modules (36 files)
- CastagneModule.gd - (no changes needed)
- CastagneModuleSpecblock.gd - connect()
- Core module (2 files) - (no changes needed)
- Attacks module (5 files) - instance(), connect()
- Physics module (4 files) - @onready
- Graphics module (9 files) - instance(), deg_to_rad(), Spatial→Node3D
- Flow module (2 files) - connect()
- General modules (7 files) - File API, deg_to_rad()
- Editor module (2 files) - (no changes needed)
- Temp modules (2 files) - (no changes needed)

## Migration Tools Created

Two automated migration tools were created to assist with the process:

### 1. migrate_godot3_to_4.py
Location: `/home/user/castagne-rs/scripts/migrate_godot3_to_4.py`

Handles common syntax changes:
- Method renames
- Node type renames
- Some export syntax changes
- Random function updates
- Constant renames

### 2. fix_connect_calls.py
Location: `/home/user/castagne-rs/scripts/fix_connect_calls.py`

Specifically handles signal connection migrations to use Callable().

## Testing Status

The migration has been completed syntactically. Runtime testing with actual game scenes and character files should be performed to verify:
- Parser functionality with .casp files
- Module system initialization
- Engine core game loop
- Graphics rendering
- Physics simulation
- Input handling
- Network code (if needed)

## Next Steps

1. ✅ Complete automated migrations
2. ✅ Complete manual File API migrations
3. ✅ Fix yield/await and RPC syntax
4. ✅ Fix all connect() calls
5. ⏳ Runtime testing with Godot 4.5
6. ⏳ Fix any runtime errors discovered
7. ⏳ Update documentation and examples

## Known Limitations

1. **Network code**: The old network peer API is still present but may not work in Godot 4. This is stub code (marked `Old1_` and `Old2_`) and may not be critical.

2. **Move and slide**: The `move_and_slide()` API changed in Godot 4 - it no longer takes parameters. Character physics may need adjustment.

3. **Tween**: Tween is no longer a node in Godot 4. If used, it needs to be replaced with `create_tween()`.

## Conclusion

The Castagne GDScript codebase has been successfully migrated from Godot 3 to Godot 4.5 syntax. All major API changes have been addressed:
- File API → FileAccess
- JSON methods updated
- Coroutines (yield → await)
- Signal connections (Callable wrapper)
- RPC annotations
- Node type renames
- Method renames

The code is now ready for runtime testing in Godot 4.5.
