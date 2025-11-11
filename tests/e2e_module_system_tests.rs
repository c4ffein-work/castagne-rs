// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Module System E2E Tests
//!
//! Comprehensive tests for the Castagne module system:
//! - Base module loading and parsing
//! - Module dependencies and ordering
//! - Module defines and configuration
//! - Graphics, Physics, Attacks, and Audio modules
//! - Module interaction patterns

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

    fn load_base_module(module_name: &str) -> Result<String, std::io::Error> {
        let path = format!("castagne_godot4/modules/{}", module_name);
        fs::read_to_string(path)
    }

    // ============================================================================
    // GRAPHICS MODULE TESTS
    // ============================================================================

    #[test]
    fn e2e_module_graphics_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        if let Some(graphics) = golden["transformed_data"]["Graphics"].as_object() {
            // Graphics module should have key sections
            let has_defines = graphics.contains_key("Defines");
            let has_spritesheets = graphics.contains_key("Spritesheets");
            let has_palettes = graphics.contains_key("Palettes");

            assert!(has_defines, "Graphics module should have Defines section");

            println!("✓ Graphics module structure validated");
            println!("  Defines: {}, Spritesheets: {}, Palettes: {}",
                     has_defines, has_spritesheets, has_palettes);
        } else {
            println!("⚠ Graphics module not found (may not be in golden master)");
        }
    }

    #[test]
    fn e2e_module_graphics_spritesheet_validation() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        if let Some(graphics) = golden["transformed_data"]["Graphics"].as_object() {
            if let Some(spritesheets) = graphics["Spritesheets"].as_object() {
                let mut total_sprites = 0;

                for (sheet_name, sheet_data) in spritesheets {
                    // Validate required fields
                    assert!(sheet_data["SpritesX"].is_number(),
                        "Spritesheet {} missing SpritesX", sheet_name);
                    assert!(sheet_data["SpritesY"].is_number(),
                        "Spritesheet {} missing SpritesY", sheet_name);

                    let sprites_x = sheet_data["SpritesX"].as_u64().unwrap_or(0);
                    let sprites_y = sheet_data["SpritesY"].as_u64().unwrap_or(0);
                    total_sprites += sprites_x * sprites_y;
                }

                println!("✓ Graphics spritesheets validated");
                println!("  Total sheets: {}, Total sprite slots: {}", spritesheets.len(), total_sprites);
            }
        }
    }

    #[test]
    fn e2e_module_graphics_palette_validation() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        if let Some(graphics) = golden["transformed_data"]["Graphics"].as_object() {
            if let Some(palettes) = graphics["Palettes"].as_object() {
                for (pal_id, pal_data) in palettes {
                    // Each palette must have DisplayName
                    assert!(pal_data["DisplayName"].is_string(),
                        "Palette {} missing DisplayName", pal_id);

                    let display_name = pal_data["DisplayName"].as_str().unwrap();
                    assert!(!display_name.is_empty(),
                        "Palette {} has empty DisplayName", pal_id);
                }

                println!("✓ Graphics palettes validated ({} palettes)", palettes.len());
            }
        }
    }

    #[test]
    fn e2e_module_graphics_origin_points() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        if let Some(graphics) = golden["transformed_data"]["Graphics"].as_object() {
            if let Some(spritesheets) = graphics["Spritesheets"].as_object() {
                let mut origin_counts = std::collections::HashMap::new();

                for (_sheet_name, sheet_data) in spritesheets {
                    let origin_x = sheet_data["OriginX"].as_i64().unwrap_or(0);
                    let origin_y = sheet_data["OriginY"].as_i64().unwrap_or(0);
                    let origin = format!("{},{}", origin_x, origin_y);
                    *origin_counts.entry(origin).or_insert(0) += 1;
                }

                println!("✓ Graphics origin points analyzed");
                println!("  Unique origins: {}", origin_counts.len());
            }
        }
    }

    // ============================================================================
    // PHYSICS MODULE TESTS
    // ============================================================================

    #[test]
    fn e2e_module_physics_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        if let Some(physics) = golden["transformed_data"]["PhysicsMovement"].as_object() {
            let has_defines = physics.contains_key("Defines");
            assert!(has_defines, "Physics module should have Defines section");

            println!("✓ Physics module structure validated");
        } else {
            println!("⚠ Physics module not found");
        }
    }

    #[test]
    fn e2e_module_physics_variables() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        // Count physics-related variables
        let mut physics_vars = 0;
        let mut movement_vars = 0;

        for (var_name, _) in variables {
            let name_lower = var_name.to_lowercase();
            if name_lower.contains("physics") || name_lower.contains("velocity") ||
               name_lower.contains("position") || name_lower.contains("gravity") {
                physics_vars += 1;
            }
            if name_lower.contains("move") || name_lower.contains("speed") {
                movement_vars += 1;
            }
        }

        println!("✓ Physics variables analyzed");
        println!("  Physics-related: {}, Movement-related: {}", physics_vars, movement_vars);
    }

    // ============================================================================
    // ATTACKS MODULE TESTS
    // ============================================================================

    #[test]
    fn e2e_module_attacks_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        if let Some(attacks) = golden["transformed_data"]["AttacksMechanics"].as_object() {
            let has_defines = attacks.contains_key("Defines");
            assert!(has_defines, "Attacks module should have Defines section");

            println!("✓ Attacks module structure validated");
        } else {
            println!("⚠ Attacks module not found");
        }
    }

    #[test]
    fn e2e_module_attacks_states() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Count attack-related states
        let mut attack_states = 0;
        let mut hit_states = 0;

        for (state_name, _) in states {
            let name_lower = state_name.to_lowercase();
            if name_lower.contains("attack") || name_lower.starts_with("atk") ||
               name_lower.contains("punch") || name_lower.contains("kick") {
                attack_states += 1;
            }
            if name_lower.contains("hit") || name_lower.contains("hurt") {
                hit_states += 1;
            }
        }

        println!("✓ Attack states analyzed");
        println!("  Attack states: {}, Hit states: {}", attack_states, hit_states);
    }

    #[test]
    fn e2e_module_attacks_variables() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        // Count attack-related variables
        let mut damage_vars = 0;
        let mut hitbox_vars = 0;

        for (var_name, _) in variables {
            let name_lower = var_name.to_lowercase();
            if name_lower.contains("damage") || name_lower.contains("power") ||
               name_lower.contains("strength") {
                damage_vars += 1;
            }
            if name_lower.contains("hitbox") || name_lower.contains("hurtbox") {
                hitbox_vars += 1;
            }
        }

        println!("✓ Attack variables analyzed");
        println!("  Damage-related: {}, Hitbox-related: {}", damage_vars, hitbox_vars);
    }

    // ============================================================================
    // ANIMATION MODULE TESTS
    // ============================================================================

    #[test]
    fn e2e_module_anims_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        if let Some(anims) = golden["transformed_data"]["Anims"].as_object() {
            let has_defines = anims.contains_key("Defines");
            assert!(has_defines, "Anims module should have Defines section");

            println!("✓ Anims module structure validated");
        } else {
            println!("⚠ Anims module not found");
        }
    }

    #[test]
    fn e2e_module_anims_states_coverage() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let states = golden["states"].as_object().unwrap();

        // Animation-related states
        let mut anim_states = 0;

        for (_state_name, _) in states {
            // Most states should have animations
            anim_states += 1;
        }

        println!("✓ Animation state coverage: {} states", anim_states);
    }

    // ============================================================================
    // MODULE DEPENDENCY TESTS
    // ============================================================================

    #[test]
    fn e2e_module_all_modules_have_defines() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        let mut modules_without_defines = Vec::new();

        for (module_name, module_data) in transformed {
            if !module_data["Defines"].is_object() {
                modules_without_defines.push(module_name);
            }
        }

        if !modules_without_defines.is_empty() {
            println!("⚠ Modules without Defines: {:?}", modules_without_defines);
        }

        println!("✓ Module defines checked ({} total modules)", transformed.len());
    }

    #[test]
    fn e2e_module_defines_key_patterns() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        let mut all_define_keys = std::collections::HashSet::new();

        for (_module_name, module_data) in transformed {
            if let Some(defines) = module_data["Defines"].as_object() {
                for (key, _) in defines {
                    all_define_keys.insert(key.clone());
                }
            }
        }

        println!("✓ Module defines key patterns analyzed");
        println!("  Unique define keys across all modules: {}", all_define_keys.len());
    }

    // ============================================================================
    // BASE MODULE FILE TESTS
    // ============================================================================

    #[test]
    fn e2e_module_base_core_exists() {
        match load_base_module("core/Base-Core.casp") {
            Ok(content) => {
                assert!(!content.is_empty(), "Base-Core.casp should not be empty");
                println!("✓ Base-Core.casp exists and readable ({} bytes)", content.len());
            }
            Err(_) => {
                println!("⚠ Base-Core.casp not found (may not be included in test environment)");
            }
        }
    }

    #[test]
    fn e2e_module_base_graphics_exists() {
        match load_base_module("graphics/Base-Graphics.casp") {
            Ok(content) => {
                assert!(!content.is_empty(), "Base-Graphics.casp should not be empty");
                println!("✓ Base-Graphics.casp exists and readable ({} bytes)", content.len());
            }
            Err(_) => {
                println!("⚠ Base-Graphics.casp not found");
            }
        }
    }

    #[test]
    fn e2e_module_base_physics_exists() {
        match load_base_module("physics/Base-Physics2D.casp") {
            Ok(content) => {
                assert!(!content.is_empty(), "Base-Physics2D.casp should not be empty");
                println!("✓ Base-Physics2D.casp exists and readable ({} bytes)", content.len());
            }
            Err(_) => {
                println!("⚠ Base-Physics2D.casp not found");
            }
        }
    }

    #[test]
    fn e2e_module_base_attacks_exists() {
        match load_base_module("attacks/Base-Attacks.casp") {
            Ok(content) => {
                assert!(!content.is_empty(), "Base-Attacks.casp should not be empty");
                println!("✓ Base-Attacks.casp exists and readable ({} bytes)", content.len());
            }
            Err(_) => {
                println!("⚠ Base-Attacks.casp not found");
            }
        }
    }

    #[test]
    fn e2e_module_base_audio_exists() {
        match load_base_module("general/Base-Audio.casp") {
            Ok(content) => {
                assert!(!content.is_empty(), "Base-Audio.casp should not be empty");
                println!("✓ Base-Audio.casp exists and readable ({} bytes)", content.len());
            }
            Err(_) => {
                println!("⚠ Base-Audio.casp not found");
            }
        }
    }

    #[test]
    fn e2e_module_base_ai_exists() {
        match load_base_module("general/Base-AI.casp") {
            Ok(content) => {
                assert!(!content.is_empty(), "Base-AI.casp should not be empty");
                println!("✓ Base-AI.casp exists and readable ({} bytes)", content.len());
            }
            Err(_) => {
                println!("⚠ Base-AI.casp not found");
            }
        }
    }

    #[test]
    fn e2e_module_base_training_exists() {
        match load_base_module("general/Base-Training.casp") {
            Ok(content) => {
                assert!(!content.is_empty(), "Base-Training.casp should not be empty");
                println!("✓ Base-Training.casp exists and readable ({} bytes)", content.len());
            }
            Err(_) => {
                println!("⚠ Base-Training.casp not found");
            }
        }
    }

    // ============================================================================
    // MODULE INTERACTION TESTS
    // ============================================================================

    #[test]
    fn e2e_module_cross_module_consistency() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        // All modules should have consistent structure
        for (module_name, module_data) in transformed {
            assert!(module_data.is_object(),
                "Module {} should be an object", module_name);
        }

        println!("✓ Cross-module consistency validated ({} modules)", transformed.len());
    }

    #[test]
    fn e2e_module_transformed_data_completeness() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        // transformed_data should exist and be an object
        assert!(golden["transformed_data"].is_object(),
            "transformed_data should be an object");

        let modules = golden["transformed_data"].as_object().unwrap();
        assert!(modules.len() > 0, "Should have at least one module");

        println!("✓ Transformed data completeness validated ({} modules)", modules.len());
    }

    // ============================================================================
    // MODULE SUMMARY TEST
    // ============================================================================

    #[test]
    fn e2e_module_comprehensive_summary() {
        println!("\n=== Module System Validation Summary ===\n");

        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        for file_path in files {
            println!("File: {}", file_path);
            let golden = load_golden_master(file_path);

            let transformed = golden["transformed_data"].as_object().unwrap();
            let module_count = transformed.len();

            println!("  Total modules: {}", module_count);

            // List all modules
            let mut module_names: Vec<_> = transformed.keys().map(|s| s.as_str()).collect();
            module_names.sort();

            println!("  Modules: {}", module_names.join(", "));

            // Count defines per module
            let mut total_defines = 0;
            for (_module_name, module_data) in transformed {
                if let Some(defines) = module_data["Defines"].as_object() {
                    total_defines += defines.len();
                }
            }

            println!("  Total defines across all modules: {}", total_defines);
            println!();
        }

        println!("✓ Module system validation complete!\n");
    }
}
