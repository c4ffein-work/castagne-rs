# Godot 4 Porting Status for Castagne

## ✅ Fixed Issues

### Core Engine
- **CastagneGlobal.gd**: Created missing Castagne.tscn scene file (ported from Godot 3)
  - Parser, Net, Loader, Menus nodes now properly initialized as scene children
  - Updated project.godot autoload to use .tscn instead of .gd

- **String API**: Fixed 50+ occurrences across codebase
  - `.empty()` → `.is_empty()`

- **String search**: Fixed 3 occurrences
  - `.find_last()` → `.rfind()`

- **CastagneNet.gd**: Fixed multiplayer signals (line 93-99)
  - `get_tree().connect("network_peer_connected")` → `get_tree().get_multiplayer().connect("peer_connected")`
  - `get_tree().connect("network_peer_disconnected")` → `get_tree().get_multiplayer().connect("peer_disconnected")`
  - `get_tree().connect("server_disconnected")` → `get_tree().get_multiplayer().connect("server_disconnected")`

- **Parent method calls**: Fixed across multiple files
  - `. Method()` → `super.Method()` (e.g., CMFlowFighting.gd:22, 171)

- **CMFlowFighting.gd**: Fixed syntax error (line 151)
  - Missing closing parenthesis in `connect()` call

## ⚠️ Remaining Issues

### CMGraphics2D.gd - BLOCKING
Multiple parse errors preventing module load:

1. **Line 37**: `CreateModel()` function not found in base CMGraphicsBase.gd
   - Need to check if function was renamed or moved

2. **Line 54**: Cannot call `new()` on Callable
   - Likely FuncRef → Callable migration issue
   - Old: `FuncRef.new()` → New: Need to use `Callable(object, method)`

3. **Line 105**: `Viewport.new()` - Cannot construct abstract Viewport class
   - Godot 4 change: Use `SubViewport.new()` instead

4. **Lines 115, 116, 120**: Viewport enum members removed
   - `Viewport.USAGE_2D` → Removed (use SubViewport properties instead)
   - `Viewport.UPDATE_ALWAYS` → Removed (use `render_target_update_mode`)
   - `Viewport.USAGE_2D_NO_SAMPLING` → Removed

### CMGraphicsBase.gd
- Missing `CreateModel()` function referenced by CMGraphics2D
  - Check if this exists and is properly defined

### CMGraphicsSBGraphics.gd
- **Line 45**: Parse error - "Expected expression for variable initial value after '='"
  - Likely related to FuncRef → Callable migration

### CMAttacks.gd
- Missing file reference: `res://castagne/modules/attacks/CMAttacksSBTypes-Graph.gd`
  - Path should be `res://castagne_godot4/modules/attacks/...`
  - File may be missing or incorrectly referenced in CMAttacks-TypesBigWindow.tscn

### CMGraphics2HalfD.gd
- Cannot resolve super class inheritance from CMGraphics3D.gd
  - CMGraphics3D.gd likely has parse errors preventing loading

### Unknown modules
- Various modules report missing functions (AIInputTransition, etc.)
- These may be cascade failures from earlier module load failures

## 🔧 Required Fixes

### High Priority (Blocking)

1. **Fix CMGraphics2D.gd Viewport usage**
   ```gdscript
   # Old (Godot 3):
   var viewport = Viewport.new()
   viewport.usage = Viewport.USAGE_2D
   viewport.render_target_update_mode = Viewport.UPDATE_ALWAYS

   # New (Godot 4):
   var viewport = SubViewport.new()
   viewport.render_target_update_mode = SubViewport.UPDATE_ALWAYS
   # USAGE_2D is no longer needed
   ```

2. **Fix FuncRef → Callable migration**
   ```gdscript
   # Old (Godot 3):
   var funcref = FuncRef.new()
   funcref.set_instance(obj)
   funcref.set_function("method_name")

   # New (Godot 4):
   var callable = Callable(obj, "method_name")
   ```

3. **Fix CreateModel() in CMGraphicsBase.gd**
   - Verify function exists and signature
   - May need to be renamed or signature updated

4. **Fix file path references**
   - `res://castagne/` → `res://castagne_godot4/`
   - Check CMAttacks-TypesBigWindow.tscn

### Medium Priority

5. **Test CMGraphics3D.gd**
   - May have similar Viewport issues as CMGraphics2D

6. **Review all FuncRef usage**
   - Search for `FuncRef` and `funcref` across codebase
   - Convert to Callable

7. **Check remaining module compatibility**
   - Once graphics modules load, test other modules

## 📊 Current Test Status

### e2e_godot_tests.rs
- **e2e_godot_available**: ✅ PASS (Godot 4.5 detected)
- **e2e_test_infrastructure_exists**: ✅ PASS (test_scenes/ exists)
- **All other tests**: ⚠️ SKIP (Castagne fails to initialize)

### Root Cause
Castagne autoload initializes but crashes during module loading at CMGraphics2D, preventing all e2e tests from running.

## 🎯 Next Steps

1. Fix CMGraphics2D.gd Viewport API usage
2. Fix FuncRef → Callable in CMGraphicsSBGraphics.gd
3. Locate/fix CreateModel() issue
4. Fix file path references in .tscn files
5. Test module loading again
6. Fix any new errors that appear
7. Run e2e tests

## 📝 Notes

- The API changes from `.empty()` and `.find_last()` are widespread but mechanically fixable
- The Viewport changes are more complex and require understanding the render pipeline
- FuncRef → Callable is a significant API change that may appear in many places
- Once graphics modules load, expect more compatibility issues in game logic code
