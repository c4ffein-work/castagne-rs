// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Debug test for parser development - see what the Rust parser outputs

use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    /// This test will show us what the parser currently outputs
    /// We can't actually run the parser here because it needs Godot runtime,
    /// but we can use this to document what we expect
    #[test]
    fn document_expected_parser_behavior() {
        // Load the test file
        let content = fs::read_to_string("test_character_complete.casp")
            .expect("Failed to load test_character_complete.casp");

        println!("=== Test File Content ===");
        println!("{}", content);
        println!("\n=== Expected Parser Behavior ===");
        println!("The parser should:");
        println!("1. Parse metadata (Character block)");
        println!("   - Name: Complete Test Fighter");
        println!("   - Author: Parser Development Team");
        println!("   - Description: A comprehensive character file testing all parser features");
        println!("   - Version: 2.0");
        println!("\n2. Parse specblocks (AttackData, PhysicsConfig)");
        println!("   - AttackData should have: LightPunchDamage, LightPunchRange, etc.");
        println!("   - PhysicsConfig should have: Gravity, JumpForce, MaxSpeed, Friction");
        println!("\n3. Parse variables");
        println!("   - var Health(Int): 150");
        println!("   - var PlayerName(Str): Fighter");
        println!("   - var IsGrounded(Bool): true");
        println!("   - var Position(Vec2): 0, 0");
        println!("   - def MAX_COMBO: 10");
        println!("\n4. Parse states");
        println!("   - Idle state with Init, Action phases");
        println!("   - Walk state");
        println!("   - Jump state");
        println!("   - LightPunch state");
        println!("   - HeavyPunch state");

        // Load golden master to compare
        let golden = fs::read_to_string("golden_masters/test_character_complete.json")
            .expect("Failed to load golden master");

        println!("\n=== Current Golden Master ===");
        println!("{}", golden);

        println!("\n=== Analysis ===");
        println!("The golden master is mostly empty - it only has metadata.");
        println!("This suggests we need to generate a proper golden master for test_character_complete.casp");
        println!("Or we should focus on testing against the Baston-Model.json golden master");
    }

    /// Check what's in the Baston golden master
    #[test]
    fn analyze_baston_golden_master() {
        let golden = fs::read_to_string("golden_masters/Baston-Model.json")
            .expect("Failed to load Baston-Model golden master");

        let json: serde_json::Value = serde_json::from_str(&golden)
            .expect("Failed to parse golden master");

        println!("=== Baston-Model Golden Master Analysis ===");

        // Metadata
        if let Some(metadata) = json["metadata"].as_object() {
            println!("\nMetadata fields:");
            for key in metadata.keys() {
                println!("  - {}", key);
            }
        }

        // Variables
        if let Some(variables) = json["variables"].as_object() {
            println!("\nVariables: {} total", variables.len());
            for (name, var_data) in variables.iter().take(5) {
                println!("  - {}: {:?}", name, var_data);
            }
            if variables.len() > 5 {
                println!("  ... and {} more", variables.len() - 5);
            }
        }

        // States
        if let Some(states) = json["states"].as_object() {
            println!("\nStates: {} total", states.len());
            for name in states.keys().take(10) {
                println!("  - {}", name);
            }
            if states.len() > 10 {
                println!("  ... and {} more", states.len() - 10);
            }
        }

        // Transformed data
        if let Some(transformed) = json["transformed_data"].as_object() {
            println!("\nTransformed data sections:");
            for key in transformed.keys() {
                println!("  - {}", key);
            }
        }
    }
}
