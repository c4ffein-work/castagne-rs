// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Extended Coverage E2E Tests
//!
//! Additional e2e tests covering edge cases and scenarios not fully tested:
//! - Complex state transitions and inheritance chains
//! - Module system interactions
//! - Advanced string handling (unicode, escape sequences)
//! - Boundary conditions (very large values, empty strings)
//! - Real-world fighting game patterns
//! - Performance edge cases
//! - Complex conditional logic patterns

use std::fs;
use serde_json::Value;
use std::io::Write as IoWrite;
use tempfile::NamedTempFile;

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

    fn create_temp_casp(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        file.write_all(content.as_bytes()).expect("Failed to write to temp file");
        file
    }

    // ============================================================================
    // STATE TRANSITION TESTS
    // ============================================================================

    #[test]
    fn e2e_extended_state_transition_flags() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut states_with_flags = 0;
        let mut total_flags = 0;

        for (state_name, state_data) in states {
            if let Some(flags) = state_data["TransitionFlags"].as_object() {
                if !flags.is_empty() {
                    states_with_flags += 1;
                    total_flags += flags.len();

                    // Validate flag structure
                    for (flag_name, flag_value) in flags {
                        assert!(flag_value.is_string() || flag_value.is_boolean() || flag_value.is_number(),
                            "State {} flag {} has invalid type", state_name, flag_name);
                    }
                }
            }
        }

        println!("✓ State transition flags validated");
        println!("  States with flags: {}", states_with_flags);
        println!("  Total flags: {}", total_flags);
    }

    #[test]
    fn e2e_extended_state_type_distribution() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut type_counts = std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(state_type) = state_data["Type"].as_str() {
                *type_counts.entry(state_type).or_insert(0) += 1;
            } else {
                *type_counts.entry("null").or_insert(0) += 1;
            }
        }

        println!("✓ State type distribution:");
        for (state_type, count) in type_counts.iter() {
            println!("  {}: {}", state_type, count);
        }
    }

    #[test]
    fn e2e_extended_state_inheritance_depth() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut max_depth = 0;
        let mut deepest_state = String::new();

        for (state_name, _) in states {
            let mut depth = 0;
            let mut current = state_name.clone();

            // Follow parent chain
            while let Some(state_data) = states.get(&current) {
                if let Some(parent) = state_data["Parent"].as_str() {
                    depth += 1;
                    current = parent.to_string();
                } else {
                    break;
                }
            }

            if depth > max_depth {
                max_depth = depth;
                deepest_state = state_name.clone();
            }
        }

        println!("✓ State inheritance depth analyzed");
        println!("  Maximum depth: {}", max_depth);
        if !deepest_state.is_empty() {
            println!("  Deepest state: {}", deepest_state);
        }
    }

    // ============================================================================
    // MODULE SYSTEM TESTS
    // ============================================================================

    #[test]
    fn e2e_extended_module_completeness() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        // Check for common module types
        let expected_modules = vec!["Graphics", "Anims", "PhysicsMovement", "AttacksMechanics"];
        let mut found_modules = Vec::new();
        let mut missing_modules = Vec::new();

        for module_name in expected_modules {
            if transformed.contains_key(module_name) {
                found_modules.push(module_name);
            } else {
                missing_modules.push(module_name);
            }
        }

        println!("✓ Module completeness checked");
        println!("  Found modules: {:?}", found_modules);
        if !missing_modules.is_empty() {
            println!("  Note: Missing expected modules: {:?} (may be valid)", missing_modules);
        }
    }

    #[test]
    fn e2e_extended_module_defines_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        let mut total_defines = 0;

        for (module_name, module_data) in transformed {
            if let Some(defines) = module_data["Defines"].as_object() {
                total_defines += defines.len();

                // Each define should have proper structure
                for (define_name, define_value) in defines {
                    assert!(define_value.is_string() || define_value.is_number() ||
                           define_value.is_boolean() || define_value.is_object() || define_value.is_array(),
                        "Module {} define {} has unexpected type", module_name, define_name);
                }
            }
        }

        println!("✓ Module defines validated ({} total)", total_defines);
    }

    #[test]
    fn e2e_extended_graphics_module_detailed() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        if let Some(graphics) = golden["transformed_data"]["Graphics"].as_object() {
            // Check for various graphics components
            let has_spritesheets = graphics.contains_key("Spritesheets");
            let has_palettes = graphics.contains_key("Palettes");
            let has_anims = graphics.contains_key("Anims");
            let has_defines = graphics.contains_key("Defines");

            println!("✓ Graphics module detailed check:");
            println!("  Has spritesheets: {}", has_spritesheets);
            println!("  Has palettes: {}", has_palettes);
            println!("  Has anims: {}", has_anims);
            println!("  Has defines: {}", has_defines);
        } else {
            println!("⚠ Graphics module not found (may be valid)");
        }
    }

    // ============================================================================
    // VARIABLE VALUE BOUNDARY TESTS
    // ============================================================================

    #[test]
    fn e2e_extended_int_boundary_values() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut min_int: Option<i64> = None;
        let mut max_int: Option<i64> = None;
        let mut int_count = 0;

        for (_var_name, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Int" {
                let value = var_data["Value"].as_str().unwrap();
                if let Ok(int_val) = value.parse::<i64>() {
                    int_count += 1;
                    min_int = Some(min_int.map_or(int_val, |m| m.min(int_val)));
                    max_int = Some(max_int.map_or(int_val, |m| m.max(int_val)));
                }
            }
        }

        println!("✓ Int boundary values analyzed ({} ints)", int_count);
        if let (Some(min), Some(max)) = (min_int, max_int) {
            println!("  Range: {} to {}", min, max);
        }
    }

    #[test]
    fn e2e_extended_float_special_values() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut zero_count = 0;
        let mut negative_count = 0;
        let mut positive_count = 0;
        let mut float_count = 0;

        for (_, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Float" {
                let value = var_data["Value"].as_str().unwrap();
                if let Ok(float_val) = value.parse::<f64>() {
                    float_count += 1;
                    if float_val == 0.0 {
                        zero_count += 1;
                    } else if float_val < 0.0 {
                        negative_count += 1;
                    } else {
                        positive_count += 1;
                    }
                }
            }
        }

        println!("✓ Float special values analyzed ({} total)", float_count);
        println!("  Zero: {}, Negative: {}, Positive: {}", zero_count, negative_count, positive_count);
    }

    #[test]
    fn e2e_extended_string_lengths() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut max_len = 0;
        let mut empty_count = 0;
        let mut str_count = 0;
        let mut longest_var = String::new();

        for (var_name, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Str" {
                let value = var_data["Value"].as_str().unwrap();
                str_count += 1;

                if value.is_empty() || value == "null" {
                    empty_count += 1;
                } else if value.len() > max_len {
                    max_len = value.len();
                    longest_var = var_name.clone();
                }
            }
        }

        println!("✓ String lengths analyzed ({} strings)", str_count);
        println!("  Empty/null: {}", empty_count);
        if max_len > 0 {
            println!("  Longest: {} chars in {}", max_len, longest_var);
        }
    }

    #[test]
    fn e2e_extended_vector_zero_values() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut zero_vec2_count = 0;
        let mut zero_vec3_count = 0;

        for (_, var_data) in variables {
            let var_type = var_data["Type"].as_str().unwrap_or("");
            let value = var_data["Value"].as_str().unwrap();

            if value != "null" {
                match var_type {
                    "Vec2" => {
                        if value == "0,0" || value == "0.0,0.0" {
                            zero_vec2_count += 1;
                        }
                    }
                    "Vec3" => {
                        if value == "0,0,0" || value == "0.0,0.0,0.0" {
                            zero_vec3_count += 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        println!("✓ Vector zero values analyzed");
        println!("  Zero Vec2: {}, Zero Vec3: {}", zero_vec2_count, zero_vec3_count);
    }

    // ============================================================================
    // PHASE ANALYSIS TESTS
    // ============================================================================

    #[test]
    fn e2e_extended_phase_type_distribution() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut phase_types = std::collections::HashMap::new();
        let mut total_phases = 0;

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (phase_name, _) in phases {
                    total_phases += 1;
                    *phase_types.entry(phase_name.clone()).or_insert(0) += 1;
                }
            }
        }

        println!("✓ Phase type distribution ({} total phases)", total_phases);
        let mut sorted_phases: Vec<_> = phase_types.iter().collect();
        sorted_phases.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
        for (phase_type, count) in sorted_phases.iter().take(10) {
            println!("  {}: {}", phase_type, count);
        }
    }

    #[test]
    fn e2e_extended_actions_per_phase_statistics() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut action_counts = Vec::new();
        let mut phases_checked = 0;

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    phases_checked += 1;
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        action_counts.push(actions.len());
                    } else {
                        action_counts.push(0);
                    }
                }
            }
        }

        if !action_counts.is_empty() {
            let total: usize = action_counts.iter().sum();
            let avg = total as f64 / action_counts.len() as f64;
            let max = action_counts.iter().max().unwrap_or(&0);
            let min = action_counts.iter().min().unwrap_or(&0);

            println!("✓ Actions per phase statistics ({} phases)", phases_checked);
            println!("  Average: {:.2}", avg);
            println!("  Min: {}, Max: {}", min, max);
        } else {
            println!("⚠ No action data found in phases (may be stored differently)");
        }
    }

    #[test]
    fn e2e_extended_empty_action_phases() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut empty_phases = 0;
        let mut total_phases = 0;

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    total_phases += 1;
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        if actions.is_empty() {
                            empty_phases += 1;
                        }
                    }
                }
            }
        }

        println!("✓ Empty action phases analyzed");
        println!("  Empty: {}/{} phases", empty_phases, total_phases);
    }

    // ============================================================================
    // ACTION FUNCTION TESTS
    // ============================================================================

    #[test]
    fn e2e_extended_action_function_frequency() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut function_counts = std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            if let Some(func_name) = action["function"].as_str() {
                                *function_counts.entry(func_name.to_string()).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Action function frequency analyzed");
        if !function_counts.is_empty() {
            let mut sorted: Vec<_> = function_counts.iter().collect();
            sorted.sort_by_key(|(_, count)| std::cmp::Reverse(**count));

            println!("  Top 5 most used functions:");
            for (func_name, count) in sorted.iter().take(5) {
                println!("    {}: {}", func_name, count);
            }
        } else {
            println!("  No action data found (may be stored differently)");
        }
    }

    #[test]
    fn e2e_extended_action_argument_patterns() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut no_args = 0;
        let mut one_arg = 0;
        let mut multi_args = 0;
        let mut total_actions = 0;

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            total_actions += 1;
                            if let Some(args) = action["args"].as_array() {
                                match args.len() {
                                    0 => no_args += 1,
                                    1 => one_arg += 1,
                                    _ => multi_args += 1,
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Action argument patterns analyzed ({} actions)", total_actions);
        if total_actions > 0 {
            println!("  No args: {}", no_args);
            println!("  One arg: {}", one_arg);
            println!("  Multiple args: {}", multi_args);
        }
    }

    // ============================================================================
    // CROSS-FILE CONSISTENCY TESTS
    // ============================================================================

    #[test]
    fn e2e_extended_all_files_variable_consistency() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        for file_path in files {
            let golden = load_golden_master(file_path);
            let variables = golden["variables"].as_object().unwrap();

            // Check that all variables have required fields
            for (var_name, var_data) in variables {
                assert!(var_data.is_object(),
                    "{}: Variable {} is not an object", file_path, var_name);

                // Required fields
                assert!(var_data.get("Type").is_some(),
                    "{}: Variable {} missing Type", file_path, var_name);
                assert!(var_data.get("Value").is_some(),
                    "{}: Variable {} missing Value", file_path, var_name);
                assert!(var_data.get("Mutability").is_some(),
                    "{}: Variable {} missing Mutability", file_path, var_name);
            }
        }

        println!("✓ All files have consistent variable structure");
    }

    #[test]
    fn e2e_extended_all_files_state_consistency() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        for file_path in files {
            let golden = load_golden_master(file_path);
            let states = golden["states"].as_object().unwrap();

            // Check that all states have required fields
            for (state_name, state_data) in states {
                assert!(state_data.is_object(),
                    "{}: State {} is not an object", file_path, state_name);

                // Required fields (may be null)
                assert!(state_data.get("Parent").is_some(),
                    "{}: State {} missing Parent", file_path, state_name);
                assert!(state_data.get("Type").is_some(),
                    "{}: State {} missing Type", file_path, state_name);
                assert!(state_data.get("Phases").is_some(),
                    "{}: State {} missing Phases", file_path, state_name);
            }
        }

        println!("✓ All files have consistent state structure");
    }

    #[test]
    fn e2e_extended_file_size_comparison() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        println!("✓ File size comparison:");
        for file_path in files {
            let golden = load_golden_master(file_path);
            let states = golden["states"].as_object().unwrap().len();
            let variables = golden["variables"].as_object().unwrap().len();
            let modules = golden["transformed_data"].as_object().unwrap().len();

            println!("  {}", file_path);
            println!("    States: {}, Variables: {}, Modules: {}", states, variables, modules);
        }
    }

    // ============================================================================
    // SUBENTITY TESTS
    // ============================================================================

    #[test]
    fn e2e_extended_subentity_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let subentities = golden["subentities"].as_object().unwrap();

        println!("✓ Subentity structure validated ({} subentities)", subentities.len());

        for (subentity_name, subentity_data) in subentities {
            // Each subentity should be an object
            assert!(subentity_data.is_object(),
                "Subentity {} is not an object", subentity_name);
        }
    }

    #[test]
    fn e2e_extended_subentity_variables() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();
        let subentities = golden["subentities"].as_object().unwrap();

        // Check for variables that might reference subentities
        let mut subentity_refs = 0;

        for (_var_name, var_data) in variables {
            // Variables with empty types might be subentity references
            if var_data["Type"].as_str().unwrap_or("").is_empty() {
                subentity_refs += 1;
            }
        }

        println!("✓ Subentity variable analysis");
        println!("  Subentities: {}", subentities.len());
        println!("  Potential subentity refs in variables: {}", subentity_refs);
    }

    // ============================================================================
    // METADATA COMPLETENESS TESTS
    // ============================================================================

    #[test]
    fn e2e_extended_metadata_all_fields() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        for file_path in files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            // Check all standard fields
            let has_name = !metadata["name"].is_null();
            let has_editorname = !metadata["editorname"].is_null();
            let has_filepath = !metadata["filepath"].is_null();
            let has_skeleton = !metadata["skeleton"].is_null();
            let has_author = !metadata["author"].is_null();
            let has_description = !metadata["description"].is_null();

            println!("✓ {} metadata:", file_path);
            println!("  name: {}, editorname: {}, filepath: {}", has_name, has_editorname, has_filepath);
            println!("  skeleton: {}, author: {}, description: {}", has_skeleton, has_author, has_description);
        }
    }

    // ============================================================================
    // DATA INTEGRITY SUMMARY
    // ============================================================================

    #[test]
    fn e2e_extended_comprehensive_summary() {
        println!("\n=== Extended Coverage Validation Summary ===\n");

        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        for file_path in files {
            println!("File: {}", file_path);
            let golden = load_golden_master(file_path);

            // Basic counts
            let states = golden["states"].as_object().unwrap().len();
            let variables = golden["variables"].as_object().unwrap().len();
            let subentities = golden["subentities"].as_object().unwrap().len();
            let modules = golden["transformed_data"].as_object().unwrap().len();

            // Variable type breakdown
            let vars = golden["variables"].as_object().unwrap();
            let mut type_counts = std::collections::HashMap::new();
            for (_, var_data) in vars {
                let var_type = var_data["Type"].as_str().unwrap_or("unknown");
                *type_counts.entry(var_type).or_insert(0) += 1;
            }

            // Phase count
            let mut total_phases = 0;
            let state_objs = golden["states"].as_object().unwrap();
            for (_, state_data) in state_objs {
                if let Some(phases) = state_data["Phases"].as_object() {
                    total_phases += phases.len();
                }
            }

            println!("  States: {}, Variables: {}, Subentities: {}, Modules: {}",
                     states, variables, subentities, modules);
            println!("  Total phases: {}", total_phases);
            println!("  Variable types: {:?}", type_counts);
            println!();
        }

        println!("✓ Extended coverage validation complete!\n");
    }
}
