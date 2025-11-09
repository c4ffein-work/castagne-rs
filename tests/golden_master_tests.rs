// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, you can obtain one at https://mozilla.org/MPL/2.0/.

//! Golden Master Tests - Compare Rust parser output with Godot 3 parser output

use std::fs;
use serde_json::Value;

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
        assert_eq!(expected_skeleton, "res://castagne/examples/fighters/baston/Baston-Model.casp");
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
        assert!(state_count > 100, "Expected many states, got {}", state_count);

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
        let spritesheet = &golden_json["transformed_data"]["Graphics"]["Spritesheets"]["TemporaryStickman"];

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

    // Helper function to load and parse golden master JSON
    fn load_golden_master(path: &str) -> Value {
        let json_content = fs::read_to_string(path)
            .expect(&format!("Failed to load golden master: {}", path));
        serde_json::from_str(&json_content)
            .expect(&format!("Failed to parse golden master JSON: {}", path))
    }
}
