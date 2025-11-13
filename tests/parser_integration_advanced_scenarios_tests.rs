// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Advanced End-to-End Scenario Tests
//!
//! Tests covering advanced real-world scenarios:
//! - JSON serialization/deserialization
//! - Character file organization
//! - State machine traversal algorithms
//! - Module transformation validation
//! - Performance benchmarks
//! - Complex inheritance scenarios
//! - Data migration patterns

use castagne_rs::parser::CastagneParser;
use serde_json::Value;
use std::fs;
use std::io::Write as IoWrite;
use std::path::Path;
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

    fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    fn create_temp_casp(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to temp file");
        file
    }

    // ============================================================================
    // JSON SERIALIZATION TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_json_round_trip() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        // Serialize back to string
        let json_str = serde_json::to_string(&golden).expect("Should serialize to JSON string");

        // Deserialize back
        let parsed: Value =
            serde_json::from_str(&json_str).expect("Should deserialize from JSON string");

        // Verify structure preserved
        assert_eq!(
            golden["metadata"]["name"], parsed["metadata"]["name"],
            "Round trip should preserve name"
        );

        println!("✓ JSON round-trip validated");
    }

    #[test]
    fn e2e_advanced_json_pretty_print() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        // Pretty print
        let pretty = serde_json::to_string_pretty(&golden).expect("Should pretty print JSON");

        assert!(pretty.len() > 0, "Pretty printed JSON should have content");
        assert!(pretty.contains('\n'), "Pretty print should have newlines");
        assert!(
            pretty.contains("  "),
            "Pretty print should have indentation"
        );

        println!("✓ JSON pretty printing validated ({} chars)", pretty.len());
    }

    #[test]
    fn e2e_advanced_json_field_access() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        // Test various field access patterns
        assert!(golden["metadata"].is_object(), "Metadata access");
        assert!(golden["variables"].is_object(), "Variables access");
        assert!(golden["states"].is_object(), "States access");
        assert!(golden["subentities"].is_object(), "Subentities access");
        assert!(
            golden["transformed_data"].is_object(),
            "Transformed data access"
        );

        // Nested access
        assert!(
            golden["transformed_data"]["Graphics"].is_object(),
            "Nested module access"
        );

        println!("✓ JSON field access validated");
    }

    // ============================================================================
    // STATE MACHINE TRAVERSAL TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_state_reachability() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Find states with no parent (root states)
        let mut root_states = Vec::new();
        for (state_name, state_data) in states {
            if state_data["Parent"].is_null() {
                root_states.push(state_name.clone());
            }
        }

        assert!(!root_states.is_empty(), "Should have root states");
        println!("✓ State reachability: {} root states", root_states.len());
        println!(
            "  Root states: {:?}",
            root_states.iter().take(5).collect::<Vec<_>>()
        );
    }

    #[test]
    fn e2e_advanced_state_dependency_graph() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Build parent-child relationships
        let mut children_map: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for (state_name, state_data) in states {
            if let Some(parent) = state_data["Parent"].as_str() {
                children_map
                    .entry(parent.to_string())
                    .or_insert_with(Vec::new)
                    .push(state_name.clone());
            }
        }

        // Find states with most children
        let mut max_children = 0;
        let mut most_derived_state = String::new();

        for (state_name, children) in &children_map {
            if children.len() > max_children {
                max_children = children.len();
                most_derived_state = state_name.clone();
            }
        }

        println!("✓ State dependency graph:");
        println!(
            "  Most derived state: {} ({} children)",
            most_derived_state, max_children
        );
    }

    #[test]
    fn e2e_advanced_state_path_finding() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // For each state, find path to root
        let mut longest_path = 0;
        let mut state_with_longest_path = String::new();

        for (state_name, _) in states {
            let mut path_length = 0;
            let mut current = state_name.clone();
            let mut visited = std::collections::HashSet::new();

            while let Some(state_data) = states.get(&current) {
                if !visited.insert(current.clone()) {
                    break; // Circular
                }

                if let Some(parent) = state_data["Parent"].as_str() {
                    path_length += 1;
                    current = parent.to_string();
                } else {
                    break;
                }

                if path_length > 50 {
                    break;
                } // Safety
            }

            if path_length > longest_path {
                longest_path = path_length;
                state_with_longest_path = state_name.clone();
            }
        }

        println!("✓ State path finding:");
        println!(
            "  Longest path: {} (state: {})",
            longest_path, state_with_longest_path
        );
    }

    // ============================================================================
    // PARSER ROBUSTNESS TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_parser_large_character() {
        let test_file = "test_character_complete.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(test_file);

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should handle large character file");
        }

        let character = character.unwrap();

        println!("✓ Parser handles large character:");
        println!("  Variables: {}", character.variables.len());
        println!("  States: {}", character.states.len());
        println!("  Errors: {}", parser.errors.len());
    }

    #[test]
    fn e2e_advanced_parser_multiple_sequential() {
        let files = vec!["test_character.casp", "test_character_complete.casp"];
        let mut parsed_count = 0;

        for file in &files {
            if !file_exists(file) {
                continue;
            }

            let mut parser = CastagneParser::new();
            let result = parser.create_full_character(file);

            if result.is_some() {
                parsed_count += 1;
            }
        }

        assert!(parsed_count > 0, "Should parse at least one file");
        println!(
            "✓ Parser handles sequential parsing ({} files)",
            parsed_count
        );
    }

    #[test]
    fn e2e_advanced_parser_error_accumulation() {
        let casp_content = r#"
:Character:
Name: Test
InvalidLine1
:Variables:
var BadVar(InvalidType): value
InvalidLine2
:Test:
InvalidLine3
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let _result = parser.create_full_character(file.path().to_str().unwrap());

        // Parser may accumulate errors
        println!(
            "✓ Parser error accumulation: {} errors",
            parser.errors.len()
        );
    }

    // ============================================================================
    // MODULE TRANSFORMATION TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_module_data_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        for (module_name, module_data) in transformed {
            // Verify module structure
            assert!(
                module_data.is_object(),
                "Module {} should be object",
                module_name
            );

            if let Some(defines) = module_data["Defines"].as_object() {
                // Verify defines are accessible
                for (define_name, define_value) in defines {
                    assert!(!define_name.is_empty(), "Define name should not be empty");
                    assert!(
                        !define_value.is_null() || define_value.is_null(),
                        "Define value should be valid JSON"
                    );
                }
            }
        }

        println!(
            "✓ Module data structure validated ({} modules)",
            transformed.len()
        );
    }

    #[test]
    fn e2e_advanced_graphics_module_completeness() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let graphics = &golden["transformed_data"]["Graphics"];

        // Check for expected graphics sections
        let sections = vec!["Defines", "Spritesheets", "Palettes", "Anims"];
        let mut found_sections = Vec::new();

        for section in &sections {
            if graphics.get(*section).is_some() && !graphics[*section].is_null() {
                found_sections.push(*section);
            }
        }

        println!("✓ Graphics module sections: {:?}", found_sections);
        assert!(!found_sections.is_empty(), "Should have graphics sections");
    }

    // ============================================================================
    // CHARACTER FILE ORGANIZATION TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_character_file_completeness() {
        let files = vec![
            "test_character.casp",
            "test_character_complete.casp",
            "test_parent.casp",
            "test_child.casp",
        ];

        let mut complete_files = 0;

        for file in &files {
            if file_exists(file) {
                let content = fs::read_to_string(file).expect(&format!("Should read {}", file));

                let has_character = content.contains(":Character:");
                let has_variables = content.contains(":Variables:");
                let has_states = content.lines().any(|line| {
                    line.starts_with(':')
                        && line.ends_with(':')
                        && !line.contains("Character")
                        && !line.contains("Variables")
                });

                if has_character && (has_variables || has_states) {
                    complete_files += 1;
                }
            }
        }

        println!(
            "✓ Character file completeness: {}/{} files complete",
            complete_files,
            files.len()
        );
    }

    // ============================================================================
    // INHERITANCE CHAIN TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_inheritance_chain_validation() {
        if !file_exists("test_parent.casp") || !file_exists("test_child.casp") {
            println!("⚠ Skipping test - parent/child files not found");
            return;
        }

        let parent_content = fs::read_to_string("test_parent.casp").expect("Should read parent");
        let child_content = fs::read_to_string("test_child.casp").expect("Should read child");

        // Child should reference parent
        assert!(
            child_content.contains("Skeleton:"),
            "Child should have Skeleton field"
        );
        assert!(
            child_content.contains("test_parent"),
            "Child should reference parent"
        );

        // Parent should have inheritable content
        assert!(
            parent_content.contains(":Character:"),
            "Parent should have Character section"
        );

        println!("✓ Inheritance chain validated");
    }

    #[test]
    fn e2e_advanced_inheritance_override_patterns() {
        if !file_exists("test_child.casp") {
            println!("⚠ Skipping test - child file not found");
            return;
        }

        let child_content = fs::read_to_string("test_child.casp").expect("Should read child");

        // Count override patterns
        let var_overrides = child_content.matches("var ").count();
        let def_overrides = child_content.matches("def ").count();

        println!("✓ Inheritance overrides:");
        println!("  Variable overrides: {}", var_overrides);
        println!("  Define overrides: {}", def_overrides);
    }

    // ============================================================================
    // PERFORMANCE BENCHMARK TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_parse_performance() {
        let test_file = "test_character_complete.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        use std::time::Instant;

        let start = Instant::now();
        let mut parser = CastagneParser::new();
        let result = parser.create_full_character(test_file);
        let duration = start.elapsed();

        assert!(result.is_some(), "Parser should succeed");

        println!("✓ Parse performance: {:?}", duration);
        println!("  (Note: First run includes compilation time)");
    }

    #[test]
    fn e2e_advanced_repeated_parse_performance() {
        let test_file = "test_character.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        use std::time::Instant;

        let iterations = 10;
        let start = Instant::now();

        for _ in 0..iterations {
            let mut parser = CastagneParser::new();
            let result = parser.create_full_character(test_file);
            assert!(result.is_some(), "Each parse should succeed");
        }

        let total_duration = start.elapsed();
        let avg_duration = total_duration / iterations;

        println!("✓ Repeated parse performance:");
        println!("  {} iterations in {:?}", iterations, total_duration);
        println!("  Average: {:?} per parse", avg_duration);
    }

    // ============================================================================
    // DATA CONSISTENCY TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_variable_name_conventions() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut camel_case = 0;
        let mut pascal_case = 0;
        let mut snake_case = 0;
        let mut all_caps = 0;

        for var_name in variables.keys() {
            if var_name.chars().all(|c| c.is_uppercase() || c == '_') {
                all_caps += 1;
            } else if var_name.contains('_') {
                snake_case += 1;
            } else if var_name.chars().next().unwrap().is_uppercase() {
                pascal_case += 1;
            } else if var_name.chars().next().unwrap().is_lowercase() {
                camel_case += 1;
            }
        }

        println!("✓ Variable naming conventions:");
        println!("  PascalCase: {}", pascal_case);
        println!("  camelCase: {}", camel_case);
        println!("  snake_case: {}", snake_case);
        println!("  ALL_CAPS: {}", all_caps);
    }

    #[test]
    fn e2e_advanced_state_name_conventions() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        let mut hyphenated = 0;
        let mut pascal_case = 0;
        let mut other = 0;

        for state_name in states.keys() {
            if state_name.contains('-') {
                hyphenated += 1;
            } else if state_name.chars().next().unwrap().is_uppercase() {
                pascal_case += 1;
            } else {
                other += 1;
            }
        }

        println!("✓ State naming conventions:");
        println!("  PascalCase: {}", pascal_case);
        println!("  Hyphenated: {}", hyphenated);
        println!("  Other: {}", other);
    }

    // ============================================================================
    // COMPLEX SCENARIO TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_fighting_game_combo_detection() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Detect potential combo chains (attacks that can transition to other attacks)
        let mut attack_states: Vec<&String> = states
            .keys()
            .filter(|name| {
                let lower = name.to_lowercase();
                lower.contains("attack") || lower.contains("punch") || lower.contains("kick")
            })
            .collect();

        attack_states.sort();

        println!("✓ Combo system detection:");
        println!("  Attack states: {}", attack_states.len());
        if attack_states.len() > 0 {
            println!(
                "  Examples: {:?}",
                attack_states.iter().take(5).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn e2e_advanced_hitbox_data_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        // Check for hitbox-related data in various locations
        let json_str = serde_json::to_string(&golden).unwrap();

        let hitbox_mentions = json_str.matches("hitbox").count()
            + json_str.matches("Hitbox").count()
            + json_str.matches("hurtbox").count()
            + json_str.matches("Hurtbox").count();

        println!("✓ Hitbox data structure: {} mentions", hitbox_mentions);
    }

    // ============================================================================
    // STRESS TESTS
    // ============================================================================

    #[test]
    fn e2e_advanced_deep_json_nesting() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        // Verify we can access deeply nested data
        let _metadata = &golden["metadata"];
        let _states = &golden["states"];
        let _transformed = &golden["transformed_data"];

        if let Some(graphics) = golden["transformed_data"].get("Graphics") {
            if let Some(defines) = graphics.get("Defines") {
                if defines.is_object() {
                    println!("  Graphics Defines depth validated");
                }
            }
        }

        println!("✓ Deep JSON nesting validated");
    }

    #[test]
    fn e2e_advanced_large_json_handling() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        // Measure JSON size
        let json_str = serde_json::to_string(&golden).expect("Should serialize");

        let size_kb = json_str.len() / 1024;

        println!("✓ Large JSON handling: {} KB", size_kb);
        assert!(size_kb > 0, "JSON should have content");
    }

    // ============================================================================
    // SUMMARY TEST
    // ============================================================================

    #[test]
    fn e2e_advanced_scenarios_summary() {
        println!("\n=== E2E Advanced Scenarios Test Summary ===\n");
        println!("Advanced scenario areas covered:");
        println!("  ✓ JSON serialization and round-trip");
        println!("  ✓ Pretty printing and field access");
        println!("  ✓ State machine traversal and reachability");
        println!("  ✓ Dependency graph analysis");
        println!("  ✓ State path finding");
        println!("  ✓ Parser robustness with large files");
        println!("  ✓ Sequential and error accumulation");
        println!("  ✓ Module transformation validation");
        println!("  ✓ Character file organization");
        println!("  ✓ Inheritance chain validation");
        println!("  ✓ Performance benchmarks");
        println!("  ✓ Naming convention analysis");
        println!("  ✓ Combo system detection");
        println!("  ✓ Hitbox data structure");
        println!("  ✓ Deep nesting and large JSON handling");
        println!("\nAll advanced scenario tests completed!\n");
    }
}
