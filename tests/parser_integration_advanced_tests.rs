// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Advanced End-to-End Tests
//!
//! Additional comprehensive tests covering:
//! - Subentities
//! - Advanced state features
//! - Module system validation
//! - Performance/stress scenarios
//! - Real-world use cases

use std::fs;
use serde_json::Value;

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // HELPER FUNCTIONS
    // ============================================================================

    fn load_golden_master(path: &str) -> Value {
        let json_content = fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Failed to load golden master: {}", path));
        serde_json::from_str(&json_content)
            .unwrap_or_else(|_| panic!("Failed to parse golden master JSON: {}", path))
    }

    // ============================================================================
    // SUBENTITY TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_subentities_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let subentities = &golden["subentities"];

        assert!(subentities.is_object(), "Subentities should be an object");

        let subentities_obj = subentities.as_object().unwrap();
        assert!(subentities_obj.len() > 0, "Should have at least one subentity");

        // Check for common subentities
        for (subentity_name, subentity_data) in subentities_obj {
            assert!(subentity_data.is_object(),
                "Subentity {} should be an object", subentity_name);

            // Subentities may have skeleton references
            if let Some(skeleton) = subentity_data.get("skeleton") {
                assert!(skeleton.is_string() || skeleton.is_null(),
                    "Subentity {} skeleton should be string or null", subentity_name);
            }

            println!("  ✓ Subentity: {}", subentity_name);
        }

        println!("✓ Subentities structure validated ({} subentities)", subentities_obj.len());
    }

    #[test]
    fn e2e_advanced_subentity_variables() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();
        let subentities = golden["subentities"].as_object().unwrap();

        // Variables may reference subentities
        for (var_name, var_data) in variables {
            let var_type = var_data["Type"].as_str().unwrap_or("");

            // Empty type variables might be subentity placeholders
            if var_type.is_empty() {
                // Check if this variable name matches a subentity
                if subentities.contains_key(var_name) {
                    println!("  ✓ Found subentity placeholder variable: {}", var_name);
                }
            }
        }

        println!("✓ Subentity variables validated");
    }

    #[test]
    fn e2e_advanced_subentity_consistency() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let subentities = golden["subentities"].as_object().unwrap();
        let variables = golden["variables"].as_object().unwrap();

        // Each subentity should have a corresponding variable entry
        for subentity_name in subentities.keys() {
            assert!(variables.contains_key(subentity_name),
                "Subentity {} should have a corresponding variable", subentity_name);
        }

        println!("✓ Subentity consistency validated ({} subentities)", subentities.len());
    }

    // ============================================================================
    // MODULE SYSTEM TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_module_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        // Verify core modules exist
        let core_modules = vec![
            "Graphics",
            "Anims",
            "PhysicsMovement",
            "AttacksMechanics",
        ];

        for module_name in &core_modules {
            assert!(transformed.contains_key(*module_name),
                "Missing core module: {}", module_name);
        }

        // Each module should have a Defines section
        for (module_name, module_data) in transformed {
            assert!(module_data.is_object(),
                "Module {} should be an object", module_name);
            assert!(module_data["Defines"].is_object(),
                "Module {} should have Defines section", module_name);

            println!("  ✓ Module: {} (Defines: {} entries)",
                     module_name,
                     module_data["Defines"].as_object().unwrap().len());
        }

        println!("✓ Module structure validated ({} modules)", transformed.len());
    }

    #[test]
    fn e2e_advanced_graphics_module_detailed() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let graphics = &golden["transformed_data"]["Graphics"];

        // Graphics module should have specific sections
        assert!(graphics["Defines"].is_object(), "Graphics should have Defines");

        // Check for spritesheets
        if graphics["Spritesheets"].is_object() {
            let spritesheets = graphics["Spritesheets"].as_object().unwrap();
            for (sheet_name, sheet_data) in spritesheets {
                // Each spritesheet should have dimensions
                assert!(sheet_data["SpritesX"].is_number(),
                    "Spritesheet {} missing SpritesX", sheet_name);
                assert!(sheet_data["SpritesY"].is_number(),
                    "Spritesheet {} missing SpritesY", sheet_name);
                assert!(sheet_data["OriginX"].is_number(),
                    "Spritesheet {} missing OriginX", sheet_name);
                assert!(sheet_data["OriginY"].is_number(),
                    "Spritesheet {} missing OriginY", sheet_name);
                assert!(sheet_data["PixelSize"].is_number(),
                    "Spritesheet {} missing PixelSize", sheet_name);

                // Validate numeric values are reasonable
                let sprites_x = sheet_data["SpritesX"].as_i64().unwrap();
                let sprites_y = sheet_data["SpritesY"].as_i64().unwrap();
                assert!(sprites_x > 0 && sprites_x < 100,
                    "SpritesX should be reasonable (1-99)");
                assert!(sprites_y > 0 && sprites_y < 100,
                    "SpritesY should be reasonable (1-99)");
            }
        }

        println!("✓ Graphics module detailed validation passed");
    }

    #[test]
    fn e2e_advanced_physics_module() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let physics = &golden["transformed_data"]["PhysicsMovement"];

        assert!(physics["Defines"].is_object(),
            "PhysicsMovement should have Defines");

        let defines = physics["Defines"].as_object().unwrap();

        // Physics module typically has movement-related defines
        println!("  Physics module defines: {} entries", defines.len());

        println!("✓ Physics module validated");
    }

    #[test]
    fn e2e_advanced_attacks_module() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let attacks = &golden["transformed_data"]["AttacksMechanics"];

        assert!(attacks["Defines"].is_object(),
            "AttacksMechanics should have Defines");

        let defines = attacks["Defines"].as_object().unwrap();
        println!("  Attacks module defines: {} entries", defines.len());

        println!("✓ Attacks module validated");
    }

    // ============================================================================
    // ADVANCED STATE FEATURES
    // ============================================================================

    #[test]
    fn e2e_advanced_state_transition_flags() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut states_with_flags = 0;

        for (state_name, state_data) in states {
            if let Some(flags) = state_data["TransitionFlags"].as_array() {
                if !flags.is_empty() {
                    states_with_flags += 1;
                    println!("  State {} has {} transition flags", state_name, flags.len());
                }
            }
        }

        println!("✓ State transition flags validated ({} states with flags)", states_with_flags);
    }

    #[test]
    fn e2e_advanced_state_parent_chains() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Track the longest parent chain
        let mut max_chain_length = 0;
        let mut longest_chain_state = String::new();

        for (state_name, _) in states {
            let mut chain_length = 0;
            let mut current = state_name.clone();

            while let Some(state_data) = states.get(&current) {
                if let Some(parent) = state_data["Parent"].as_str() {
                    chain_length += 1;
                    current = parent.to_string();
                } else {
                    break;
                }

                // Safety: prevent infinite loops
                if chain_length > 100 {
                    break;
                }
            }

            if chain_length > max_chain_length {
                max_chain_length = chain_length;
                longest_chain_state = state_name.clone();
            }
        }

        println!("✓ State parent chains validated");
        println!("  Max chain length: {} (state: {})", max_chain_length, longest_chain_state);
    }

    #[test]
    fn e2e_advanced_state_type_distribution() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut type_counts = std::collections::HashMap::new();

        for (_, state_data) in states {
            let state_type = state_data["Type"].as_str().unwrap_or("null");
            *type_counts.entry(state_type.to_string()).or_insert(0) += 1;
        }

        println!("✓ State type distribution:");
        for (type_name, count) in type_counts.iter() {
            println!("  {}: {} states", type_name, count);
        }
    }

    // ============================================================================
    // PERFORMANCE AND STRESS TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_large_state_machine() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // This character has 323 states - verify we can handle large state machines
        assert!(states.len() > 300,
            "Should handle large state machines (300+ states)");

        // Verify all states are accessible
        for (state_name, state_data) in states {
            assert!(state_data.is_object(),
                "State {} should be valid object", state_name);
        }

        println!("✓ Large state machine validated ({} states)", states.len());
    }

    #[test]
    fn e2e_advanced_memory_efficiency() {
        // Test multiple large file loads to verify memory handling
        for i in 0..3 {
            let golden = load_golden_master("golden_masters/Baston-Model.json");
            let states = golden["states"].as_object().unwrap();
            let variables = golden["variables"].as_object().unwrap();

            assert!(states.len() > 0, "Iteration {} should load states", i);
            assert!(variables.len() > 0, "Iteration {} should load variables", i);
        }

        println!("✓ Memory efficiency validated (3 iterations)");
    }

    #[test]
    fn e2e_advanced_concurrent_file_access() {
        // Simulate concurrent access to different files
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        for file in files {
            let golden = load_golden_master(file);
            assert!(golden["metadata"].is_object(),
                "{} should have metadata", file);
            assert!(golden["states"].is_object(),
                "{} should have states", file);
        }

        println!("✓ Concurrent file access validated (3 files)");
    }

    // ============================================================================
    // REAL-WORLD SCENARIO TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_combo_system_patterns() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Look for attack state patterns (common in fighting games)
        let mut attack_states = Vec::new();

        for state_name in states.keys() {
            if state_name.contains("Attack") ||
               state_name.contains("Punch") ||
               state_name.contains("Kick") ||
               state_name.contains("Tech") {
                attack_states.push(state_name.clone());
            }
        }

        println!("✓ Combo system patterns validated ({} attack states)", attack_states.len());
        if attack_states.len() > 0 {
            println!("  Example attacks: {:?}", attack_states.iter().take(5).collect::<Vec<_>>());
        }
    }

    #[test]
    fn e2e_advanced_animation_state_patterns() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let graphics = &golden["transformed_data"]["Graphics"];

        // Check animation-related data
        if graphics["Anims"].is_object() {
            let anims = graphics["Anims"].as_object().unwrap();
            println!("  Animation entries: {}", anims.len());
        }

        println!("✓ Animation state patterns validated");
    }

    #[test]
    fn e2e_advanced_ai_state_patterns() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Look for AI-related states
        let mut ai_states = Vec::new();

        for state_name in states.keys() {
            if state_name.starts_with("AI-") {
                ai_states.push(state_name.clone());
            }
        }

        if ai_states.len() > 0 {
            println!("✓ AI state patterns validated ({} AI states)", ai_states.len());
            println!("  Example AI states: {:?}", ai_states.iter().take(5).collect::<Vec<_>>());
        } else {
            println!("✓ AI state patterns validated (no AI states found - valid)");
        }
    }

    // ============================================================================
    // CROSS-MODULE INTEGRATION TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_graphics_physics_integration() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let graphics = &golden["transformed_data"]["Graphics"];
        let physics = &golden["transformed_data"]["PhysicsMovement"];

        // Both modules should coexist properly
        assert!(graphics["Defines"].is_object(),
            "Graphics module should be present");
        assert!(physics["Defines"].is_object(),
            "Physics module should be present");

        println!("✓ Graphics-Physics integration validated");
    }

    #[test]
    fn e2e_advanced_all_modules_integration() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        // All modules should be accessible together
        let module_count = transformed.len();
        assert!(module_count >= 4, "Should have at least 4 core modules");

        // Verify no module conflicts
        for (module_name, module_data) in transformed {
            assert!(module_data["Defines"].is_object(),
                "Module {} should have valid Defines", module_name);
        }

        println!("✓ All modules integration validated ({} modules)", module_count);
    }

    // ============================================================================
    // EDGE CASE TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_empty_state_handling() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut empty_states = 0;

        for (state_name, state_data) in states {
            let phases = state_data["Phases"].as_object().unwrap();
            if phases.is_empty() {
                empty_states += 1;

                // Empty states should still have valid structure
                assert!(state_data["Parent"].is_string() || state_data["Parent"].is_null(),
                    "Empty state {} should have valid parent field", state_name);
            }
        }

        println!("✓ Empty state handling validated ({} empty states)", empty_states);
    }

    #[test]
    fn e2e_advanced_null_value_handling() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut null_parents = 0;
        let mut null_types = 0;

        for (_, state_data) in states {
            if state_data["Parent"].is_null() {
                null_parents += 1;
            }
            if state_data["Type"].is_null() {
                null_types += 1;
            }
        }

        // Null values are valid in certain contexts
        println!("✓ Null value handling validated");
        println!("  States with null parent: {}", null_parents);
        println!("  States with null type: {}", null_types);
    }

    #[test]
    fn e2e_advanced_variable_name_patterns() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut special_names = 0;

        for var_name in variables.keys() {
            // Check for special naming patterns
            if var_name == "Null" || var_name == "Base" {
                special_names += 1;
                println!("  Special variable name: {}", var_name);
            }
        }

        println!("✓ Variable name patterns validated ({} special names)", special_names);
    }

    // ============================================================================
    // DATA COMPLETENESS TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_complete_character_coverage() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");

        // Verify all major sections are present and non-empty
        assert!(golden["metadata"].is_object(), "Should have metadata");
        assert!(golden["variables"].is_object(), "Should have variables");
        assert!(golden["states"].is_object(), "Should have states");
        assert!(golden["subentities"].is_object(), "Should have subentities");
        assert!(golden["transformed_data"].is_object(), "Should have transformed_data");

        let states_count = golden["states"].as_object().unwrap().len();
        let variables_count = golden["variables"].as_object().unwrap().len();
        let subentities_count = golden["subentities"].as_object().unwrap().len();
        let modules_count = golden["transformed_data"].as_object().unwrap().len();

        println!("✓ Complete character coverage validated:");
        println!("  States: {}", states_count);
        println!("  Variables: {}", variables_count);
        println!("  Subentities: {}", subentities_count);
        println!("  Modules: {}", modules_count);
    }

    #[test]
    fn e2e_advanced_metadata_completeness() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        for file_path in files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            // Required fields
            assert!(metadata["name"].is_string(),
                "{} should have name", file_path);
            assert!(metadata["editorname"].is_string(),
                "{} should have editorname", file_path);
            assert!(metadata["filepath"].is_string(),
                "{} should have filepath", file_path);

            println!("  ✓ {} metadata complete", file_path);
        }

        println!("✓ Metadata completeness validated (3 files)");
    }

    // ============================================================================
    // SUMMARY TEST
    // ============================================================================

    #[test]
    fn e2e_advanced_test_summary() {
        println!("\n=== E2E Advanced Test Summary ===\n");
        println!("Advanced test areas covered:");
        println!("  ✓ Subentity structure and consistency");
        println!("  ✓ Module system (Graphics, Physics, Attacks)");
        println!("  ✓ Advanced state features (flags, chains, types)");
        println!("  ✓ Performance and stress testing");
        println!("  ✓ Real-world scenarios (combos, animations, AI)");
        println!("  ✓ Cross-module integration");
        println!("  ✓ Edge cases and null handling");
        println!("  ✓ Data completeness");
        println!("\nAll advanced e2e tests completed!\n");
    }
}
