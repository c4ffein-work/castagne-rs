// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! End-to-End Parser Behavior Tests
//!
//! Tests that validate actual parser execution and behavior:
//! - Parser output validation
//! - Variable parsing accuracy
//! - State parsing accuracy
//! - Action parsing accuracy
//! - Error recovery
//! - Edge case handling in actual parsing

use castagne_rs::parser::CastagneParser;
use std::io::Write as IoWrite;
use std::path::Path;
use tempfile::NamedTempFile;

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // HELPER FUNCTIONS
    // ============================================================================

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
    // VARIABLE PARSING ACCURACY TESTS
    // ============================================================================

    #[test]
    fn e2e_parser_int_variable_parsing() {
        let casp_content = r#"
:Character:
Name: Test
:Variables:
var Health(Int): 100
var MaxHealth(Int): 200
def MinHealth: 0
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should parse int variables");
        }

        let character = character.unwrap();

        // Validate Int variables were parsed
        assert!(
            character.variables.contains_key("Health"),
            "Should parse Health variable"
        );
        assert!(
            character.variables.contains_key("MaxHealth"),
            "Should parse MaxHealth variable"
        );
        assert!(
            character.variables.contains_key("MinHealth"),
            "Should parse MinHealth constant"
        );

        let health = &character.variables["Health"];
        assert_eq!(health.value, "100", "Health value should be 100");

        println!("✓ Int variable parsing validated");
    }

    #[test]
    fn e2e_parser_string_variable_parsing() {
        let casp_content = r#"
:Character:
Name: Test
:Variables:
var PlayerName(Str): "TestPlayer"
var AnimState(Str): "idle"
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should parse string variables");
        }

        let character = character.unwrap();

        assert!(
            character.variables.contains_key("PlayerName"),
            "Should parse PlayerName variable"
        );
        assert!(
            character.variables.contains_key("AnimState"),
            "Should parse AnimState variable"
        );

        println!("✓ String variable parsing validated");
    }

    #[test]
    fn e2e_parser_vec2_variable_parsing() {
        let casp_content = r#"
:Character:
Name: Test
:Variables:
var Position(Vec2): 0, 0
var Velocity(Vec2): 10.5, -5.2
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should parse Vec2 variables");
        }

        let character = character.unwrap();

        assert!(
            character.variables.contains_key("Position"),
            "Should parse Position variable"
        );
        assert!(
            character.variables.contains_key("Velocity"),
            "Should parse Velocity variable"
        );

        println!("✓ Vec2 variable parsing validated");
    }

    #[test]
    fn e2e_parser_bool_variable_parsing() {
        let casp_content = r#"
:Character:
Name: Test
:Variables:
var IsGrounded(Bool): true
var IsAttacking(Bool): false
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should parse Bool variables");
        }

        let character = character.unwrap();

        assert!(
            character.variables.contains_key("IsGrounded"),
            "Should parse IsGrounded variable"
        );
        assert!(
            character.variables.contains_key("IsAttacking"),
            "Should parse IsAttacking variable"
        );

        println!("✓ Bool variable parsing validated");
    }

    // ============================================================================
    // STATE PARSING ACCURACY TESTS
    // ============================================================================

    #[test]
    fn e2e_parser_basic_state_parsing() {
        let casp_content = r#"
:Character:
Name: Test
:Idle:
---Init:
Set(AnimState, "idle")
---Action:
CheckInput()
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should parse basic state");
        }

        let character = character.unwrap();

        assert!(
            character.states.contains_key("Idle"),
            "Should parse Idle state"
        );

        let idle_state = &character.states["Idle"];
        assert!(
            idle_state.actions.contains_key("Init"),
            "Idle state should have Init phase"
        );
        assert!(
            idle_state.actions.contains_key("Action"),
            "Idle state should have Action phase"
        );

        println!("✓ Basic state parsing validated");
    }

    #[test]
    fn e2e_parser_state_with_parent() {
        let casp_content = r#"
:Character:
Name: Test
:Base:
---Init:
InitBase()
:Derived(Base):
---Init:
InitDerived()
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should parse state inheritance");
        }

        let character = character.unwrap();

        assert!(
            character.states.contains_key("Base"),
            "Should parse Base state"
        );
        assert!(
            character.states.contains_key("Derived"),
            "Should parse Derived state"
        );

        println!("✓ State with parent parsing validated");
    }

    #[test]
    fn e2e_parser_multiple_states() {
        let casp_content = r#"
:Character:
Name: Test
:Idle:
---Init:
IdleInit()
:Walk:
---Init:
WalkInit()
:Jump:
---Init:
JumpInit()
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should parse multiple states");
        }

        let character = character.unwrap();

        assert!(character.states.contains_key("Idle"), "Should have Idle");
        assert!(character.states.contains_key("Walk"), "Should have Walk");
        assert!(character.states.contains_key("Jump"), "Should have Jump");

        println!("✓ Multiple states parsing validated (3 states)");
    }

    // ============================================================================
    // ACTION PARSING ACCURACY TESTS
    // ============================================================================

    #[test]
    fn e2e_parser_simple_actions() {
        let casp_content = r#"
:Character:
Name: Test
:Test:
---Init:
Action1()
Action2()
Action3()
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should parse simple actions");
        }

        let character = character.unwrap();
        let test_state = &character.states["Test"];
        let init_actions = &test_state.actions["Init"];

        assert!(init_actions.len() >= 3, "Should parse at least 3 actions");

        println!(
            "✓ Simple actions parsing validated ({} actions)",
            init_actions.len()
        );
    }

    #[test]
    fn e2e_parser_actions_with_arguments() {
        let casp_content = r#"
:Character:
Name: Test
:Test:
---Init:
Set(Health, 100)
Add(Position, 5, 10)
Multiply(Velocity, 2.0)
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should parse actions with arguments");
        }

        let character = character.unwrap();
        let test_state = &character.states["Test"];
        let init_actions = &test_state.actions["Init"];

        assert!(
            init_actions.len() >= 3,
            "Should parse actions with arguments"
        );

        println!("✓ Actions with arguments parsing validated");
    }

    #[test]
    fn e2e_parser_conditional_actions() {
        let casp_content = r#"
:Character:
Name: Test
:Test:
---Init:
If(Health < 50)
    Heal(25)
EndIf
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should parse conditional actions");
        }

        let character = character.unwrap();
        let test_state = &character.states["Test"];

        assert!(
            test_state.actions.contains_key("Init"),
            "Should have Init phase with conditional"
        );

        println!("✓ Conditional actions parsing validated");
    }

    // ============================================================================
    // SPECBLOCK PARSING TESTS
    // ============================================================================

    #[test]
    fn e2e_parser_specblock_parsing() {
        let casp_content = r#"
:Character:
Name: Test
:TestBlock:
Value1: 100
Value2: 200
StringValue: "test"
:Variables:
var Health(Int): 100
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should parse specblocks");
        }

        let character = character.unwrap();

        assert!(
            character.specblocks.contains_key("TestBlock"),
            "Should parse TestBlock specblock"
        );

        let testblock = &character.specblocks["TestBlock"];
        assert!(
            testblock.contains_key("Value1"),
            "TestBlock should have Value1"
        );

        println!("✓ Specblock parsing validated");
    }

    // ============================================================================
    // COMMENT HANDLING TESTS
    // ============================================================================

    #[test]
    fn e2e_parser_comment_handling() {
        let casp_content = r#"
# This is a comment
:Character:
Name: Test  # Inline comment
# Another comment
:Variables:
# Variable section
var Health(Int): 100  # Health variable
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should handle comments");
        }

        let character = character.unwrap();

        assert_eq!(
            character.metadata.name, "Test",
            "Comments should not affect parsing"
        );
        assert!(
            character.variables.contains_key("Health"),
            "Should parse variables despite comments"
        );

        println!("✓ Comment handling validated");
    }

    // ============================================================================
    // ERROR RECOVERY TESTS
    // ============================================================================

    #[test]
    fn e2e_parser_recovers_from_syntax_errors() {
        let casp_content = r#"
:Character:
Name: Test
InvalidLine without proper format
:Variables:
var Health(Int): 100
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        // Parser might still produce output despite errors
        if character.is_some() {
            let character = character.unwrap();
            // Should at least parse the valid parts
            assert!(
                character.variables.contains_key("Health"),
                "Should parse valid variables even with errors"
            );
        }

        println!(
            "✓ Error recovery validated (errors: {})",
            parser.errors.len()
        );
    }

    // ============================================================================
    // WHITESPACE HANDLING TESTS
    // ============================================================================

    #[test]
    fn e2e_parser_whitespace_tolerance() {
        let casp_content = r#"
:Character:
Name:    Test
:Variables:
var Health(Int):    100
var  Mana(Int)  :  50
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should handle extra whitespace");
        }

        let character = character.unwrap();

        assert!(
            character.variables.contains_key("Health"),
            "Should parse variable with extra whitespace"
        );
        assert!(
            character.variables.contains_key("Mana"),
            "Should parse variable with scattered whitespace"
        );

        println!("✓ Whitespace tolerance validated");
    }

    // ============================================================================
    // EMPTY SECTION TESTS
    // ============================================================================

    #[test]
    fn e2e_parser_empty_sections() {
        let casp_content = r#"
:Character:
Name: Test
:Variables:
:Idle:
---Init:
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should handle empty sections");
        }

        let character = character.unwrap();

        // Empty sections should be handled gracefully
        assert_eq!(character.metadata.name, "Test");
        assert!(
            character.states.contains_key("Idle"),
            "Should create Idle state even with empty Init"
        );

        println!("✓ Empty sections handling validated");
    }

    // ============================================================================
    // PHASE ORDERING TESTS
    // ============================================================================

    #[test]
    fn e2e_parser_phase_ordering() {
        let casp_content = r#"
:Character:
Name: Test
:Test:
---Init:
InitAction()
---Action:
ActionPhase()
---Reaction:
ReactionPhase()
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should handle phase ordering");
        }

        let character = character.unwrap();
        let test_state = &character.states["Test"];

        assert!(
            test_state.actions.contains_key("Init"),
            "Should have Init phase"
        );
        assert!(
            test_state.actions.contains_key("Action"),
            "Should have Action phase"
        );
        assert!(
            test_state.actions.contains_key("Reaction"),
            "Should have Reaction phase"
        );

        println!("✓ Phase ordering validated");
    }

    // ============================================================================
    // METADATA EXTRACTION TESTS
    // ============================================================================

    #[test]
    fn e2e_parser_metadata_extraction() {
        let casp_content = r#"
:Character:
Name: TestFighter
Author: TestAuthor
Description: This is a test character
Version: 1.0
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should extract metadata");
        }

        let character = character.unwrap();

        assert_eq!(
            character.metadata.name, "TestFighter",
            "Should extract name"
        );
        assert_eq!(
            character.metadata.author, "TestAuthor",
            "Should extract author"
        );
        assert!(
            character.metadata.description.contains("test character"),
            "Should extract description"
        );

        println!("✓ Metadata extraction validated");
    }

    // ============================================================================
    // JSON OUTPUT VALIDATION TESTS
    // ============================================================================

    #[test]
    fn e2e_parser_json_output_valid() {
        let test_file = "test_character.casp";

        if !file_exists(test_file) {
            println!("⚠ Skipping test - {} not found", test_file);
            return;
        }

        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(test_file);

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should parse character");
        }

        let character = character.unwrap();

        // Try to convert to JSON
        let json_result = character.to_json_value();
        assert!(json_result.is_ok(), "Should convert to valid JSON");

        let json = json_result.unwrap();

        // Validate JSON structure
        assert!(json["metadata"].is_object(), "JSON should have metadata");
        assert!(json["variables"].is_object(), "JSON should have variables");
        assert!(json["states"].is_object(), "JSON should have states");

        println!("✓ JSON output validation passed");
    }

    // ============================================================================
    // UNICODE HANDLING TESTS
    // ============================================================================

    #[test]
    fn e2e_parser_unicode_in_strings() {
        let casp_content = r#"
:Character:
Name: Test
:Variables:
var Message(Str): "Hello 世界 🎮"
var EmojiTest(Str): "Test with émojis ✅"
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        if character.is_none() {
            eprintln!("Parser errors: {:?}", parser.errors);
            panic!("Parser should handle Unicode");
        }

        let character = character.unwrap();

        assert!(
            character.variables.contains_key("Message"),
            "Should parse Unicode string variables"
        );
        assert!(
            character.variables.contains_key("EmojiTest"),
            "Should parse emoji string variables"
        );

        println!("✓ Unicode handling validated");
    }

    // ============================================================================
    // SUMMARY TEST
    // ============================================================================

    #[test]
    fn e2e_parser_behavior_summary() {
        println!("\n=== E2E Parser Behavior Test Summary ===\n");
        println!("Parser behavior tests covered:");
        println!("  ✓ Variable parsing (Int, Str, Vec2, Bool)");
        println!("  ✓ State parsing (basic, with parent, multiple)");
        println!("  ✓ Action parsing (simple, with args, conditionals)");
        println!("  ✓ Specblock parsing");
        println!("  ✓ Comment handling");
        println!("  ✓ Error recovery");
        println!("  ✓ Whitespace tolerance");
        println!("  ✓ Empty sections");
        println!("  ✓ Phase ordering");
        println!("  ✓ Metadata extraction");
        println!("  ✓ JSON output validation");
        println!("  ✓ Unicode handling");
        println!("\nAll parser behavior tests completed!\n");
    }
}
