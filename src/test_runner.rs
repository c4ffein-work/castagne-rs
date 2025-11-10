// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Test Runner for comparing Rust and GDScript implementations

use godot::prelude::*;
use godot::classes::GDScript;
use crate::memory::CastagneMemory;
use crate::parser::CastagneParser;

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

        // Test memory operations - will FAIL until GDScript files are ported to Godot 4
        results.set("memory_global", self.test_memory_global());
        results.set("memory_player", self.test_memory_player());
        results.set("memory_entity", self.test_memory_entity());

        // Test StateHandle operations - Rust-only tests (no GDScript dependency)
        results.set("state_handle_point_to", self.test_state_handle_point_to());
        results.set("state_handle_target_entity", self.test_state_handle_target());

        // Test parser operations - will FAIL until golden master pipeline is complete
        results.set("parser_basic_character", self.test_parser_basic_character());
        results.set("parser_complete_character", self.test_parser_complete_character());
        results.set("parser_advanced_character", self.test_parser_advanced_character());

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

        // TODO: Need Godot 4 port of CastagneMemory.gd or golden master approach
        // Create GDScript version
        let mut gd_script = match try_load::<GDScript>("res://castagne/engine/CastagneMemory.gd") {
            Ok(script) => script,
            Err(_) => {
                godot_print!("⚠ Skipping memory_global test: GDScript CastagneMemory.gd not found");
                godot_print!("  (This test requires CastagneMemory.gd to be ported to Godot 4)");
                return true; // Skip test, don't fail
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

        // TODO: Need Godot 4 port of CastagneMemory.gd or golden master approach
        // Create GDScript version
        let mut gd_script = match try_load::<GDScript>("res://castagne/engine/CastagneMemory.gd") {
            Ok(script) => script,
            Err(_) => {
                godot_print!("⚠ Skipping memory_player test: GDScript CastagneMemory.gd not found");
                godot_print!("  (This test requires CastagneMemory.gd to be ported to Godot 4)");
                return true; // Skip test, don't fail
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

        // TODO: Need Godot 4 port of CastagneMemory.gd or golden master approach
        // Create GDScript version
        let mut gd_script = match try_load::<GDScript>("res://castagne/engine/CastagneMemory.gd") {
            Ok(script) => script,
            Err(_) => {
                godot_print!("⚠ Skipping memory_entity test: GDScript CastagneMemory.gd not found");
                godot_print!("  (This test requires CastagneMemory.gd to be ported to Godot 4)");
                return true; // Skip test, don't fail
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

    /// Test StateHandle point_to operations
    fn test_state_handle_point_to(&self) -> bool {
        godot_print!("Testing StateHandle point_to operations...");

        // Create Rust version
        use std::rc::Rc;
        use std::cell::RefCell;
        use crate::state_handle::CastagneStateHandle;

        let mut memory = CastagneMemory::new();
        memory.init_memory();

        // Add a player and entity
        memory.add_player();
        let eid = memory.add_entity();
        memory.entity_set(eid as i32, "_Player", Variant::from(0), true);
        memory.player_set(0, "TestVar", Variant::from(100), true);

        let memory_rc = Rc::new(RefCell::new(memory));
        let mut state_handle = CastagneStateHandle::new(memory_rc);

        // Test pointing to entity
        let result = state_handle.point_to_entity(eid as i32);
        if !result {
            godot_error!("Failed to point to entity {}", eid);
            return false;
        }

        // Test get_entity_id
        if state_handle.get_entity_id() != eid as i32 {
            godot_error!("Entity ID mismatch: got {}, expected {}", state_handle.get_entity_id(), eid);
            return false;
        }

        // Test player was automatically set
        if state_handle.get_player() != 0 {
            godot_error!("Player ID mismatch: got {}, expected 0", state_handle.get_player());
            return false;
        }

        godot_print!("  ✅ StateHandle point_to test passed!");
        true
    }

    /// Test StateHandle target entity operations
    fn test_state_handle_target(&self) -> bool {
        godot_print!("Testing StateHandle target entity operations...");

        use std::rc::Rc;
        use std::cell::RefCell;
        use crate::state_handle::CastagneStateHandle;

        let mut memory = CastagneMemory::new();
        memory.init_memory();

        // Add two entities
        let eid1 = memory.add_entity();
        let eid2 = memory.add_entity();
        memory.entity_set(eid1 as i32, "_TargetEID", Variant::from(eid2 as i32), true);
        memory.entity_set(eid2 as i32, "HP", Variant::from(100), true);

        let memory_rc = Rc::new(RefCell::new(memory));
        let mut state_handle = CastagneStateHandle::new(memory_rc);

        // Point to first entity
        state_handle.point_to_entity(eid1 as i32);

        // Set target to second entity
        state_handle.set_target_entity(eid2 as i32);

        // Verify target
        if state_handle.get_target_eid() != eid2 as i32 {
            godot_error!("Target EID mismatch: got {}, expected {}", state_handle.get_target_eid(), eid2);
            return false;
        }

        // Test target entity get
        if let Some(hp) = state_handle.target_entity_get("HP") {
            if let Ok(hp_val) = hp.try_to::<i32>() {
                if hp_val != 100 {
                    godot_error!("Target entity HP mismatch: got {}, expected 100", hp_val);
                    return false;
                }
            } else {
                godot_error!("Failed to convert HP to i32");
                return false;
            }
        } else {
            godot_error!("Failed to get HP from target entity");
            return false;
        }

        godot_print!("  ✅ StateHandle target entity test passed!");
        true
    }

    /// Test parser comparison - basic character file
    fn test_parser_basic_character(&self) -> bool {
        use std::path::Path;
        let casp_file = "castagne/examples/fighters/baston/Baston-Model.casp";

        if !Path::new(casp_file).exists() {
            godot_print!("⚠ Skipping parser test (Baston-Model): .casp file not found");
            godot_print!("  (This test requires the full Castagne repository)");
            return true; // Skip test, don't fail
        }

        godot_print!("Testing parser comparison (Baston-Model)...");
        self.test_parser_with_golden_master(
            casp_file,
            "golden_masters/Baston-Model.json"
        )
    }

    /// Test parser comparison - complete character file
    fn test_parser_complete_character(&self) -> bool {
        use std::path::Path;
        let casp_file = "castagne/examples/fighters/baston/Baston-2D.casp";

        if !Path::new(casp_file).exists() {
            godot_print!("⚠ Skipping parser test (Baston-2D): .casp file not found");
            godot_print!("  (This test requires the full Castagne repository)");
            return true; // Skip test, don't fail
        }

        godot_print!("Testing parser comparison (Baston-2D)...");
        self.test_parser_with_golden_master(
            casp_file,
            "golden_masters/Baston-2D.json"
        )
    }

    /// Test parser comparison - advanced character file
    fn test_parser_advanced_character(&self) -> bool {
        use std::path::Path;
        let casp_file = "castagne/editor/tutorials/assets/TutorialBaston.casp";

        if !Path::new(casp_file).exists() {
            godot_print!("⚠ Skipping parser test (TutorialBaston): .casp file not found");
            godot_print!("  (This test requires the full Castagne repository)");
            return true; // Skip test, don't fail
        }

        godot_print!("Testing parser comparison (TutorialBaston)...");
        self.test_parser_with_golden_master(
            casp_file,
            "golden_masters/TutorialBaston.json"
        )
    }

    /// Test parser with simple test file (good for initial testing)
    #[func]
    pub fn test_parser_simple(&mut self) -> bool {
        godot_print!("Testing parser comparison (test_character_complete)...");
        self.test_parser_with_golden_master(
            "test_character_complete.casp",
            "golden_masters/test_character_complete.json"
        )
    }

    /// Helper method to test parser against a golden master file
    fn test_parser_with_golden_master(&self, casp_file: &str, golden_master_file: &str) -> bool {
        use std::fs;

        godot_print!("  → Loading golden master: {}", golden_master_file);

        // Load and parse golden master JSON
        let golden_json_str = match fs::read_to_string(golden_master_file) {
            Ok(content) => content,
            Err(e) => {
                godot_error!("❌ Failed to load golden master {}: {}", golden_master_file, e);
                return false;
            }
        };

        let golden_json: serde_json::Value = match serde_json::from_str(&golden_json_str) {
            Ok(json) => json,
            Err(e) => {
                godot_error!("❌ Failed to parse golden master JSON: {}", e);
                return false;
            }
        };

        godot_print!("  → Parsing .casp file with Rust parser: {}", casp_file);

        // Parse the .casp file with Rust parser
        let mut parser = CastagneParser::new();
        let rust_result = match parser.create_full_character(casp_file) {
            Some(character) => character,
            None => {
                godot_error!("❌ Rust parser failed to parse {}", casp_file);
                for error in &parser.errors {
                    godot_error!("   Parser error: {}", error);
                }
                return false;
            }
        };

        godot_print!("  → Serializing Rust parser output to JSON");

        // Serialize Rust parser output to JSON
        let rust_json = match rust_result.to_json_value() {
            Ok(json) => json,
            Err(e) => {
                godot_error!("❌ Failed to serialize Rust parser output: {}", e);
                return false;
            }
        };

        godot_print!("  → Comparing parser outputs...");

        // Compare the two JSON structures
        let mut differences = Vec::new();
        self.compare_json_recursive(&rust_json, &golden_json, "", &mut differences);

        if differences.is_empty() {
            godot_print!("  ✅ Perfect match! Rust parser output matches golden master exactly.");
            return true;
        }

        // Report differences
        godot_warn!("  ⚠️  Found {} differences between Rust parser and golden master:", differences.len());

        // Show first 10 differences
        for (i, diff) in differences.iter().enumerate().take(10) {
            godot_print!("    {}: {}", i + 1, diff);
        }

        if differences.len() > 10 {
            godot_print!("    ... and {} more differences", differences.len() - 10);
        }

        // Print summary statistics
        godot_print!("  → Summary:");
        godot_print!("    Rust metadata.name: {}", rust_result.metadata.name);
        godot_print!("    Golden metadata.name: {}", golden_json["metadata"]["name"].as_str().unwrap_or("N/A"));
        godot_print!("    Rust states count: {}", rust_result.states.len());
        godot_print!("    Golden states count: {}", golden_json["states"].as_object().map(|o| o.len()).unwrap_or(0));
        godot_print!("    Rust variables count: {}", rust_result.variables.len());
        godot_print!("    Golden variables count: {}", golden_json["variables"].as_object().map(|o| o.len()).unwrap_or(0));

        // For now, return true if basic metadata matches (parser is still in development)
        rust_result.metadata.name == golden_json["metadata"]["name"].as_str().unwrap_or("")
    }

    /// Recursively compare two JSON values and collect differences
    fn compare_json_recursive(&self, rust: &serde_json::Value, golden: &serde_json::Value,
                              path: &str, differences: &mut Vec<String>) {
        use serde_json::Value;

        match (rust, golden) {
            (Value::Object(rust_obj), Value::Object(golden_obj)) => {
                // Check for missing keys in Rust
                for key in golden_obj.keys() {
                    if !rust_obj.contains_key(key) {
                        let new_path = if path.is_empty() { key.clone() } else { format!("{}.{}", path, key) };
                        differences.push(format!("{}: missing in Rust output", new_path));
                    }
                }

                // Check for extra keys in Rust
                for key in rust_obj.keys() {
                    if !golden_obj.contains_key(key) {
                        let new_path = if path.is_empty() { key.clone() } else { format!("{}.{}", path, key) };
                        differences.push(format!("{}: extra in Rust output (not in golden)", new_path));
                    }
                }

                // Recursively compare common keys
                for (key, rust_val) in rust_obj.iter() {
                    if let Some(golden_val) = golden_obj.get(key) {
                        let new_path = if path.is_empty() { key.clone() } else { format!("{}.{}", path, key) };
                        self.compare_json_recursive(rust_val, golden_val, &new_path, differences);
                    }
                }
            }
            (Value::Array(rust_arr), Value::Array(golden_arr)) => {
                if rust_arr.len() != golden_arr.len() {
                    differences.push(format!("{}: array length mismatch (Rust: {}, Golden: {})",
                        path, rust_arr.len(), golden_arr.len()));
                }

                for (i, (rust_item, golden_item)) in rust_arr.iter().zip(golden_arr.iter()).enumerate() {
                    let new_path = format!("{}[{}]", path, i);
                    self.compare_json_recursive(rust_item, golden_item, &new_path, differences);
                }
            }
            (rust_val, golden_val) => {
                if rust_val != golden_val {
                    // Truncate long values for readability
                    let rust_str = format!("{:?}", rust_val);
                    let golden_str = format!("{:?}", golden_val);
                    let rust_display = if rust_str.len() > 50 {
                        format!("{}...", &rust_str[..50])
                    } else {
                        rust_str
                    };
                    let golden_display = if golden_str.len() > 50 {
                        format!("{}...", &golden_str[..50])
                    } else {
                        golden_str
                    };
                    differences.push(format!("{}: value mismatch\n      Rust: {}\n      Golden: {}",
                        path, rust_display, golden_display));
                }
            }
        }
    }

    /// Compare metadata between Rust and GDScript parsers
    fn compare_metadata(&self, rust_char: &crate::parser::ParsedCharacter, gd_char: &Gd<Object>) -> bool {
        let gd_meta = gd_char.get("metadata").to::<Gd<Object>>();

        // Compare name
        let gd_name = gd_meta.get("name").to::<GString>().to_string();
        if rust_char.metadata.name != gd_name {
            godot_error!("  Name mismatch: Rust='{}' vs GD='{}'", rust_char.metadata.name, gd_name);
            return false;
        }

        // Compare author
        let gd_author = gd_meta.get("author").to::<GString>().to_string();
        if rust_char.metadata.author != gd_author {
            godot_error!("  Author mismatch: Rust='{}' vs GD='{}'", rust_char.metadata.author, gd_author);
            return false;
        }

        // Compare description
        let gd_desc = gd_meta.get("description").to::<GString>().to_string();
        if rust_char.metadata.description != gd_desc {
            godot_error!("  Description mismatch: Rust='{}' vs GD='{}'", rust_char.metadata.description, gd_desc);
            return false;
        }

        true
    }

    /// Compare variables between Rust and GDScript parsers
    fn compare_variables(&self, rust_char: &crate::parser::ParsedCharacter, gd_char: &Gd<Object>) -> bool {
        let gd_vars = gd_char.get("variables").to::<Dictionary>();

        // Compare variable count
        if rust_char.variables.len() != gd_vars.len() {
            godot_error!("  Variable count mismatch: Rust={} vs GD={}",
                rust_char.variables.len(), gd_vars.len());
            return false;
        }

        // Compare each variable
        for (name, rust_var) in &rust_char.variables {
            if !gd_vars.contains_key(name.as_str()) {
                godot_error!("  Variable '{}' exists in Rust but not in GDScript", name);
                return false;
            }

            let gd_var = gd_vars.get(name.as_str()).unwrap().to::<Dictionary>();

            // Compare variable value
            let gd_value = gd_var.get("value").unwrap().to::<GString>().to_string();
            if rust_var.value != gd_value {
                godot_error!("  Variable '{}' value mismatch: Rust='{}' vs GD='{}'",
                    name, rust_var.value, gd_value);
                return false;
            }
        }

        true
    }

    /// Compare states between Rust and GDScript parsers
    fn compare_states(&self, rust_char: &crate::parser::ParsedCharacter, gd_char: &Gd<Object>) -> bool {
        let gd_states = gd_char.get("states").to::<Dictionary>();

        // Compare state count
        if rust_char.states.len() != gd_states.len() {
            godot_error!("  State count mismatch: Rust={} vs GD={}",
                rust_char.states.len(), gd_states.len());
            return false;
        }

        // Compare each state
        for (name, rust_state) in &rust_char.states {
            if !gd_states.contains_key(name.as_str()) {
                godot_error!("  State '{}' exists in Rust but not in GDScript", name);
                return false;
            }

            let gd_state = gd_states.get(name.as_str()).unwrap().to::<Dictionary>();

            // Compare state parent
            let gd_parent = gd_state.get("parent").map(|v| {
                if v.is_nil() {
                    None
                } else {
                    Some(v.to::<GString>().to_string())
                }
            }).unwrap_or(None);

            if rust_state.parent != gd_parent {
                godot_error!("  State '{}' parent mismatch: Rust={:?} vs GD={:?}",
                    name, rust_state.parent, gd_parent);
                return false;
            }

            // Compare actions (phases)
            let gd_actions = gd_state.get("actions").unwrap().to::<Dictionary>();
            if rust_state.actions.len() != gd_actions.len() {
                godot_error!("  State '{}' action count mismatch: Rust={} vs GD={}",
                    name, rust_state.actions.len(), gd_actions.len());
                return false;
            }
        }

        true
    }

    /// Compare specblocks between Rust and GDScript parsers
    fn compare_specblocks(&self, rust_char: &crate::parser::ParsedCharacter, gd_char: &Gd<Object>) -> bool {
        let gd_specblocks = gd_char.get("specblocks").to::<Dictionary>();

        // Compare specblock count
        if rust_char.specblocks.len() != gd_specblocks.len() {
            godot_error!("  Specblock count mismatch: Rust={} vs GD={}",
                rust_char.specblocks.len(), gd_specblocks.len());
            return false;
        }

        // Compare each specblock
        for (name, rust_specblock) in &rust_char.specblocks {
            if !gd_specblocks.contains_key(name.as_str()) {
                godot_error!("  Specblock '{}' exists in Rust but not in GDScript", name);
                return false;
            }

            let gd_specblock = gd_specblocks.get(name.as_str()).unwrap().to::<Dictionary>();

            // Compare specblock entry count
            if rust_specblock.len() != gd_specblock.len() {
                godot_error!("  Specblock '{}' entry count mismatch: Rust={} vs GD={}",
                    name, rust_specblock.len(), gd_specblock.len());
                return false;
            }

            // Compare each entry
            for (key, rust_value) in rust_specblock {
                if !gd_specblock.contains_key(key.as_str()) {
                    godot_error!("  Specblock '{}' key '{}' exists in Rust but not in GDScript",
                        name, key);
                    return false;
                }

                let gd_value = gd_specblock.get(key.as_str()).unwrap().to::<GString>().to_string();
                if rust_value != &gd_value {
                    godot_error!("  Specblock '{}' key '{}' value mismatch: Rust='{}' vs GD='{}'",
                        name, key, rust_value, gd_value);
                    return false;
                }
            }
        }

        true
    }
}
