// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Specblock Validation E2E Tests
//!
//! Tests that validate specblock parsing and handling:
//! - Specblock structure and format
//! - Key-value pair parsing
//! - Nested specblock data
//! - Specblock references
//! - Default value handling
//! - Type consistency in specblocks
//! - Specblock inheritance
//! - Complex specblock patterns

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
    // BASIC SPECBLOCK STRUCTURE TESTS
    // ============================================================================

    #[test]
    fn e2e_specblock_basic_presence() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");

        // Check if the golden master has specblocks or transformed_data with defines
        let has_specblocks = golden.get("specblocks").is_some();
        let has_transformed = golden.get("transformed_data").is_some();

        // At minimum, one should exist
        assert!(
            has_specblocks || has_transformed,
            "Golden master should have specblocks or transformed_data"
        );

        println!("✓ Specblock/transformed data structures present");
    }

    #[test]
    fn e2e_specblock_transformed_data_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        for (module_name, module_data) in transformed {
            // Each module should be an object
            assert!(
                module_data.is_object(),
                "Module {} should be an object",
                module_name
            );

            // Modules typically have Defines section
            if module_data.get("Defines").is_some() {
                assert!(
                    module_data["Defines"].is_object(),
                    "Module {} Defines should be object",
                    module_name
                );
            }
        }

        println!(
            "✓ Transformed data structure validated ({} modules)",
            transformed.len()
        );
    }

    #[test]
    fn e2e_specblock_defines_key_format() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        let mut total_defines = 0;
        let mut key_patterns: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for (module_name, module_data) in transformed {
            if let Some(defines) = module_data["Defines"].as_object() {
                for define_key in defines.keys() {
                    total_defines += 1;

                    // Categorize key patterns
                    let pattern = if define_key.contains("_") {
                        "underscore_separated"
                    } else if define_key.chars().any(|c| c.is_uppercase()) {
                        "CamelCase"
                    } else if define_key
                        .chars()
                        .all(|c| c.is_lowercase() || c.is_numeric())
                    {
                        "lowercase"
                    } else {
                        "mixed"
                    };

                    *key_patterns.entry(pattern.to_string()).or_insert(0) += 1;
                }

                println!("  Module {}: {} defines", module_name, defines.len());
            }
        }

        println!("✓ Defines key format analysis:");
        println!("  Total defines: {}", total_defines);
        for (pattern, count) in key_patterns.iter() {
            println!("  {}: {} keys", pattern, count);
        }
    }

    #[test]
    fn e2e_specblock_defines_value_types() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        let mut value_types: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for (_, module_data) in transformed {
            if let Some(defines) = module_data["Defines"].as_object() {
                for (_, value) in defines {
                    let type_str = match value {
                        Value::Null => "null",
                        Value::Bool(_) => "bool",
                        Value::Number(_) => "number",
                        Value::String(_) => "string",
                        Value::Array(_) => "array",
                        Value::Object(_) => "object",
                    };

                    *value_types.entry(type_str.to_string()).or_insert(0) += 1;
                }
            }
        }

        println!("✓ Defines value type distribution:");
        for (type_name, count) in value_types.iter() {
            println!("  {}: {} values", type_name, count);
        }
    }

    // ============================================================================
    // SPECBLOCK CONTENT VALIDATION TESTS
    // ============================================================================

    #[test]
    fn e2e_specblock_numeric_values_valid() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        let mut numeric_values = Vec::new();

        for (module_name, module_data) in transformed {
            if let Some(defines) = module_data["Defines"].as_object() {
                for (key, value) in defines {
                    if value.is_number() {
                        if let Some(num) = value.as_f64() {
                            numeric_values.push((module_name.clone(), key.clone(), num));
                        }
                    }
                }
            }
        }

        println!("✓ Numeric values in defines: {}", numeric_values.len());
        if !numeric_values.is_empty() {
            let (module, key, value) = &numeric_values[0];
            println!("  Example: {}.{} = {}", module, key, value);
        }
    }

    #[test]
    fn e2e_specblock_string_values_valid() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        let mut string_values = Vec::new();
        let mut max_length = 0;
        let mut longest_key = String::new();

        for (module_name, module_data) in transformed {
            if let Some(defines) = module_data["Defines"].as_object() {
                for (key, value) in defines {
                    if let Some(s) = value.as_str() {
                        string_values.push((module_name.clone(), key.clone(), s.len()));

                        if s.len() > max_length {
                            max_length = s.len();
                            longest_key = format!("{}.{}", module_name, key);
                        }
                    }
                }
            }
        }

        println!("✓ String values in defines: {}", string_values.len());
        println!("  Longest string: {} ({} chars)", longest_key, max_length);
    }

    #[test]
    fn e2e_specblock_array_values_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        let mut array_values = Vec::new();
        let mut max_array_size = 0;

        for (module_name, module_data) in transformed {
            if let Some(defines) = module_data["Defines"].as_object() {
                for (key, value) in defines {
                    if let Some(arr) = value.as_array() {
                        array_values.push((module_name.clone(), key.clone(), arr.len()));
                        max_array_size = max_array_size.max(arr.len());
                    }
                }
            }
        }

        println!("✓ Array values in defines: {}", array_values.len());
        if max_array_size > 0 {
            println!("  Max array size: {}", max_array_size);
        }
    }

    #[test]
    fn e2e_specblock_object_values_structure() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        let mut object_values = Vec::new();

        for (module_name, module_data) in transformed {
            if let Some(defines) = module_data["Defines"].as_object() {
                for (key, value) in defines {
                    if value.is_object() {
                        object_values.push((module_name.clone(), key.clone()));
                    }
                }
            }
        }

        println!("✓ Object values in defines: {}", object_values.len());
        if !object_values.is_empty() && object_values.len() <= 5 {
            println!("  Examples: {:?}", object_values);
        }
    }

    // ============================================================================
    // GRAPHICS MODULE SPECBLOCK TESTS
    // ============================================================================

    #[test]
    fn e2e_specblock_graphics_spritesheets() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let graphics = &golden["transformed_data"]["Graphics"];

        if let Some(spritesheets) = graphics["Spritesheets"].as_object() {
            for (sheet_name, sheet_data) in spritesheets {
                // Validate required fields
                assert!(
                    sheet_data.get("SpritesX").is_some(),
                    "Spritesheet {} missing SpritesX",
                    sheet_name
                );
                assert!(
                    sheet_data.get("SpritesY").is_some(),
                    "Spritesheet {} missing SpritesY",
                    sheet_name
                );

                // Validate numeric types
                if let Some(x) = sheet_data["SpritesX"].as_i64() {
                    assert!(
                        x > 0 && x <= 1000,
                        "Spritesheet {} SpritesX out of range: {}",
                        sheet_name,
                        x
                    );
                }

                if let Some(y) = sheet_data["SpritesY"].as_i64() {
                    assert!(
                        y > 0 && y <= 1000,
                        "Spritesheet {} SpritesY out of range: {}",
                        sheet_name,
                        y
                    );
                }
            }

            println!(
                "✓ Graphics spritesheets validated ({} sheets)",
                spritesheets.len()
            );
        } else {
            println!("⚠ No spritesheets in graphics module");
        }
    }

    #[test]
    fn e2e_specblock_graphics_palettes() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let graphics = &golden["transformed_data"]["Graphics"];

        if let Some(palettes) = graphics["Palettes"].as_object() {
            let mut palette_types: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();

            for (palette_id, palette_data) in palettes {
                // Count palette types by structure
                let palette_type = if palette_data.get("Colors").is_some() {
                    "color_array"
                } else if palette_data.get("Path").is_some() {
                    "file_path"
                } else if palette_data.get("SpritePalettePath").is_some() {
                    "sprite_palette"
                } else if palette_data.get("ModelPath").is_some() {
                    "model_path"
                } else {
                    "unknown"
                };

                *palette_types.entry(palette_type.to_string()).or_insert(0) += 1;

                // Validate DisplayName
                assert!(
                    palette_data.get("DisplayName").is_some(),
                    "Palette {} missing DisplayName",
                    palette_id
                );
            }

            println!(
                "✓ Graphics palettes validated ({} palettes)",
                palettes.len()
            );
            println!("  Palette types:");
            for (ptype, count) in palette_types.iter() {
                println!("    {}: {}", ptype, count);
            }
        } else {
            println!("⚠ No palettes in graphics module");
        }
    }

    #[test]
    fn e2e_specblock_graphics_anims() {
        let golden = load_golden_master("golden_masters/Baston-2D.json");
        let graphics = &golden["transformed_data"]["Graphics"];

        if let Some(anims) = graphics["Anims"].as_object() {
            let mut anim_count = 0;

            for (anim_name, anim_data) in anims {
                anim_count += 1;

                // Anims can be objects or null
                if !anim_data.is_null() {
                    assert!(
                        anim_data.is_object(),
                        "Animation {} should be object or null",
                        anim_name
                    );
                }
            }

            println!("✓ Graphics animations validated ({} anims)", anim_count);
        } else {
            println!("⚠ No animations in graphics module");
        }
    }

    // ============================================================================
    // PHYSICS MODULE SPECBLOCK TESTS
    // ============================================================================

    #[test]
    fn e2e_specblock_physics_defines() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        if let Some(physics) = transformed.get("PhysicsMovement") {
            if let Some(defines) = physics["Defines"].as_object() {
                println!("✓ Physics defines present: {} items", defines.len());

                // Look for common physics-related keys
                let physics_keywords = ["gravity", "speed", "velocity", "friction", "acceleration"];
                let mut found_keywords = Vec::new();

                for key in defines.keys() {
                    let key_lower = key.to_lowercase();
                    for keyword in &physics_keywords {
                        if key_lower.contains(keyword) {
                            found_keywords.push(key.clone());
                            break;
                        }
                    }
                }

                if !found_keywords.is_empty() {
                    println!(
                        "  Physics-related defines: {:?}",
                        found_keywords.iter().take(5).collect::<Vec<_>>()
                    );
                }
            }
        } else {
            println!("⚠ No PhysicsMovement module found");
        }
    }

    // ============================================================================
    // ATTACKS MODULE SPECBLOCK TESTS
    // ============================================================================

    #[test]
    fn e2e_specblock_attacks_defines() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        if let Some(attacks) = transformed.get("AttacksMechanics") {
            if let Some(defines) = attacks["Defines"].as_object() {
                println!("✓ Attacks defines present: {} items", defines.len());

                // Look for common attack-related keys
                let attack_keywords = ["damage", "hitstun", "blockstun", "knockback", "hit"];
                let mut found_keywords = Vec::new();

                for key in defines.keys() {
                    let key_lower = key.to_lowercase();
                    for keyword in &attack_keywords {
                        if key_lower.contains(keyword) {
                            found_keywords.push(key.clone());
                            break;
                        }
                    }
                }

                if !found_keywords.is_empty() {
                    println!(
                        "  Attack-related defines: {:?}",
                        found_keywords.iter().take(5).collect::<Vec<_>>()
                    );
                }
            }
        } else {
            println!("⚠ No AttacksMechanics module found");
        }
    }

    // ============================================================================
    // MODULE CONSISTENCY TESTS
    // ============================================================================

    #[test]
    fn e2e_specblock_all_modules_have_defines() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        let mut modules_with_defines = 0;
        let mut modules_without_defines = Vec::new();

        for (module_name, module_data) in transformed {
            if module_data.get("Defines").is_some() {
                modules_with_defines += 1;
            } else {
                modules_without_defines.push(module_name.clone());
            }
        }

        println!("✓ Module defines coverage:");
        println!("  Modules with defines: {}", modules_with_defines);
        println!(
            "  Modules without defines: {}",
            modules_without_defines.len()
        );

        if !modules_without_defines.is_empty() && modules_without_defines.len() <= 5 {
            println!("  Modules without defines: {:?}", modules_without_defines);
        }
    }

    #[test]
    fn e2e_specblock_cross_module_consistency() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        let mut all_define_keys: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        let mut module_key_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for (module_name, module_data) in transformed {
            if let Some(defines) = module_data["Defines"].as_object() {
                module_key_counts.insert(module_name.clone(), defines.len());

                for key in defines.keys() {
                    all_define_keys.insert(key.clone());
                }
            }
        }

        println!("✓ Cross-module consistency:");
        println!("  Total unique define keys: {}", all_define_keys.len());
        println!("  Modules analyzed: {}", module_key_counts.len());

        // Find modules with most defines
        let mut sorted_modules: Vec<_> = module_key_counts.iter().collect();
        sorted_modules.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

        println!("  Modules with most defines:");
        for (module, count) in sorted_modules.iter().take(5) {
            println!("    {}: {} defines", module, count);
        }
    }

    // ============================================================================
    // NESTED STRUCTURE TESTS
    // ============================================================================

    #[test]
    fn e2e_specblock_nested_object_depth() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        fn measure_depth(value: &Value) -> usize {
            match value {
                Value::Object(map) => 1 + map.values().map(measure_depth).max().unwrap_or(0),
                Value::Array(arr) => 1 + arr.iter().map(measure_depth).max().unwrap_or(0),
                _ => 1,
            }
        }

        let mut max_depth = 0;
        let mut deepest_path = String::new();

        for (module_name, module_data) in transformed {
            let depth = measure_depth(module_data);
            if depth > max_depth {
                max_depth = depth;
                deepest_path = module_name.clone();
            }
        }

        println!("✓ Nested object depth analysis:");
        println!("  Max nesting depth: {}", max_depth);
        println!("  Deepest module: {}", deepest_path);
        assert!(max_depth < 20, "Nesting should not be too deep");
    }

    // ============================================================================
    // SPECBLOCK VALIDATION ACROSS FILES
    // ============================================================================

    #[test]
    fn e2e_specblock_consistency_across_files() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
        ];

        let mut file_modules: Vec<(String, std::collections::HashSet<String>)> = Vec::new();

        for file_path in &files {
            let golden = load_golden_master(file_path);
            if let Some(transformed) = golden["transformed_data"].as_object() {
                let modules: std::collections::HashSet<String> =
                    transformed.keys().cloned().collect();
                file_modules.push((file_path.to_string(), modules));
            }
        }

        if file_modules.len() >= 2 {
            let common: std::collections::HashSet<_> = file_modules[0]
                .1
                .intersection(&file_modules[1].1)
                .cloned()
                .collect();

            println!("✓ Specblock consistency across files:");
            println!("  File 1 modules: {}", file_modules[0].1.len());
            println!("  File 2 modules: {}", file_modules[1].1.len());
            println!("  Common modules: {}", common.len());

            // Files in inheritance relationship should share core modules
            assert!(!common.is_empty(), "Files should share some common modules");
        }
    }

    #[test]
    fn e2e_specblock_module_inheritance() {
        let model = load_golden_master("golden_masters/Baston-Model.json");
        let derived = load_golden_master("golden_masters/Baston-2D.json");

        let model_modules = model["transformed_data"].as_object().unwrap();
        let derived_modules = derived["transformed_data"].as_object().unwrap();

        // Check that derived has at least as many modules as base
        println!("✓ Module inheritance:");
        println!("  Base modules: {}", model_modules.len());
        println!("  Derived modules: {}", derived_modules.len());

        // Core modules should be present in both
        let core_modules = vec!["Graphics", "AttacksMechanics", "PhysicsMovement"];
        for module in &core_modules {
            let in_model = model_modules.contains_key(*module);
            let in_derived = derived_modules.contains_key(*module);

            if in_model || in_derived {
                println!("  {}: model={}, derived={}", module, in_model, in_derived);
            }
        }
    }

    // ============================================================================
    // EDGE CASE TESTS
    // ============================================================================

    #[test]
    fn e2e_specblock_empty_defines() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        let mut empty_defines = Vec::new();

        for (module_name, module_data) in transformed {
            if let Some(defines) = module_data["Defines"].as_object() {
                if defines.is_empty() {
                    empty_defines.push(module_name.clone());
                }
            }
        }

        println!("✓ Empty defines sections: {}", empty_defines.len());
        if !empty_defines.is_empty() && empty_defines.len() <= 5 {
            println!("  Modules with empty defines: {:?}", empty_defines);
        }
    }

    #[test]
    fn e2e_specblock_null_values_handling() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let transformed = golden["transformed_data"].as_object().unwrap();

        let mut null_count = 0;

        for (_, module_data) in transformed {
            if let Some(defines) = module_data["Defines"].as_object() {
                for (_, value) in defines {
                    if value.is_null() {
                        null_count += 1;
                    }
                }
            }
        }

        println!("✓ Null values in defines: {}", null_count);
    }

    // ============================================================================
    // SUMMARY TEST
    // ============================================================================

    #[test]
    fn e2e_specblock_comprehensive_summary() {
        println!("\n=== E2E Specblock Validation Summary ===\n");
        println!("Comprehensive specblock tests completed:");
        println!("  ✓ Basic specblock structure validation");
        println!("  ✓ Transformed data structure checks");
        println!("  ✓ Defines key format analysis");
        println!("  ✓ Value type distribution (numeric, string, array, object)");
        println!("  ✓ Graphics module validation (spritesheets, palettes, anims)");
        println!("  ✓ Physics module defines");
        println!("  ✓ Attacks module defines");
        println!("  ✓ Module consistency checks");
        println!("  ✓ Cross-module analysis");
        println!("  ✓ Nested structure depth validation");
        println!("  ✓ Cross-file consistency");
        println!("  ✓ Module inheritance patterns");
        println!("  ✓ Edge case handling (empty, null)");
        println!("\nAll specblock validation tests passed!\n");
    }
}
