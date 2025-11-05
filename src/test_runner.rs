// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Test Runner for comparing Rust and GDScript implementations

use godot::prelude::*;
use godot::classes::GDScript;
use crate::memory::CastagneMemory;

/// Test runner that compares Rust and GDScript implementations
#[derive(GodotClass)]
#[class(base=Node)]
pub struct CastagneTestRunner {
    base: Base<Node>,
}

#[godot_api]
impl INode for CastagneTestRunner {
    fn init(base: Base<Node>) -> Self {
        godot_print!("CastagneTestRunner initialized");
        Self { base }
    }
}

#[godot_api]
impl CastagneTestRunner {
    /// Run comparison tests between Rust and GDScript implementations
    #[func]
    pub fn run_comparison_tests(&mut self) -> Dictionary {
        godot_print!("=== Running Castagne Comparison Tests ===");
        let mut results = Dictionary::new();

        // Test memory operations
        results.set("memory_global", self.test_memory_global());
        results.set("memory_player", self.test_memory_player());
        results.set("memory_entity", self.test_memory_entity());

        // Print summary
        let passed = results.iter_shared()
            .filter(|(_, v)| v.try_to::<bool>().unwrap_or(false))
            .count();
        let total = results.len();

        godot_print!("=== Test Summary: {}/{} passed ===", passed, total);

        results
    }

    /// Test global memory operations
    fn test_memory_global(&self) -> bool {
        godot_print!("Testing global memory operations...");

        // Create GDScript version
        let mut gd_script = match try_load::<GDScript>("res://castagne/engine/CastagneMemory.gd") {
            Ok(script) => script,
            Err(_) => {
                godot_error!("Failed to load GDScript CastagneMemory.gd");
                return false;
            }
        };

        let gd_memory_variant = gd_script.instantiate(&[]);
        let mut gd_memory = gd_memory_variant.to::<Gd<Object>>();
        gd_memory.call("InitMemory", &[]);

        // Create Rust version
        let mut rust_memory = CastagneMemory::new();
        rust_memory.init_memory();

        // Test cases
        let test_cases = vec![
            ("TestInt", Variant::from(42)),
            ("TestNegative", Variant::from(-100)),
            ("TestString", Variant::from("Hello")),
        ];

        for (key, value) in test_cases {
            // Set in both
            gd_memory.call(
                "GlobalSet",
                &[Variant::from(key), value.clone(), Variant::from(true)]
            );
            rust_memory.global_set(key, value.clone(), true);

            // Get from both
            let gd_result = gd_memory.call("GlobalGet", &[Variant::from(key)]);
            let rust_result = match rust_memory.global_get(key) {
                Some(v) => v,
                None => {
                    godot_error!("Rust memory.global_get failed for key: {}", key);
                    return false;
                }
            };

            // Compare
            if gd_result != rust_result {
                godot_error!("Mismatch for {}: GD={:?}, Rust={:?}", key, gd_result, rust_result);
                return false;
            }

            // Test Has
            let gd_has = gd_memory.call("GlobalHas", &[Variant::from(key)])
                .try_to::<bool>().unwrap_or(false);
            let rust_has = rust_memory.global_has(key);

            if gd_has != rust_has {
                godot_error!("Has mismatch for {}: GD={}, Rust={}", key, gd_has, rust_has);
                return false;
            }
        }

        godot_print!("  ✅ Global memory test passed!");
        true
    }

    /// Test player memory operations
    fn test_memory_player(&self) -> bool {
        godot_print!("Testing player memory operations...");

        // Create GDScript version
        let mut gd_script = match try_load::<GDScript>("res://castagne/engine/CastagneMemory.gd") {
            Ok(script) => script,
            Err(_) => {
                godot_error!("Failed to load GDScript CastagneMemory.gd");
                return false;
            }
        };

        let gd_memory_variant = gd_script.instantiate(&[]);
        let mut gd_memory = gd_memory_variant.to::<Gd<Object>>();
        gd_memory.call("InitMemory", &[]);

        // Create Rust version
        let mut rust_memory = CastagneMemory::new();
        rust_memory.init_memory();

        // Add players to both
        gd_memory.call("AddPlayer", &[]);
        rust_memory.add_player();

        // Test operations
        let test_cases = vec![
            ("HP", Variant::from(100)),
            ("Meter", Variant::from(50)),
        ];

        for (key, value) in test_cases {
            // Set in both
            gd_memory.call(
                "PlayerSet",
                &[Variant::from(0), Variant::from(key), value.clone(), Variant::from(true)]
            );
            rust_memory.player_set(0, key, value.clone(), true);

            // Get from both
            let gd_result = gd_memory.call(
                "PlayerGet",
                &[Variant::from(0), Variant::from(key)]
            );
            let rust_result = match rust_memory.player_get(0, key) {
                Some(v) => v,
                None => {
                    godot_error!("Rust memory.player_get failed for key: {}", key);
                    return false;
                }
            };

            // Compare
            if gd_result != rust_result {
                godot_error!("Player mismatch for {}: GD={:?}, Rust={:?}", key, gd_result, rust_result);
                return false;
            }
        }

        godot_print!("  ✅ Player memory test passed!");
        true
    }

    /// Test entity memory operations
    fn test_memory_entity(&self) -> bool {
        godot_print!("Testing entity memory operations...");

        // Create GDScript version
        let mut gd_script = match try_load::<GDScript>("res://castagne/engine/CastagneMemory.gd") {
            Ok(script) => script,
            Err(_) => {
                godot_error!("Failed to load GDScript CastagneMemory.gd");
                return false;
            }
        };

        let gd_memory_variant = gd_script.instantiate(&[]);
        let mut gd_memory = gd_memory_variant.to::<Gd<Object>>();
        gd_memory.call("InitMemory", &[]);

        // Create Rust version
        let mut rust_memory = CastagneMemory::new();
        rust_memory.init_memory();

        // Add entities
        let gd_eid = gd_memory.call("AddEntity", &[])
            .try_to::<i32>().unwrap();
        let rust_eid = rust_memory.add_entity() as i32;

        // Should return same EID (both start at 0)
        if gd_eid != rust_eid {
            godot_error!("Entity ID mismatch: GD={}, Rust={}", gd_eid, rust_eid);
            return false;
        }

        // Test validity check
        let gd_valid = gd_memory.call("IsEIDValid", &[Variant::from(gd_eid)])
            .try_to::<bool>().unwrap_or(false);
        let rust_valid = rust_memory.is_eid_valid(rust_eid);

        if gd_valid != rust_valid {
            godot_error!("Entity validity mismatch: GD={}, Rust={}", gd_valid, rust_valid);
            return false;
        }

        // Test entity data
        gd_memory.call(
            "EntitySet",
            &[Variant::from(gd_eid), Variant::from("Position"), Variant::from(10), Variant::from(true)]
        );
        rust_memory.entity_set(rust_eid, "Position", Variant::from(10), true);

        let gd_pos = gd_memory.call(
            "EntityGet",
            &[Variant::from(gd_eid), Variant::from("Position")]
        );
        let rust_pos = rust_memory.entity_get(rust_eid, "Position").unwrap();

        if gd_pos != rust_pos {
            godot_error!("Entity data mismatch: GD={:?}, Rust={:?}", gd_pos, rust_pos);
            return false;
        }

        godot_print!("  ✅ Entity memory test passed!");
        true
    }

    /// Benchmark performance comparison
    #[func]
    pub fn benchmark_comparison(&mut self, iterations: i32) -> Dictionary {
        godot_print!("=== Running Performance Benchmark ({} iterations) ===", iterations);
        let mut results = Dictionary::new();

        // Benchmark Rust
        let rust_time = self.benchmark_rust(iterations as usize);

        // Benchmark GDScript
        let gd_time = self.benchmark_gdscript(iterations);

        results.set("rust_microseconds", rust_time);
        results.set("gdscript_microseconds", gd_time);
        results.set("speedup", gd_time as f64 / rust_time as f64);

        godot_print!("Rust: {}μs", rust_time);
        godot_print!("GDScript: {}μs", gd_time);
        godot_print!("Speedup: {:.2}x", gd_time as f64 / rust_time as f64);

        results
    }

    fn benchmark_rust(&self, iterations: usize) -> u64 {
        use std::time::Instant;

        let start = Instant::now();
        let mut memory = CastagneMemory::new();

        for i in 0..iterations {
            memory.global_set(&format!("Key{}", i), Variant::from(i as i32), true);
        }

        start.elapsed().as_micros() as u64
    }

    fn benchmark_gdscript(&self, iterations: i32) -> u64 {
        use std::time::Instant;

        let mut gd_script = match try_load::<GDScript>("res://castagne/engine/CastagneMemory.gd") {
            Ok(script) => script,
            Err(_) => {
                godot_error!("Failed to load GDScript for benchmark");
                return 0;
            }
        };

        let gd_memory_variant = gd_script.instantiate(&[]);
        let mut gd_memory = gd_memory_variant.to::<Gd<Object>>();
        gd_memory.call("InitMemory", &[]);

        let start = Instant::now();

        for i in 0..iterations {
            gd_memory.call(
                "GlobalSet",
                &[Variant::from(format!("Key{}", i)), Variant::from(i), Variant::from(true)]
            );
        }

        start.elapsed().as_micros() as u64
    }
}
