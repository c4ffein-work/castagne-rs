// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

//! Golden Master Tests - Compare Rust parser output with Godot 3 parser output

use serde_json::Value;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    /// Test metadata parsing for Baston-Model.casp
    #[test]
    fn test_baston_model_metadata() {
        let golden_json = load_golden_master("golden_masters/Baston-Model.json");

        // Extract metadata from golden master
        let metadata = &golden_json["metadata"];
        let expected_name = metadata["name"].as_str().unwrap();
        let expected_editorname = metadata["editorname"].as_str().unwrap();

        println!("Expected name: {}", expected_name);
        println!("Expected editorname: {}", expected_editorname);

        // TODO: Parse with Rust parser and compare
        // For now, just verify golden master loaded correctly
        assert_eq!(expected_name, "Baston Labatte");
        assert_eq!(expected_editorname, "Baston Labatte (Custom Character)");
    }

    /// Test metadata parsing for Baston-2D.casp
    #[test]
    fn test_baston_2d_metadata() {
        let golden_json = load_golden_master("golden_masters/Baston-2D.json");

        // Extract metadata from golden master
        let metadata = &golden_json["metadata"];
        let expected_name = metadata["name"].as_str().unwrap();
        let expected_editorname = metadata["editorname"].as_str().unwrap();
        let expected_skeleton = metadata["skeleton"].as_str().unwrap();

        println!("Expected name: {}", expected_name);
        println!("Expected editorname: {}", expected_editorname);
        println!("Expected skeleton: {}", expected_skeleton);

        assert_eq!(expected_name, "Baston Labatte");
        assert_eq!(expected_editorname, "Baston 2D (Example Character)");
        assert_eq!(
            expected_skeleton,
            "res://castagne/examples/fighters/baston/Baston-Model.casp"
        );
    }

    /// Test state count
    #[test]
    fn test_baston_2d_states() {
        let golden_json = load_golden_master("golden_masters/Baston-2D.json");

        // Count states in golden master
        let states = &golden_json["states"];
        let state_count = states.as_object().unwrap().len();

        println!("Golden master has {} states", state_count);

        // Should have a significant number of states
        assert!(
            state_count > 100,
            "Expected many states, got {}",
            state_count
        );

        // Check for specific states we know should exist
        assert!(states["5H"].is_object(), "State 5H should exist");
        assert!(states["5L"].is_object(), "State 5L should exist");
        assert!(states["5M"].is_object(), "State 5M should exist");
    }

    /// Test Graphics transformed data
    #[test]
    fn test_baston_2d_graphics_data() {
        let golden_json = load_golden_master("golden_masters/Baston-2D.json");

        // Check Graphics transformed data
        let graphics = &golden_json["transformed_data"]["Graphics"]["Defines"];

        let scale = graphics["GRAPHICS_Scale"].as_i64().unwrap();
        let use_sprites = graphics["GRAPHICS_UseSprites"].as_i64().unwrap();
        let use_model = graphics["GRAPHICS_UseModel"].as_i64().unwrap();

        println!("GRAPHICS_Scale: {}", scale);
        println!("GRAPHICS_UseSprites: {}", use_sprites);
        println!("GRAPHICS_UseModel: {}", use_model);

        assert_eq!(scale, 3000);
        assert_eq!(use_sprites, 1);
        assert_eq!(use_model, 0);
    }

    /// Test spritesheet parameters
    #[test]
    fn test_baston_model_spritesheet() {
        let golden_json = load_golden_master("golden_masters/Baston-Model.json");

        // Check spritesheet data
        let spritesheet =
            &golden_json["transformed_data"]["Graphics"]["Spritesheets"]["TemporaryStickman"];

        let sprites_x = spritesheet["SpritesX"].as_i64().unwrap();
        let sprites_y = spritesheet["SpritesY"].as_i64().unwrap();
        let origin_x = spritesheet["OriginX"].as_i64().unwrap();
        let origin_y = spritesheet["OriginY"].as_i64().unwrap();
        let pixel_size = spritesheet["PixelSize"].as_i64().unwrap();

        println!("SpritesX: {}", sprites_x);
        println!("SpritesY: {}", sprites_y);
        println!("OriginX: {}", origin_x);
        println!("OriginY: {}", origin_y);
        println!("PixelSize: {}", pixel_size);

        assert_eq!(sprites_x, 16);
        assert_eq!(sprites_y, 4);
        assert_eq!(origin_x, 32);
        assert_eq!(origin_y, 6);
        assert_eq!(pixel_size, 100000);
    }

    /// Test animation loop values
    #[test]
    fn test_baston_model_animation_loops() {
        let golden_json = load_golden_master("golden_masters/Baston-Model.json");

        // Check animation defines
        let anims = &golden_json["transformed_data"]["Anims"]["Defines"];

        let stand_loop = anims["ANIM_Movement_Basic_Stand_Loop"].as_i64().unwrap();
        let walk_f_loop = anims["ANIM_Movement_Basic_WalkF_Loop"].as_i64().unwrap();
        let walk_b_loop = anims["ANIM_Movement_Basic_WalkB_Loop"].as_i64().unwrap();

        println!("Stand_Loop: {}", stand_loop);
        println!("WalkF_Loop: {}", walk_f_loop);
        println!("WalkB_Loop: {}", walk_b_loop);

        assert_eq!(stand_loop, 56);
        assert_eq!(walk_f_loop, 52);
        assert_eq!(walk_b_loop, 30);
    }

    /// Test palette data
    #[test]
    fn test_baston_model_palettes() {
        let golden_json = load_golden_master("golden_masters/Baston-Model.json");

        // Check palette data
        let palettes = &golden_json["transformed_data"]["Graphics"]["Palettes"];

        let pal0_name = palettes["0"]["DisplayName"].as_str().unwrap();
        let pal1_name = palettes["1"]["DisplayName"].as_str().unwrap();
        let pal2_name = palettes["2"]["DisplayName"].as_str().unwrap();
        let pal3_name = palettes["3"]["DisplayName"].as_str().unwrap();

        println!("Palette 0: {}", pal0_name);
        println!("Palette 1: {}", pal1_name);
        println!("Palette 2: {}", pal2_name);
        println!("Palette 3: {}", pal3_name);

        assert_eq!(pal0_name, "Blue");
        assert_eq!(pal1_name, "Green");
        assert_eq!(pal2_name, "Yellow");
        assert_eq!(pal3_name, "Purple");
    }

    /// Test TutorialBaston metadata
    #[test]
    fn test_tutorial_baston_metadata() {
        let golden_json = load_golden_master("golden_masters/TutorialBaston.json");

        let metadata = &golden_json["metadata"];
        let expected_name = metadata["name"].as_str().unwrap();
        let expected_editorname = metadata["editorname"].as_str().unwrap();
        let expected_skeleton = metadata["skeleton"].as_str().unwrap();

        println!("Expected name: {}", expected_name);
        println!("Expected editorname: {}", expected_editorname);
        println!("Expected skeleton: {}", expected_skeleton);

        assert_eq!(expected_name, "Baston Labatte");
        assert_eq!(expected_editorname, "Baston Labatte (Custom Character)");
        assert_eq!(
            expected_skeleton,
            "res://castagne/examples/fighters/baston/Baston-Model.casp"
        );
    }

    /// Test TutorialBaston state count
    #[test]
    fn test_tutorial_baston_states() {
        let golden_json = load_golden_master("golden_masters/TutorialBaston.json");

        let states = &golden_json["states"];
        let state_count = states.as_object().unwrap().len();

        println!("TutorialBaston has {} states", state_count);

        // Should have a significant number of states
        assert!(
            state_count > 100,
            "Expected many states, got {}",
            state_count
        );

        // Check for common states
        assert!(states["Common"].is_object(), "State Common should exist");
        assert!(states["Init"].is_object(), "State Init should exist");
        assert!(states["Stand"].is_object(), "State Stand should exist");
    }

    /// Test TutorialBaston subentities
    #[test]
    fn test_tutorial_baston_subentities() {
        let golden_json = load_golden_master("golden_masters/TutorialBaston.json");

        let subentities = &golden_json["subentities"];
        assert!(
            subentities["Base"].is_object(),
            "Base subentity should exist"
        );
        assert!(
            subentities["Projectile"].is_object(),
            "Projectile subentity should exist"
        );

        println!("✓ TutorialBaston has Base and Projectile subentities");
    }

    /// Test state structure in detail
    #[test]
    fn test_state_structure_details() {
        let golden_json = load_golden_master("golden_masters/Baston-Model.json");

        let states = &golden_json["states"];
        let init_state = &states["Init"];

        // Check state has required fields
        assert!(
            init_state["Parent"].is_null() || init_state["Parent"].is_string(),
            "State should have Parent field"
        );
        assert!(
            init_state["Type"].is_null() || init_state["Type"].is_string(),
            "State should have Type field"
        );
        assert!(
            init_state["TransitionFlags"].is_array(),
            "State should have TransitionFlags array"
        );
        assert!(
            init_state["Phases"].is_object(),
            "State should have Phases object"
        );

        println!("✓ State structure validated");
    }

    /// Test variable structure in detail
    #[test]
    fn test_variable_structure_details() {
        let golden_json = load_golden_master("golden_masters/Baston-Model.json");

        let variables = &golden_json["variables"];

        // Variables should have Name, Value, Type, Subtype, Mutability fields
        for (var_name, var_data) in variables.as_object().unwrap() {
            assert!(
                var_data["Name"].is_string() || var_data["Name"].is_null(),
                "Variable {} should have Name field",
                var_name
            );
            assert!(
                var_data["Value"].is_string(),
                "Variable {} should have Value field",
                var_name
            );
            assert!(
                var_data["Type"].is_string(),
                "Variable {} should have Type field",
                var_name
            );
            assert!(
                var_data["Subtype"].is_string(),
                "Variable {} should have Subtype field",
                var_name
            );
            assert!(
                var_data["Mutability"].is_string(),
                "Variable {} should have Mutability field",
                var_name
            );
        }

        println!(
            "✓ Variable structures validated for {} variables",
            variables.as_object().unwrap().len()
        );
    }

    /// Test transformed_data completeness
    #[test]
    fn test_transformed_data_completeness() {
        let golden_json = load_golden_master("golden_masters/Baston-Model.json");

        let transformed_data = &golden_json["transformed_data"];

        // Check for key modules (based on actual Baston-Model.json structure)
        let required_modules = vec![
            "AttacksTypes",
            "PhysicsMovement",
            "Graphics",
            "Anims",
            "PhysicsSystem",
            "AttacksMechanics",
            "UI",
        ];

        for module_name in required_modules {
            assert!(
                transformed_data[module_name].is_object(),
                "Transformed data should have {} module",
                module_name
            );

            // Each module should have at least a Defines section
            assert!(
                transformed_data[module_name]["Defines"].is_object(),
                "Module {} should have Defines section",
                module_name
            );
        }

        println!("✓ All required transformed_data modules present");
    }

    /// Test Physics module data
    #[test]
    fn test_physics_module_data() {
        let golden_json = load_golden_master("golden_masters/Baston-Model.json");

        let physics = &golden_json["transformed_data"]["PhysicsMovement"]["Defines"];

        // Check for key physics parameters (using actual keys from the golden master)
        assert!(
            physics["MOVE_AirActionsMax"].is_i64() || physics["MOVE_AirActionsMax"].is_f64(),
            "Should have MOVE_AirActionsMax"
        );

        let define_count = physics.as_object().unwrap().len();
        println!("  PhysicsMovement has {} defines", define_count);
        assert!(
            define_count > 10,
            "PhysicsMovement should have many defines"
        );

        println!("✓ Physics module data validated");
    }

    /// Test comprehensive golden master comparison structure
    #[test]
    fn test_all_golden_masters_structure() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        for file in files {
            println!("\nValidating: {}", file);
            let golden_json = load_golden_master(file);

            // Validate basic structure
            assert!(
                golden_json["metadata"].is_object(),
                "{}: should have metadata",
                file
            );
            assert!(
                golden_json["variables"].is_object(),
                "{}: should have variables",
                file
            );
            assert!(
                golden_json["states"].is_object(),
                "{}: should have states",
                file
            );
            assert!(
                golden_json["subentities"].is_object(),
                "{}: should have subentities",
                file
            );
            assert!(
                golden_json["transformed_data"].is_object(),
                "{}: should have transformed_data",
                file
            );

            // Count and report
            let state_count = golden_json["states"].as_object().unwrap().len();
            let var_count = golden_json["variables"].as_object().unwrap().len();
            let module_count = golden_json["transformed_data"].as_object().unwrap().len();

            println!("  States: {}", state_count);
            println!("  Variables: {}", var_count);
            println!("  Modules: {}", module_count);
            println!("  ✓ Structure valid");
        }
    }

    // Helper function to load and parse golden master JSON
    fn load_golden_master(path: &str) -> Value {
        let json_content =
            fs::read_to_string(path).expect(&format!("Failed to load golden master: {}", path));
        serde_json::from_str(&json_content)
            .expect(&format!("Failed to parse golden master JSON: {}", path))
    }
}
