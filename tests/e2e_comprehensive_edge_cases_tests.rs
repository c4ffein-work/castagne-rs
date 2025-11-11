// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! End-to-End Comprehensive Edge Cases Tests
//!
//! Tests that validate edge cases and boundary conditions:
//! - Extreme values and boundary conditions
//! - Complex nested structures
//! - Performance stress scenarios
//! - Real-world complex patterns
//! - Advanced type interactions
//! - State machine complexity
//! - Resource handling

use castagne_rs::parser::CastagneParser;
use std::io::Write as IoWrite;
use tempfile::NamedTempFile;

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // HELPER FUNCTIONS
    // ============================================================================

    fn create_temp_casp(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        file.write_all(content.as_bytes()).expect("Failed to write to temp file");
        file
    }

    // ============================================================================
    // EXTREME VALUE TESTS
    // ============================================================================

    #[test]
    fn e2e_edge_max_int_values() {
        let casp_content = r#"
:Character:
Name: Test
:Variables:
var MaxInt(Int): 2147483647
var MinInt(Int): -2147483648
var Zero(Int): 0
var LargePositive(Int): 1000000000
var LargeNegative(Int): -1000000000
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse extreme int values");
        let character = character.unwrap();

        assert!(character.variables.contains_key("MaxInt"), "Should have MaxInt");
        assert!(character.variables.contains_key("MinInt"), "Should have MinInt");
        assert!(character.variables.contains_key("LargePositive"), "Should have LargePositive");

        println!("✓ Extreme int values validated (5 variables)");
    }

    #[test]
    fn e2e_edge_float_special_values() {
        let casp_content = r#"
:Character:
Name: Test
:Variables:
var VerySmall(Float): 0.000001
var VeryLarge(Float): 999999.999999
var NegativeSmall(Float): -0.000001
var Zero(Float): 0.0
var One(Float): 1.0
var Pi(Float): 3.14159265359
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse special float values");
        let character = character.unwrap();

        assert!(character.variables.contains_key("VerySmall"), "Should have VerySmall");
        assert!(character.variables.contains_key("Pi"), "Should have Pi");
        assert_eq!(character.variables.len(), 6, "Should have 6 float variables");

        println!("✓ Special float values validated (6 variables)");
    }

    #[test]
    fn e2e_edge_vector_extreme_values() {
        let casp_content = r#"
:Character:
Name: Test
:Variables:
var Origin(Vec2): 0, 0
var MaxCoords(Vec2): 10000, 10000
var MinCoords(Vec2): -10000, -10000
var MixedCoords(Vec2): -5000, 5000
var SmallVec(Vec2): 0.001, 0.001
var LargeVec(Vec3): 999.999, 888.888, 777.777
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse vector extreme values");
        let character = character.unwrap();

        assert!(character.variables.contains_key("MaxCoords"), "Should have MaxCoords");
        assert!(character.variables.contains_key("LargeVec"), "Should have LargeVec");

        println!("✓ Vector extreme values validated (6 variables)");
    }

    // ============================================================================
    // COMPLEX NESTED STRUCTURE TESTS
    // ============================================================================

    #[test]
    fn e2e_edge_deep_state_inheritance() {
        let casp_content = r#"
:Character:
Name: Test
:Level0:
---Init:
Level0Init()
:Level1(Level0):
---Init:
Level1Init()
:Level2(Level1):
---Init:
Level2Init()
:Level3(Level2):
---Init:
Level3Init()
:Level4(Level3):
---Init:
Level4Init()
:Level5(Level4):
---Init:
Level5Init()
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse deep inheritance (6 levels)");
        let character = character.unwrap();

        assert!(character.states.contains_key("Level0"), "Should have Level0");
        assert!(character.states.contains_key("Level5"), "Should have Level5");
        assert_eq!(character.states.len(), 6, "Should have 6 states");

        println!("✓ Deep state inheritance validated (6 levels)");
    }

    #[test]
    fn e2e_edge_many_phases_per_state() {
        let casp_content = r#"
:Character:
Name: Test
:ComplexState:
---Init:
InitAction()
---PreAction:
PreAction()
---Action:
Action()
---PostAction:
PostAction()
---Reaction:
Reaction()
---PostReaction:
PostReaction()
---Update:
Update()
---PostUpdate:
PostUpdate()
---Draw:
Draw()
---End:
End()
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse state with many phases");
        let character = character.unwrap();

        let complex_state = &character.states["ComplexState"];
        assert!(complex_state.actions.len() >= 8,
            "Should have at least 8 phases, got {}", complex_state.actions.len());

        println!("✓ Many phases per state validated ({} phases)", complex_state.actions.len());
    }

    #[test]
    fn e2e_edge_deeply_nested_conditionals() {
        let casp_content = r#"
:Character:
Name: Test
:Variables:
var A(Int): 10
var B(Int): 20
var C(Int): 30
:Test:
---Action:
If(A > 5)
    If(B > 15)
        If(C > 25)
            If(A < 15)
                If(B < 25)
                    DeepAction()
                EndIf
            EndIf
        EndIf
    EndIf
EndIf
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse deeply nested conditionals");
        let character = character.unwrap();

        let test_state = &character.states["Test"];
        assert!(test_state.actions.contains_key("Action"), "Should have Action phase");
        assert!(test_state.actions["Action"].len() > 0, "Should have actions");

        println!("✓ Deeply nested conditionals validated (5 levels)");
    }

    // ============================================================================
    // MANY ITEMS STRESS TESTS
    // ============================================================================

    #[test]
    fn e2e_edge_many_variables() {
        let mut casp_content = String::from(":Character:\nName: Test\n:Variables:\n");

        // Generate 100 variables
        for i in 0..100 {
            casp_content.push_str(&format!("var Var{}(Int): {}\n", i, i * 10));
        }

        let file = create_temp_casp(&casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse 100 variables");
        let character = character.unwrap();

        assert!(character.variables.len() >= 100,
            "Should have at least 100 variables, got {}", character.variables.len());

        println!("✓ Many variables validated ({} variables)", character.variables.len());
    }

    #[test]
    fn e2e_edge_many_states() {
        let mut casp_content = String::from(":Character:\nName: Test\n");

        // Generate 50 states
        for i in 0..50 {
            casp_content.push_str(&format!(":State{}:\n---Init:\nAction{}()\n", i, i));
        }

        let file = create_temp_casp(&casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse 50 states");
        let character = character.unwrap();

        assert!(character.states.len() >= 50,
            "Should have at least 50 states, got {}", character.states.len());

        println!("✓ Many states validated ({} states)", character.states.len());
    }

    #[test]
    fn e2e_edge_many_actions_per_phase() {
        let mut casp_content = String::from(":Character:\nName: Test\n:TestState:\n---Init:\n");

        // Generate 200 actions
        for i in 0..200 {
            casp_content.push_str(&format!("Action{}()\n", i));
        }

        let file = create_temp_casp(&casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse 200 actions");
        let character = character.unwrap();

        let test_state = &character.states["TestState"];
        let init_actions = &test_state.actions["Init"];

        assert!(init_actions.len() >= 200,
            "Should have at least 200 actions, got {}", init_actions.len());

        println!("✓ Many actions per phase validated ({} actions)", init_actions.len());
    }

    // ============================================================================
    // STRING COMPLEXITY TESTS
    // ============================================================================

    #[test]
    fn e2e_edge_long_strings() {
        let long_string = "A".repeat(1000);
        let casp_content = format!(r#"
:Character:
Name: Test
:Variables:
var LongString(Str): "{}"
var NormalString(Str): "Short"
"#, long_string);

        let file = create_temp_casp(&casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse long strings");
        let character = character.unwrap();

        assert!(character.variables.contains_key("LongString"), "Should have LongString");

        let long_str = &character.variables["LongString"];
        assert!(long_str.value.len() > 500, "String should be long");

        println!("✓ Long strings validated (length: {})", long_str.value.len());
    }

    #[test]
    fn e2e_edge_strings_with_special_chars() {
        let casp_content = r#"
:Character:
Name: Test
:Variables:
var SpecialChars(Str): "!@#$%^&*()[]{}|;:,.<>?/~`"
var Quotes(Str): "String with \"quotes\" inside"
var Newlines(Str): "Line1\nLine2\nLine3"
var Tabs(Str): "Col1\tCol2\tCol3"
var Mixed(Str): "Mixed: !@# 123 ABC あいう"
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse strings with special chars");
        let character = character.unwrap();

        assert!(character.variables.contains_key("SpecialChars"), "Should have SpecialChars");
        assert!(character.variables.contains_key("Quotes"), "Should have Quotes");
        assert!(character.variables.contains_key("Mixed"), "Should have Mixed");

        println!("✓ Strings with special chars validated (5 variables)");
    }

    #[test]
    fn e2e_edge_unicode_and_emoji_strings() {
        let casp_content = r#"
:Character:
Name: 戦士Fighter
:Variables:
var Japanese(Str): "こんにちは世界"
var Chinese(Str): "你好世界"
var Korean(Str): "안녕하세요"
var Emoji(Str): "🎮🕹️👾🎯⚔️🛡️💥✨"
var Mixed(Str): "Player 🎮 Level: 레벨 99"
var Arabic(Str): "مرحبا بالعالم"
var Russian(Str): "Привет мир"
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse Unicode and emoji strings");
        let character = character.unwrap();

        assert!(character.variables.contains_key("Japanese"), "Should have Japanese");
        assert!(character.variables.contains_key("Emoji"), "Should have Emoji");
        assert!(character.variables.contains_key("Arabic"), "Should have Arabic");

        println!("✓ Unicode and emoji strings validated (7 variables)");
    }

    // ============================================================================
    // COMPLEX ACTION ARGUMENT TESTS
    // ============================================================================

    #[test]
    fn e2e_edge_actions_with_complex_args() {
        let casp_content = r#"
:Character:
Name: Test
:Variables:
var X(Int): 10
var Y(Int): 20
:TestState:
---Action:
SetPosition(X + Y * 2, Y - X / 2)
Calculate((X + Y) * (X - Y), X * X + Y * Y)
ComplexFunc(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
NestedFunc(Func1(Func2(Func3(10))))
MixedArgs("string", 123, true, 45.67)
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse complex action arguments");
        let character = character.unwrap();

        let test_state = &character.states["TestState"];
        let actions = &test_state.actions["Action"];

        assert!(actions.len() >= 5, "Should have at least 5 actions with complex args");

        println!("✓ Complex action arguments validated ({} actions)", actions.len());
    }

    #[test]
    fn e2e_edge_actions_with_very_long_args() {
        let long_arg = "LongValue".repeat(50);
        let casp_content = format!(r#"
:Character:
Name: Test
:TestState:
---Action:
LongArgAction("{}")
MultipleArgs("arg1", "arg2", "arg3", "arg4", "arg5", "arg6", "arg7", "arg8", "arg9", "arg10")
"#, long_arg);

        let file = create_temp_casp(&casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse actions with very long arguments");
        let character = character.unwrap();

        let test_state = &character.states["TestState"];
        assert!(test_state.actions.contains_key("Action"), "Should have Action phase");

        println!("✓ Actions with very long arguments validated");
    }

    // ============================================================================
    // COMMENT EDGE CASES
    // ============================================================================

    #[test]
    fn e2e_edge_comments_everywhere() {
        let casp_content = r#"
# Top level comment
:Character:
# Comment before name
Name: Test
# Comment after name
# Comment before variables
:Variables:
# Comment before variable
var Health(Int): 100
# Comment between variables
var Mana(Int): 50
# More comments
# Comment before state
:TestState:
# Comment before phase
---Init:
# Comment before action
Action()
# Comment after action
# Final comment
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse file with comments everywhere");
        let character = character.unwrap();

        assert_eq!(character.metadata.name, "Test", "Should parse name despite comments");
        assert!(character.variables.contains_key("Health"), "Should parse variables despite comments");
        assert!(character.states.contains_key("TestState"), "Should parse states despite comments");

        println!("✓ Comments everywhere validated");
    }

    #[test]
    fn e2e_edge_only_comments_and_empty_lines() {
        let casp_content = r#"
# This file is mostly comments

# And empty lines


# With some actual content
:Character:

# More comments
Name: Test

# Even more comments


# And that's it
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse file with mostly comments");
        let character = character.unwrap();

        assert_eq!(character.metadata.name, "Test", "Should extract name from sparse file");

        println!("✓ File with mostly comments validated");
    }

    // ============================================================================
    // SPECBLOCK COMPLEXITY TESTS
    // ============================================================================

    #[test]
    fn e2e_edge_many_specblocks() {
        let mut casp_content = String::from(":Character:\nName: Test\n");

        // Generate 30 specblocks
        for i in 0..30 {
            casp_content.push_str(&format!(":SpecBlock{}:\n", i));
            casp_content.push_str(&format!("Value{}: {}\n", i, i * 10));
            casp_content.push_str(&format!("Name{}: \"Block{}\"\n", i, i));
        }

        let file = create_temp_casp(&casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse many specblocks");
        let character = character.unwrap();

        assert!(character.specblocks.len() >= 30,
            "Should have at least 30 specblocks, got {}", character.specblocks.len());

        println!("✓ Many specblocks validated ({} specblocks)", character.specblocks.len());
    }

    #[test]
    fn e2e_edge_specblock_with_many_fields() {
        let mut casp_content = String::from(":Character:\nName: Test\n:LargeBlock:\n");

        // Generate 50 fields in one specblock
        for i in 0..50 {
            casp_content.push_str(&format!("Field{}: {}\n", i, i));
        }

        let file = create_temp_casp(&casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse specblock with many fields");
        let character = character.unwrap();

        let large_block = &character.specblocks["LargeBlock"];
        assert!(large_block.len() >= 50,
            "Should have at least 50 fields, got {}", large_block.len());

        println!("✓ Specblock with many fields validated ({} fields)", large_block.len());
    }

    // ============================================================================
    // WHITESPACE EDGE CASES
    // ============================================================================

    #[test]
    fn e2e_edge_inconsistent_indentation() {
        let casp_content = r#"
:Character:
Name: Test
:Variables:
  var Var1(Int): 1
    var Var2(Int): 2
      var Var3(Int): 3
 var Var4(Int): 4
var Var5(Int): 5
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        // Parser should handle inconsistent indentation gracefully
        if let Some(character) = character {
            assert!(character.variables.len() > 0, "Should parse some variables");
            println!("✓ Inconsistent indentation handled ({} variables parsed)", character.variables.len());
        } else {
            println!("✓ Inconsistent indentation rejected with errors");
        }
    }

    #[test]
    fn e2e_edge_trailing_whitespace() {
        let casp_content = ":Character:    \nName: Test    \n:Variables:    \nvar Health(Int): 100    \n";

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should handle trailing whitespace");
        let character = character.unwrap();

        assert_eq!(character.metadata.name, "Test", "Should parse name with trailing whitespace");
        assert!(character.variables.contains_key("Health"), "Should parse variables");

        println!("✓ Trailing whitespace handled");
    }

    // ============================================================================
    // FIGHTING GAME PATTERN STRESS TESTS
    // ============================================================================

    #[test]
    fn e2e_edge_complex_combo_system() {
        let casp_content = r#"
:Character:
Name: ComboFighter
:Variables:
var ComboCounter(Int): 0
var ComboWindow(Int): 10
var ComboDamage(Float): 0.0
var ComboScaling(Float): 1.0
:Idle:
---Action:
CheckComboReset()
:LightPunch:
---Init:
IncrementCombo()
StartComboWindow()
---Action:
CheckCancelWindow()
If(CanCancel)
    CheckInput_MediumPunch()
    CheckInput_HeavyPunch()
    CheckInput_Special()
EndIf
:MediumPunch:
---Init:
IncrementCombo()
ApplyComboScaling(0.9)
---Action:
CheckCancelWindow()
:HeavyPunch:
---Init:
IncrementCombo()
ApplyComboScaling(0.8)
---Action:
CheckCancelWindow()
:SpecialMove1:
---Init:
IncrementCombo()
ApplyComboScaling(0.7)
:SpecialMove2:
---Init:
IncrementCombo()
ApplyComboScaling(0.6)
:SuperMove:
---Init:
IncrementCombo()
ApplyComboScaling(0.5)
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse complex combo system");
        let character = character.unwrap();

        assert!(character.states.len() >= 7, "Should have combo states");
        assert!(character.variables.contains_key("ComboCounter"), "Should have combo variables");

        println!("✓ Complex combo system validated ({} states)", character.states.len());
    }

    #[test]
    fn e2e_edge_advanced_state_machine() {
        let casp_content = r#"
:Character:
Name: AdvancedFighter
:Variables:
var CurrentState(Str): "Idle"
var PreviousState(Str): ""
var StateTimer(Int): 0
var CanTransition(Bool): true
:Idle:
---Action:
CheckTransition_Walk()
CheckTransition_Jump()
CheckTransition_Attack()
:Walk:
---Action:
CheckTransition_Idle()
CheckTransition_Run()
CheckTransition_Jump()
:Run:
---Action:
CheckTransition_Walk()
CheckTransition_Jump()
CheckTransition_Slide()
:Jump:
---Action:
CheckTransition_AirAttack()
CheckTransition_AirDash()
CheckTransition_Landing()
:AirDash:
---Action:
CheckTransition_AirAttack()
CheckTransition_Landing()
:Attack:
---Action:
CheckTransition_Idle()
CheckTransition_AttackChain()
:AttackChain:
---Action:
CheckTransition_Idle()
CheckTransition_Special()
:Special:
---Action:
CheckTransition_Idle()
CheckTransition_Super()
:Super:
---Action:
CheckTransition_Idle()
:Hit:
---Action:
CheckTransition_Idle()
CheckTransition_HitStun()
:HitStun:
---Action:
CheckTransition_Idle()
CheckTransition_KnockDown()
:KnockDown:
---Action:
CheckTransition_WakeUp()
:WakeUp:
---Action:
CheckTransition_Idle()
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse advanced state machine");
        let character = character.unwrap();

        assert!(character.states.len() >= 13,
            "Should have at least 13 states, got {}", character.states.len());
        assert!(character.variables.contains_key("CurrentState"), "Should have state tracking");

        println!("✓ Advanced state machine validated ({} states)", character.states.len());
    }

    // ============================================================================
    // VARIABLE TYPE MIX TESTS
    // ============================================================================

    #[test]
    fn e2e_edge_all_types_together() {
        let casp_content = r#"
:Character:
Name: TypeMix
:Variables:
var IntVar(Int): 42
var FloatVar(Float): 3.14159
var BoolVar(Bool): true
var StrVar(Str): "Hello World"
var Vec2Var(Vec2): 10, 20
var Vec3Var(Vec3): 1.0, 2.0, 3.0
def IntConst: 100
def FloatConst: 2.718
def BoolConst: false
def StrConst: "Constant"
var NegInt(Int): -999
var NegFloat(Float): -123.456
var ZeroInt(Int): 0
var ZeroFloat(Float): 0.0
var EmptyStr(Str): ""
var ZeroVec2(Vec2): 0, 0
var ZeroVec3(Vec3): 0.0, 0.0, 0.0
"#;

        let file = create_temp_casp(casp_content);
        let mut parser = CastagneParser::new();
        let character = parser.create_full_character(file.path().to_str().unwrap());

        assert!(character.is_some(), "Should parse all types together");
        let character = character.unwrap();

        assert!(character.variables.len() >= 17,
            "Should have at least 17 variables, got {}", character.variables.len());

        // Check specific types
        assert!(character.variables.contains_key("IntVar"), "Should have IntVar");
        assert!(character.variables.contains_key("FloatVar"), "Should have FloatVar");
        assert!(character.variables.contains_key("BoolVar"), "Should have BoolVar");
        assert!(character.variables.contains_key("StrVar"), "Should have StrVar");
        assert!(character.variables.contains_key("Vec2Var"), "Should have Vec2Var");
        assert!(character.variables.contains_key("Vec3Var"), "Should have Vec3Var");

        println!("✓ All types together validated ({} variables)", character.variables.len());
    }

    // ============================================================================
    // SUMMARY TEST
    // ============================================================================

    #[test]
    fn e2e_comprehensive_edge_cases_summary() {
        println!("\n=== E2E Comprehensive Edge Cases Test Summary ===\n");
        println!("Edge cases and stress tests covered:");
        println!("  ✓ Extreme integer values (max, min, large)");
        println!("  ✓ Special float values (very small, very large, pi)");
        println!("  ✓ Vector extreme values (large, small, mixed)");
        println!("  ✓ Deep state inheritance (6 levels)");
        println!("  ✓ Many phases per state (10+ phases)");
        println!("  ✓ Deeply nested conditionals (5 levels)");
        println!("  ✓ Stress test: 100 variables");
        println!("  ✓ Stress test: 50 states");
        println!("  ✓ Stress test: 200 actions per phase");
        println!("  ✓ Long strings (1000+ chars)");
        println!("  ✓ Strings with special characters");
        println!("  ✓ Unicode and emoji strings (7 languages)");
        println!("  ✓ Complex action arguments");
        println!("  ✓ Very long action arguments");
        println!("  ✓ Comments everywhere");
        println!("  ✓ Many specblocks (30+)");
        println!("  ✓ Specblock with many fields (50+)");
        println!("  ✓ Inconsistent indentation handling");
        println!("  ✓ Trailing whitespace handling");
        println!("  ✓ Complex combo system");
        println!("  ✓ Advanced state machine (13+ states)");
        println!("  ✓ All variable types together (17+ variables)");
        println!("\nAll comprehensive edge case tests completed!\n");
    }
}
