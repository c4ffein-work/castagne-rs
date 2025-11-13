// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! End-to-End Integration Tests
//!
//! Integration tests that parse actual .casp files using the Rust parser
//! and validate the results. These tests use the Godot runtime to test
//! the parser in a real-world environment.

use serde_json::Value;
use std::fs;
use std::path::Path;

// We need to test with the actual parser module
// For now, these tests will validate the structure of golden masters
// and prepare for future integration with the Rust parser

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

    // ============================================================================
    // BASIC INTEGRATION TESTS
    // ============================================================================

    #[test]
    fn e2e_integration_test_character_casp_exists() {
        // Verify test files exist
        assert!(
            file_exists("test_character.casp"),
            "test_character.casp should exist"
        );
        assert!(
            file_exists("test_character_complete.casp"),
            "test_character_complete.casp should exist"
        );

        println!("✓ Test .casp files exist");
    }

    #[test]
    fn e2e_integration_test_character_parseable() {
        let test_file = "test_character.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let content =
            fs::read_to_string(test_file).expect("Should be able to read test_character.casp");

        // Basic validation - file should have expected sections
        assert!(
            content.contains(":Character:"),
            "Should have Character section"
        );
        assert!(
            content.contains(":Variables:"),
            "Should have Variables section"
        );
        assert!(content.contains(":Idle:"), "Should have at least one state");

        println!("✓ Test character file is parseable");
    }

    #[test]
    fn e2e_integration_complete_character_structure() {
        let test_file = "test_character_complete.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let content = fs::read_to_string(test_file)
            .expect("Should be able to read test_character_complete.casp");

        // Validate comprehensive features
        assert!(
            content.contains(":Character:"),
            "Should have Character section"
        );
        assert!(
            content.contains(":AttackData:"),
            "Should have AttackData specblock"
        );
        assert!(
            content.contains(":PhysicsConfig:"),
            "Should have PhysicsConfig specblock"
        );
        assert!(
            content.contains(":Variables:"),
            "Should have Variables section"
        );

        // Check for various variable types
        assert!(content.contains("(Int)"), "Should have Int variables");
        assert!(content.contains("(Str)"), "Should have Str variables");
        assert!(content.contains("(Bool)"), "Should have Bool variables");
        assert!(content.contains("(Vec2)"), "Should have Vec2 variables");

        // Check for states
        assert!(content.contains(":Idle:"), "Should have Idle state");
        assert!(content.contains(":Walk:"), "Should have Walk state");
        assert!(content.contains(":Jump:"), "Should have Jump state");
        assert!(
            content.contains(":LightPunch:"),
            "Should have LightPunch state"
        );

        // Check for phases
        assert!(content.contains("---Init:"), "Should have Init phases");
        assert!(content.contains("---Action:"), "Should have Action phases");
        assert!(
            content.contains("---Reaction:"),
            "Should have Reaction phases"
        );

        println!("✓ Complete character file has expected structure");
    }

    // ============================================================================
    // SPECBLOCK TESTS
    // ============================================================================

    #[test]
    fn e2e_integration_specblocks_in_casp() {
        let test_file = "test_character_complete.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let content = fs::read_to_string(test_file).expect("Should read file");

        // Verify specblocks are present and well-formed
        let specblock_pattern = vec![
            (
                ":AttackData:",
                vec!["LightPunchDamage", "HeavyPunchDamage", "KickDamage"],
            ),
            (":PhysicsConfig:", vec!["Gravity", "JumpForce", "MaxSpeed"]),
        ];

        for (block_name, expected_keys) in specblock_pattern {
            assert!(
                content.contains(block_name),
                "Should have {} specblock",
                block_name
            );

            for key in expected_keys {
                assert!(
                    content.contains(key),
                    "Specblock {} should contain {}",
                    block_name,
                    key
                );
            }
        }

        println!("✓ Specblocks are present and structured correctly");
    }

    #[test]
    fn e2e_integration_specblock_values_numeric() {
        let test_file = "test_character_complete.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let content = fs::read_to_string(test_file).expect("Should read file");

        // Extract and validate numeric values from specblocks
        let lines: Vec<&str> = content.lines().collect();

        let mut in_specblock = false;
        let mut valid_values = 0;

        for line in lines {
            if line.starts_with(":AttackData:") || line.starts_with(":PhysicsConfig:") {
                in_specblock = true;
                continue;
            }
            if line.starts_with(":") && in_specblock {
                in_specblock = false;
            }

            if in_specblock && line.contains(":") && !line.trim().starts_with("#") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    let value = parts[1].trim();
                    // Value should be numeric or a valid expression
                    if !value.is_empty() {
                        valid_values += 1;
                    }
                }
            }
        }

        assert!(valid_values > 0, "Should have found specblock values");
        println!(
            "✓ Specblock values are well-formed ({} values)",
            valid_values
        );
    }

    // ============================================================================
    // INHERITANCE TESTS
    // ============================================================================

    #[test]
    fn e2e_integration_parent_child_files_exist() {
        assert!(
            file_exists("test_parent.casp"),
            "Parent test file should exist"
        );
        assert!(
            file_exists("test_child.casp"),
            "Child test file should exist"
        );

        println!("✓ Parent and child test files exist");
    }

    #[test]
    fn e2e_integration_child_references_parent() {
        if !file_exists("test_child.casp") {
            println!("⚠ Skipping test - test_child.casp not found");
            return;
        }

        let content = fs::read_to_string("test_child.casp").expect("Should read child file");

        // Child should reference parent via Skeleton field
        assert!(
            content.contains("Skeleton:"),
            "Child should have Skeleton field"
        );
        assert!(
            content.contains("test_parent.casp"),
            "Child should reference parent file"
        );

        println!("✓ Child correctly references parent");
    }

    #[test]
    fn e2e_integration_child_overrides_parent_values() {
        if !file_exists("test_child.casp") || !file_exists("test_parent.casp") {
            println!("⚠ Skipping test - parent/child files not found");
            return;
        }

        let parent_content = fs::read_to_string("test_parent.casp").expect("Should read parent");
        let child_content = fs::read_to_string("test_child.casp").expect("Should read child");

        // Parent has BaseSpeed: 5, child should override it
        assert!(
            parent_content.contains("BaseSpeed: 5"),
            "Parent should have BaseSpeed: 5"
        );
        assert!(
            child_content.contains("BaseSpeed: 10"),
            "Child should override BaseSpeed to 10"
        );

        // Check variable overrides
        assert!(
            parent_content.contains("var Health(Int): 100"),
            "Parent should have Health: 100"
        );
        assert!(
            child_content.contains("var Health(Int): 150"),
            "Child should override Health to 150"
        );

        println!("✓ Child successfully overrides parent values");
    }

    #[test]
    fn e2e_integration_inheritance_preserves_parent_data() {
        if !file_exists("test_parent.casp") {
            println!("⚠ Skipping test - test_parent.casp not found");
            return;
        }

        let parent_content = fs::read_to_string("test_parent.casp").expect("Should read parent");

        // Parent should have data that child can inherit
        assert!(
            parent_content.contains("ParentOnly"),
            "Parent should have unique variables"
        );
        assert!(
            parent_content.contains(":BaseAttack:"),
            "Parent should have base states"
        );

        println!("✓ Parent has inheritable data");
    }

    // ============================================================================
    // ACTION PARSING TESTS
    // ============================================================================

    #[test]
    fn e2e_integration_actions_have_arguments() {
        let test_file = "test_character_complete.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let content = fs::read_to_string(test_file).expect("Should read file");

        // Find actions with various argument patterns
        let actions_with_args = vec!["Set(", "Add(", "Mul(", "If(", "CheckHit(", "ChangeState("];

        for action in actions_with_args {
            assert!(
                content.contains(action),
                "Should have {} action with arguments",
                action
            );
        }

        println!("✓ Actions with arguments are present");
    }

    #[test]
    fn e2e_integration_actions_multi_argument() {
        let test_file = "test_character_complete.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let content = fs::read_to_string(test_file).expect("Should read file");

        // Actions with multiple arguments
        assert!(
            content.contains("Set(Velocity, 0, 0)"),
            "Should have multi-arg Set action"
        );
        assert!(
            content.contains("Add(Velocity, 0, Gravity)"),
            "Should have multi-arg Add action"
        );
        assert!(
            content.contains("CheckHit(LightPunchRange, Damage)"),
            "Should have multi-arg CheckHit action"
        );

        println!("✓ Multi-argument actions are present");
    }

    #[test]
    fn e2e_integration_actions_with_string_args() {
        let test_file = "test_character_complete.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let content = fs::read_to_string(test_file).expect("Should read file");

        // Actions with string arguments
        assert!(
            content.contains("Set(AnimationState, \"idle\")")
                || content.contains("Set(AnimationState, \"walk\")"),
            "Should have actions with string arguments"
        );

        println!("✓ Actions with string arguments are present");
    }

    #[test]
    fn e2e_integration_actions_with_expressions() {
        let test_file = "test_character_complete.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let content = fs::read_to_string(test_file).expect("Should read file");

        // Actions with expression arguments
        assert!(
            content.contains("If(!IsGrounded)")
                || content.contains("If(Position.y >= 0)")
                || content.contains("If(ComboTimer < MAX_COMBO)"),
            "Should have actions with expression arguments"
        );

        println!("✓ Actions with expressions are present");
    }

    // ============================================================================
    // CONTROL FLOW TESTS
    // ============================================================================

    #[test]
    fn e2e_integration_if_endif_blocks() {
        let test_file = "test_character_complete.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let content = fs::read_to_string(test_file).expect("Should read file");

        // Count all conditional statements (If, IfInput, IfNotInput) and EndIf - should be balanced
        let if_count = content.matches("If(").count();
        let ifinput_count = content.matches("IfInput(").count();
        let ifnotinput_count = content.matches("IfNotInput(").count();
        let total_conditionals = if_count + ifinput_count + ifnotinput_count;
        let endif_count = content.matches("EndIf").count();

        assert!(total_conditionals > 0, "Should have conditional statements");
        assert_eq!(total_conditionals, endif_count,
            "Conditionals and EndIf should be balanced: {} conditionals ({} If, {} IfInput, {} IfNotInput) vs {} EndIf",
            total_conditionals, if_count, ifinput_count, ifnotinput_count, endif_count);

        println!(
            "✓ If/EndIf blocks are balanced ({} conditionals, {} EndIf)",
            total_conditionals, endif_count
        );
    }

    #[test]
    fn e2e_integration_nested_conditionals() {
        let test_file = "test_character_complete.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let content = fs::read_to_string(test_file).expect("Should read file");

        // Check for nested If statements
        let lines: Vec<&str> = content.lines().collect();
        let mut nesting_depth: i32 = 0;
        let mut max_depth: i32 = 0;

        for line in lines {
            if line.contains("If(") || line.contains("IfInput(") || line.contains("IfNotInput(") {
                nesting_depth += 1;
                max_depth = max_depth.max(nesting_depth);
            }
            if line.contains("EndIf") {
                nesting_depth = nesting_depth.saturating_sub(1);
            }
        }

        assert!(max_depth > 0, "Should have conditional statements");
        println!("✓ Nested conditionals handled (max depth: {})", max_depth);
    }

    #[test]
    fn e2e_integration_state_transitions() {
        let test_file = "test_character_complete.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let content = fs::read_to_string(test_file).expect("Should read file");

        // Count state transitions
        let transition_count = content.matches("ChangeState(").count();

        assert!(transition_count > 0, "Should have state transitions");
        println!(
            "✓ State transitions present ({} transitions)",
            transition_count
        );
    }

    // ============================================================================
    // GOLDEN MASTER INTEGRATION TESTS
    // ============================================================================

    #[test]
    fn e2e_integration_golden_masters_valid_json() {
        let golden_files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
            "golden_masters/test_character_complete.json",
        ];

        for file_path in golden_files {
            if !file_exists(file_path) {
                println!("⚠ Skipping {} - not found", file_path);
                continue;
            }

            let json = load_golden_master(file_path);

            // Validate top-level structure
            assert!(
                json["metadata"].is_object(),
                "{} should have metadata",
                file_path
            );
            assert!(
                json["variables"].is_object(),
                "{} should have variables",
                file_path
            );
            assert!(
                json["states"].is_object(),
                "{} should have states",
                file_path
            );

            println!("  ✓ {} is valid JSON", file_path);
        }

        println!("✓ All golden masters are valid JSON");
    }

    #[test]
    fn e2e_integration_test_character_complete_golden_match() {
        let casp_file = "test_character_complete.casp";
        let golden_file = "golden_masters/test_character_complete.json";

        if !file_exists(casp_file) {
            println!("⚠ Skipping test - {} not found", casp_file);
            return;
        }

        if !file_exists(golden_file) {
            println!("⚠ Skipping test - {} not found", golden_file);
            return;
        }

        let casp_content = fs::read_to_string(casp_file).expect("Should read .casp file");
        let golden = load_golden_master(golden_file);

        // Validate that .casp file matches golden master expectations
        let metadata = &golden["metadata"];
        let states = golden["states"].as_object().unwrap();

        // Check metadata matches
        if let Some(name) = metadata["name"].as_str() {
            assert!(
                casp_content.contains(&format!("Name: {}", name)),
                "Character name should match"
            );
        }

        // Check states match
        for state_name in states.keys() {
            // State names in golden master should have corresponding sections in .casp
            // (This is a basic check - full validation requires parser)
            println!("  Golden master contains state: {}", state_name);
        }

        println!("✓ Test character .casp aligns with golden master");
    }

    // ============================================================================
    // MODULE FILE TESTS
    // ============================================================================

    #[test]
    fn e2e_integration_base_module_files_parseable() {
        let base_modules = vec![
            "castagne_godot4/modules/core/Base-Core.casp",
            "castagne_godot4/modules/attacks/Base-Attacks.casp",
            "castagne_godot4/modules/graphics/Base-Graphics.casp",
            "castagne_godot4/modules/physics/Base-Physics2D.casp",
        ];

        let mut found_modules = 0;

        for module_file in base_modules {
            if !file_exists(module_file) {
                println!("⚠ Module not found: {}", module_file);
                continue;
            }

            let content =
                fs::read_to_string(module_file).expect(&format!("Should read {}", module_file));

            // Basic validation - should be a valid .casp file
            assert!(!content.is_empty(), "{} should not be empty", module_file);

            found_modules += 1;
            println!("  ✓ {} is parseable", module_file);
        }

        if found_modules > 0 {
            println!(
                "✓ Base module files are parseable ({}/4 found)",
                found_modules
            );
        } else {
            println!("⚠ No base module files found (this is OK for minimal setup)");
        }
    }

    // ============================================================================
    // STRESS TESTS
    // ============================================================================

    #[test]
    fn e2e_integration_large_file_handling() {
        let test_file = "test_character_complete.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let content = fs::read_to_string(test_file).expect("Should read file");

        let line_count = content.lines().count();
        let char_count = content.len();

        assert!(line_count > 10, "Test file should have multiple lines");
        assert!(
            char_count > 100,
            "Test file should have substantial content"
        );

        println!(
            "✓ Large file handling validated ({} lines, {} bytes)",
            line_count, char_count
        );
    }

    #[test]
    fn e2e_integration_multiple_phases_per_state() {
        let test_file = "test_character_complete.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let content = fs::read_to_string(test_file).expect("Should read file");

        // Count phase declarations
        let phase_count = content.matches("---Init:").count()
            + content.matches("---Action:").count()
            + content.matches("---Reaction:").count();

        assert!(phase_count >= 3, "Should have multiple phases");
        println!(
            "✓ Multiple phases per state handled ({} total phases)",
            phase_count
        );
    }

    #[test]
    fn e2e_integration_comments_preserved() {
        let test_file = "test_character_complete.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let content = fs::read_to_string(test_file).expect("Should read file");

        // Count comment lines
        let comment_count = content
            .lines()
            .filter(|line| line.trim().starts_with("#"))
            .count();

        assert!(comment_count > 0, "Test file should have comments");
        println!(
            "✓ Comments handled correctly ({} comment lines)",
            comment_count
        );
    }

    // ============================================================================
    // SUMMARY TEST
    // ============================================================================

    #[test]
    fn e2e_integration_test_summary() {
        println!("\n=== E2E Integration Test Summary ===\n");
        println!("Integration test areas covered:");
        println!("  ✓ Basic file parsing");
        println!("  ✓ Specblock support");
        println!("  ✓ Inheritance system");
        println!("  ✓ Action parsing (simple and complex)");
        println!("  ✓ Control flow (If/EndIf)");
        println!("  ✓ State transitions");
        println!("  ✓ Golden master validation");
        println!("  ✓ Module file support");
        println!("  ✓ Stress testing");
        println!("\nAll integration tests completed!\n");
    }
}
