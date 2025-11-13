// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Test Runner for Rust Parser Validation
//!
//! This test runner validates the Rust parser against golden master JSON files.
//! The engine logic is now in GDScript, so we only test the parser here.

use crate::parser::CastagneParser;
use godot::prelude::*;

/// Test runner for parser validation
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
    /// Run parser comparison tests against golden masters
    #[func]
    pub fn run_comparison_tests(&mut self) -> Dictionary {
        godot_print!("=== Running Castagne Parser Tests ===");
        let mut results = Dictionary::new();

        // Test parser operations against golden masters
        results.set("parser_basic_character", self.test_parser_basic_character());
        results.set(
            "parser_complete_character",
            self.test_parser_complete_character(),
        );
        results.set(
            "parser_advanced_character",
            self.test_parser_advanced_character(),
        );

        // Print summary
        let passed = results
            .iter_shared()
            .filter(|(_, v)| v.try_to::<bool>().unwrap_or(false))
            .count();
        let total = results.len();

        godot_print!("=== Test Summary: {}/{} passed ===", passed, total);

        results
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
        self.test_parser_with_golden_master(casp_file, "golden_masters/Baston-Model.json")
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
        self.test_parser_with_golden_master(casp_file, "golden_masters/Baston-2D.json")
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
        self.test_parser_with_golden_master(casp_file, "golden_masters/TutorialBaston.json")
    }

    /// Test parser with simple test file (good for initial testing)
    #[func]
    pub fn test_parser_simple(&mut self) -> bool {
        godot_print!("Testing parser comparison (test_character_complete)...");
        self.test_parser_with_golden_master(
            "test_character_complete.casp",
            "golden_masters/test_character_complete.json",
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
                godot_error!(
                    "❌ Failed to load golden master {}: {}",
                    golden_master_file,
                    e
                );
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

        godot_print!("  → Comparing outputs");

        // Compare the two JSON structures
        if self.compare_json_values(&golden_json, &rust_json, "") {
            godot_print!("  ✅ Parser test passed!");
            true
        } else {
            godot_error!("  ❌ Parser test failed - outputs differ");
            false
        }
    }

    /// Compare two JSON values recursively
    fn compare_json_values(
        &self,
        golden: &serde_json::Value,
        rust: &serde_json::Value,
        path: &str,
    ) -> bool {
        use serde_json::Value;

        match (golden, rust) {
            (Value::Object(g_map), Value::Object(r_map)) => {
                // Check all keys in golden exist in rust
                for (key, g_value) in g_map {
                    let new_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };

                    if let Some(r_value) = r_map.get(key) {
                        if !self.compare_json_values(g_value, r_value, &new_path) {
                            return false;
                        }
                    } else {
                        godot_error!("  Missing key in Rust output: {}", new_path);
                        return false;
                    }
                }
                true
            }
            (Value::Array(g_arr), Value::Array(r_arr)) => {
                if g_arr.len() != r_arr.len() {
                    godot_error!(
                        "  Array length mismatch at {}: {} vs {}",
                        path,
                        g_arr.len(),
                        r_arr.len()
                    );
                    return false;
                }
                for (i, (g_val, r_val)) in g_arr.iter().zip(r_arr.iter()).enumerate() {
                    let new_path = format!("{}[{}]", path, i);
                    if !self.compare_json_values(g_val, r_val, &new_path) {
                        return false;
                    }
                }
                true
            }
            (g, r) if g == r => true,
            (g, r) => {
                godot_error!("  Value mismatch at {}: {:?} vs {:?}", path, g, r);
                false
            }
        }
    }
}
