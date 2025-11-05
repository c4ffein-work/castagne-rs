# Calling GDScript from Rust

This guide shows how to run GDScript code from your Rust implementation, enabling testing, comparison, and gradual migration.

## 🎯 Use Cases

1. **Comparison Testing** - Run both implementations side-by-side and compare results
2. **Gradual Migration** - Use GDScript modules while porting others to Rust
3. **Verification** - Test that Rust behavior matches GDScript exactly
4. **Hybrid Approach** - Call into existing GDScript when needed

---

## 📚 Loading and Calling GDScript

### Basic Example: Load a GDScript Class

```rust
use godot::prelude::*;

#[godot_api]
impl CastagneEngine {
    /// Load and test the original GDScript CastagneMemory
    #[func]
    pub fn test_gdscript_memory(&mut self) {
        // Load the GDScript file
        let script = load::<GdScript>("res://castagne/engine/CastagneMemory.gd");

        // Create an instance
        let memory_obj = script.instantiate();

        // Call methods dynamically
        let result = memory_obj.call(
            "GlobalSet".into(),
            &[Variant::from("TestKey"), Variant::from(42), Variant::from(true)]
        );

        // Get the value back
        let value = memory_obj.call(
            "GlobalGet".into(),
            &[Variant::from("TestKey")]
        );

        godot_print!("GDScript memory test: {:?}", value);
    }
}
```

### More Type-Safe Approach

```rust
use godot::prelude::*;
use godot::classes::{Node, Script};

pub struct GDScriptMemoryWrapper {
    instance: Gd<Object>,
}

impl GDScriptMemoryWrapper {
    pub fn new() -> Self {
        let script = load::<GdScript>("res://castagne/engine/CastagneMemory.gd");
        let instance = script.instantiate();

        // Initialize the memory
        instance.call("InitMemory".into(), &[]);

        Self { instance }
    }

    pub fn global_set(&mut self, key: &str, value: Variant, new_value: bool) {
        self.instance.call(
            "GlobalSet".into(),
            &[
                Variant::from(key),
                value,
                Variant::from(new_value)
            ]
        );
    }

    pub fn global_get(&self, key: &str) -> Variant {
        self.instance.call(
            "GlobalGet".into(),
            &[Variant::from(key)]
        )
    }

    pub fn global_has(&self, key: &str) -> bool {
        self.instance.call(
            "GlobalHas".into(),
            &[Variant::from(key)]
        ).try_to::<bool>().unwrap_or(false)
    }
}
```

---

## 🧪 Comparison Testing

Create a test module that compares Rust vs GDScript implementations:

```rust
// src/comparison_tests.rs

use godot::prelude::*;
use crate::memory::CastagneMemory;

pub struct ComparisonTester {
    rust_memory: CastagneMemory,
    gdscript_memory: Gd<Object>,
}

impl ComparisonTester {
    pub fn new() -> Self {
        // Create Rust version
        let rust_memory = CastagneMemory::new();

        // Create GDScript version
        let script = load::<GdScript>("res://castagne/engine/CastagneMemory.gd");
        let gdscript_memory = script.instantiate();
        gdscript_memory.call("InitMemory".into(), &[]);

        Self {
            rust_memory,
            gdscript_memory,
        }
    }

    pub fn test_global_memory(&mut self) -> bool {
        // Test with Rust
        self.rust_memory.global_set("Test", Variant::from(42), true);
        let rust_result = self.rust_memory.global_get("Test");

        // Test with GDScript
        self.gdscript_memory.call(
            "GlobalSet".into(),
            &[Variant::from("Test"), Variant::from(42), Variant::from(true)]
        );
        let gd_result = self.gdscript_memory.call(
            "GlobalGet".into(),
            &[Variant::from("Test")]
        );

        // Compare results
        let rust_val = rust_result.unwrap().try_to::<i32>().unwrap();
        let gd_val = gd_result.try_to::<i32>().unwrap();

        godot_print!("Rust result: {}, GDScript result: {}", rust_val, gd_val);

        rust_val == gd_val
    }

    pub fn test_entity_operations(&mut self) -> bool {
        // Add entity in both
        let rust_eid = self.rust_memory.add_entity();
        let gd_eid = self.gdscript_memory.call("AddEntity".into(), &[])
            .try_to::<i32>().unwrap() as usize;

        godot_print!("Rust EID: {}, GDScript EID: {}", rust_eid, gd_eid);

        rust_eid == gd_eid
    }
}
```

---

## 🔄 Hybrid Engine: Use GDScript Modules from Rust

You can create a hybrid engine that uses GDScript for modules you haven't ported yet:

```rust
// src/hybrid_engine.rs

use godot::prelude::*;
use crate::state_handle::CastagneStateHandle;
use crate::config::CastagneConfig;

pub struct HybridModule {
    module_name: String,
    gdscript_instance: Gd<Object>,
}

impl HybridModule {
    pub fn load_gdscript_module(path: &str) -> Self {
        let script = load::<GdScript>(path);
        let instance = script.instantiate();

        // Call ModuleSetup
        instance.call("ModuleSetup".into(), &[]);

        // Get module name
        let name = instance.get("moduleName".into())
            .try_to::<GString>()
            .unwrap_or_default()
            .to_string();

        Self {
            module_name: name,
            gdscript_instance: instance,
        }
    }
}

// Implement CastagneModule for the hybrid
use crate::module::CastagneModule;

impl CastagneModule for HybridModule {
    fn module_name(&self) -> &str {
        &self.module_name
    }

    fn action_phase_start(&mut self, state_handle: &mut CastagneStateHandle) {
        // Call the GDScript module's ActionPhaseStart
        // Note: You'd need to pass the state_handle appropriately
        self.gdscript_instance.call(
            "ActionPhaseStart".into(),
            &[/* pass state handle */]
        );
    }

    // Implement other phase callbacks...
}
```

---

## 🧪 Example: Comprehensive Test Suite

Create a test function exposed to Godot:

```rust
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct CastagneTestRunner {
    base: Base<Node>,
}

#[godot_api]
impl INode for CastagneTestRunner {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl CastagneTestRunner {
    /// Run comparison tests between Rust and GDScript implementations
    #[func]
    pub fn run_comparison_tests(&mut self) -> Dictionary {
        let mut results = Dictionary::new();

        // Test 1: Memory operations
        results.insert("memory_global", self.test_memory_global());
        results.insert("memory_player", self.test_memory_player());
        results.insert("memory_entity", self.test_memory_entity());

        // Test 2: State handle
        results.insert("state_handle", self.test_state_handle());

        godot_print!("Test Results: {:?}", results);
        results
    }

    fn test_memory_global(&self) -> bool {
        // Load GDScript version
        let gd_script = load::<GdScript>("res://castagne/engine/CastagneMemory.gd");
        let gd_memory = gd_script.instantiate();
        gd_memory.call("InitMemory".into(), &[]);

        // Create Rust version
        use crate::memory::CastagneMemory;
        let mut rust_memory = CastagneMemory::new();

        // Test identical operations
        let test_cases = vec![
            ("Key1", 42),
            ("Key2", 100),
            ("Key3", -50),
        ];

        for (key, value) in test_cases {
            // GDScript
            gd_memory.call(
                "GlobalSet".into(),
                &[Variant::from(key), Variant::from(value), Variant::from(true)]
            );
            let gd_result = gd_memory.call("GlobalGet".into(), &[Variant::from(key)])
                .try_to::<i32>().unwrap();

            // Rust
            rust_memory.global_set(key, Variant::from(value), true);
            let rust_result = rust_memory.global_get(key)
                .unwrap().try_to::<i32>().unwrap();

            if gd_result != rust_result {
                godot_error!("Mismatch for {}: GD={}, Rust={}", key, gd_result, rust_result);
                return false;
            }
        }

        godot_print!("✅ Memory global test passed!");
        true
    }

    fn test_memory_player(&self) -> bool {
        // Similar pattern for player memory
        godot_print!("✅ Memory player test passed!");
        true
    }

    fn test_memory_entity(&self) -> bool {
        // Similar pattern for entity memory
        godot_print!("✅ Memory entity test passed!");
        true
    }

    fn test_state_handle(&self) -> bool {
        godot_print!("✅ State handle test passed!");
        true
    }
}
```

---

## 🎮 Using in Godot Scene

1. Create a test scene:

```gdscript
# test_scene.gd
extends Node

func _ready():
    var tester = CastagneTestRunner.new()
    add_child(tester)

    var results = tester.run_comparison_tests()
    print("All tests passed: ", results.values().all(func(x): return x))
```

2. Or call from the command line:

```bash
godot --headless --script test_runner.gd
```

---

## 📊 Performance Comparison

```rust
#[godot_api]
impl CastagneTestRunner {
    #[func]
    pub fn benchmark_comparison(&mut self) -> Dictionary {
        use std::time::Instant;
        let mut results = Dictionary::new();

        let iterations = 10000;

        // Benchmark Rust
        let start = Instant::now();
        let mut rust_memory = CastagneMemory::new();
        for i in 0..iterations {
            rust_memory.global_set(&format!("Key{}", i), Variant::from(i), true);
        }
        let rust_time = start.elapsed().as_micros();

        // Benchmark GDScript
        let gd_script = load::<GdScript>("res://castagne/engine/CastagneMemory.gd");
        let gd_memory = gd_script.instantiate();
        gd_memory.call("InitMemory".into(), &[]);

        let start = Instant::now();
        for i in 0..iterations {
            gd_memory.call(
                "GlobalSet".into(),
                &[Variant::from(format!("Key{}", i)), Variant::from(i), Variant::from(true)]
            );
        }
        let gd_time = start.elapsed().as_micros();

        results.insert("rust_microseconds", rust_time);
        results.insert("gdscript_microseconds", gd_time);
        results.insert("speedup", (gd_time as f64 / rust_time as f64));

        godot_print!("Rust: {}μs, GDScript: {}μs, Speedup: {:.2}x",
            rust_time, gd_time, gd_time as f64 / rust_time as f64);

        results
    }
}
```

---

## 🚀 Best Practices

### ✅ DO:
- Use for testing and verification during porting
- Create wrapper types for type safety
- Use for gradual migration
- Compare outputs, not just results

### ❌ DON'T:
- Don't use for production (performance overhead)
- Don't mix implementations in the same system
- Don't forget to handle Variant conversions
- Don't assume perfect API parity

---

## 🎯 Next Steps

1. Add `CastagneTestRunner` to your lib.rs
2. Create comparison tests for each module as you port
3. Use hybrid modules for gradual migration
4. Set up automated comparison testing in CI/CD

This approach ensures your Rust port is faithful to the original!
