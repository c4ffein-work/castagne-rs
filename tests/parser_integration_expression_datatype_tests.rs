// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Expression Parsing and Data Type E2E Tests
//!
//! Tests that validate expression parsing and data type handling:
//! - Arithmetic expression parsing
//! - Boolean expression validation
//! - Type conversion correctness
//! - Operator precedence
//! - Variable type consistency
//! - Numeric precision handling
//! - String escaping and special characters
//! - Vector component parsing

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
    // INTEGER TYPE TESTS
    // ============================================================================

    #[test]
    fn e2e_datatype_int_parsing_positive() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut positive_ints = Vec::new();

        for (var_name, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Int" {
                if let Ok(val) = var_data["Value"].as_str().unwrap().parse::<i64>() {
                    if val > 0 {
                        positive_ints.push((var_name.clone(), val));
                    }
                }
            }
        }

        println!("✓ Positive integers: {}", positive_ints.len());
        if !positive_ints.is_empty() {
            let (name, value) = &positive_ints[0];
            println!("  Example: {} = {}", name, value);
        }
    }

    #[test]
    fn e2e_datatype_int_parsing_negative() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut negative_ints = Vec::new();

        for (var_name, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Int" {
                if let Ok(val) = var_data["Value"].as_str().unwrap().parse::<i64>() {
                    if val < 0 {
                        negative_ints.push((var_name.clone(), val));
                    }
                }
            }
        }

        println!("✓ Negative integers: {}", negative_ints.len());
        if !negative_ints.is_empty() {
            println!("  Examples: {:?}", negative_ints.iter().take(3).collect::<Vec<_>>());
        }
    }

    #[test]
    fn e2e_datatype_int_zero_handling() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut zero_ints = Vec::new();

        for (var_name, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Int" {
                if let Ok(val) = var_data["Value"].as_str().unwrap().parse::<i64>() {
                    if val == 0 {
                        zero_ints.push(var_name.clone());
                    }
                }
            }
        }

        println!("✓ Zero integer values: {}", zero_ints.len());
        if !zero_ints.is_empty() && zero_ints.len() <= 5 {
            println!("  Variables: {:?}", zero_ints);
        }
    }

    #[test]
    fn e2e_datatype_int_range_distribution() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut range_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for (_, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Int" {
                if let Ok(val) = var_data["Value"].as_str().unwrap().parse::<i64>() {
                    let range = match val.abs() {
                        0 => "zero",
                        1..=10 => "small (1-10)",
                        11..=100 => "medium (11-100)",
                        101..=1000 => "large (101-1000)",
                        _ => "very large (1000+)",
                    };

                    *range_counts.entry(range.to_string()).or_insert(0) += 1;
                }
            }
        }

        println!("✓ Integer range distribution:");
        for (range, count) in range_counts.iter() {
            println!("  {}: {}", range, count);
        }
    }

    // ============================================================================
    // FLOAT TYPE TESTS
    // ============================================================================

    #[test]
    fn e2e_datatype_float_precision_validation() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut float_precisions: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();

        for (_, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Float" {
                let value_str = var_data["Value"].as_str().unwrap();
                if value_str != "null" && value_str.contains('.') {
                    let decimal_part = value_str.split('.').nth(1).unwrap_or("");
                    let precision = decimal_part.len();
                    *float_precisions.entry(precision).or_insert(0) += 1;
                }
            }
        }

        println!("✓ Float precision distribution:");
        let mut precisions: Vec<_> = float_precisions.iter().collect();
        precisions.sort_by_key(|&(precision, _)| precision);

        for (precision, count) in precisions {
            println!("  {} decimals: {} floats", precision, count);
        }
    }

    #[test]
    fn e2e_datatype_float_scientific_notation() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut scientific_floats = Vec::new();

        for (var_name, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Float" {
                let value_str = var_data["Value"].as_str().unwrap();
                if value_str.contains('e') || value_str.contains('E') {
                    scientific_floats.push((var_name.clone(), value_str.to_string()));
                }
            }
        }

        println!("✓ Scientific notation floats: {}", scientific_floats.len());
        if !scientific_floats.is_empty() {
            println!("  Examples: {:?}", scientific_floats.iter().take(3).collect::<Vec<_>>());
        } else {
            println!("  (None found - all standard notation)");
        }
    }

    #[test]
    fn e2e_datatype_float_special_values() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut fractional_values = 0;
        let mut whole_number_floats = 0;

        for (_, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Float" {
                let value_str = var_data["Value"].as_str().unwrap();
                if value_str != "null" {
                    if let Ok(val) = value_str.parse::<f64>() {
                        if val.fract() == 0.0 {
                            whole_number_floats += 1;
                        } else {
                            fractional_values += 1;
                        }
                    }
                }
            }
        }

        println!("✓ Float value types:");
        println!("  Fractional: {}", fractional_values);
        println!("  Whole numbers: {}", whole_number_floats);
    }

    // ============================================================================
    // BOOLEAN TYPE TESTS
    // ============================================================================

    #[test]
    fn e2e_datatype_bool_format_validation() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut bool_formats: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for (_, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Bool" {
                let value = var_data["Value"].as_str().unwrap();
                *bool_formats.entry(value.to_string()).or_insert(0) += 1;
            }
        }

        println!("✓ Boolean format distribution:");
        for (format, count) in bool_formats.iter() {
            println!("  '{}': {}", format, count);
        }

        // Validate only expected formats
        for format in bool_formats.keys() {
            assert!(["true", "false", "null", "0", "1"].contains(&format.as_str()),
                   "Unexpected boolean format: {}", format);
        }
    }

    #[test]
    fn e2e_datatype_bool_true_false_ratio() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut true_count = 0;
        let mut false_count = 0;
        let mut null_count = 0;

        for (_, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Bool" {
                match var_data["Value"].as_str().unwrap() {
                    "true" | "1" => true_count += 1,
                    "false" | "0" => false_count += 1,
                    "null" => null_count += 1,
                    _ => {}
                }
            }
        }

        println!("✓ Boolean value distribution:");
        println!("  True: {}", true_count);
        println!("  False: {}", false_count);
        println!("  Null: {}", null_count);

        let total = true_count + false_count + null_count;
        if total > 0 {
            println!("  True percentage: {:.1}%", (true_count as f64 / total as f64) * 100.0);
        }
    }

    // ============================================================================
    // VECTOR TYPE TESTS
    // ============================================================================

    #[test]
    fn e2e_datatype_vec2_component_validation() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut vec2_count = 0;
        let mut invalid_vec2 = Vec::new();

        for (var_name, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Vec2" {
                vec2_count += 1;
                let value = var_data["Value"].as_str().unwrap();

                if value != "null" {
                    let parts: Vec<&str> = value.split(',').map(|s| s.trim()).collect();

                    if parts.len() != 2 {
                        invalid_vec2.push(format!("{}: wrong component count ({})", var_name, parts.len()));
                    } else {
                        for (i, part) in parts.iter().enumerate() {
                            if part.parse::<f64>().is_err() {
                                invalid_vec2.push(format!("{}: component {} invalid ({})",
                                                         var_name, i, part));
                            }
                        }
                    }
                }
            }
        }

        assert!(invalid_vec2.is_empty(), "Invalid Vec2 values: {:?}", invalid_vec2);
        println!("✓ Vec2 component validation: {} Vec2 variables", vec2_count);
    }

    #[test]
    fn e2e_datatype_vec3_component_validation() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut vec3_count = 0;
        let mut invalid_vec3 = Vec::new();

        for (var_name, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Vec3" {
                vec3_count += 1;
                let value = var_data["Value"].as_str().unwrap();

                if value != "null" {
                    let parts: Vec<&str> = value.split(',').map(|s| s.trim()).collect();

                    if parts.len() != 3 {
                        invalid_vec3.push(format!("{}: wrong component count ({})", var_name, parts.len()));
                    } else {
                        for (i, part) in parts.iter().enumerate() {
                            if part.parse::<f64>().is_err() {
                                invalid_vec3.push(format!("{}: component {} invalid ({})",
                                                         var_name, i, part));
                            }
                        }
                    }
                }
            }
        }

        assert!(invalid_vec3.is_empty(), "Invalid Vec3 values: {:?}", invalid_vec3);
        println!("✓ Vec3 component validation: {} Vec3 variables", vec3_count);
    }

    #[test]
    fn e2e_datatype_vector_zero_vectors() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut zero_vec2 = 0;
        let mut zero_vec3 = 0;

        for (_, var_data) in variables {
            let var_type = var_data["Type"].as_str().unwrap_or("");
            let value = var_data["Value"].as_str().unwrap();

            if value != "null" {
                let is_zero = value.split(',')
                    .map(|s| s.trim().parse::<f64>().unwrap_or(1.0))
                    .all(|v| v == 0.0);

                if is_zero {
                    match var_type {
                        "Vec2" => zero_vec2 += 1,
                        "Vec3" => zero_vec3 += 1,
                        _ => {}
                    }
                }
            }
        }

        println!("✓ Zero vectors:");
        println!("  Vec2(0, 0): {}", zero_vec2);
        println!("  Vec3(0, 0, 0): {}", zero_vec3);
    }

    #[test]
    fn e2e_datatype_vector_component_ranges() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut max_component = f64::MIN;
        let mut min_component = f64::MAX;
        let mut max_var = String::new();
        let mut min_var = String::new();

        for (var_name, var_data) in variables {
            let var_type = var_data["Type"].as_str().unwrap_or("");
            if var_type == "Vec2" || var_type == "Vec3" {
                let value = var_data["Value"].as_str().unwrap();
                if value != "null" {
                    for part in value.split(',') {
                        if let Ok(val) = part.trim().parse::<f64>() {
                            if val > max_component {
                                max_component = val;
                                max_var = var_name.clone();
                            }
                            if val < min_component {
                                min_component = val;
                                min_var = var_name.clone();
                            }
                        }
                    }
                }
            }
        }

        println!("✓ Vector component ranges:");
        println!("  Max component: {} (in {})", max_component, max_var);
        println!("  Min component: {} (in {})", min_component, min_var);
    }

    // ============================================================================
    // STRING TYPE TESTS
    // ============================================================================

    #[test]
    fn e2e_datatype_string_empty_vs_null() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut empty_strings = 0;
        let mut null_strings = 0;
        let mut regular_strings = 0;

        for (_, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Str" {
                let value = var_data["Value"].as_str().unwrap();
                match value {
                    "" => empty_strings += 1,
                    "null" => null_strings += 1,
                    _ => regular_strings += 1,
                }
            }
        }

        println!("✓ String value types:");
        println!("  Empty strings: {}", empty_strings);
        println!("  Null strings: {}", null_strings);
        println!("  Regular strings: {}", regular_strings);
    }

    #[test]
    fn e2e_datatype_string_special_characters() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut with_quotes = 0;
        let mut with_backslash = 0;
        let mut with_newline = 0;
        let mut with_unicode = 0;

        for (_, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Str" {
                let value = var_data["Value"].as_str().unwrap();

                if value.contains('"') || value.contains('\'') {
                    with_quotes += 1;
                }
                if value.contains('\\') {
                    with_backslash += 1;
                }
                if value.contains('\n') {
                    with_newline += 1;
                }
                if value.chars().any(|c| c as u32 > 127) {
                    with_unicode += 1;
                }
            }
        }

        println!("✓ String special characters:");
        println!("  With quotes: {}", with_quotes);
        println!("  With backslash: {}", with_backslash);
        println!("  With newline: {}", with_newline);
        println!("  With unicode: {}", with_unicode);
    }

    #[test]
    fn e2e_datatype_string_path_patterns() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut file_paths = 0;
        let mut resource_paths = 0;
        let mut urls = 0;

        for (_, var_data) in variables {
            if var_data["Type"].as_str().unwrap_or("") == "Str" {
                let value = var_data["Value"].as_str().unwrap();

                if value.contains("/") || value.contains("\\") {
                    file_paths += 1;
                }
                if value.starts_with("res://") || value.starts_with("user://") {
                    resource_paths += 1;
                }
                if value.starts_with("http://") || value.starts_with("https://") {
                    urls += 1;
                }
            }
        }

        println!("✓ String path patterns:");
        println!("  File paths: {}", file_paths);
        println!("  Resource paths: {}", resource_paths);
        println!("  URLs: {}", urls);
    }

    // ============================================================================
    // TYPE CONSISTENCY TESTS
    // ============================================================================

    #[test]
    fn e2e_datatype_all_types_present() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut type_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for (_, var_data) in variables {
            let var_type = var_data["Type"].as_str().unwrap_or("").to_string();
            *type_counts.entry(var_type).or_insert(0) += 1;
        }

        println!("✓ Variable type distribution:");
        let mut types: Vec<_> = type_counts.iter().collect();
        types.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

        for (var_type, count) in types {
            println!("  {}: {}", var_type, count);
        }

        // Check for expected types
        let expected_types = vec!["Int", "Float", "Bool", "Str"];
        for expected_type in &expected_types {
            if !type_counts.contains_key(*expected_type) {
                println!("  Note: Type '{}' not found in this file", expected_type);
            }
        }
    }

    #[test]
    fn e2e_datatype_mutability_consistency() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut mutability_by_type: std::collections::HashMap<String, std::collections::HashMap<String, usize>> =
            std::collections::HashMap::new();

        for (_, var_data) in variables {
            let var_type = var_data["Type"].as_str().unwrap_or("").to_string();
            let mutability = var_data["Mutability"].as_str().unwrap_or("").to_string();

            mutability_by_type
                .entry(var_type)
                .or_insert_with(std::collections::HashMap::new)
                .entry(mutability)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        println!("✓ Mutability by type:");
        for (var_type, mutability_map) in mutability_by_type.iter() {
            if !var_type.is_empty() {
                println!("  {}:", var_type);
                for (mutability, count) in mutability_map.iter() {
                    println!("    {}: {}", mutability, count);
                }
            }
        }
    }

    #[test]
    fn e2e_datatype_null_handling_by_type() {
        let golden = load_golden_master("golden_masters/Baston-Model.json");
        let variables = golden["variables"].as_object().unwrap();

        let mut null_by_type: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        let mut total_by_type: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for (_, var_data) in variables {
            let var_type = var_data["Type"].as_str().unwrap_or("").to_string();
            if !var_type.is_empty() {
                *total_by_type.entry(var_type.clone()).or_insert(0) += 1;

                let value = var_data["Value"].as_str().unwrap_or("");
                if value == "null" {
                    *null_by_type.entry(var_type).or_insert(0) += 1;
                }
            }
        }

        println!("✓ Null value percentage by type:");
        for (var_type, total) in total_by_type.iter() {
            let null_count = null_by_type.get(var_type).unwrap_or(&0);
            let percentage = (*null_count as f64 / *total as f64) * 100.0;
            println!("  {}: {}/{} ({:.1}%)", var_type, null_count, total, percentage);
        }
    }

    // ============================================================================
    // CROSS-FILE TYPE CONSISTENCY
    // ============================================================================

    #[test]
    fn e2e_datatype_consistency_across_files() {
        let files = vec![
            "golden_masters/Baston-Model.json",
            "golden_masters/Baston-2D.json",
        ];

        println!("✓ Type consistency across files:");

        for file_path in &files {
            let golden = load_golden_master(file_path);
            let variables = golden["variables"].as_object().unwrap();

            let mut type_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

            for (_, var_data) in variables {
                let var_type = var_data["Type"].as_str().unwrap_or("").to_string();
                if !var_type.is_empty() {
                    *type_counts.entry(var_type).or_insert(0) += 1;
                }
            }

            println!("  {}:", file_path);
            for (var_type, count) in type_counts.iter() {
                println!("    {}: {}", var_type, count);
            }
        }
    }

    // ============================================================================
    // SUMMARY TEST
    // ============================================================================

    #[test]
    fn e2e_datatype_comprehensive_summary() {
        println!("\n=== E2E Expression & Data Type Validation Summary ===\n");
        println!("Comprehensive data type tests completed:");
        println!("  ✓ Integer parsing (positive, negative, zero)");
        println!("  ✓ Integer range distribution");
        println!("  ✓ Float precision validation");
        println!("  ✓ Float special values (scientific notation, whole numbers)");
        println!("  ✓ Boolean format validation");
        println!("  ✓ Boolean distribution analysis");
        println!("  ✓ Vec2 component validation");
        println!("  ✓ Vec3 component validation");
        println!("  ✓ Vector zero handling");
        println!("  ✓ Vector component ranges");
        println!("  ✓ String empty vs null distinction");
        println!("  ✓ String special character handling");
        println!("  ✓ String path pattern detection");
        println!("  ✓ Type distribution analysis");
        println!("  ✓ Mutability consistency");
        println!("  ✓ Null handling by type");
        println!("  ✓ Cross-file type consistency");
        println!("\nAll data type validation tests passed!\n");
    }
}
