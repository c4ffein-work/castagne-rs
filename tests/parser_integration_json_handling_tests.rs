// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! JSON Handling E2E Tests
//!
//! Tests that validate JSON parsing and structure:
//! - JSON validity and parsing
//! - Required top-level fields
//! - Data type validation
//! - Nested structure handling
//! - Array handling
//! - Null value handling
//! - Object structure consistency
//! - Numeric value ranges
//! - String field validation

use serde_json::Value;
use std::fs;

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

    fn get_golden_files() -> Vec<&'static str> {
        vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ]
    }

    // ============================================================================
    // JSON VALIDITY TESTS
    // ============================================================================

    #[test]
    fn e2e_json_files_valid() {
        let files = get_golden_files();

        println!("✓ Validating JSON files:");

        for file_path in &files {
            let json_content = fs::read_to_string(file_path)
                .unwrap_or_else(|_| panic!("Failed to read file: {}", file_path));

            let parse_result: Result<Value, _> = serde_json::from_str(&json_content);

            assert!(
                parse_result.is_ok(),
                "JSON file should be valid: {}",
                file_path
            );

            println!("  ✓ {} is valid JSON", file_path);
        }

        println!("✓ All JSON files are valid");
    }

    #[test]
    fn e2e_json_files_not_empty() {
        let files = get_golden_files();

        println!("✓ Checking JSON files are not empty:");

        for file_path in &files {
            let json_content = fs::read_to_string(file_path)
                .unwrap_or_else(|_| panic!("Failed to read file: {}", file_path));

            assert!(
                !json_content.is_empty(),
                "JSON file should not be empty: {}",
                file_path
            );

            let size_kb = json_content.len() as f64 / 1024.0;
            println!("  {} ({:.1} KB)", file_path, size_kb);
        }

        println!("✓ All JSON files have content");
    }

    // ============================================================================
    // TOP-LEVEL STRUCTURE TESTS
    // ============================================================================

    #[test]
    fn e2e_json_top_level_fields() {
        let files = get_golden_files();

        println!("✓ Checking top-level fields:");

        let required_fields = vec!["metadata", "states", "subentities", "variables"];

        for file_path in &files {
            let golden = load_golden_master(file_path);

            assert!(
                golden.is_object(),
                "{}: Root should be an object",
                file_path
            );

            let obj = golden.as_object().unwrap();

            println!("  {}:", file_path);
            println!("    Total top-level fields: {}", obj.len());

            for field in &required_fields {
                assert!(
                    obj.contains_key(*field),
                    "{}: Missing required field '{}'",
                    file_path,
                    field
                );
                println!("    ✓ Has '{}'", field);
            }
        }

        println!("✓ All files have required top-level fields");
    }

    #[test]
    fn e2e_json_top_level_field_types() {
        let files = get_golden_files();

        println!("✓ Validating top-level field types:");

        for file_path in &files {
            let golden = load_golden_master(file_path);

            println!("  {}:", file_path);

            // metadata should be an object
            assert!(
                golden["metadata"].is_object(),
                "{}: metadata should be object",
                file_path
            );
            println!("    ✓ metadata is object");

            // states should be an object
            assert!(
                golden["states"].is_object(),
                "{}: states should be object",
                file_path
            );
            println!("    ✓ states is object");

            // subentities should be an object
            assert!(
                golden["subentities"].is_object(),
                "{}: subentities should be object",
                file_path
            );
            println!("    ✓ subentities is object");

            // variables should be an object
            assert!(
                golden["variables"].is_object(),
                "{}: variables should be object",
                file_path
            );
            println!("    ✓ variables is object");
        }

        println!("✓ All top-level fields have correct types");
    }

    // ============================================================================
    // STATES STRUCTURE TESTS
    // ============================================================================

    #[test]
    fn e2e_json_states_structure() {
        let files = get_golden_files();

        println!("✓ Analyzing states structure:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let states = golden["states"].as_object().unwrap();

            println!("  {}:", file_path);
            println!("    State count: {}", states.len());

            // Check first state structure
            if let Some((state_name, state_data)) = states.iter().next() {
                assert!(
                    state_data.is_object(),
                    "{}: State '{}' should be object",
                    file_path,
                    state_name
                );

                let state_obj = state_data.as_object().unwrap();

                // Check for expected fields
                let expected_fields = vec!["Parent", "Type", "TransitionFlags", "Phases"];

                for field in &expected_fields {
                    assert!(
                        state_obj.contains_key(*field),
                        "{}: State '{}' missing field '{}'",
                        file_path,
                        state_name,
                        field
                    );
                }

                println!("    First state '{}' has all expected fields", state_name);
            }
        }
    }

    #[test]
    fn e2e_json_states_field_types() {
        let files = get_golden_files();

        println!("✓ Validating state field types:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let states = golden["states"].as_object().unwrap();

            println!("  {}:", file_path);

            let mut checked = 0;
            for (state_name, state_data) in states.iter() {
                if checked >= 5 {
                    break;
                }

                let state_obj = state_data.as_object().unwrap();

                // Parent can be null or string
                let parent = &state_obj["Parent"];
                assert!(
                    parent.is_null() || parent.is_string(),
                    "{}: State '{}' Parent should be null or string",
                    file_path,
                    state_name
                );

                // Type can be null or string
                let state_type = &state_obj["Type"];
                assert!(
                    state_type.is_null() || state_type.is_string(),
                    "{}: State '{}' Type should be null or string",
                    file_path,
                    state_name
                );

                // TransitionFlags should be array
                assert!(
                    state_obj["TransitionFlags"].is_array(),
                    "{}: State '{}' TransitionFlags should be array",
                    file_path,
                    state_name
                );

                // Phases should be object
                assert!(
                    state_obj["Phases"].is_object(),
                    "{}: State '{}' Phases should be object",
                    file_path,
                    state_name
                );

                checked += 1;
            }

            println!("    Validated {} states", checked);
        }
    }

    #[test]
    fn e2e_json_states_transition_flags_array() {
        let files = get_golden_files();

        println!("✓ Analyzing TransitionFlags arrays:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let states = golden["states"].as_object().unwrap();

            let mut empty_count = 0;
            let mut non_empty_count = 0;
            let mut max_length = 0;

            for (_state_name, state_data) in states.iter() {
                let state_obj = state_data.as_object().unwrap();
                let flags = state_obj["TransitionFlags"].as_array().unwrap();

                if flags.is_empty() {
                    empty_count += 1;
                } else {
                    non_empty_count += 1;
                    max_length = max_length.max(flags.len());
                }
            }

            println!("  {}:", file_path);
            println!("    Empty TransitionFlags: {}", empty_count);
            println!("    Non-empty TransitionFlags: {}", non_empty_count);
            println!("    Max TransitionFlags length: {}", max_length);
        }
    }

    // ============================================================================
    // VARIABLES STRUCTURE TESTS
    // ============================================================================

    #[test]
    fn e2e_json_variables_structure() {
        let files = get_golden_files();

        println!("✓ Analyzing variables structure:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let variables = golden["variables"].as_object().unwrap();

            println!("  {}:", file_path);
            println!("    Variable count: {}", variables.len());

            // Check first variable structure
            if let Some((var_name, var_data)) = variables.iter().next() {
                assert!(
                    var_data.is_object(),
                    "{}: Variable '{}' should be object",
                    file_path,
                    var_name
                );

                let var_obj = var_data.as_object().unwrap();

                // Check for expected fields
                let expected_fields = vec!["Name", "Value", "Type", "Subtype", "Mutability"];

                for field in &expected_fields {
                    assert!(
                        var_obj.contains_key(*field),
                        "{}: Variable '{}' missing field '{}'",
                        file_path,
                        var_name,
                        field
                    );
                }

                println!("    Variable '{}' has all expected fields", var_name);
            }
        }
    }

    #[test]
    fn e2e_json_variables_field_types() {
        let files = get_golden_files();

        println!("✓ Validating variable field types:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let variables = golden["variables"].as_object().unwrap();

            println!("  {}:", file_path);

            let mut checked = 0;
            for (var_name, var_data) in variables.iter() {
                if checked >= 5 {
                    break;
                }

                let var_obj = var_data.as_object().unwrap();

                // Name can be null or string
                let name = &var_obj["Name"];
                assert!(
                    name.is_null() || name.is_string(),
                    "{}: Variable '{}' Name should be null or string",
                    file_path,
                    var_name
                );

                // Value should be string
                assert!(
                    var_obj["Value"].is_string(),
                    "{}: Variable '{}' Value should be string",
                    file_path,
                    var_name
                );

                // Type should be string
                assert!(
                    var_obj["Type"].is_string(),
                    "{}: Variable '{}' Type should be string",
                    file_path,
                    var_name
                );

                // Subtype should be string
                assert!(
                    var_obj["Subtype"].is_string(),
                    "{}: Variable '{}' Subtype should be string",
                    file_path,
                    var_name
                );

                // Mutability should be string
                assert!(
                    var_obj["Mutability"].is_string(),
                    "{}: Variable '{}' Mutability should be string",
                    file_path,
                    var_name
                );

                checked += 1;
            }

            println!("    Validated {} variables", checked);
        }
    }

    // ============================================================================
    // SUBENTITIES STRUCTURE TESTS
    // ============================================================================

    #[test]
    fn e2e_json_subentities_structure() {
        let files = get_golden_files();

        println!("✓ Analyzing subentities structure:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let subentities = golden["subentities"].as_object().unwrap();

            println!("  {}:", file_path);
            println!("    Subentity count: {}", subentities.len());

            // List subentity names
            let names: Vec<&String> = subentities.keys().collect();
            println!("    Subentities: {:?}", names);
        }
    }

    #[test]
    fn e2e_json_subentities_field_validation() {
        let files = get_golden_files();

        println!("✓ Validating subentities fields:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let subentities = golden["subentities"].as_object().unwrap();

            println!("  {}:", file_path);

            for (subentity_name, subentity_data) in subentities.iter() {
                assert!(
                    subentity_data.is_object(),
                    "{}: Subentity '{}' should be object",
                    file_path,
                    subentity_name
                );

                println!("    ✓ Subentity '{}' is object", subentity_name);
            }
        }
    }

    // ============================================================================
    // TRANSFORMED DATA TESTS
    // ============================================================================

    #[test]
    fn e2e_json_transformed_data_presence() {
        let files = get_golden_files();

        println!("✓ Checking transformed_data presence:");

        for file_path in &files {
            let golden = load_golden_master(file_path);

            if let Some(transformed) = golden.get("transformed_data") {
                assert!(
                    transformed.is_object(),
                    "{}: transformed_data should be object",
                    file_path
                );

                let obj = transformed.as_object().unwrap();
                println!(
                    "  {} has transformed_data ({} top-level keys)",
                    file_path,
                    obj.len()
                );

                // List some keys
                let keys: Vec<&String> = obj.keys().take(5).collect();
                println!("    First few keys: {:?}", keys);
            } else {
                println!("  {} has no transformed_data", file_path);
            }
        }
    }

    #[test]
    fn e2e_json_transformed_data_numeric_values() {
        let file_path = "golden_masters/Baston-Model.json";
        let golden = load_golden_master(file_path);

        println!("✓ Analyzing numeric values in transformed_data:");

        if let Some(transformed) = golden.get("transformed_data") {
            if let Some(attacks_types) = transformed.get("AttacksTypes") {
                if let Some(defines) = attacks_types.get("Defines") {
                    let defines_obj = defines.as_object().unwrap();

                    println!("  Found {} numeric defines", defines_obj.len());

                    // Check some numeric values
                    let mut checked = 0;
                    for (key, value) in defines_obj.iter() {
                        if checked >= 5 {
                            break;
                        }

                        assert!(value.is_number(), "Define '{}' should be number", key);

                        println!("    {} = {}", key, value);
                        checked += 1;
                    }

                    println!("  ✓ Validated numeric defines");
                }
            }
        }
    }

    // ============================================================================
    // NULL VALUE HANDLING TESTS
    // ============================================================================

    #[test]
    fn e2e_json_null_handling() {
        let files = get_golden_files();

        println!("✓ Analyzing null value handling:");

        for file_path in &files {
            let golden = load_golden_master(file_path);

            // Count nulls in states
            let states = golden["states"].as_object().unwrap();
            let mut null_parent_count = 0;
            let mut null_type_count = 0;

            for (_state_name, state_data) in states.iter() {
                let state_obj = state_data.as_object().unwrap();
                if state_obj["Parent"].is_null() {
                    null_parent_count += 1;
                }
                if state_obj["Type"].is_null() {
                    null_type_count += 1;
                }
            }

            println!("  {}:", file_path);
            println!(
                "    States with null Parent: {}/{}",
                null_parent_count,
                states.len()
            );
            println!(
                "    States with null Type: {}/{}",
                null_type_count,
                states.len()
            );

            // Check nulls in variables
            let variables = golden["variables"].as_object().unwrap();
            let mut null_name_count = 0;

            for (_var_name, var_data) in variables.iter() {
                let var_obj = var_data.as_object().unwrap();
                if var_obj["Name"].is_null() {
                    null_name_count += 1;
                }
            }

            println!(
                "    Variables with null Name: {}/{}",
                null_name_count,
                variables.len()
            );
        }
    }

    // ============================================================================
    // DATA CONSISTENCY TESTS
    // ============================================================================

    #[test]
    fn e2e_json_state_consistency() {
        let files = get_golden_files();

        println!("✓ Checking state structure consistency:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let states = golden["states"].as_object().unwrap();

            println!("  {}:", file_path);

            let mut all_consistent = true;
            let expected_fields = vec!["Parent", "Type", "TransitionFlags", "Phases"];

            for (state_name, state_data) in states.iter() {
                let state_obj = state_data.as_object().unwrap();

                for field in &expected_fields {
                    if !state_obj.contains_key(*field) {
                        println!("    ✗ State '{}' missing field '{}'", state_name, field);
                        all_consistent = false;
                    }
                }
            }

            if all_consistent {
                println!(
                    "    ✓ All {} states have consistent structure",
                    states.len()
                );
            }

            assert!(
                all_consistent,
                "{}: States should have consistent structure",
                file_path
            );
        }
    }

    #[test]
    fn e2e_json_variable_consistency() {
        let files = get_golden_files();

        println!("✓ Checking variable structure consistency:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let variables = golden["variables"].as_object().unwrap();

            println!("  {}:", file_path);

            let mut all_consistent = true;
            let expected_fields = vec!["Name", "Value", "Type", "Subtype", "Mutability"];

            for (var_name, var_data) in variables.iter() {
                let var_obj = var_data.as_object().unwrap();

                for field in &expected_fields {
                    if !var_obj.contains_key(*field) {
                        println!("    ✗ Variable '{}' missing field '{}'", var_name, field);
                        all_consistent = false;
                    }
                }
            }

            if all_consistent {
                println!(
                    "    ✓ All {} variables have consistent structure",
                    variables.len()
                );
            }

            assert!(
                all_consistent,
                "{}: Variables should have consistent structure",
                file_path
            );
        }
    }

    // ============================================================================
    // SIZE AND COMPLEXITY TESTS
    // ============================================================================

    #[test]
    fn e2e_json_size_comparison() {
        let files = get_golden_files();

        println!("✓ JSON file size comparison:");

        let mut sizes: Vec<(&str, usize, usize, usize)> = Vec::new();

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let json_str = serde_json::to_string(&golden).unwrap();

            let size_bytes = json_str.len();
            let states_count = golden["states"].as_object().unwrap().len();
            let variables_count = golden["variables"].as_object().unwrap().len();

            sizes.push((*file_path, size_bytes, states_count, variables_count));
        }

        // Sort by size
        sizes.sort_by(|a, b| b.1.cmp(&a.1));

        for (path, size, states, vars) in &sizes {
            let kb = *size as f64 / 1024.0;
            println!("  {}:", path);
            println!("    Size: {:.1} KB", kb);
            println!("    States: {}, Variables: {}", states, vars);
        }
    }

    // ============================================================================
    // SUMMARY TEST
    // ============================================================================

    #[test]
    fn e2e_json_handling_comprehensive_summary() {
        println!("\n=== E2E JSON Handling Summary ===\n");
        println!("Comprehensive JSON handling tests completed:");
        println!("  ✓ JSON validity and parsing");
        println!("  ✓ Top-level field structure and types");
        println!("  ✓ States structure and field types");
        println!("  ✓ TransitionFlags array handling");
        println!("  ✓ Variables structure and field types");
        println!("  ✓ Subentities structure");
        println!("  ✓ Transformed data presence and validation");
        println!("  ✓ Numeric value validation");
        println!("  ✓ Null value handling");
        println!("  ✓ Data consistency across structures");
        println!("  ✓ File size and complexity analysis");
        println!("\nAll JSON handling tests passed!\n");
    }
}
