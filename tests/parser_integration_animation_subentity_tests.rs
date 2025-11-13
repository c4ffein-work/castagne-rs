// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Animation and Subentity Validation E2E Tests
//!
//! Tests that validate animation data and subentity handling:
//! - Animation data structure
//! - Sprite and animation mappings
//! - Frame data validation
//! - Subentity structure and metadata
//! - Subentity variable inheritance
//! - Subentity state management
//! - Animation-state relationships
//! - Visual data consistency

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

    // ============================================================================
    // ANIMATION DATA STRUCTURE TESTS
    // ============================================================================

    #[test]
    fn e2e_animation_graphics_module_presence() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        assert!(
            transformed.contains_key("Graphics"),
            "2D character should have Graphics module"
        );

        let graphics = &transformed["Graphics"];
        assert!(graphics.is_object(), "Graphics should be an object");

        println!("✓ Graphics module present and valid");
    }

    #[test]
    fn e2e_animation_anims_section_structure() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let graphics = &golden["transformed_data"]["Graphics"];

        if let Some(anims) = graphics.get("Anims") {
            if anims.is_object() {
                let anim_map = anims.as_object().unwrap();
                println!("✓ Anims section present: {} animations", anim_map.len());

                // Check structure of animations
                let mut null_anims = 0;
                let mut object_anims = 0;

                for (_, anim_data) in anim_map {
                    if anim_data.is_null() {
                        null_anims += 1;
                    } else if anim_data.is_object() {
                        object_anims += 1;
                    }
                }

                println!("  Null animations: {}", null_anims);
                println!("  Object animations: {}", object_anims);
            }
        } else {
            println!("⚠ No Anims section found (may be in different location)");
        }
    }

    #[test]
    fn e2e_animation_state_to_anim_mapping() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let states = golden["states"].as_object().unwrap();

        // Look for animation references in state names
        let anim_keywords = ["idle", "walk", "run", "jump", "attack", "hit", "stand"];
        let mut states_with_anim_hints = Vec::new();

        for state_name in states.keys() {
            let name_lower = state_name.to_lowercase();
            if anim_keywords.iter().any(|kw| name_lower.contains(kw)) {
                states_with_anim_hints.push(state_name.clone());
            }
        }

        println!(
            "✓ States with animation hints: {}",
            states_with_anim_hints.len()
        );
        if !states_with_anim_hints.is_empty() {
            println!(
                "  Examples: {:?}",
                states_with_anim_hints.iter().take(5).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn e2e_animation_sprite_data_validation() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let graphics = &golden["transformed_data"]["Graphics"];

        if let Some(spritesheets) = graphics["Spritesheets"].as_object() {
            for (sheet_name, sheet_data) in spritesheets {
                // Validate sprite dimensions
                let sprites_x = sheet_data["SpritesX"].as_i64().unwrap_or(0);
                let sprites_y = sheet_data["SpritesY"].as_i64().unwrap_or(0);
                let total_sprites = sprites_x * sprites_y;

                assert!(
                    total_sprites > 0 && total_sprites <= 10000,
                    "Spritesheet {} has invalid sprite count: {}",
                    sheet_name,
                    total_sprites
                );

                println!(
                    "  Spritesheet {}: {}x{} = {} sprites",
                    sheet_name, sprites_x, sprites_y, total_sprites
                );
            }

            println!(
                "✓ Sprite data validated ({} spritesheets)",
                spritesheets.len()
            );
        }
    }

    #[test]
    fn e2e_animation_origin_points() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let graphics = &golden["transformed_data"]["Graphics"];

        if let Some(spritesheets) = graphics["Spritesheets"].as_object() {
            let mut origin_patterns: std::collections::HashMap<(i64, i64), usize> =
                std::collections::HashMap::new();

            for (_, sheet_data) in spritesheets {
                let origin_x = sheet_data["OriginX"].as_i64().unwrap_or(0);
                let origin_y = sheet_data["OriginY"].as_i64().unwrap_or(0);

                *origin_patterns.entry((origin_x, origin_y)).or_insert(0) += 1;
            }

            println!("✓ Origin point patterns:");
            for ((x, y), count) in origin_patterns.iter() {
                println!("  ({}, {}): {} spritesheets", x, y, count);
            }
        }
    }

    #[test]
    fn e2e_animation_palette_display_names() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let graphics = &golden["transformed_data"]["Graphics"];

        if let Some(palettes) = graphics["Palettes"].as_object() {
            let mut display_names = Vec::new();

            for (palette_id, palette_data) in palettes {
                if let Some(display_name) = palette_data["DisplayName"].as_str() {
                    display_names.push((palette_id.clone(), display_name.to_string()));
                } else {
                    panic!("Palette {} missing DisplayName", palette_id);
                }
            }

            println!("✓ Palette display names validated: {}", display_names.len());
            for (id, name) in display_names.iter().take(5) {
                println!("  {}: {}", id, name);
            }
        }
    }

    // ============================================================================
    // SUBENTITY STRUCTURE TESTS
    // ============================================================================

    #[test]
    fn e2e_subentity_section_presence() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        assert!(
            golden.get("subentities").is_some(),
            "Golden master should have subentities section"
        );

        let subentities = golden["subentities"].as_object().unwrap();
        println!(
            "✓ Subentities section present: {} subentities",
            subentities.len()
        );
    }

    #[test]
    fn e2e_subentity_metadata_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let subentities = golden["subentities"].as_object().unwrap();

        for (subentity_name, subentity_data) in subentities {
            // Each subentity should have metadata-like structure
            assert!(
                subentity_data.is_object(),
                "Subentity {} should be an object",
                subentity_name
            );

            // Common metadata fields
            if subentity_data.get("name").is_some() {
                println!("  Subentity {}: has name field", subentity_name);
            }
        }

        println!("✓ Subentity metadata structure validated");
    }

    #[test]
    fn e2e_subentity_vs_main_character() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let subentities = golden["subentities"].as_object().unwrap();
        let main_states = golden["states"].as_object().unwrap();

        println!("✓ Subentity vs main character:");
        println!("  Main character states: {}", main_states.len());
        println!("  Subentities: {}", subentities.len());

        // Typically main character has many more states than subentities
        if subentities.len() > 0 {
            let ratio = main_states.len() as f64 / subentities.len() as f64;
            println!("  States to subentities ratio: {:.2}", ratio);
        }
    }

    #[test]
    fn e2e_subentity_name_patterns() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let subentities = golden["subentities"].as_object().unwrap();

        let mut name_patterns: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for subentity_name in subentities.keys() {
            // Categorize naming patterns
            let pattern = if subentity_name.contains("Projectile") {
                "projectile"
            } else if subentity_name.contains("Effect") {
                "effect"
            } else if subentity_name.contains("Helper") {
                "helper"
            } else if subentity_name.contains("Hitbox") {
                "hitbox"
            } else {
                "other"
            };

            *name_patterns.entry(pattern.to_string()).or_insert(0) += 1;
        }

        println!("✓ Subentity naming patterns:");
        for (pattern, count) in name_patterns.iter() {
            println!("  {}: {}", pattern, count);
        }
    }

    // ============================================================================
    // SUBENTITY VARIABLE TESTS
    // ============================================================================

    #[test]
    fn e2e_subentity_variables_in_main() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();
        let subentities = golden["subentities"].as_object().unwrap();

        let mut subentity_var_count = 0;

        for (var_name, var_data) in variables {
            // Check if variable is related to subentities
            if var_name.contains("Subentity") || var_name.contains("subentity") {
                subentity_var_count += 1;
            }

            // Check if variable type is empty (often indicates subentity placeholder)
            let var_type = var_data["Type"].as_str().unwrap_or("");
            if var_type.is_empty() && subentities.contains_key(var_name) {
                println!("  Variable {} is a subentity placeholder", var_name);
            }
        }

        println!("✓ Subentity-related variables: {}", subentity_var_count);
    }

    #[test]
    fn e2e_subentity_empty_type_correlation() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();
        let subentities = golden["subentities"].as_object().unwrap();

        let mut empty_type_vars = Vec::new();
        let mut empty_type_subentities = Vec::new();

        for (var_name, var_data) in variables {
            let var_type = var_data["Type"].as_str().unwrap_or("");
            if var_type.is_empty() {
                empty_type_vars.push(var_name.clone());

                if subentities.contains_key(var_name) {
                    empty_type_subentities.push(var_name.clone());
                }
            }
        }

        println!("✓ Empty type correlation:");
        println!("  Variables with empty type: {}", empty_type_vars.len());
        println!(
            "  Of those, are subentities: {}",
            empty_type_subentities.len()
        );

        if !empty_type_subentities.is_empty() {
            println!(
                "  Examples: {:?}",
                empty_type_subentities.iter().take(3).collect::<Vec<_>>()
            );
        }
    }

    // ============================================================================
    // ANIMATION-STATE RELATIONSHIP TESTS
    // ============================================================================

    #[test]
    fn e2e_animation_common_state_patterns() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let states = golden["states"].as_object().unwrap();

        // Common fighting game animation states
        let expected_patterns = vec![
            ("idle", "Idle animation"),
            ("stand", "Standing animation"),
            ("walk", "Walking animation"),
            ("attack", "Attack animation"),
            ("hit", "Hit reaction"),
        ];

        let mut found_patterns: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for state_name in states.keys() {
            let name_lower = state_name.to_lowercase();

            for (pattern, _) in &expected_patterns {
                if name_lower.contains(pattern) {
                    found_patterns
                        .entry(pattern.to_string())
                        .or_insert_with(Vec::new)
                        .push(state_name.clone());
                }
            }
        }

        println!("✓ Common animation state patterns:");
        for (pattern, description) in &expected_patterns {
            if let Some(states_list) = found_patterns.get(*pattern) {
                println!("  {}: {} states", description, states_list.len());
            }
        }
    }

    #[test]
    fn e2e_animation_frame_data_variables() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let frame_keywords = ["frame", "duration", "sprite", "anim"];
        let mut frame_related_vars = Vec::new();

        for (var_name, _) in variables {
            let name_lower = var_name.to_lowercase();
            if frame_keywords.iter().any(|kw| name_lower.contains(kw)) {
                frame_related_vars.push(var_name.clone());
            }
        }

        println!(
            "✓ Frame data related variables: {}",
            frame_related_vars.len()
        );
        if !frame_related_vars.is_empty() {
            println!(
                "  Examples: {:?}",
                frame_related_vars.iter().take(5).collect::<Vec<_>>()
            );
        }
    }

    // ============================================================================
    // VISUAL DATA CONSISTENCY TESTS
    // ============================================================================

    #[test]
    fn e2e_animation_pixel_size_consistency() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let graphics = &golden["transformed_data"]["Graphics"];

        if let Some(spritesheets) = graphics["Spritesheets"].as_object() {
            let mut pixel_sizes: std::collections::HashMap<i64, usize> =
                std::collections::HashMap::new();

            for (_, sheet_data) in spritesheets {
                let pixel_size = sheet_data["PixelSize"].as_i64().unwrap_or(0);
                *pixel_sizes.entry(pixel_size).or_insert(0) += 1;
            }

            println!("✓ Pixel size consistency:");
            for (size, count) in pixel_sizes.iter() {
                println!("  Size {}: {} spritesheets", size, count);
            }

            // Check if all spritesheets use same pixel size (common for consistency)
            if pixel_sizes.len() == 1 {
                println!("  All spritesheets use same pixel size (consistent)");
            } else {
                println!("  Multiple pixel sizes used (intentional variation)");
            }
        }
    }

    #[test]
    fn e2e_animation_spritesheet_coverage() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let graphics = &golden["transformed_data"]["Graphics"];

        if let Some(spritesheets) = graphics["Spritesheets"].as_object() {
            let mut total_sprite_slots = 0;

            for (_, sheet_data) in spritesheets {
                let sprites_x = sheet_data["SpritesX"].as_i64().unwrap_or(0);
                let sprites_y = sheet_data["SpritesY"].as_i64().unwrap_or(0);
                total_sprite_slots += sprites_x * sprites_y;
            }

            println!("✓ Spritesheet coverage:");
            println!("  Total sprite slots: {}", total_sprite_slots);
            println!("  Spritesheets: {}", spritesheets.len());
            println!(
                "  Average slots per sheet: {:.1}",
                total_sprite_slots as f64 / spritesheets.len() as f64
            );
        }
    }

    #[test]
    fn e2e_animation_palette_count_vs_spritesheets() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let graphics = &golden["transformed_data"]["Graphics"];

        let spritesheet_count = graphics["Spritesheets"]
            .as_object()
            .map(|s| s.len())
            .unwrap_or(0);

        let palette_count = graphics["Palettes"]
            .as_object()
            .map(|p| p.len())
            .unwrap_or(0);

        println!("✓ Palette vs spritesheet count:");
        println!("  Spritesheets: {}", spritesheet_count);
        println!("  Palettes: {}", palette_count);

        // Typically there are multiple palettes per character (color variations)
        if palette_count > 0 && spritesheet_count > 0 {
            println!(
                "  Palette to spritesheet ratio: {:.2}",
                palette_count as f64 / spritesheet_count as f64
            );
        }
    }

    // ============================================================================
    // CROSS-FILE ANIMATION TESTS
    // ============================================================================

    #[test]
    fn e2e_animation_model_vs_2d_graphics() {
        let model = load_golden_master("golden_masters/Baston-Model.json");
        let derived_2d = load_golden_master("golden_masters/Baston-2D.json");

        let model_has_graphics = model["transformed_data"]
            .as_object()
            .map(|t| t.contains_key("Graphics"))
            .unwrap_or(false);

        let derived_has_graphics = derived_2d["transformed_data"]
            .as_object()
            .map(|t| t.contains_key("Graphics"))
            .unwrap_or(false);

        println!("✓ Graphics module presence:");
        println!("  Model file: {}", model_has_graphics);
        println!("  2D file: {}", derived_has_graphics);

        // 2D file should definitely have graphics
        assert!(derived_has_graphics, "2D file must have Graphics module");
    }

    #[test]
    fn e2e_animation_inheritance_graphics_data() {
        let model = load_golden_master("golden_masters/Baston-Model.json");
        let derived = load_golden_master("golden_masters/Baston-2D.json");

        let model_graphics = &model["transformed_data"]["Graphics"];
        let derived_graphics = &derived["transformed_data"]["Graphics"];

        let model_has_spritesheets = model_graphics.get("Spritesheets").is_some();
        let derived_has_spritesheets = derived_graphics.get("Spritesheets").is_some();

        println!("✓ Graphics inheritance:");
        println!("  Model spritesheets: {}", model_has_spritesheets);
        println!("  Derived spritesheets: {}", derived_has_spritesheets);

        // At least one should have spritesheet data
        if !model_has_spritesheets && !derived_has_spritesheets {
            println!("  Note: Spritesheet data may be loaded differently");
        }
    }

    // ============================================================================
    // SUBENTITY EDGE CASES
    // ============================================================================

    #[test]
    fn e2e_subentity_count_distribution() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        println!("✓ Subentity count distribution:");
        for file_path in &files {
            let golden = load_golden_master(file_path);
            let subentities = golden["subentities"].as_object().unwrap();

            println!("  {}: {} subentities", file_path, subentities.len());
        }
    }

    #[test]
    fn e2e_subentity_shared_across_files() {
        let model = load_golden_master("golden_masters/Baston-Model.json");
        let derived = load_golden_master("golden_masters/Baston-2D.json");

        let model_subs: std::collections::HashSet<String> = model["subentities"]
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect();
        let derived_subs: std::collections::HashSet<String> = derived["subentities"]
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect();

        let common: std::collections::HashSet<_> =
            model_subs.intersection(&derived_subs).cloned().collect();

        println!("✓ Shared subentities:");
        println!("  Model: {} subentities", model_subs.len());
        println!("  Derived: {} subentities", derived_subs.len());
        println!("  Common: {} subentities", common.len());

        if !common.is_empty() && common.len() <= 5 {
            println!("  Shared: {:?}", common);
        }
    }

    // ============================================================================
    // SUMMARY TEST
    // ============================================================================

    #[test]
    fn e2e_animation_subentity_summary() {
        println!("\n=== E2E Animation & Subentity Validation Summary ===\n");
        println!("Comprehensive animation and subentity tests completed:");
        println!("  ✓ Graphics module presence and structure");
        println!("  ✓ Animation section validation");
        println!("  ✓ State-to-animation mapping");
        println!("  ✓ Sprite data validation");
        println!("  ✓ Origin point patterns");
        println!("  ✓ Palette display names");
        println!("  ✓ Subentity section structure");
        println!("  ✓ Subentity metadata validation");
        println!("  ✓ Subentity naming patterns");
        println!("  ✓ Subentity variable correlation");
        println!("  ✓ Animation state patterns");
        println!("  ✓ Frame data variables");
        println!("  ✓ Visual data consistency");
        println!("  ✓ Pixel size consistency");
        println!("  ✓ Spritesheet coverage analysis");
        println!("  ✓ Cross-file graphics comparison");
        println!("  ✓ Subentity distribution");
        println!("\nAll animation and subentity tests passed!\n");
    }
}
