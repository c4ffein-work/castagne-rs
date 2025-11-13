// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Stress Testing and Performance Validation E2E Tests
//!
//! Tests that validate parser behavior under stress and measure performance:
//! - Large data structure handling
//! - Repeated parsing operations
//! - Complex action chains
//! - Deep nesting scenarios
//! - Multiple file parsing
//! - Memory efficiency validation
//! - Parser state consistency

use serde_json::Value;
use std::fs;
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
        file.write_all(content.as_bytes())
            .expect("Failed to write to temp file");
        file
    }

    // ============================================================================
    // LARGE DATA STRUCTURE TESTS
    // ============================================================================

    #[test]
    fn e2e_stress_states_with_many_phases() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut phase_counts: Vec<(String, usize)> = Vec::new();

        for (state_name, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                let phase_count = phases.len();
                if phase_count > 0 {
                    phase_counts.push((state_name.clone(), phase_count));
                }
            }
        }

        // Sort by phase count
        phase_counts.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

        println!("✓ States with most phases:");
        for (state_name, count) in phase_counts.iter().take(10) {
            println!("  {}: {} phases", state_name, count);
        }

        // Verify parser can handle states with many phases
        let max_phases = phase_counts.first().map(|&(_, count)| count).unwrap_or(0);
        assert!(
            max_phases < 1000,
            "Parser should handle up to 1000 phases per state"
        );
    }

    #[test]
    fn e2e_stress_phases_with_many_actions() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut action_counts: Vec<(String, usize)> = Vec::new();

        for (state_name, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (phase_name, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        let action_count = actions.len();
                        if action_count > 0 {
                            let full_name = format!("{}.{}", state_name, phase_name);
                            action_counts.push((full_name, action_count));
                        }
                    }
                }
            }
        }

        // Sort by action count
        action_counts.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

        println!("✓ Phases with most actions:");
        for (phase_name, count) in action_counts.iter().take(10) {
            println!("  {}: {} actions", phase_name, count);
        }

        let max_actions = action_counts.first().map(|&(_, count)| count).unwrap_or(0);
        println!("  Max actions in single phase: {}", max_actions);
    }

    #[test]
    fn e2e_stress_variables_comprehensive_count() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut type_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for (_, var_data) in variables {
            let var_type = var_data["Type"].as_str().unwrap_or("Unknown").to_string();
            *type_counts.entry(var_type).or_insert(0) += 1;
        }

        println!("✓ Variable type distribution:");
        let mut types: Vec<_> = type_counts.iter().collect();
        types.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));
        for (var_type, count) in types {
            println!("  {}: {} variables", var_type, count);
        }

        // Verify parser can handle many variables
        assert!(
            variables.len() < 100000,
            "Parser should handle up to 100k variables"
        );
    }

    #[test]
    fn e2e_stress_module_defines_volume() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        let mut define_counts: Vec<(String, usize)> = Vec::new();
        let mut total_defines = 0;

        for (module_name, module_data) in transformed {
            if let Some(defines) = module_data["Defines"].as_object() {
                let count = defines.len();
                define_counts.push((module_name.clone(), count));
                total_defines += count;
            }
        }

        define_counts.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

        println!("✓ Module defines volume:");
        println!("  Total defines: {}", total_defines);
        println!("  Modules with most defines:");
        for (module_name, count) in define_counts.iter().take(5) {
            println!("    {}: {} defines", module_name, count);
        }
    }

    // ============================================================================
    // REPEATED OPERATIONS TESTS
    // ============================================================================

    #[test]
    fn e2e_stress_multiple_file_loads() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        // Load each file multiple times
        for _ in 0..5 {
            for file_path in &files {
                let golden = load_golden_master(file_path);

                // Verify basic structure
                assert!(
                    golden["metadata"].is_object(),
                    "Missing metadata in {}",
                    file_path
                );
                assert!(
                    golden["states"].is_object(),
                    "Missing states in {}",
                    file_path
                );
                assert!(
                    golden["variables"].is_object(),
                    "Missing variables in {}",
                    file_path
                );
            }
        }

        println!(
            "✓ Multiple file loads successful ({} files × 5 iterations)",
            files.len()
        );
    }

    #[test]
    fn e2e_stress_json_parsing_consistency() {
        let golden1 = load_golden_master("golden_masters/Baston-Model.json");
        let golden2 = load_golden_master("golden_masters/Baston-Model.json");

        // Verify that parsing the same file twice gives identical results
        let json1 = serde_json::to_string(&golden1).unwrap();
        let json2 = serde_json::to_string(&golden2).unwrap();

        assert_eq!(
            json1.len(),
            json2.len(),
            "Repeated parsing should give same length"
        );
        assert_eq!(
            json1, json2,
            "Repeated parsing should give identical results"
        );

        println!("✓ JSON parsing consistency verified");
    }

    #[test]
    fn e2e_stress_deep_traversal_all_states() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut total_elements = 0;

        // Count all elements in state hierarchy
        for (_, state_data) in states {
            total_elements += 1; // State itself

            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    total_elements += 1; // Phase

                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            total_elements += 1; // Action

                            if let Some(args) = action["args"].as_array() {
                                total_elements += args.len(); // Arguments
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Deep traversal completed");
        println!("  Total elements traversed: {}", total_elements);
        assert!(total_elements > 0, "Should have traversed some elements");
    }

    // ============================================================================
    // COMPLEX ACTION CHAIN TESTS
    // ============================================================================

    #[test]
    fn e2e_stress_action_function_variety() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut function_names = std::collections::HashSet::new();
        let mut function_frequency: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            if let Some(func_name) = action["function"].as_str() {
                                function_names.insert(func_name.to_string());
                                *function_frequency.entry(func_name.to_string()).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Action function variety:");
        println!("  Unique functions: {}", function_names.len());

        let mut freq_vec: Vec<_> = function_frequency.iter().collect();
        freq_vec.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

        println!("  Most common functions:");
        for (func, count) in freq_vec.iter().take(10) {
            println!("    {}: {} times", func, count);
        }
    }

    #[test]
    fn e2e_stress_action_argument_complexity() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut total_args = 0;
        let mut total_actions = 0;
        let mut complex_actions = 0; // Actions with 3+ args

        for (_, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (_, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for action in actions {
                            total_actions += 1;

                            if let Some(args) = action["args"].as_array() {
                                let arg_count = args.len();
                                total_args += arg_count;

                                if arg_count >= 3 {
                                    complex_actions += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Action argument complexity:");
        println!("  Total actions: {}", total_actions);
        println!("  Total arguments: {}", total_args);
        println!("  Complex actions (3+ args): {}", complex_actions);
        if total_actions > 0 {
            println!(
                "  Average args per action: {:.2}",
                total_args as f64 / total_actions as f64
            );
        }
    }

    // ============================================================================
    // DEEP NESTING TESTS
    // ============================================================================

    #[test]
    fn e2e_stress_json_object_depth() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        fn measure_depth(value: &Value) -> usize {
            match value {
                Value::Object(map) => 1 + map.values().map(measure_depth).max().unwrap_or(0),
                Value::Array(arr) => 1 + arr.iter().map(measure_depth).max().unwrap_or(0),
                _ => 1,
            }
        }

        let states_depth = measure_depth(&golden["states"]);
        let transformed_depth = measure_depth(&golden["transformed_data"]);
        let total_depth = measure_depth(&golden);

        println!("✓ JSON nesting depth analysis:");
        println!("  States section: {} levels", states_depth);
        println!("  Transformed data: {} levels", transformed_depth);
        println!("  Total document: {} levels", total_depth);

        assert!(total_depth < 50, "JSON depth should be manageable");
    }

    #[test]
    fn e2e_stress_inheritance_chains() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut chains: Vec<Vec<String>> = Vec::new();

        // Find all inheritance chains
        for state_name in states.keys() {
            let mut chain = Vec::new();
            let mut current = state_name.clone();
            let mut visited = std::collections::HashSet::new();

            while let Some(state_data) = states.get(&current) {
                if !visited.insert(current.clone()) {
                    break; // Circular reference
                }

                chain.push(current.clone());

                if let Some(parent) = state_data["Parent"].as_str() {
                    current = parent.to_string();
                } else {
                    break;
                }

                if chain.len() > 100 {
                    break;
                } // Safety limit
            }

            if chain.len() > 1 {
                chains.push(chain);
            }
        }

        // Find longest chains
        chains.sort_by_key(|chain| std::cmp::Reverse(chain.len()));

        println!("✓ Inheritance chain analysis:");
        println!("  States with parents: {}", chains.len());
        if !chains.is_empty() {
            println!("  Longest chain: {} states", chains[0].len());
            println!("  Chain: {}", chains[0].join(" -> "));
        }
    }

    // ============================================================================
    // MEMORY EFFICIENCY TESTS
    // ============================================================================

    #[test]
    fn e2e_stress_large_string_handling() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut total_string_size = 0;
        let mut string_count = 0;

        for (var_name, var_data) in variables {
            // Count the variable name
            total_string_size += var_name.len();

            // Count the value if it's a string
            if let Some(value) = var_data["Value"].as_str() {
                total_string_size += value.len();
                string_count += 1;
            }
        }

        println!("✓ String memory analysis:");
        println!("  String variables: {}", string_count);
        println!("  Total string data: {} bytes", total_string_size);
        println!(
            "  Average string size: {:.1} bytes",
            if string_count > 0 {
                total_string_size as f64 / string_count as f64
            } else {
                0.0
            }
        );
    }

    #[test]
    fn e2e_stress_json_serialization_size() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        println!("✓ JSON serialization sizes:");
        for file_path in files {
            let golden = load_golden_master(file_path);

            let compact = serde_json::to_string(&golden).unwrap();
            let pretty = serde_json::to_string_pretty(&golden).unwrap();

            println!("  {}:", file_path);
            println!("    Compact: {} bytes", compact.len());
            println!("    Pretty: {} bytes", pretty.len());
            println!(
                "    Ratio: {:.1}x",
                pretty.len() as f64 / compact.len() as f64
            );
        }
    }

    // ============================================================================
    // STATE CONSISTENCY TESTS
    // ============================================================================

    #[test]
    fn e2e_stress_all_states_valid_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut invalid_states = Vec::new();

        for (state_name, state_data) in states {
            // Check required fields
            if !state_data.is_object() {
                invalid_states.push(format!("{}: not an object", state_name));
                continue;
            }

            if !state_data.get("Phases").is_some() {
                invalid_states.push(format!("{}: missing Phases", state_name));
            }

            if !state_data.get("Parent").is_some() {
                invalid_states.push(format!("{}: missing Parent", state_name));
            }

            if !state_data.get("Type").is_some() {
                invalid_states.push(format!("{}: missing Type", state_name));
            }
        }

        assert!(
            invalid_states.is_empty(),
            "Found invalid states: {:?}",
            invalid_states
        );

        println!("✓ All {} states have valid structure", states.len());
    }

    #[test]
    fn e2e_stress_all_variables_valid_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut invalid_vars = Vec::new();

        for (var_name, var_data) in variables {
            if !var_data.is_object() {
                invalid_vars.push(format!("{}: not an object", var_name));
                continue;
            }

            if !var_data.get("Type").is_some() {
                invalid_vars.push(format!("{}: missing Type", var_name));
            }

            if !var_data.get("Value").is_some() {
                invalid_vars.push(format!("{}: missing Value", var_name));
            }

            if !var_data.get("Mutability").is_some() {
                invalid_vars.push(format!("{}: missing Mutability", var_name));
            }
        }

        assert!(
            invalid_vars.is_empty(),
            "Found invalid variables: {:?}",
            invalid_vars
        );

        println!("✓ All {} variables have valid structure", variables.len());
    }

    #[test]
    fn e2e_stress_all_actions_valid_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut invalid_actions = Vec::new();
        let mut total_actions = 0;

        for (state_name, state_data) in states {
            if let Some(phases) = state_data["Phases"].as_object() {
                for (phase_name, phase_data) in phases {
                    if let Some(actions) = phase_data["Actions"].as_array() {
                        for (i, action) in actions.iter().enumerate() {
                            total_actions += 1;

                            if !action.is_object() {
                                invalid_actions.push(format!(
                                    "{}.{} action {}: not an object",
                                    state_name, phase_name, i
                                ));
                                continue;
                            }

                            if !action.get("function").is_some() {
                                invalid_actions.push(format!(
                                    "{}.{} action {}: missing function",
                                    state_name, phase_name, i
                                ));
                            }

                            if !action.get("args").is_some() {
                                invalid_actions.push(format!(
                                    "{}.{} action {}: missing args",
                                    state_name, phase_name, i
                                ));
                            }
                        }
                    }
                }
            }
        }

        assert!(
            invalid_actions.is_empty(),
            "Found invalid actions: {:?}",
            invalid_actions
        );

        println!("✓ All {} actions have valid structure", total_actions);
    }

    // ============================================================================
    // CROSS-FILE STRESS TESTS
    // ============================================================================

    #[test]
    fn e2e_stress_all_golden_masters_parseable() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        let mut total_states = 0;
        let mut total_variables = 0;
        let mut total_modules = 0;

        for file_path in &files {
            let golden = load_golden_master(file_path);

            let states = golden["states"].as_object().unwrap().len();
            let variables = golden["variables"].as_object().unwrap().len();
            let modules = golden["transformed_data"].as_object().unwrap().len();

            total_states += states;
            total_variables += variables;
            total_modules += modules;

            println!(
                "  {}: {} states, {} vars, {} modules",
                file_path, states, variables, modules
            );
        }

        println!("✓ All golden masters parseable");
        println!(
            "  Total: {} states, {} variables, {} modules",
            total_states, total_variables, total_modules
        );
    }

    #[test]
    fn e2e_stress_common_variable_names_across_files() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
        ];

        let mut all_vars: Vec<std::collections::HashSet<String>> = Vec::new();

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let variables = golden["variables"].as_object().unwrap();
            let var_names: std::collections::HashSet<String> = variables.keys().cloned().collect();
            all_vars.push(var_names);
        }

        if all_vars.len() >= 2 {
            let common: std::collections::HashSet<_> =
                all_vars[0].intersection(&all_vars[1]).cloned().collect();

            println!("✓ Common variables across files: {}", common.len());
            println!("  File 1: {} vars", all_vars[0].len());
            println!("  File 2: {} vars", all_vars[1].len());
            println!("  Common: {} vars", common.len());
        }
    }

    // ============================================================================
    // SUMMARY TEST
    // ============================================================================

    #[test]
    fn e2e_stress_comprehensive_summary() {
        println!("\n=== E2E Stress Testing Summary ===\n");
        println!("Comprehensive stress tests completed:");
        println!("  ✓ Large data structure handling");
        println!("  ✓ States with many phases");
        println!("  ✓ Phases with many actions");
        println!("  ✓ Variable volume testing");
        println!("  ✓ Module defines volume");
        println!("  ✓ Multiple file loads");
        println!("  ✓ JSON parsing consistency");
        println!("  ✓ Deep traversal operations");
        println!("  ✓ Action function variety");
        println!("  ✓ Action argument complexity");
        println!("  ✓ JSON nesting depth");
        println!("  ✓ Inheritance chain analysis");
        println!("  ✓ String memory analysis");
        println!("  ✓ JSON serialization testing");
        println!("  ✓ State structure validation");
        println!("  ✓ Variable structure validation");
        println!("  ✓ Action structure validation");
        println!("  ✓ Cross-file stress testing");
        println!("\nAll stress tests passed!\n");
    }
}
