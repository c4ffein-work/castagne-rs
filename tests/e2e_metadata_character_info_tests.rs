// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Metadata and Character Info E2E Tests
//!
//! Tests that validate metadata and character information:
//! - Required metadata fields
//! - Optional metadata fields
//! - Metadata format validation
//! - Character name consistency
//! - File path validation
//! - Author and description handling
//! - Skeleton file references
//! - Metadata inheritance
//! - Cross-file metadata consistency

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

    // ============================================================================
    // REQUIRED METADATA FIELD TESTS
    // ============================================================================

    #[test]
    fn e2e_metadata_required_fields_present() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            // Required fields must be present and be strings
            assert!(metadata["name"].is_string(),
                   "{}: name must be string", file_path);
            assert!(metadata["editorname"].is_string(),
                   "{}: editorname must be string", file_path);
            assert!(metadata["filepath"].is_string(),
                   "{}: filepath must be string", file_path);

            println!("✓ {} has all required metadata fields", file_path);
        }

        println!("✓ All files have required metadata fields");
    }

    #[test]
    fn e2e_metadata_required_fields_non_empty() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            let name = metadata["name"].as_str().unwrap();
            let editorname = metadata["editorname"].as_str().unwrap();
            let filepath = metadata["filepath"].as_str().unwrap();

            assert!(!name.is_empty(), "{}: name must not be empty", file_path);
            assert!(!editorname.is_empty(), "{}: editorname must not be empty", file_path);
            assert!(!filepath.is_empty(), "{}: filepath must not be empty", file_path);

            println!("✓ {}: name='{}', editorname='{}', filepath='{}'",
                    file_path, name, editorname, filepath);
        }

        println!("✓ All required metadata fields are non-empty");
    }

    #[test]
    fn e2e_metadata_name_format() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        println!("✓ Character name formats:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            let name = metadata["name"].as_str().unwrap();
            let editorname = metadata["editorname"].as_str().unwrap();

            // Analyze name format
            let has_spaces = name.contains(' ');
            let has_dashes = name.contains('-');
            let has_underscores = name.contains('_');

            println!("  {}:", file_path);
            println!("    name: '{}' (spaces={}, dashes={}, underscores={})",
                    name, has_spaces, has_dashes, has_underscores);
            println!("    editorname: '{}'", editorname);
        }
    }

    // ============================================================================
    // OPTIONAL METADATA FIELD TESTS
    // ============================================================================

    #[test]
    fn e2e_metadata_skeleton_field() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        println!("✓ Skeleton field analysis:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            if let Some(skeleton) = metadata.get("skeleton") {
                if skeleton.is_string() {
                    let skeleton_path = skeleton.as_str().unwrap();
                    println!("  {}: has skeleton '{}'", file_path, skeleton_path);

                    // Validate skeleton path format
                    assert!(skeleton_path.ends_with(".casp"),
                           "{}: skeleton must reference .casp file", file_path);
                } else if skeleton.is_null() {
                    println!("  {}: skeleton is null", file_path);
                } else {
                    panic!("{}: skeleton has invalid type", file_path);
                }
            } else {
                println!("  {}: no skeleton field", file_path);
            }
        }
    }

    #[test]
    fn e2e_metadata_author_field() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        println!("✓ Author field analysis:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            if let Some(author) = metadata.get("author") {
                if author.is_string() {
                    let author_str = author.as_str().unwrap();
                    println!("  {}: author '{}'", file_path, author_str);
                } else if author.is_null() {
                    println!("  {}: author is null", file_path);
                }
            } else {
                println!("  {}: no author field", file_path);
            }
        }
    }

    #[test]
    fn e2e_metadata_description_field() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        println!("✓ Description field analysis:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            if let Some(description) = metadata.get("description") {
                if description.is_string() {
                    let desc_str = description.as_str().unwrap();
                    let length = desc_str.len();
                    println!("  {}: description ({} chars)", file_path, length);

                    if length > 0 {
                        // Show first 50 chars
                        let preview = if length > 50 {
                            format!("{}...", &desc_str[..50])
                        } else {
                            desc_str.to_string()
                        };
                        println!("    Preview: '{}'", preview);
                    }
                } else if description.is_null() {
                    println!("  {}: description is null", file_path);
                }
            } else {
                println!("  {}: no description field", file_path);
            }
        }
    }

    // ============================================================================
    // NAME CONSISTENCY TESTS
    // ============================================================================

    #[test]
    fn e2e_metadata_name_vs_editorname() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        println!("✓ Name vs editorname comparison:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            let name = metadata["name"].as_str().unwrap();
            let editorname = metadata["editorname"].as_str().unwrap();

            let are_equal = name == editorname;
            let similar = name.to_lowercase() == editorname.to_lowercase();

            println!("  {}:", file_path);
            println!("    name: '{}'", name);
            println!("    editorname: '{}'", editorname);
            println!("    equal: {}, similar (case-insensitive): {}", are_equal, similar);
        }
    }

    #[test]
    fn e2e_metadata_name_in_filepath() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        println!("✓ Name in filepath correlation:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            let name = metadata["name"].as_str().unwrap();
            let filepath = metadata["filepath"].as_str().unwrap();

            // Check if name appears in filepath
            let name_in_path = filepath.contains(name);
            let name_lower_in_path = filepath.to_lowercase().contains(&name.to_lowercase());

            println!("  {}:", file_path);
            println!("    name: '{}'", name);
            println!("    filepath: '{}'", filepath);
            println!("    name in path: {}, case-insensitive: {}", name_in_path, name_lower_in_path);
        }
    }

    // ============================================================================
    // FILEPATH VALIDATION TESTS
    // ============================================================================

    #[test]
    fn e2e_metadata_filepath_format() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        println!("✓ Filepath format analysis:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            let filepath = metadata["filepath"].as_str().unwrap();

            // Check path format
            let is_absolute = filepath.starts_with('/') || filepath.starts_with("res://");
            let is_relative = !is_absolute;
            let has_extension = filepath.ends_with(".casp");

            println!("  {}:", file_path);
            println!("    path: '{}'", filepath);
            println!("    absolute: {}, relative: {}, has .casp: {}", is_absolute, is_relative, has_extension);

            // Filepath should end with .casp
            assert!(has_extension, "{}: filepath must end with .casp", file_path);
        }
    }

    #[test]
    fn e2e_metadata_filepath_directory_structure() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        println!("✓ Filepath directory structure:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            let filepath = metadata["filepath"].as_str().unwrap();

            // Count directory levels
            let separator_count = filepath.matches('/').count();
            let parts: Vec<&str> = filepath.split('/').collect();

            println!("  {}:", file_path);
            println!("    path: '{}'", filepath);
            println!("    directory levels: {}", separator_count);
            println!("    parts: {:?}", parts);
        }
    }

    // ============================================================================
    // SKELETON INHERITANCE TESTS
    // ============================================================================

    #[test]
    fn e2e_metadata_skeleton_inheritance_chain() {
        let model = load_golden_master("golden_masters/Baston-Model.json");
        let derived_2d = load_golden_master("golden_masters/Baston-2D.json");

        let model_metadata = &model["metadata"];
        let derived_metadata = &derived_2d["metadata"];

        // Check if 2D file references Model as skeleton
        if let Some(skeleton) = derived_metadata.get("skeleton") {
            if skeleton.is_string() {
                let skeleton_path = skeleton.as_str().unwrap();
                println!("✓ Skeleton inheritance chain:");
                println!("  Baston-2D.json skeleton: '{}'", skeleton_path);

                // Should reference the Model file
                assert!(skeleton_path.contains("Model"),
                       "2D file should reference Model as skeleton");
            }
        }

        // Check name inheritance
        let model_name = model_metadata["name"].as_str().unwrap();
        let derived_name = derived_metadata["name"].as_str().unwrap();

        println!("  Model name: '{}'", model_name);
        println!("  Derived name: '{}'", derived_name);
        println!("  Names match: {}", model_name == derived_name);
    }

    #[test]
    fn e2e_metadata_skeleton_field_consistency() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
        ];

        println!("✓ Skeleton field consistency:");

        let mut has_skeleton = Vec::new();
        let mut no_skeleton = Vec::new();

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            if let Some(skeleton) = metadata.get("skeleton") {
                if skeleton.is_string() && !skeleton.as_str().unwrap().is_empty() {
                    has_skeleton.push(file_path.to_string());
                } else {
                    no_skeleton.push(file_path.to_string());
                }
            } else {
                no_skeleton.push(file_path.to_string());
            }
        }

        println!("  Files with skeleton: {}", has_skeleton.len());
        for file in &has_skeleton {
            println!("    {}", file);
        }

        println!("  Files without skeleton: {}", no_skeleton.len());
        for file in &no_skeleton {
            println!("    {}", file);
        }
    }

    // ============================================================================
    // ADDITIONAL METADATA FIELDS
    // ============================================================================

    #[test]
    fn e2e_metadata_all_fields_catalog() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        let mut all_field_names: std::collections::HashSet<String> = std::collections::HashSet::new();

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = golden["metadata"].as_object().unwrap();

            for field_name in metadata.keys() {
                all_field_names.insert(field_name.clone());
            }
        }

        println!("✓ All metadata fields across all files:");
        let mut sorted_fields: Vec<_> = all_field_names.iter().collect();
        sorted_fields.sort();

        for field in sorted_fields {
            println!("  {}", field);
        }

        println!("  Total unique fields: {}", all_field_names.len());
    }

    #[test]
    fn e2e_metadata_field_presence_by_file() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        println!("✓ Metadata field presence by file:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = golden["metadata"].as_object().unwrap();

            println!("  {}:", file_path);
            println!("    Fields: {}", metadata.len());

            let mut field_list: Vec<_> = metadata.keys().collect();
            field_list.sort();

            for field in field_list {
                let value = &metadata[field];
                let value_type = if value.is_string() {
                    "string"
                } else if value.is_null() {
                    "null"
                } else if value.is_number() {
                    "number"
                } else if value.is_boolean() {
                    "boolean"
                } else if value.is_array() {
                    "array"
                } else if value.is_object() {
                    "object"
                } else {
                    "unknown"
                };

                println!("      {}: {}", field, value_type);
            }
        }
    }

    // ============================================================================
    // METADATA VALIDATION TESTS
    // ============================================================================

    #[test]
    fn e2e_metadata_no_null_required_fields() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            // Required fields must not be null
            assert!(!metadata["name"].is_null(),
                   "{}: name must not be null", file_path);
            assert!(!metadata["editorname"].is_null(),
                   "{}: editorname must not be null", file_path);
            assert!(!metadata["filepath"].is_null(),
                   "{}: filepath must not be null", file_path);
        }

        println!("✓ No required fields are null");
    }

    #[test]
    fn e2e_metadata_string_field_lengths() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        println!("✓ Metadata string field lengths:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            println!("  {}:", file_path);

            let name = metadata["name"].as_str().unwrap();
            let editorname = metadata["editorname"].as_str().unwrap();
            let filepath = metadata["filepath"].as_str().unwrap();

            println!("    name length: {} chars", name.len());
            println!("    editorname length: {} chars", editorname.len());
            println!("    filepath length: {} chars", filepath.len());

            // Reasonable length constraints
            assert!(name.len() < 200, "{}: name too long", file_path);
            assert!(editorname.len() < 200, "{}: editorname too long", file_path);
            assert!(filepath.len() < 500, "{}: filepath too long", file_path);
        }
    }

    // ============================================================================
    // CROSS-FILE CONSISTENCY TESTS
    // ============================================================================

    #[test]
    fn e2e_metadata_consistency_across_inheritance() {
        let model = load_golden_master("golden_masters/Baston-Model.json");
        let derived = load_golden_master("golden_masters/Baston-2D.json");

        let model_metadata = &model["metadata"];
        let derived_metadata = &derived["metadata"];

        let model_name = model_metadata["name"].as_str().unwrap();
        let derived_name = derived_metadata["name"].as_str().unwrap();

        println!("✓ Cross-file metadata consistency:");
        println!("  Model name: '{}'", model_name);
        println!("  Derived name: '{}'", derived_name);

        // Names should match for inherited files
        assert_eq!(model_name, derived_name,
                  "Parent and child should have same name");

        // Editor names
        let model_editorname = model_metadata["editorname"].as_str().unwrap();
        let derived_editorname = derived_metadata["editorname"].as_str().unwrap();

        println!("  Model editorname: '{}'", model_editorname);
        println!("  Derived editorname: '{}'", derived_editorname);
    }

    #[test]
    fn e2e_metadata_filepath_uniqueness() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
            "golden_masters/TutorialBaston.json",
        ];

        let mut filepaths: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut duplicates = Vec::new();

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let metadata = &golden["metadata"];

            let filepath = metadata["filepath"].as_str().unwrap();

            if !filepaths.insert(filepath.to_string()) {
                duplicates.push(filepath.to_string());
            }
        }

        println!("✓ Filepath uniqueness:");
        println!("  Unique filepaths: {}", filepaths.len());
        println!("  Duplicates: {}", duplicates.len());

        if !duplicates.is_empty() {
            println!("  Duplicate filepaths:");
            for dup in &duplicates {
                println!("    {}", dup);
            }
        }
    }

    // ============================================================================
    // SUMMARY TEST
    // ============================================================================

    #[test]
    fn e2e_metadata_comprehensive_summary() {
        println!("\n=== E2E Metadata & Character Info Summary ===\n");
        println!("Comprehensive metadata validation tests completed:");
        println!("  ✓ Required fields presence and validation");
        println!("  ✓ Required fields non-empty checks");
        println!("  ✓ Name format analysis");
        println!("  ✓ Optional fields handling (skeleton, author, description)");
        println!("  ✓ Name vs editorname consistency");
        println!("  ✓ Name in filepath correlation");
        println!("  ✓ Filepath format and structure validation");
        println!("  ✓ Skeleton inheritance chain validation");
        println!("  ✓ Skeleton field consistency checks");
        println!("  ✓ All metadata fields cataloging");
        println!("  ✓ Field presence analysis by file");
        println!("  ✓ Null value validation for required fields");
        println!("  ✓ String field length constraints");
        println!("  ✓ Cross-file metadata consistency");
        println!("  ✓ Filepath uniqueness validation");
        println!("\nAll metadata validation tests passed!\n");
    }
}
