// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! CastagneParser - Parses .casp character files
//!
//! This is a minimal v0 implementation of the Castagne parser.
//! The original GDScript version is ~2279 lines of complex parsing logic.
//! This version provides the basic structure with TODOs for full implementation.

use godot::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Import vector types for type conversion
use godot::builtin::{Vector2, Vector3};

/// Phases that can have events
const PHASES_BASE: &[&str] = &[
    "Init", "Action", "Reaction", "Freeze", "Manual", "AI", "Subentity", "Halt",
];

/// Variable mutability types
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum VariableMutability {
    Variable,
    Define,
    Internal,
}

/// Variable types
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum VariableType {
    Int,
    Str,
    Var,
    Vec2,
    Vec3,
    Box,
    Bool,
}

/// State type
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum StateType {
    Normal,
    BaseState,
    Helper,
    Special,
    Specblock,
}

/// Parsed variable definition
#[derive(Debug, Clone, Serialize)]
pub struct ParsedVariable {
    pub name: String,
    pub mutability: VariableMutability,
    pub var_type: VariableType,
    pub subtype: String,
    pub value: String,
}

impl ParsedVariable {
    /// Convert the string value to a Godot Variant based on the variable type
    pub fn to_variant(&self) -> Variant {
        CastagneParser::parse_value_to_variant(&self.value, &self.var_type)
    }

    /// Get the value as an integer (if possible)
    pub fn as_int(&self) -> Option<i32> {
        self.value.trim().parse::<i32>().ok()
    }

    /// Get the value as a boolean (if possible)
    pub fn as_bool(&self) -> Option<bool> {
        match self.value.trim().to_lowercase().as_str() {
            "true" | "1" => Some(true),
            "false" | "0" => Some(false),
            _ => None,
        }
    }

    /// Get the value as a float (if possible)
    pub fn as_float(&self) -> Option<f64> {
        self.value.trim().parse::<f64>().ok()
    }
}

/// Parsed state information
#[derive(Debug, Clone, Serialize)]
pub struct ParsedState {
    pub name: String,
    pub state_type: StateType,
    pub parent: Option<String>,
    pub actions: HashMap<String, Vec<ParsedAction>>, // Phase -> Actions
}

/// A parsed action/instruction
#[derive(Debug, Clone, Serialize)]
pub struct ParsedAction {
    pub instruction: String,
    pub args: Vec<String>,
    pub line_number: usize,
}

/// Character metadata
#[derive(Debug, Clone, Serialize)]
pub struct CharacterMetadata {
    pub name: String,
    pub author: String,
    pub description: String,
    pub skeleton: Option<String>,
    #[serde(flatten)]
    pub other_fields: HashMap<String, String>,
}

/// Full parsed character data
#[derive(Debug, Clone, Serialize)]
pub struct ParsedCharacter {
    pub metadata: CharacterMetadata,
    pub variables: HashMap<String, ParsedVariable>,
    pub states: HashMap<String, ParsedState>,
    pub specblocks: HashMap<String, HashMap<String, String>>,
    pub subentities: HashMap<String, CharacterMetadata>,
    pub transformed_data: HashMap<String, HashMap<String, String>>,
}

impl ParsedCharacter {
    /// Serialize this character to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Serialize this character to a JSON Value
    pub fn to_json_value(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

/// CastagneParser - Main parser struct
///
/// Parses .casp files to create Castagne characters.
/// This is a minimal implementation - the original is much more complex!
pub struct CastagneParser {
    logs_active: bool,
    pub errors: Vec<String>,

    // Parsing state
    current_lines: Vec<String>,
    line_ids: Vec<usize>,
    file_paths: Vec<String>,
    current_file: usize,

    // Parsed data
    metadata: CharacterMetadata,
    variables: HashMap<String, ParsedVariable>,
    states: HashMap<String, ParsedState>,
    specblocks: HashMap<String, HashMap<String, String>>, // Specblock name -> key-value pairs
    specblock_defines: HashMap<String, ParsedVariable>,

    // Flags
    pub aborting: bool,
    pub invalid_file: bool,
}

impl CastagneParser {
    /// Create a new parser instance
    pub fn new() -> Self {
        Self {
            logs_active: false,
            errors: Vec::new(),
            current_lines: Vec::new(),
            line_ids: Vec::new(),
            file_paths: Vec::new(),
            current_file: 0,
            metadata: CharacterMetadata {
                name: String::new(),
                author: String::new(),
                description: String::new(),
                skeleton: None,
                other_fields: HashMap::new(),
            },
            variables: HashMap::new(),
            states: HashMap::new(),
            specblocks: HashMap::new(),
            specblock_defines: HashMap::new(),
            aborting: false,
            invalid_file: false,
        }
    }

    /// Strip inline comments from a line (everything after # that's not in a string)
    fn strip_inline_comment(&self, line: &str) -> String {
        let mut result = String::new();
        let mut in_string = false;
        let mut escape_next = false;
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            if escape_next {
                result.push(ch);
                escape_next = false;
                continue;
            }

            match ch {
                '\\' if in_string => {
                    escape_next = true;
                    result.push(ch);
                }
                '"' => {
                    in_string = !in_string;
                    result.push(ch);
                }
                '#' if !in_string => {
                    // Found comment marker outside string, stop here
                    break;
                }
                _ => {
                    result.push(ch);
                }
            }
        }

        result
    }

    /// Get only character metadata (lightweight parse)
    pub fn get_character_metadata(&mut self, file_path: &str) -> Option<CharacterMetadata> {
        self.start_parsing(file_path);
        self.parse_metadata(0);
        self.end_parsing().map(|c| c.metadata)
    }

    /// Get character info (parse metadata and specblocks)
    pub fn get_character_info(&mut self, file_path: &str) -> Option<ParsedCharacter> {
        self.start_parsing(file_path);
        // TODO: Implement parse_full_file with stop_after_specblocks=true
        self.end_parsing()
    }

    /// Parse a full character file
    pub fn create_full_character(&mut self, file_path: &str) -> Option<ParsedCharacter> {
        self.start_parsing(file_path);
        self.parse_full_file();
        self.end_parsing()
    }

    /// Reset error list
    pub fn reset_errors(&mut self) {
        self.errors.clear();
    }

    // -------------------------------------------------------------------------
    // Internal parsing methods

    fn start_parsing(&mut self, file_path: &str) {
        self.reset_errors();
        self.current_lines.clear();
        self.line_ids.clear();
        self.file_paths.clear();
        self.current_file = 0;
        self.variables.clear();
        self.states.clear();
        self.specblocks.clear();
        self.specblock_defines.clear();
        self.aborting = false;
        self.invalid_file = false;

        self.open_file(file_path);
    }

    pub fn end_parsing(&mut self) -> Option<ParsedCharacter> {
        if self.aborting || self.invalid_file {
            return None;
        }

        Some(ParsedCharacter {
            metadata: self.metadata.clone(),
            variables: self.variables.clone(),
            states: self.states.clone(),
            specblocks: self.specblocks.clone(),
            subentities: HashMap::new(), // TODO: Implement subentity parsing
            transformed_data: HashMap::new(), // TODO: Implement data transformation
        })
    }

    pub fn open_file(&mut self, file_path: &str) {
        self.log(&format!("Opening file {}", file_path));

        let file_id = self.file_paths.len();
        self.file_paths.push(file_path.to_string());

        // Read the file
        match File::open(file_path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for (line_num, line_result) in reader.lines().enumerate() {
                    match line_result {
                        Ok(line) => {
                            self.current_lines.push(line);
                            self.line_ids.push(line_num + 1); // 1-indexed for user display
                        }
                        Err(e) => {
                            self.fatal_error(&format!("Error reading line {} from {}: {}", line_num, file_path, e));
                            return;
                        }
                    }
                }
                self.log(&format!("Successfully loaded {} lines from {}", self.current_lines.len(), file_path));
            }
            Err(e) => {
                self.fatal_error(&format!("File {} does not exist or cannot be opened: {}", file_path, e));
            }
        }
    }

    pub fn parse_full_file(&mut self) {
        if self.aborting {
            return;
        }

        self.log(">>> Starting to parse the full file.");

        // Step 1: Parse metadata
        self.parse_metadata(0);

        // Step 2: If metadata has skeleton, load and parse parent file first
        if let Some(skeleton_path) = self.metadata.skeleton.clone() {
            self.log(&format!("Loading skeleton file: {}", skeleton_path));
            self.load_skeleton(&skeleton_path);
            if self.aborting {
                return;
            }
        }

        // Step 3: Parse specblocks
        self.parse_specblocks(0);

        // Step 4: Parse variables
        self.parse_variables(0);

        // Step 5: Parse states
        self.parse_states(0);

        // TODO: Step 6: Optimize
        self.log(">>> Parsing complete!");
    }

    fn load_skeleton(&mut self, skeleton_path: &str) {
        // Save current parsing state
        let current_lines = self.current_lines.clone();
        let current_line_ids = self.line_ids.clone();
        let current_file_paths = self.file_paths.clone();

        // Store child metadata separately (we want to keep the child's metadata)
        let child_metadata = self.metadata.clone();

        // Parse the skeleton file
        let mut skeleton_parser = CastagneParser::new();
        skeleton_parser.logs_active = self.logs_active;

        match skeleton_parser.create_full_character(skeleton_path) {
            Some(skeleton_character) => {
                self.log(&format!("Successfully loaded skeleton: {}", skeleton_path));

                // Merge skeleton data into current parser
                // Parent data is added first, child can override

                // Merge specblocks (child overrides parent on a per-key basis)
                for (block_name, parent_data) in skeleton_character.specblocks {
                    let child_block = self.specblocks.entry(block_name).or_insert_with(HashMap::new);
                    // Insert parent values that don't exist in child
                    for (key, value) in parent_data {
                        child_block.entry(key).or_insert(value);
                    }
                }

                // Merge variables (child overrides parent)
                for (name, var) in skeleton_character.variables {
                    self.variables.entry(name).or_insert(var);
                }

                // Merge states (child overrides parent)
                for (name, state) in skeleton_character.states {
                    self.states.entry(name).or_insert(state);
                }

                self.log("Skeleton data merged successfully");
            }
            None => {
                self.fatal_error(&format!("Failed to load skeleton file: {}", skeleton_path));
                return;
            }
        }

        // Restore current parsing state
        self.current_lines = current_lines;
        self.line_ids = current_line_ids;
        self.file_paths = current_file_paths;
        self.metadata = child_metadata;
    }

    fn parse_metadata(&mut self, _file_id: usize) -> &CharacterMetadata {
        self.log("Parsing metadata...");

        // Find :Character: block
        let mut in_character_block = false;
        let mut i = 0;

        while i < self.current_lines.len() {
            let line = self.current_lines[i].trim();

            // Check for :Character: block start
            if line == ":Character:" {
                in_character_block = true;
                i += 1;
                continue;
            }

            // Check for end of block (another : block or empty line after content)
            if in_character_block && line.starts_with(':') && line.ends_with(':') && line != ":Character:" {
                break;
            }

            if in_character_block && !line.is_empty() && !line.starts_with('#') {
                // Strip inline comments and parse metadata fields
                let cleaned_line = self.strip_inline_comment(line);
                let cleaned = cleaned_line.trim();

                if !cleaned.is_empty() {
                    if let Some(colon_pos) = cleaned.find(':') {
                        let key = cleaned[..colon_pos].trim();
                        let value = cleaned[colon_pos + 1..].trim().to_string();

                        match key {
                            "Name" => self.metadata.name = value,
                            "Author" => self.metadata.author = value,
                            "Description" => self.metadata.description = value,
                            "Skeleton" => self.metadata.skeleton = Some(value),
                            _ => {
                                self.metadata.other_fields.insert(key.to_string(), value);
                            }
                        }
                    }
                }
            }

            i += 1;
        }

        self.log(&format!("Parsed metadata: Name={}", self.metadata.name));
        &self.metadata
    }

    fn parse_specblocks(&mut self, _file_id: usize) -> HashMap<String, String> {
        self.log("Parsing specblocks...");

        let mut i = 0;
        while i < self.current_lines.len() {
            let line = self.current_lines[i].trim();

            // Check if this is a specblock definition (starts with ':' and ends with ':')
            // but exclude known special blocks
            if line.starts_with(':') && line.ends_with(':') {
                // Extract block name (including any parentheses for states)
                let full_block_name = line[1..line.len() - 1].to_string();

                // Extract just the name part (before any parentheses) for comparison
                let block_name = if let Some(paren_pos) = full_block_name.find('(') {
                    full_block_name[..paren_pos].trim()
                } else {
                    full_block_name.as_str()
                };

                // Check if this is a specblock (not Character, Variables, or a state)
                // Specblocks typically have specific patterns, but for now we'll identify them
                // by checking if the content is key-value pairs (not phase markers or actions)
                if block_name != "Character" && block_name != "Variables" {
                    // Peek ahead to see if this looks like a specblock
                    if self.is_specblock(block_name, i + 1) {
                        self.parse_specblock(block_name.to_string(), &mut i);
                    }
                }
            }

            i += 1;
        }

        self.log(&format!("Parsed {} specblocks", self.specblocks.len()));
        HashMap::new() // Return empty for compatibility with existing code
    }

    fn is_specblock(&self, _block_name: &str, start_idx: usize) -> bool {
        // Look at the first few non-empty lines to determine if this is a specblock
        // Specblocks contain key-value pairs (Key: Value) without phase markers (---)
        let mut idx = start_idx;
        let mut line_count = 0;

        while idx < self.current_lines.len() && line_count < 5 {
            let line = self.current_lines[idx].trim();

            // Stop at next block
            if line.starts_with(':') && line.ends_with(':') {
                break;
            }

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                idx += 1;
                continue;
            }

            // If we see a phase marker, this is definitely a state, not a specblock
            if line.starts_with("---") {
                return false;
            }

            // If we see a line that looks like an action (function call), it's a state
            if line.contains('(') && line.contains(')') && !line.contains(':') {
                return false;
            }

            // If we see a key-value pair, it might be a specblock
            if line.contains(':') {
                line_count += 1;
            }

            idx += 1;
        }

        // If we found any key-value pairs, consider it a specblock
        line_count > 0
    }

    fn parse_specblock(&mut self, block_name: String, i: &mut usize) {
        self.log(&format!("Parsing specblock: {}", block_name));

        let mut specblock_data = HashMap::new();
        *i += 1; // Move past the block name line

        while *i < self.current_lines.len() {
            let line = self.current_lines[*i].trim();

            // Check if we've reached another block
            if line.starts_with(':') && line.ends_with(':') {
                break;
            }

            // Parse key-value pairs (strip inline comments first)
            if !line.is_empty() && !line.starts_with('#') {
                let cleaned_line = self.strip_inline_comment(line);
                let cleaned = cleaned_line.trim();

                if !cleaned.is_empty() {
                    if let Some(colon_pos) = cleaned.find(':') {
                        let key = cleaned[..colon_pos].trim().to_string();
                        let value = cleaned[colon_pos + 1..].trim().to_string();
                        specblock_data.insert(key, value);
                    }
                }
            }

            *i += 1;
        }

        if !specblock_data.is_empty() {
            // Merge with existing specblock (if from parent) instead of replacing
            let existing_block = self.specblocks.entry(block_name.clone()).or_insert_with(HashMap::new);
            for (key, value) in specblock_data {
                // Child values override parent values
                existing_block.insert(key, value);
            }
        }
        *i -= 1; // Back up one so the outer loop doesn't skip a line
    }

    fn parse_variables(&mut self, _file_id: usize) {
        self.log("Parsing variables...");

        // Find :Variables: block
        let mut in_variables_block = false;
        let mut i = 0;

        while i < self.current_lines.len() {
            let line = self.current_lines[i].trim().to_string();

            // Check for :Variables: block start
            if line == ":Variables:" {
                in_variables_block = true;
                i += 1;
                continue;
            }

            // Check for end of block
            if in_variables_block && line.starts_with(':') && line.ends_with(':') && line != ":Variables:" {
                break;
            }

            if in_variables_block && !line.is_empty() && !line.starts_with('#') {
                let cleaned_line = self.strip_inline_comment(&line);
                let cleaned = cleaned_line.trim();

                if !cleaned.is_empty() {
                    self.parse_variable_line(cleaned);
                }
            }

            i += 1;
        }

        self.log(&format!("Parsed {} variables", self.variables.len()));
    }

    fn parse_variable_line(&mut self, line: &str) {
        // Parse variable definition: var VariableName(Type): DefaultValue
        // or constant definition: def ConstantName: Value

        if line.starts_with("var ") {
            self.parse_var_declaration(&line[4..]);
        } else if line.starts_with("def ") {
            self.parse_def_declaration(&line[4..]);
        }
    }

    fn parse_var_declaration(&mut self, line: &str) {
        // Format: VariableName(Type): DefaultValue
        // or: VariableName(Type, Subtype): DefaultValue

        if let Some(colon_pos) = line.find(':') {
            let name_part = line[..colon_pos].trim();
            let value_part = line[colon_pos + 1..].trim();

            // Parse name and type
            if let Some(open_paren) = name_part.find('(') {
                if let Some(close_paren) = name_part.find(')') {
                    let name = name_part[..open_paren].trim().to_string();
                    let type_str = name_part[open_paren + 1..close_paren].trim();

                    // Parse type and optional subtype
                    let (var_type, subtype) = if let Some(comma_pos) = type_str.find(',') {
                        let main_type = type_str[..comma_pos].trim();
                        let sub = type_str[comma_pos + 1..].trim().to_string();
                        (self.parse_variable_type(main_type), sub)
                    } else {
                        (self.parse_variable_type(type_str), String::new())
                    };

                    let var = ParsedVariable {
                        name: name.clone(),
                        mutability: VariableMutability::Variable,
                        var_type,
                        subtype,
                        value: value_part.to_string(),
                    };

                    self.variables.insert(name, var);
                }
            }
        }
    }

    fn parse_def_declaration(&mut self, line: &str) {
        // Format: ConstantName: Value

        if let Some(colon_pos) = line.find(':') {
            let name = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim().to_string();

            let var = ParsedVariable {
                name: name.clone(),
                mutability: VariableMutability::Define,
                var_type: VariableType::Var, // Defines can be any type
                subtype: String::new(),
                value,
            };

            self.variables.insert(name, var);
        }
    }

    fn parse_variable_type(&self, type_str: &str) -> VariableType {
        match type_str {
            "Int" => VariableType::Int,
            "Str" => VariableType::Str,
            "Var" => VariableType::Var,
            "Vec2" => VariableType::Vec2,
            "Vec3" => VariableType::Vec3,
            "Box" => VariableType::Box,
            "Bool" => VariableType::Bool,
            _ => {
                self.log(&format!("Unknown variable type: {}, defaulting to Var", type_str));
                VariableType::Var
            }
        }
    }

    fn parse_states(&mut self, _file_id: usize) {
        self.log("Parsing states...");

        let mut i = 0;
        while i < self.current_lines.len() {
            let line = self.current_lines[i].trim();

            // Check if this is a state definition (starts and ends with ':' but not a known special block)
            if line.starts_with(':') && line.ends_with(':') {
                let full_state_name = line[1..line.len() - 1].to_string();

                // Extract just the name part (before any parentheses) for comparison
                let state_name = if let Some(paren_pos) = full_state_name.find('(') {
                    full_state_name[..paren_pos].trim()
                } else {
                    full_state_name.as_str()
                };

                // Skip special blocks we've already handled, and skip specblocks
                if state_name != "Character" && state_name != "Variables" && !self.specblocks.contains_key(state_name) {
                    self.parse_state(full_state_name, &mut i);
                }
            }

            i += 1;
        }

        self.log(&format!("Parsed {} states", self.states.len()));
    }

    fn parse_state_header(&self, state_header: &str) -> (String, StateType, Option<String>) {
        // Parse state header to extract name, type, and parent
        // Formats:
        // - StateName -> (StateName, Normal, None)
        // - StateName(Helper) -> (StateName, Helper, None)
        // - StateName(Idle) -> (StateName, Normal, Some(Idle))
        // - StateName(Helper, Idle) -> (StateName, Helper, Some(Idle))

        if let Some(paren_start) = state_header.find('(') {
            if let Some(paren_end) = state_header.rfind(')') {
                let name = state_header[..paren_start].trim().to_string();
                let params = state_header[paren_start + 1..paren_end].trim();

                // Check if params contains a comma (type and parent)
                if let Some(comma_pos) = params.find(',') {
                    let type_str = params[..comma_pos].trim();
                    let parent_str = params[comma_pos + 1..].trim().to_string();
                    let state_type = self.parse_state_type(type_str);
                    return (name, state_type, Some(parent_str));
                } else {
                    // Single parameter - could be type or parent
                    // Types are: Helper, BaseState, Special, Specblock
                    let state_type = self.parse_state_type(params);
                    if state_type != StateType::Normal {
                        // It's a type
                        return (name, state_type, None);
                    } else {
                        // It's a parent state
                        return (name, StateType::Normal, Some(params.to_string()));
                    }
                }
            }
        }

        // No parentheses, just a simple state name
        (state_header.to_string(), StateType::Normal, None)
    }

    fn parse_state_type(&self, type_str: &str) -> StateType {
        match type_str {
            "Helper" => StateType::Helper,
            "BaseState" => StateType::BaseState,
            "Special" => StateType::Special,
            "Specblock" => StateType::Specblock,
            _ => StateType::Normal,
        }
    }

    fn parse_state(&mut self, state_name: String, i: &mut usize) {
        self.log(&format!("Parsing state: {}", state_name));

        // Parse state name with optional type and parent
        // Format: :StateName: or :StateName(Type): or :StateName(Parent): or :StateName(Type, Parent):
        let (actual_name, state_type, parent) = self.parse_state_header(&state_name);

        let mut state = ParsedState {
            name: actual_name.clone(),
            state_type,
            parent,
            actions: HashMap::new(),
        };

        let mut current_phase: Option<String> = None;
        *i += 1; // Move past the state name line

        while *i < self.current_lines.len() {
            let line = self.current_lines[*i].trim();

            // Check if we've reached another state
            if line.starts_with(':') && line.ends_with(':') {
                break;
            }

            // Check for phase marker (---PhaseName:)
            if line.starts_with("---") {
                if let Some(colon_pos) = line.find(':') {
                    let phase_name = line[3..colon_pos].trim().to_string();
                    current_phase = Some(phase_name.clone());
                    state.actions.entry(phase_name).or_insert_with(Vec::new);
                }
            }
            // Parse action line (strip inline comments first)
            else if !line.is_empty() && !line.starts_with('#') {
                let cleaned_line = self.strip_inline_comment(line);
                let cleaned = cleaned_line.trim();

                if !cleaned.is_empty() {
                    if let Some(ref phase) = current_phase {
                        if let Some(action) = self.parse_action_line(cleaned, *i) {
                            state.actions
                                .entry(phase.clone())
                                .or_insert_with(Vec::new)
                                .push(action);
                        }
                    }
                }
            }

            *i += 1;
        }

        self.states.insert(actual_name, state);
        *i -= 1; // Back up one so the outer loop doesn't skip a line
    }

    fn parse_action_line(&self, line: &str, line_number: usize) -> Option<ParsedAction> {
        // Parse function call: FunctionName(Arg1, Arg2, ...)
        // or simple instruction: FunctionName

        if let Some(open_paren) = line.find('(') {
            if let Some(close_paren) = line.rfind(')') {
                let instruction = line[..open_paren].trim().to_string();
                let args_str = &line[open_paren + 1..close_paren];

                // Parse arguments with better handling of nested calls and strings
                let args = self.parse_arguments(args_str);

                return Some(ParsedAction {
                    instruction,
                    args,
                    line_number,
                });
            }
        } else {
            // No parentheses, treat as instruction with no args
            return Some(ParsedAction {
                instruction: line.to_string(),
                args: Vec::new(),
                line_number,
            });
        }

        None
    }

    fn parse_arguments(&self, args_str: &str) -> Vec<String> {
        // Split arguments by comma, but respect nested parentheses and quotes
        let mut args = Vec::new();
        let mut current_arg = String::new();
        let mut paren_depth = 0;
        let mut in_string = false;
        let mut escape_next = false;

        for ch in args_str.chars() {
            if escape_next {
                current_arg.push(ch);
                escape_next = false;
                continue;
            }

            match ch {
                '\\' => {
                    escape_next = true;
                    current_arg.push(ch);
                }
                '"' => {
                    in_string = !in_string;
                    current_arg.push(ch);
                }
                '(' if !in_string => {
                    paren_depth += 1;
                    current_arg.push(ch);
                }
                ')' if !in_string => {
                    paren_depth -= 1;
                    current_arg.push(ch);
                }
                ',' if !in_string && paren_depth == 0 => {
                    // Found a separator at the top level
                    if !current_arg.trim().is_empty() {
                        args.push(current_arg.trim().to_string());
                    }
                    current_arg.clear();
                }
                _ => {
                    current_arg.push(ch);
                }
            }
        }

        // Add the last argument
        if !current_arg.trim().is_empty() {
            args.push(current_arg.trim().to_string());
        }

        args
    }

    // -------------------------------------------------------------------------
    // Instruction execution (for runtime)

    /// Standard parse function (used by modules to register functions)
    pub fn standard_parse_function(&self, function_name: &str, args: &[String]) -> Vec<String> {
        // TODO: Parse function arguments according to function signature
        // For now, just return the args as-is
        args.to_vec()
    }

    // Note: Instruction execution is handled by the GDScript engine.
    // The parser's job is to parse .casp files into data structures only.

    // -------------------------------------------------------------------------
    // Logging and errors

    fn log(&self, message: &str) {
        if self.logs_active {
            godot_print!("[CastagneParser] {}", message);
        }
    }

    fn fatal_error(&mut self, message: &str) {
        self.errors.push(message.to_string());
        godot_error!("[CastagneParser] FATAL: {}", message);
        self.aborting = true;
        self.invalid_file = true;
    }

    #[allow(dead_code)]
    fn error(&mut self, message: &str) {
        self.errors.push(message.to_string());
        godot_error!("[CastagneParser] ERROR: {}", message);
    }

    /// Get all errors from last parse
    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }

    // -------------------------------------------------------------------------
    // Type conversion utilities

    /// Parse a string value into a Variant based on the variable type
    pub fn parse_value_to_variant(value_str: &str, var_type: &VariableType) -> Variant {
        let trimmed = value_str.trim();

        match var_type {
            VariableType::Int => {
                trimmed.parse::<i32>()
                    .map(Variant::from)
                    .unwrap_or_else(|_| Variant::nil())
            }
            VariableType::Bool => {
                let bool_val = match trimmed.to_lowercase().as_str() {
                    "true" | "1" => true,
                    "false" | "0" => false,
                    _ => false,
                };
                Variant::from(bool_val)
            }
            VariableType::Str => {
                // Remove quotes if present
                let unquoted = if (trimmed.starts_with('"') && trimmed.ends_with('"'))
                    || (trimmed.starts_with('\'') && trimmed.ends_with('\'')) {
                    &trimmed[1..trimmed.len() - 1]
                } else {
                    trimmed
                };
                Variant::from(GString::from(unquoted))
            }
            VariableType::Vec2 => {
                // Parse (x, y) or x, y
                Self::parse_vec2(trimmed).unwrap_or_else(|| Variant::nil())
            }
            VariableType::Vec3 => {
                // Parse (x, y, z) or x, y, z
                Self::parse_vec3(trimmed).unwrap_or_else(|| Variant::nil())
            }
            VariableType::Var | VariableType::Box => {
                // Try to infer the type
                // First try int
                if let Ok(i) = trimmed.parse::<i32>() {
                    return Variant::from(i);
                }
                // Then try float
                if let Ok(f) = trimmed.parse::<f64>() {
                    return Variant::from(f);
                }
                // Then try bool
                match trimmed.to_lowercase().as_str() {
                    "true" | "false" => {
                        return Variant::from(trimmed.to_lowercase() == "true");
                    }
                    _ => {}
                }
                // Otherwise treat as string
                Variant::from(GString::from(trimmed))
            }
        }
    }

    /// Parse a Vec2 from string (supports "x, y" or "(x, y)")
    fn parse_vec2(s: &str) -> Option<Variant> {
        let cleaned = s.trim().trim_matches(|c| c == '(' || c == ')');
        let parts: Vec<&str> = cleaned.split(',').collect();

        if parts.len() == 2 {
            if let (Ok(x), Ok(y)) = (
                parts[0].trim().parse::<f32>(),
                parts[1].trim().parse::<f32>(),
            ) {
                return Some(Variant::from(Vector2::new(x, y)));
            }
        }
        None
    }

    /// Parse a Vec3 from string (supports "x, y, z" or "(x, y, z)")
    fn parse_vec3(s: &str) -> Option<Variant> {
        let cleaned = s.trim().trim_matches(|c| c == '(' || c == ')');
        let parts: Vec<&str> = cleaned.split(',').collect();

        if parts.len() == 3 {
            if let (Ok(x), Ok(y), Ok(z)) = (
                parts[0].trim().parse::<f32>(),
                parts[1].trim().parse::<f32>(),
                parts[2].trim().parse::<f32>(),
            ) {
                return Some(Variant::from(Vector3::new(x, y, z)));
            }
        }
        None
    }
}

impl Default for CastagneParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_metadata() {
        let mut parser = CastagneParser::new();
        // Don't enable logs in unit tests (requires Godot runtime)

        // Simulate file content
        parser.current_lines = vec![
            ":Character:".to_string(),
            "Name: Test Fighter".to_string(),
            "Author: Test Author".to_string(),
            "Description: A test character".to_string(),
            "".to_string(),
            ":Variables:".to_string(),
        ];
        parser.line_ids = vec![1, 2, 3, 4, 5, 6];
        parser.file_paths = vec!["test.casp".to_string()];

        parser.parse_metadata(0);

        assert_eq!(parser.metadata.name, "Test Fighter");
        assert_eq!(parser.metadata.author, "Test Author");
        assert_eq!(parser.metadata.description, "A test character");
    }

    #[test]
    fn test_parse_variables() {
        let mut parser = CastagneParser::new();
        // Don't enable logs in unit tests (requires Godot runtime)

        parser.current_lines = vec![
            ":Variables:".to_string(),
            "var Health(Int): 100".to_string(),
            "var Name(Str): TestName".to_string(),
            "def MAX_HP: 150".to_string(),
            "".to_string(),
            ":Idle:".to_string(),
        ];
        parser.line_ids = vec![1, 2, 3, 4, 5, 6];
        parser.file_paths = vec!["test.casp".to_string()];

        parser.parse_variables(0);

        assert_eq!(parser.variables.len(), 3);

        let health = parser.variables.get("Health").unwrap();
        assert_eq!(health.var_type, VariableType::Int);
        assert_eq!(health.value, "100");
        assert_eq!(health.mutability, VariableMutability::Variable);

        let max_hp = parser.variables.get("MAX_HP").unwrap();
        assert_eq!(max_hp.value, "150");
        assert_eq!(max_hp.mutability, VariableMutability::Define);
    }

    #[test]
    fn test_parse_states() {
        let mut parser = CastagneParser::new();
        // Don't enable logs in unit tests (requires Godot runtime)

        parser.current_lines = vec![
            ":Idle:".to_string(),
            "---Init:".to_string(),
            "Set(Health, 100)".to_string(),
            "---Action:".to_string(),
            "CheckInput()".to_string(),
            "".to_string(),
        ];
        parser.line_ids = vec![1, 2, 3, 4, 5, 6];
        parser.file_paths = vec!["test.casp".to_string()];

        parser.parse_states(0);

        assert_eq!(parser.states.len(), 1);

        let idle_state = parser.states.get("Idle").unwrap();
        assert_eq!(idle_state.name, "Idle");

        let init_actions = idle_state.actions.get("Init").unwrap();
        assert_eq!(init_actions.len(), 1);
        assert_eq!(init_actions[0].instruction, "Set");
        assert_eq!(init_actions[0].args.len(), 2);
        assert_eq!(init_actions[0].args[0], "Health");
        assert_eq!(init_actions[0].args[1], "100");

        let action_actions = idle_state.actions.get("Action").unwrap();
        assert_eq!(action_actions.len(), 1);
        assert_eq!(action_actions[0].instruction, "CheckInput");
        assert_eq!(action_actions[0].args.len(), 0);
    }

    #[test]
    fn test_parse_action_line() {
        let parser = CastagneParser::new();

        // Test with arguments
        let action = parser.parse_action_line("Set(Health, 100)", 1).unwrap();
        assert_eq!(action.instruction, "Set");
        assert_eq!(action.args.len(), 2);
        assert_eq!(action.args[0], "Health");
        assert_eq!(action.args[1], "100");

        // Test without arguments
        let action2 = parser.parse_action_line("DoSomething()", 1).unwrap();
        assert_eq!(action2.instruction, "DoSomething");
        assert_eq!(action2.args.len(), 0);

        // Test without parentheses
        let action3 = parser.parse_action_line("SimpleInstruction", 1).unwrap();
        assert_eq!(action3.instruction, "SimpleInstruction");
        assert_eq!(action3.args.len(), 0);
    }

    #[test]
    fn test_parse_arguments() {
        let parser = CastagneParser::new();

        // Simple arguments
        let args = parser.parse_arguments("a, b, c");
        assert_eq!(args.len(), 3);
        assert_eq!(args[0], "a");
        assert_eq!(args[1], "b");
        assert_eq!(args[2], "c");

        // Nested function calls
        let args2 = parser.parse_arguments("Health, Add(10, 5), Position");
        assert_eq!(args2.len(), 3);
        assert_eq!(args2[0], "Health");
        assert_eq!(args2[1], "Add(10, 5)");
        assert_eq!(args2[2], "Position");

        // String arguments with commas
        let args3 = parser.parse_arguments(r#""Hello, World", Test"#);
        assert_eq!(args3.len(), 2);
        assert_eq!(args3[0], r#""Hello, World""#);
        assert_eq!(args3[1], "Test");

        // Complex nested calls - note this is just the arguments part, not the full call
        let args4 = parser.parse_arguments("Greater(Health, 50), Set(Color, Red), Set(Color, Blue)");
        assert_eq!(args4.len(), 3);
        assert_eq!(args4[0], "Greater(Health, 50)");
        assert_eq!(args4[1], "Set(Color, Red)");
        assert_eq!(args4[2], "Set(Color, Blue)");
    }

    #[test]
    fn test_parse_complex_actions() {
        let parser = CastagneParser::new();

        // Nested function call as argument
        let action = parser.parse_action_line("Set(Health, Add(100, 50))", 1).unwrap();
        assert_eq!(action.instruction, "Set");
        assert_eq!(action.args.len(), 2);
        assert_eq!(action.args[0], "Health");
        assert_eq!(action.args[1], "Add(100, 50)");

        // String with special characters
        let action2 = parser.parse_action_line(r#"Log("Player health: ", Health)"#, 1).unwrap();
        assert_eq!(action2.instruction, "Log");
        assert_eq!(action2.args.len(), 2);
        assert_eq!(action2.args[0], r#""Player health: ""#);
        assert_eq!(action2.args[1], "Health");
    }

    #[test]
    fn test_full_file_parse() {
        let mut parser = CastagneParser::new();

        // Test with the test_character.casp file if it exists
        // This test will be skipped if the file doesn't exist
        if std::path::Path::new("test_character.casp").exists() {
            let result = parser.create_full_character("test_character.casp");

            assert!(result.is_some(), "Failed to parse test_character.casp");
            let character = result.unwrap();

            // Check metadata
            assert_eq!(character.metadata.name, "Test Character");
            assert_eq!(character.metadata.author, "Parser Test");

            // Check variables
            assert!(character.variables.contains_key("Health"));
            assert!(character.variables.contains_key("MoveSpeed"));
            assert!(character.variables.contains_key("MAX_HEALTH"));

            // Check states
            assert!(character.states.contains_key("Idle"));
            let idle_state = character.states.get("Idle").unwrap();
            assert!(idle_state.actions.contains_key("Init"));
            assert!(idle_state.actions.contains_key("Action"));
        }
    }

    #[test]
    fn test_parse_specblocks() {
        let mut parser = CastagneParser::new();

        parser.current_lines = vec![
            ":Character:".to_string(),
            "Name: Test".to_string(),
            "".to_string(),
            ":Attacks:".to_string(),
            "Damage: 10".to_string(),
            "Range: 5".to_string(),
            "Knockback: 3".to_string(),
            "".to_string(),
            ":Config:".to_string(),
            "MaxSpeed: 100".to_string(),
            "JumpHeight: 50".to_string(),
            "".to_string(),
            ":Variables:".to_string(),
            "var Health(Int): 100".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["test.casp".to_string()];

        parser.parse_specblocks(0);

        // Should have parsed 2 specblocks (Attacks and Config)
        assert_eq!(parser.specblocks.len(), 2);

        // Check Attacks specblock
        let attacks = parser.specblocks.get("Attacks").unwrap();
        assert_eq!(attacks.get("Damage"), Some(&"10".to_string()));
        assert_eq!(attacks.get("Range"), Some(&"5".to_string()));
        assert_eq!(attacks.get("Knockback"), Some(&"3".to_string()));

        // Check Config specblock
        let config = parser.specblocks.get("Config").unwrap();
        assert_eq!(config.get("MaxSpeed"), Some(&"100".to_string()));
        assert_eq!(config.get("JumpHeight"), Some(&"50".to_string()));
    }

    #[test]
    fn test_specblock_vs_state_detection() {
        let mut parser = CastagneParser::new();

        // This should be detected as a state (has phase markers)
        parser.current_lines = vec![
            ":Idle:".to_string(),
            "---Init:".to_string(),
            "Set(Health, 100)".to_string(),
            "".to_string(),
        ];
        parser.line_ids = vec![1, 2, 3, 4];
        parser.file_paths = vec!["test.casp".to_string()];

        assert!(!parser.is_specblock("Idle", 1));

        // This should be detected as a specblock (has key-value pairs)
        parser.current_lines = vec![
            ":Config:".to_string(),
            "MaxSpeed: 100".to_string(),
            "JumpHeight: 50".to_string(),
            "".to_string(),
        ];
        parser.line_ids = vec![1, 2, 3, 4];

        assert!(parser.is_specblock("Config", 1));
    }

    #[test]
    fn test_full_parse_with_specblocks() {
        let mut parser = CastagneParser::new();

        parser.current_lines = vec![
            ":Character:".to_string(),
            "Name: Fighter".to_string(),
            "".to_string(),
            ":Attacks:".to_string(),
            "LightDamage: 5".to_string(),
            "HeavyDamage: 15".to_string(),
            "".to_string(),
            ":Variables:".to_string(),
            "var Health(Int): 100".to_string(),
            "".to_string(),
            ":Idle:".to_string(),
            "---Init:".to_string(),
            "Set(Health, 100)".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["test.casp".to_string()];

        parser.parse_full_file();

        let result = parser.end_parsing();
        assert!(result.is_some());

        let character = result.unwrap();

        // Should have metadata
        assert_eq!(character.metadata.name, "Fighter");

        // Should have specblock
        assert_eq!(character.specblocks.len(), 1);
        let attacks = character.specblocks.get("Attacks").unwrap();
        assert_eq!(attacks.get("LightDamage"), Some(&"5".to_string()));
        assert_eq!(attacks.get("HeavyDamage"), Some(&"15".to_string()));

        // Should have variables
        assert!(character.variables.contains_key("Health"));

        // Should have states
        assert!(character.states.contains_key("Idle"));
    }

    #[test]
    fn test_skeleton_inheritance() {
        // Test with actual files if they exist
        if std::path::Path::new("test_parent.casp").exists()
            && std::path::Path::new("test_child.casp").exists() {

            let mut parser = CastagneParser::new();
            let result = parser.create_full_character("test_child.casp");

            assert!(result.is_some(), "Failed to parse child character with skeleton");
            let character = result.unwrap();

            // Check that child metadata is preserved (not parent's)
            assert_eq!(character.metadata.name, "Child Character");
            assert_eq!(character.metadata.author, "Parser Test");

            // Check that child has parent's variables
            assert!(character.variables.contains_key("ParentOnly"),
                "Child should inherit ParentOnly variable from parent");

            // Check that child has its own variables
            assert!(character.variables.contains_key("ChildOnly"),
                "Child should have its own ChildOnly variable");

            // Check that child overrides parent's variables
            let health = character.variables.get("Health").unwrap();
            assert_eq!(health.value, "150",
                "Child should override parent's Health value");

            // Check that child has parent's Speed variable
            assert!(character.variables.contains_key("Speed"),
                "Child should inherit Speed variable from parent");

            // Check specblock inheritance and override
            let config = character.specblocks.get("Config").unwrap();
            assert_eq!(config.get("BaseSpeed"), Some(&"10".to_string()),
                "Child should override parent's BaseSpeed");
            assert_eq!(config.get("BaseJump"), Some(&"10".to_string()),
                "Child should inherit parent's BaseJump");
            assert_eq!(config.get("ChildSpeed"), Some(&"8".to_string()),
                "Child should have its own ChildSpeed");

            // Check state inheritance
            assert!(character.states.contains_key("BaseAttack"),
                "Child should inherit BaseAttack state from parent");
            assert!(character.states.contains_key("Idle"),
                "Child should have its own Idle state");
        }
    }

    #[test]
    fn test_skeleton_inheritance_unit() {
        // Unit test that doesn't require external files
        let mut parent_parser = CastagneParser::new();
        parent_parser.current_lines = vec![
            ":Character:".to_string(),
            "Name: Parent".to_string(),
            "".to_string(),
            ":Variables:".to_string(),
            "var ParentVar(Int): 10".to_string(),
            "var SharedVar(Int): 5".to_string(),
            "".to_string(),
        ];
        parent_parser.line_ids = (1..=parent_parser.current_lines.len()).collect();
        parent_parser.file_paths = vec!["parent.casp".to_string()];

        parent_parser.parse_full_file();

        // Manually simulate child inheriting from parent
        let mut child_parser = CastagneParser::new();
        child_parser.current_lines = vec![
            ":Character:".to_string(),
            "Name: Child".to_string(),
            "".to_string(),
            ":Variables:".to_string(),
            "var ChildVar(Str): ChildValue".to_string(),
            "var SharedVar(Int): 20".to_string(),
            "".to_string(),
        ];
        child_parser.line_ids = (1..=child_parser.current_lines.len()).collect();
        child_parser.file_paths = vec!["child.casp".to_string()];

        // Parse child metadata first
        child_parser.parse_metadata(0);

        // Manually merge parent variables (simulating skeleton loading)
        child_parser.variables = parent_parser.variables.clone();

        // Then parse child variables (which should override)
        child_parser.parse_variables(0);

        // Child should have both parent and child variables
        assert!(child_parser.variables.contains_key("ParentVar"),
            "Child should have parent's ParentVar");
        assert!(child_parser.variables.contains_key("ChildVar"),
            "Child should have its own ChildVar");

        // Child should override parent's SharedVar
        let shared = child_parser.variables.get("SharedVar").unwrap();
        assert_eq!(shared.value, "20",
            "Child should override parent's SharedVar with value 20");
    }

    #[test]
    fn test_type_conversion_int() {
        let var = ParsedVariable {
            name: "TestInt".to_string(),
            mutability: VariableMutability::Variable,
            var_type: VariableType::Int,
            subtype: String::new(),
            value: "42".to_string(),
        };

        // Test the helper methods that don't require Godot runtime
        assert_eq!(var.as_int(), Some(42));

        // Note: to_variant() requires Godot runtime and is tested in integration tests
    }

    #[test]
    fn test_type_conversion_bool() {
        let var_true = ParsedVariable {
            name: "TestBool".to_string(),
            mutability: VariableMutability::Variable,
            var_type: VariableType::Bool,
            subtype: String::new(),
            value: "true".to_string(),
        };

        assert_eq!(var_true.as_bool(), Some(true));

        let var_false = ParsedVariable {
            name: "TestBool2".to_string(),
            mutability: VariableMutability::Variable,
            var_type: VariableType::Bool,
            subtype: String::new(),
            value: "false".to_string(),
        };

        assert_eq!(var_false.as_bool(), Some(false));

        // Test numeric bool representations
        let var_one = ParsedVariable {
            name: "TestBool3".to_string(),
            mutability: VariableMutability::Variable,
            var_type: VariableType::Bool,
            subtype: String::new(),
            value: "1".to_string(),
        };

        assert_eq!(var_one.as_bool(), Some(true));
    }

    #[test]
    fn test_type_conversion_string() {
        let var = ParsedVariable {
            name: "TestStr".to_string(),
            mutability: VariableMutability::Variable,
            var_type: VariableType::Str,
            subtype: String::new(),
            value: "Hello World".to_string(),
        };

        assert_eq!(var.value, "Hello World");
    }

    #[test]
    fn test_type_conversion_float() {
        let var = ParsedVariable {
            name: "TestFloat".to_string(),
            mutability: VariableMutability::Variable,
            var_type: VariableType::Var,
            subtype: String::new(),
            value: "3.14".to_string(),
        };

        assert_eq!(var.as_float(), Some(3.14));
    }

    #[test]
    fn test_parse_vec2_string() {
        // Test parsing Vec2 values (string representation)
        // Note: Actual Variant creation requires Godot runtime, tested in integration tests

        // Test that we can identify valid Vec2 format
        let vec2_str = "(10, 20)";
        let cleaned = vec2_str.trim().trim_matches(|c| c == '(' || c == ')');
        let parts: Vec<&str> = cleaned.split(',').collect();
        assert_eq!(parts.len(), 2);
        assert!(parts[0].trim().parse::<f32>().is_ok());
        assert!(parts[1].trim().parse::<f32>().is_ok());

        // Test another valid format
        let vec2_str2 = "5.5, 7.5";
        let cleaned2 = vec2_str2.trim().trim_matches(|c| c == '(' || c == ')');
        let parts2: Vec<&str> = cleaned2.split(',').collect();
        assert_eq!(parts2.len(), 2);
        assert!(parts2[0].trim().parse::<f32>().is_ok());
        assert!(parts2[1].trim().parse::<f32>().is_ok());

        // Test invalid format
        let invalid = "invalid";
        let cleaned_invalid = invalid.trim().trim_matches(|c| c == '(' || c == ')');
        let parts_invalid: Vec<&str> = cleaned_invalid.split(',').collect();
        assert!(parts_invalid.len() != 2 || parts_invalid[0].trim().parse::<f32>().is_err());
    }

    #[test]
    fn test_parse_vec3_string() {
        // Test parsing Vec3 values (string representation)
        // Note: Actual Variant creation requires Godot runtime, tested in integration tests

        // Test valid Vec3 format
        let vec3_str = "(10, 20, 30)";
        let cleaned = vec3_str.trim().trim_matches(|c| c == '(' || c == ')');
        let parts: Vec<&str> = cleaned.split(',').collect();
        assert_eq!(parts.len(), 3);
        assert!(parts[0].trim().parse::<f32>().is_ok());
        assert!(parts[1].trim().parse::<f32>().is_ok());
        assert!(parts[2].trim().parse::<f32>().is_ok());

        // Test without parentheses
        let vec3_str2 = "5.5, 7.5, 9.5";
        let cleaned2 = vec3_str2.trim().trim_matches(|c| c == '(' || c == ')');
        let parts2: Vec<&str> = cleaned2.split(',').collect();
        assert_eq!(parts2.len(), 3);

        // Test invalid (only 2 components)
        let invalid = "10, 20";
        let cleaned_invalid = invalid.trim().trim_matches(|c| c == '(' || c == ')');
        let parts_invalid: Vec<&str> = cleaned_invalid.split(',').collect();
        assert_ne!(parts_invalid.len(), 3);
    }

    #[test]
    fn test_type_inference_var_type() {
        // For Var type, parser should infer the actual type

        // Should infer as int
        let int_var = ParsedVariable {
            name: "AutoInt".to_string(),
            mutability: VariableMutability::Variable,
            var_type: VariableType::Var,
            subtype: String::new(),
            value: "100".to_string(),
        };
        assert_eq!(int_var.as_int(), Some(100));

        // Should infer as bool
        let bool_var = ParsedVariable {
            name: "AutoBool".to_string(),
            mutability: VariableMutability::Variable,
            var_type: VariableType::Var,
            subtype: String::new(),
            value: "true".to_string(),
        };
        assert_eq!(bool_var.as_bool(), Some(true));

        // Should infer as float
        let float_var = ParsedVariable {
            name: "AutoFloat".to_string(),
            mutability: VariableMutability::Variable,
            var_type: VariableType::Var,
            subtype: String::new(),
            value: "2.5".to_string(),
        };
        assert_eq!(float_var.as_float(), Some(2.5));
    }

    #[test]
    fn test_complete_character_file() {
        // Test parsing the comprehensive character file
        if std::path::Path::new("test_character_complete.casp").exists() {
            let mut parser = CastagneParser::new();
            let result = parser.create_full_character("test_character_complete.casp");

            assert!(result.is_some(), "Failed to parse complete character file");
            let character = result.unwrap();

            // Verify metadata
            assert_eq!(character.metadata.name, "Complete Test Fighter");
            assert_eq!(character.metadata.author, "Parser Development Team");

            // Verify specblocks
            assert!(character.specblocks.contains_key("AttackData"));
            assert!(character.specblocks.contains_key("PhysicsConfig"));

            let attack_data = character.specblocks.get("AttackData").unwrap();
            assert_eq!(attack_data.get("LightPunchDamage"), Some(&"5".to_string()));
            assert_eq!(attack_data.get("HeavyPunchDamage"), Some(&"15".to_string()));

            let physics = character.specblocks.get("PhysicsConfig").unwrap();
            assert_eq!(physics.get("Gravity"), Some(&"10".to_string()));
            assert_eq!(physics.get("JumpForce"), Some(&"25".to_string()));

            // Verify variables with different types
            assert!(character.variables.contains_key("Health"));
            assert!(character.variables.contains_key("PlayerName"));
            assert!(character.variables.contains_key("IsGrounded"));
            assert!(character.variables.contains_key("Position"));
            assert!(character.variables.contains_key("Meter"));
            assert!(character.variables.contains_key("MAX_COMBO"));

            let health = character.variables.get("Health").unwrap();
            assert_eq!(health.var_type, VariableType::Int);
            assert_eq!(health.value, "150");

            let is_grounded = character.variables.get("IsGrounded").unwrap();
            assert_eq!(is_grounded.var_type, VariableType::Bool);
            assert_eq!(is_grounded.value, "true");

            let position = character.variables.get("Position").unwrap();
            assert_eq!(position.var_type, VariableType::Vec2);
            assert_eq!(position.value, "0, 0");

            // Verify states
            assert!(character.states.contains_key("Idle"));
            assert!(character.states.contains_key("Walk"));
            assert!(character.states.contains_key("Jump"));
            assert!(character.states.contains_key("LightPunch"));
            assert!(character.states.contains_key("HeavyPunch"));

            // Verify state phases
            let idle = character.states.get("Idle").unwrap();
            assert!(idle.actions.contains_key("Init"));
            assert!(idle.actions.contains_key("Action"));

            let jump = character.states.get("Jump").unwrap();
            assert!(jump.actions.contains_key("Init"));
            assert!(jump.actions.contains_key("Action"));

            // Verify complex actions
            let light_punch = character.states.get("LightPunch").unwrap();
            assert!(light_punch.actions.contains_key("Init"));
            assert!(light_punch.actions.contains_key("Action"));
            assert!(light_punch.actions.contains_key("Reaction"));
        }
    }

    #[test]
    fn test_parse_state_header_simple() {
        let parser = CastagneParser::new();

        // Simple state name
        let (name, state_type, parent) = parser.parse_state_header("Idle");
        assert_eq!(name, "Idle");
        assert_eq!(state_type, StateType::Normal);
        assert_eq!(parent, None);
    }

    #[test]
    fn test_parse_state_directly_simple() {
        let mut parser = CastagneParser::new();

        parser.current_lines = vec![
            ":Idle:".to_string(),
            "---Init:".to_string(),
            "Set(x, 1)".to_string(),
            "".to_string(),
        ];
        parser.line_ids = vec![1, 2, 3, 4];
        parser.file_paths = vec!["test.casp".to_string()];

        parser.parse_states(0);

        // This should work since we tested it before
        assert_eq!(parser.states.len(), 1);
        assert!(parser.states.contains_key("Idle"));
    }

    #[test]
    fn test_parse_state_header_with_type() {
        let parser = CastagneParser::new();

        // State with Helper type
        let (name, state_type, parent) = parser.parse_state_header("GroundBounce(Helper)");
        assert_eq!(name, "GroundBounce");
        assert_eq!(state_type, StateType::Helper);
        assert_eq!(parent, None);

        // State with BaseState type
        let (name2, state_type2, parent2) = parser.parse_state_header("CommonActions(BaseState)");
        assert_eq!(name2, "CommonActions");
        assert_eq!(state_type2, StateType::BaseState);
        assert_eq!(parent2, None);

        // State with Special type
        let (name3, state_type3, parent3) = parser.parse_state_header("SuperMove(Special)");
        assert_eq!(name3, "SuperMove");
        assert_eq!(state_type3, StateType::Special);
        assert_eq!(parent3, None);

        // State with Specblock type
        let (name4, state_type4, parent4) = parser.parse_state_header("ConfigBlock(Specblock)");
        assert_eq!(name4, "ConfigBlock");
        assert_eq!(state_type4, StateType::Specblock);
        assert_eq!(parent4, None);
    }

    #[test]
    fn test_parse_state_header_with_parent() {
        let parser = CastagneParser::new();

        // State with parent (no type specified)
        let (name, state_type, parent) = parser.parse_state_header("Walk(Idle)");
        assert_eq!(name, "Walk");
        assert_eq!(state_type, StateType::Normal);
        assert_eq!(parent, Some("Idle".to_string()));

        // State with different parent
        let (name2, state_type2, parent2) = parser.parse_state_header("Run(Walk)");
        assert_eq!(name2, "Run");
        assert_eq!(state_type2, StateType::Normal);
        assert_eq!(parent2, Some("Walk".to_string()));
    }

    #[test]
    fn test_parse_state_header_with_type_and_parent() {
        let parser = CastagneParser::new();

        // State with both type and parent
        let (name, state_type, parent) = parser.parse_state_header("Projectile(Helper, Idle)");
        assert_eq!(name, "Projectile");
        assert_eq!(state_type, StateType::Helper);
        assert_eq!(parent, Some("Idle".to_string()));

        // Another combination
        let (name2, state_type2, parent2) = parser.parse_state_header("CustomMove(Special, BaseAttack)");
        assert_eq!(name2, "CustomMove");
        assert_eq!(state_type2, StateType::Special);
        assert_eq!(parent2, Some("BaseAttack".to_string()));
    }

    #[test]
    fn test_parse_state_with_attributes() {
        let mut parser = CastagneParser::new();

        parser.current_lines = vec![
            ":GroundBounce(Helper):".to_string(),
            "---Init:".to_string(),
            "Set(Velocity, 0, 10)".to_string(),
            "".to_string(),
        ];
        parser.line_ids = vec![1, 2, 3, 4];
        parser.file_paths = vec!["test.casp".to_string()];

        // Don't parse specblocks first - just parse states directly
        parser.parse_states(0);

        assert_eq!(parser.states.len(), 1, "Expected 1 state, found {}: {:?}", parser.states.len(), parser.states.keys().collect::<Vec<_>>());
        let state = parser.states.get("GroundBounce").unwrap();
        assert_eq!(state.name, "GroundBounce");
        assert_eq!(state.state_type, StateType::Helper);
        assert_eq!(state.parent, None);
    }

    #[test]
    fn test_parse_state_with_parent_state() {
        let mut parser = CastagneParser::new();

        parser.current_lines = vec![
            ":Walk(Idle):".to_string(),
            "---Init:".to_string(),
            "Set(Speed, 5)".to_string(),
            "---Action:".to_string(),
            "Move()".to_string(),
            "".to_string(),
        ];
        parser.line_ids = vec![1, 2, 3, 4, 5, 6];
        parser.file_paths = vec!["test.casp".to_string()];

        parser.parse_states(0);

        assert_eq!(parser.states.len(), 1);
        let state = parser.states.get("Walk").unwrap();
        assert_eq!(state.name, "Walk");
        assert_eq!(state.state_type, StateType::Normal);
        assert_eq!(state.parent, Some("Idle".to_string()));
        assert!(state.actions.contains_key("Init"));
        assert!(state.actions.contains_key("Action"));
    }

    #[test]
    fn test_parse_state_with_type_and_parent_combined() {
        let mut parser = CastagneParser::new();

        parser.current_lines = vec![
            ":Fireball(Helper, Idle):".to_string(),
            "---Init:".to_string(),
            "Set(Damage, 10)".to_string(),
            "Set(Speed, 20)".to_string(),
            "---Action:".to_string(),
            "MoveForward()".to_string(),
            "CheckCollision()".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["test.casp".to_string()];

        parser.parse_states(0);

        assert_eq!(parser.states.len(), 1);
        let state = parser.states.get("Fireball").unwrap();
        assert_eq!(state.name, "Fireball");
        assert_eq!(state.state_type, StateType::Helper);
        assert_eq!(state.parent, Some("Idle".to_string()));

        // Verify actions were parsed correctly
        let init_actions = state.actions.get("Init").unwrap();
        assert_eq!(init_actions.len(), 2);
        assert_eq!(init_actions[0].instruction, "Set");
        assert_eq!(init_actions[0].args[0], "Damage");
        assert_eq!(init_actions[0].args[1], "10");

        let action_actions = state.actions.get("Action").unwrap();
        assert_eq!(action_actions.len(), 2);
        assert_eq!(action_actions[0].instruction, "MoveForward");
        assert_eq!(action_actions[1].instruction, "CheckCollision");
    }

    #[test]
    fn test_multiple_states_with_different_types() {
        let mut parser = CastagneParser::new();

        parser.current_lines = vec![
            ":Idle:".to_string(),
            "---Init:".to_string(),
            "Set(State, 0)".to_string(),
            "".to_string(),
            ":Walk(Idle):".to_string(),
            "---Init:".to_string(),
            "Set(State, 1)".to_string(),
            "".to_string(),
            ":Attack(Helper):".to_string(),
            "---Init:".to_string(),
            "Set(State, 2)".to_string(),
            "".to_string(),
            ":SuperMove(Special, Attack):".to_string(),
            "---Init:".to_string(),
            "Set(State, 3)".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["test.casp".to_string()];

        parser.parse_states(0);

        assert_eq!(parser.states.len(), 4);

        // Check Idle (simple state)
        let idle = parser.states.get("Idle").unwrap();
        assert_eq!(idle.state_type, StateType::Normal);
        assert_eq!(idle.parent, None);

        // Check Walk (has parent)
        let walk = parser.states.get("Walk").unwrap();
        assert_eq!(walk.state_type, StateType::Normal);
        assert_eq!(walk.parent, Some("Idle".to_string()));

        // Check Attack (has type)
        let attack = parser.states.get("Attack").unwrap();
        assert_eq!(attack.state_type, StateType::Helper);
        assert_eq!(attack.parent, None);

        // Check SuperMove (has both type and parent)
        let super_move = parser.states.get("SuperMove").unwrap();
        assert_eq!(super_move.state_type, StateType::Special);
        assert_eq!(super_move.parent, Some("Attack".to_string()));
    }

    #[test]
    fn test_strip_inline_comment() {
        let parser = CastagneParser::new();

        // Simple comment
        assert_eq!(parser.strip_inline_comment("Set(x, 5) # This sets x"), "Set(x, 5) ");

        // Comment with no space
        assert_eq!(parser.strip_inline_comment("Set(x, 5)# comment"), "Set(x, 5)");

        // Comment in string should be preserved
        assert_eq!(parser.strip_inline_comment(r#"Set(msg, "Hello # World")"#), r#"Set(msg, "Hello # World")"#);

        // Empty line with just comment
        assert_eq!(parser.strip_inline_comment("# Just a comment"), "");

        // No comment
        assert_eq!(parser.strip_inline_comment("Set(x, 5)"), "Set(x, 5)");

        // Multiple # in string
        assert_eq!(parser.strip_inline_comment("Log(\"Test#123\") # comment"), "Log(\"Test#123\") ");

        // Escaped quote in string
        assert_eq!(parser.strip_inline_comment(r#"Set(x, "He said \"Hi\"") # comment"#), r#"Set(x, "He said \"Hi\"") "#);
    }

    #[test]
    fn test_parse_with_inline_comments() {
        let mut parser = CastagneParser::new();

        parser.current_lines = vec![
            ":Character:".to_string(),
            "Name: Test Fighter # Main character".to_string(),
            "Author: Test # Test author".to_string(),
            "".to_string(),
            ":Variables:".to_string(),
            "var Health(Int): 100 # Player health".to_string(),
            "def MAX_HP: 150 # Maximum health".to_string(),
            "".to_string(),
            ":Idle:".to_string(),
            "---Init: # Initialization phase".to_string(),
            "Set(Health, 100) # Set initial health".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["test.casp".to_string()];

        parser.parse_full_file();
        let result = parser.end_parsing();

        assert!(result.is_some());
        let character = result.unwrap();

        // Verify metadata was parsed correctly (without comments)
        assert_eq!(character.metadata.name, "Test Fighter");
        assert_eq!(character.metadata.author, "Test");

        // Verify variables were parsed correctly (without comments)
        assert!(character.variables.contains_key("Health"));
        let health = character.variables.get("Health").unwrap();
        assert_eq!(health.value, "100");

        assert!(character.variables.contains_key("MAX_HP"));
        let max_hp = character.variables.get("MAX_HP").unwrap();
        assert_eq!(max_hp.value, "150");

        // Verify states were parsed correctly (without comments)
        assert!(character.states.contains_key("Idle"));
        let idle = character.states.get("Idle").unwrap();
        let init_actions = idle.actions.get("Init").unwrap();
        assert_eq!(init_actions.len(), 1);
        assert_eq!(init_actions[0].instruction, "Set");
        assert_eq!(init_actions[0].args[0], "Health");
        assert_eq!(init_actions[0].args[1], "100");
    }

    #[test]
    fn test_inline_comment_with_strings() {
        let mut parser = CastagneParser::new();

        parser.current_lines = vec![
            ":Idle:".to_string(),
            "---Init:".to_string(),
            r#"Log("Test # not a comment") # This is a comment"#.to_string(),
            r#"Set(Message, "Hello, World!") # Set greeting"#.to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["test.casp".to_string()];

        parser.parse_states(0);

        let idle = parser.states.get("Idle").unwrap();
        let init_actions = idle.actions.get("Init").unwrap();

        assert_eq!(init_actions.len(), 2);

        // First action: Log with # in string
        assert_eq!(init_actions[0].instruction, "Log");
        assert_eq!(init_actions[0].args.len(), 1);
        assert_eq!(init_actions[0].args[0], r#""Test # not a comment""#);

        // Second action: Set with comment after
        assert_eq!(init_actions[1].instruction, "Set");
        assert_eq!(init_actions[1].args.len(), 2);
        assert_eq!(init_actions[1].args[0], "Message");
        assert_eq!(init_actions[1].args[1], r#""Hello, World!""#);
    }

    #[test]
    fn test_specblock_with_inline_comments() {
        let mut parser = CastagneParser::new();

        parser.current_lines = vec![
            ":Config:".to_string(),
            "MaxSpeed: 100 # Maximum movement speed".to_string(),
            "JumpHeight: 50 # Jump force".to_string(),
            "Gravity: 10 # Gravity value".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["test.casp".to_string()];

        parser.parse_specblocks(0);

        assert!(parser.specblocks.contains_key("Config"));
        let config = parser.specblocks.get("Config").unwrap();

        // Values should be parsed without comments
        assert_eq!(config.get("MaxSpeed"), Some(&"100".to_string()));
        assert_eq!(config.get("JumpHeight"), Some(&"50".to_string()));
        assert_eq!(config.get("Gravity"), Some(&"10".to_string()));
    }

    // =========================================================================
    // Error Handling Tests
    // =========================================================================

    #[test]
    fn test_error_empty_file() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![];
        parser.line_ids = vec![];
        parser.file_paths = vec!["empty.casp".to_string()];

        parser.parse_full_file();
        let result = parser.end_parsing();

        // Empty file should still parse (with no data)
        assert!(result.is_some());
        let character = result.unwrap();
        assert_eq!(character.states.len(), 0);
        assert_eq!(character.variables.len(), 0);
    }

    #[test]
    fn test_error_only_comments() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            "# This is just a comment".to_string(),
            "# Another comment".to_string(),
            "".to_string(),
            "# More comments".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["comments.casp".to_string()];

        parser.parse_full_file();
        let result = parser.end_parsing();

        // File with only comments should parse successfully
        assert!(result.is_some());
        let character = result.unwrap();
        assert_eq!(character.states.len(), 0);
        assert_eq!(character.variables.len(), 0);
    }

    #[test]
    fn test_error_missing_character_block() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Variables:".to_string(),
            "var Health(Int): 100".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["no_character.casp".to_string()];

        parser.parse_full_file();
        let result = parser.end_parsing();

        // Should still parse even without Character block
        assert!(result.is_some());
        let character = result.unwrap();
        assert!(character.variables.contains_key("Health"));
    }

    #[test]
    fn test_error_invalid_variable_syntax() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Variables:".to_string(),
            "var NoColon 100".to_string(), // Missing colon
            "var Health(Int): 100".to_string(), // Valid for comparison
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["invalid_var.casp".to_string()];

        parser.parse_variables(0);

        // Valid variable should parse
        assert!(parser.variables.contains_key("Health"));

        // Invalid variable should not parse
        assert!(!parser.variables.contains_key("NoColon"));
    }

    #[test]
    fn test_error_malformed_state_header() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            "Idle".to_string(), // Missing colons
            "---Init:".to_string(),
            "Set(x, 1)".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["bad_state.casp".to_string()];

        parser.parse_states(0);

        // Malformed state header should not create a state
        assert!(!parser.states.contains_key("Idle"));
    }

    #[test]
    fn test_error_unclosed_parentheses_in_action() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Idle:".to_string(),
            "---Init:".to_string(),
            "Set(x, 1".to_string(), // Missing closing paren
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["unclosed_paren.casp".to_string()];

        parser.parse_states(0);

        // Should still create state, but action might be malformed
        assert!(parser.states.contains_key("Idle"));
        let idle = parser.states.get("Idle").unwrap();
        let init_actions = idle.actions.get("Init");

        // Action parsing should handle this gracefully
        assert!(init_actions.is_some());
    }

    #[test]
    fn test_error_invalid_type_in_variable() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Variables:".to_string(),
            "var BadType(InvalidType): 100".to_string(),
            "var GoodType(Int): 100".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["invalid_type.casp".to_string()];

        parser.parse_variables(0);

        // Valid variable should parse
        assert!(parser.variables.contains_key("GoodType"));

        // Invalid type should default to Var type
        if parser.variables.contains_key("BadType") {
            let bad_var = parser.variables.get("BadType").unwrap();
            assert_eq!(bad_var.var_type, VariableType::Var);
        }
    }

    #[test]
    fn test_error_invalid_state_type() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":BadState(InvalidType):".to_string(),
            "---Init:".to_string(),
            "Set(x, 1)".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["invalid_state_type.casp".to_string()];

        parser.parse_states(0);

        // Should still parse but might default to Normal type or handle gracefully
        if parser.states.contains_key("BadState") {
            let state = parser.states.get("BadState").unwrap();
            // Should default to Normal or handle gracefully
            assert!(matches!(state.state_type, StateType::Normal | StateType::Helper | StateType::Special));
        }
    }

    #[test]
    fn test_error_duplicate_variable_names() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Variables:".to_string(),
            "var Health(Int): 100".to_string(),
            "var Health(Int): 200".to_string(), // Duplicate
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["duplicate_var.casp".to_string()];

        parser.parse_variables(0);

        // Last definition should win (or first, depending on implementation)
        assert!(parser.variables.contains_key("Health"));
        let health = parser.variables.get("Health").unwrap();
        // Could be either 100 or 200 depending on implementation
        assert!(health.value == "100" || health.value == "200");
    }

    #[test]
    fn test_error_duplicate_state_names() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Idle:".to_string(),
            "---Init:".to_string(),
            "Set(x, 1)".to_string(),
            "".to_string(),
            ":Idle:".to_string(), // Duplicate
            "---Init:".to_string(),
            "Set(x, 2)".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["duplicate_state.casp".to_string()];

        parser.parse_states(0);

        // Should have one Idle state (last one should overwrite or merge)
        assert!(parser.states.contains_key("Idle"));
        assert_eq!(parser.states.len(), 1);
    }

    #[test]
    fn test_error_missing_phase_marker() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Idle:".to_string(),
            "Set(x, 1)".to_string(), // No phase marker
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["no_phase.casp".to_string()];

        parser.parse_states(0);

        // Should handle gracefully - might skip action or assign to default phase
        assert!(parser.states.contains_key("Idle"));
    }

    #[test]
    fn test_error_empty_state() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Empty:".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["empty_state.casp".to_string()];

        parser.parse_states(0);

        // Empty state should still be created
        assert!(parser.states.contains_key("Empty"));
        let empty = parser.states.get("Empty").unwrap();
        assert_eq!(empty.actions.len(), 0);
    }

    #[test]
    fn test_error_special_characters_in_names() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Variables:".to_string(),
            "var Test@Name(Int): 100".to_string(), // Special char in name
            "var Normal_Name123(Int): 200".to_string(), // Valid name
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["special_chars.casp".to_string()];

        parser.parse_variables(0);

        // Normal name should parse
        assert!(parser.variables.contains_key("Normal_Name123"));

        // Special character name might be rejected or sanitized
        // (depends on implementation - just check it doesn't crash)
    }

    #[test]
    fn test_error_very_long_line() {
        let mut parser = CastagneParser::new();

        // Create a very long action line
        let long_args = (0..1000).map(|i| format!("arg{}", i)).collect::<Vec<_>>().join(", ");
        let long_line = format!("Function({})", long_args);

        parser.current_lines = vec![
            ":Test:".to_string(),
            "---Init:".to_string(),
            long_line,
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["long_line.casp".to_string()];

        parser.parse_states(0);

        // Should handle long lines without crashing
        assert!(parser.states.contains_key("Test"));
    }

    #[test]
    fn test_error_nested_quotes_in_strings() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Test:".to_string(),
            "---Init:".to_string(),
            r#"Set(msg, "He said \"Hello\" to me")"#.to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["nested_quotes.casp".to_string()];

        parser.parse_states(0);

        let test = parser.states.get("Test").unwrap();
        let actions = test.actions.get("Init").unwrap();

        // Should parse escaped quotes correctly
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].instruction, "Set");
    }

    #[test]
    fn test_error_unicode_in_values() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Character:".to_string(),
            "Name: 戦士".to_string(), // Japanese characters
            "Description: Étoile ⭐".to_string(), // French + emoji
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["unicode.casp".to_string()];

        parser.parse_metadata(0);

        // Should handle unicode gracefully
        assert_eq!(parser.metadata.name, "戦士");
        assert_eq!(parser.metadata.description, "Étoile ⭐");
    }

    #[test]
    fn test_error_mixing_tabs_and_spaces() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Variables:".to_string(),
            "var Health(Int): 100".to_string(), // Using spaces
            "\tvar\tMana(Int):\t50".to_string(), // Using tabs
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["mixed_whitespace.casp".to_string()];

        parser.parse_variables(0);

        // Should handle both gracefully
        assert!(parser.variables.contains_key("Health"));
        // Mana might or might not parse depending on implementation
    }

    // =========================================================================
    // Advanced Edge Case Tests
    // =========================================================================

    #[test]
    fn test_edge_deeply_nested_function_calls() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Test:".to_string(),
            "---Init:".to_string(),
            "Call(Outer(Middle(Inner(Deepest(5)))))".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["nested.casp".to_string()];

        parser.parse_states(0);

        let test = parser.states.get("Test").unwrap();
        let actions = test.actions.get("Init").unwrap();

        // Should parse deeply nested calls
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].instruction, "Call");
    }

    #[test]
    fn test_edge_mixed_nested_arguments() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Test:".to_string(),
            "---Init:".to_string(),
            r#"Complex(1, "text", Vec2(10, 20), true, Nested(a, b, c))"#.to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["complex_args.casp".to_string()];

        parser.parse_states(0);

        let test = parser.states.get("Test").unwrap();
        let actions = test.actions.get("Init").unwrap();

        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].instruction, "Complex");
        // Should have multiple arguments including nested call
        assert!(actions[0].args.len() > 0);
    }

    #[test]
    fn test_edge_empty_arguments() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Test:".to_string(),
            "---Init:".to_string(),
            "EmptyCall()".to_string(),
            "OneArg(x)".to_string(),
            "TwoArgs(x, y)".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["empty_args.casp".to_string()];

        parser.parse_states(0);

        let test = parser.states.get("Test").unwrap();
        let actions = test.actions.get("Init").unwrap();

        assert_eq!(actions.len(), 3);
        assert_eq!(actions[0].instruction, "EmptyCall");
        assert_eq!(actions[0].args.len(), 0);
        assert_eq!(actions[1].args.len(), 1);
        assert_eq!(actions[2].args.len(), 2);
    }

    #[test]
    fn test_edge_whitespace_variations() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Test:".to_string(),
            "---Init:".to_string(),
            "NoSpace(x,y,z)".to_string(),
            "WithSpace(x, y, z)".to_string(),
            "ExtraSpace(x  ,  y  ,  z)".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["whitespace.casp".to_string()];

        parser.parse_states(0);

        let test = parser.states.get("Test").unwrap();
        let actions = test.actions.get("Init").unwrap();

        // All three should parse successfully
        assert_eq!(actions.len(), 3);
        for action in actions {
            assert_eq!(action.args.len(), 3);
        }
    }

    #[test]
    fn test_edge_multiple_phases_same_state() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Idle:".to_string(),
            "---Init:".to_string(),
            "InitAction()".to_string(),
            "---Action:".to_string(),
            "ActionPhase()".to_string(),
            "---Reaction:".to_string(),
            "ReactionPhase()".to_string(),
            "---Freeze:".to_string(),
            "FreezePhase()".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["multi_phase.casp".to_string()];

        parser.parse_states(0);

        let idle = parser.states.get("Idle").unwrap();

        // Should have all four phases
        assert!(idle.actions.contains_key("Init"));
        assert!(idle.actions.contains_key("Action"));
        assert!(idle.actions.contains_key("Reaction"));
        assert!(idle.actions.contains_key("Freeze"));

        // Each phase should have one action
        assert_eq!(idle.actions.get("Init").unwrap().len(), 1);
        assert_eq!(idle.actions.get("Action").unwrap().len(), 1);
        assert_eq!(idle.actions.get("Reaction").unwrap().len(), 1);
        assert_eq!(idle.actions.get("Freeze").unwrap().len(), 1);
    }

    #[test]
    fn test_edge_repeated_phases() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Idle:".to_string(),
            "---Init:".to_string(),
            "First()".to_string(),
            "---Action:".to_string(),
            "Middle()".to_string(),
            "---Init:".to_string(), // Repeated phase
            "Second()".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["repeated_phase.casp".to_string()];

        parser.parse_states(0);

        let idle = parser.states.get("Idle").unwrap();

        // Init should have actions from both declarations
        let init_actions = idle.actions.get("Init").unwrap();
        assert!(init_actions.len() >= 1); // At least one action
    }

    #[test]
    fn test_edge_all_variable_types() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Variables:".to_string(),
            "var IntVar(Int): 42".to_string(),
            "var StrVar(Str): Hello".to_string(),
            "var BoolVar(Bool): true".to_string(),
            "var Vec2Var(Vec2): 10, 20".to_string(),
            "var Vec3Var(Vec3): 1, 2, 3".to_string(),
            "var VarVar(Var): dynamic".to_string(),
            "var BoxVar(Box): boxed".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["all_types.casp".to_string()];

        parser.parse_variables(0);

        // All variable types should parse
        assert!(parser.variables.contains_key("IntVar"));
        assert!(parser.variables.contains_key("StrVar"));
        assert!(parser.variables.contains_key("BoolVar"));
        assert!(parser.variables.contains_key("Vec2Var"));
        assert!(parser.variables.contains_key("Vec3Var"));
        assert!(parser.variables.contains_key("VarVar"));
        assert!(parser.variables.contains_key("BoxVar"));

        // Check types are correct
        assert_eq!(parser.variables.get("IntVar").unwrap().var_type, VariableType::Int);
        assert_eq!(parser.variables.get("StrVar").unwrap().var_type, VariableType::Str);
        assert_eq!(parser.variables.get("BoolVar").unwrap().var_type, VariableType::Bool);
        assert_eq!(parser.variables.get("Vec2Var").unwrap().var_type, VariableType::Vec2);
        assert_eq!(parser.variables.get("Vec3Var").unwrap().var_type, VariableType::Vec3);
        assert_eq!(parser.variables.get("VarVar").unwrap().var_type, VariableType::Var);
        assert_eq!(parser.variables.get("BoxVar").unwrap().var_type, VariableType::Box);
    }

    #[test]
    fn test_edge_all_state_types() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":NormalState:".to_string(),
            "---Init:".to_string(),
            "Normal()".to_string(),
            "".to_string(),
            ":HelperState(Helper):".to_string(),
            "---Init:".to_string(),
            "Helper()".to_string(),
            "".to_string(),
            ":SpecialState(Special):".to_string(),
            "---Init:".to_string(),
            "Special()".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["all_state_types.casp".to_string()];

        parser.parse_states(0);

        assert_eq!(parser.states.get("NormalState").unwrap().state_type, StateType::Normal);
        assert_eq!(parser.states.get("HelperState").unwrap().state_type, StateType::Helper);
        assert_eq!(parser.states.get("SpecialState").unwrap().state_type, StateType::Special);
    }

    #[test]
    fn test_edge_string_with_escaped_characters() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Test:".to_string(),
            "---Init:".to_string(),
            r#"Log("Line1\nLine2\tTabbed")"#.to_string(),
            r#"Set(quote, "He said: \"Hi\"")"#.to_string(),
            r#"Set(path, "C:\\Users\\Game")"#.to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["escaped.casp".to_string()];

        parser.parse_states(0);

        let test = parser.states.get("Test").unwrap();
        let actions = test.actions.get("Init").unwrap();

        // Should parse all escaped strings
        assert_eq!(actions.len(), 3);
    }

    #[test]
    fn test_edge_negative_numbers() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Variables:".to_string(),
            "var NegInt(Int): -100".to_string(),
            "var NegVec2(Vec2): -5, -10".to_string(),
            "var NegVec3(Vec3): -1, -2, -3".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["negative.casp".to_string()];

        parser.parse_variables(0);

        assert!(parser.variables.contains_key("NegInt"));
        assert!(parser.variables.contains_key("NegVec2"));
        assert!(parser.variables.contains_key("NegVec3"));

        let neg_int = parser.variables.get("NegInt").unwrap();
        assert_eq!(neg_int.as_int(), Some(-100));
    }

    #[test]
    fn test_edge_float_numbers() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Variables:".to_string(),
            "var FloatVar(Int): 3.14".to_string(),
            "var FloatVec2(Vec2): 1.5, 2.5".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["floats.casp".to_string()];

        parser.parse_variables(0);

        // Should parse floats
        assert!(parser.variables.contains_key("FloatVar"));
        assert!(parser.variables.contains_key("FloatVec2"));
    }

    #[test]
    fn test_edge_action_without_parentheses() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Test:".to_string(),
            "---Init:".to_string(),
            "SimpleAction".to_string(), // No parentheses
            "ActionWithParens()".to_string(), // With parentheses
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["no_parens.casp".to_string()];

        parser.parse_states(0);

        let test = parser.states.get("Test").unwrap();
        let actions = test.actions.get("Init").unwrap();

        // Both should parse (or at least not crash)
        assert!(actions.len() >= 1);
    }

    #[test]
    fn test_edge_multiple_specblocks() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Config:".to_string(),
            "Speed: 100".to_string(),
            "".to_string(),
            ":Attacks:".to_string(),
            "Punch: 10".to_string(),
            "Kick: 20".to_string(),
            "".to_string(),
            ":CustomData:".to_string(),
            "Field1: Value1".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["multi_specblock.casp".to_string()];

        parser.parse_specblocks(0);

        // All specblocks should be parsed
        assert!(parser.specblocks.contains_key("Config"));
        assert!(parser.specblocks.contains_key("Attacks"));
        assert!(parser.specblocks.contains_key("CustomData"));

        assert_eq!(parser.specblocks.get("Config").unwrap().get("Speed"), Some(&"100".to_string()));
        assert_eq!(parser.specblocks.get("Attacks").unwrap().get("Punch"), Some(&"10".to_string()));
    }

    #[test]
    fn test_edge_state_inheritance_chain() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Base:".to_string(),
            "---Init:".to_string(),
            "BaseAction()".to_string(),
            "".to_string(),
            ":Middle(Base):".to_string(),
            "---Init:".to_string(),
            "MiddleAction()".to_string(),
            "".to_string(),
            ":Final(Middle):".to_string(),
            "---Init:".to_string(),
            "FinalAction()".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["chain.casp".to_string()];

        parser.parse_states(0);

        // All states should exist
        assert!(parser.states.contains_key("Base"));
        assert!(parser.states.contains_key("Middle"));
        assert!(parser.states.contains_key("Final"));

        // Check parent relationships
        assert_eq!(parser.states.get("Base").unwrap().parent, None);
        assert_eq!(parser.states.get("Middle").unwrap().parent, Some("Base".to_string()));
        assert_eq!(parser.states.get("Final").unwrap().parent, Some("Middle".to_string()));
    }

    #[test]
    fn test_edge_bool_value_variations() {
        let mut parser = CastagneParser::new();
        parser.current_lines = vec![
            ":Variables:".to_string(),
            "var Bool1(Bool): true".to_string(),
            "var Bool2(Bool): false".to_string(),
            "var Bool3(Bool): 1".to_string(),
            "var Bool4(Bool): 0".to_string(),
            "var Bool5(Bool): True".to_string(), // Capitalized
            "var Bool6(Bool): FALSE".to_string(), // All caps
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["bool_variations.casp".to_string()];

        parser.parse_variables(0);

        // All should parse
        assert!(parser.variables.contains_key("Bool1"));
        assert!(parser.variables.contains_key("Bool2"));
        assert!(parser.variables.contains_key("Bool3"));
        assert!(parser.variables.contains_key("Bool4"));
        assert!(parser.variables.contains_key("Bool5"));
        assert!(parser.variables.contains_key("Bool6"));

        // Check values are correct
        assert_eq!(parser.variables.get("Bool1").unwrap().as_bool(), Some(true));
        assert_eq!(parser.variables.get("Bool2").unwrap().as_bool(), Some(false));
        assert_eq!(parser.variables.get("Bool3").unwrap().as_bool(), Some(true));
        assert_eq!(parser.variables.get("Bool4").unwrap().as_bool(), Some(false));
    }

    // =========================================================================
    // Performance and Stress Tests
    // =========================================================================

    #[test]
    fn test_stress_many_states() {
        let mut parser = CastagneParser::new();

        // Create 100 states
        let mut lines = vec![];
        for i in 0..100 {
            lines.push(format!(":State{}:", i));
            lines.push("---Init:".to_string());
            lines.push(format!("Set(StateID, {})", i));
            lines.push("".to_string());
        }

        parser.current_lines = lines.clone();
        parser.line_ids = (1..=lines.len()).collect();
        parser.file_paths = vec!["stress.casp".to_string()];

        parser.parse_states(0);

        // All states should parse
        assert_eq!(parser.states.len(), 100);

        // Spot check a few states
        assert!(parser.states.contains_key("State0"));
        assert!(parser.states.contains_key("State50"));
        assert!(parser.states.contains_key("State99"));
    }

    #[test]
    fn test_stress_many_variables() {
        let mut parser = CastagneParser::new();

        // Create 200 variables
        let mut lines = vec![":Variables:".to_string()];
        for i in 0..200 {
            lines.push(format!("var Var{}(Int): {}", i, i * 10));
        }
        lines.push("".to_string());

        parser.current_lines = lines.clone();
        parser.line_ids = (1..=lines.len()).collect();
        parser.file_paths = vec!["stress_vars.casp".to_string()];

        parser.parse_variables(0);

        // All variables should parse
        assert_eq!(parser.variables.len(), 200);

        // Spot check values
        assert_eq!(parser.variables.get("Var0").unwrap().value, "0");
        assert_eq!(parser.variables.get("Var100").unwrap().value, "1000");
        assert_eq!(parser.variables.get("Var199").unwrap().value, "1990");
    }

    #[test]
    fn test_stress_many_actions_per_state() {
        let mut parser = CastagneParser::new();

        // Create state with 100 actions
        let mut lines = vec![
            ":StressTest:".to_string(),
            "---Init:".to_string(),
        ];

        for i in 0..100 {
            lines.push(format!("Action{}(arg{})", i, i));
        }
        lines.push("".to_string());

        parser.current_lines = lines.clone();
        parser.line_ids = (1..=lines.len()).collect();
        parser.file_paths = vec!["stress_actions.casp".to_string()];

        parser.parse_states(0);

        let state = parser.states.get("StressTest").unwrap();
        let actions = state.actions.get("Init").unwrap();

        // All actions should parse
        assert_eq!(actions.len(), 100);

        // Check first and last action
        assert_eq!(actions[0].instruction, "Action0");
        assert_eq!(actions[99].instruction, "Action99");
    }

    #[test]
    fn test_stress_deeply_nested_10_levels() {
        let mut parser = CastagneParser::new();

        // Create deeply nested function call
        let nested = "Call(L1(L2(L3(L4(L5(L6(L7(L8(L9(L10(value))))))))))";

        parser.current_lines = vec![
            ":Test:".to_string(),
            "---Init:".to_string(),
            nested.to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["deep_nest.casp".to_string()];

        parser.parse_states(0);

        let test = parser.states.get("Test").unwrap();
        let actions = test.actions.get("Init").unwrap();

        // Should handle deep nesting
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].instruction, "Call");
    }

    #[test]
    fn test_stress_many_arguments() {
        let mut parser = CastagneParser::new();

        // Create action with 50 arguments
        let args: Vec<String> = (0..50).map(|i| format!("arg{}", i)).collect();
        let action_line = format!("ManyArgs({})", args.join(", "));

        parser.current_lines = vec![
            ":Test:".to_string(),
            "---Init:".to_string(),
            action_line,
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["many_args.casp".to_string()];

        parser.parse_states(0);

        let test = parser.states.get("Test").unwrap();
        let actions = test.actions.get("Init").unwrap();

        // Should handle many arguments
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].args.len(), 50);
    }

    // =========================================================================
    // Integration Tests with Complex Scenarios
    // =========================================================================

    #[test]
    fn test_integration_fighting_game_character() {
        let mut parser = CastagneParser::new();

        parser.current_lines = vec![
            ":Character:".to_string(),
            "Name: Ryu".to_string(),
            "Author: Capcom".to_string(),
            "Description: Wandering fighter".to_string(),
            "".to_string(),
            ":Variables:".to_string(),
            "var Health(Int): 1000".to_string(),
            "var MaxHealth(Int): 1000".to_string(),
            "var Meter(Int): 0".to_string(),
            "var MaxMeter(Int): 10000".to_string(),
            "var Damage(Int): 0".to_string(),
            "var PosX(Int): 0".to_string(),
            "var PosY(Int): 0".to_string(),
            "var VelX(Int): 0".to_string(),
            "var VelY(Int): 0".to_string(),
            "var Facing(Int): 1".to_string(),
            "var ComboCounter(Int): 0".to_string(),
            "def GROUND_Y: 0".to_string(),
            "def JUMP_FORCE: -150".to_string(),
            "def WALK_SPEED: 30".to_string(),
            "".to_string(),
            ":Config:".to_string(),
            "MaxSpeed: 100".to_string(),
            "JumpHeight: 150".to_string(),
            "Gravity: 10".to_string(),
            "".to_string(),
            ":Idle:".to_string(),
            "---Init:".to_string(),
            "Set(VelX, 0)".to_string(),
            "Set(VelY, 0)".to_string(),
            "ResetCombo()".to_string(),
            "---Action:".to_string(),
            "CheckInput(Left, Walk)".to_string(),
            "CheckInput(Right, Walk)".to_string(),
            "CheckInput(Jump, Jump)".to_string(),
            "CheckInput(Attack, LightPunch)".to_string(),
            "".to_string(),
            ":Walk(Idle):".to_string(),
            "---Init:".to_string(),
            "Set(VelX, WALK_SPEED)".to_string(),
            "PlayAnimation(Walk)".to_string(),
            "---Action:".to_string(),
            "Move(VelX, 0)".to_string(),
            "CheckInput(None, Idle)".to_string(),
            "CheckInput(Jump, Jump)".to_string(),
            "".to_string(),
            ":Jump(Idle):".to_string(),
            "---Init:".to_string(),
            "Set(VelY, JUMP_FORCE)".to_string(),
            "PlayAnimation(Jump)".to_string(),
            "PlaySound(Jump)".to_string(),
            "---Action:".to_string(),
            "Add(VelY, Gravity)".to_string(),
            "Move(VelX, VelY)".to_string(),
            "If(Grounded())".to_string(),
            "ChangeState(Idle)".to_string(),
            "EndIf()".to_string(),
            "".to_string(),
            ":LightPunch(Helper):".to_string(),
            "---Init:".to_string(),
            "Set(Damage, 100)".to_string(),
            "Set(Duration, 15)".to_string(),
            "PlayAnimation(LightPunch)".to_string(),
            "---Action:".to_string(),
            "CreateHitbox(10, 10, 40, 20, Damage)".to_string(),
            "---Reaction:".to_string(),
            "If(Hit())".to_string(),
            "Add(ComboCounter, 1)".to_string(),
            "Add(Meter, 100)".to_string(),
            "EndIf()".to_string(),
            "".to_string(),
            ":Hadoken(Special):".to_string(),
            "---Init:".to_string(),
            "Sub(Meter, 1000)".to_string(),
            "CreateProjectile(Fireball)".to_string(),
            "PlayAnimation(Hadoken)".to_string(),
            "PlaySound(Hadoken)".to_string(),
            "---Action:".to_string(),
            "If(AnimationFinished())".to_string(),
            "ChangeState(Idle)".to_string(),
            "EndIf()".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["ryu.casp".to_string()];

        parser.parse_full_file();
        let result = parser.end_parsing();

        assert!(result.is_some());
        let character = result.unwrap();

        // Check metadata
        assert_eq!(character.metadata.name, "Ryu");
        assert_eq!(character.metadata.author, "Capcom");

        // Check variables (11 var + 3 def = 14 total)
        assert_eq!(character.variables.len(), 14);
        assert!(character.variables.contains_key("Health"));
        assert!(character.variables.contains_key("Meter"));
        assert!(character.variables.contains_key("WALK_SPEED"));

        // Check states
        assert_eq!(character.states.len(), 5);
        assert!(character.states.contains_key("Idle"));
        assert!(character.states.contains_key("Walk"));
        assert!(character.states.contains_key("Jump"));
        assert!(character.states.contains_key("LightPunch"));
        assert!(character.states.contains_key("Hadoken"));

        // Check state types
        assert_eq!(character.states.get("Idle").unwrap().state_type, StateType::Normal);
        assert_eq!(character.states.get("Walk").unwrap().state_type, StateType::Normal);
        assert_eq!(character.states.get("LightPunch").unwrap().state_type, StateType::Helper);
        assert_eq!(character.states.get("Hadoken").unwrap().state_type, StateType::Special);

        // Check state parents
        assert_eq!(character.states.get("Walk").unwrap().parent, Some("Idle".to_string()));

        // Check specblocks
        assert!(character.specblocks.contains_key("Config"));
        assert_eq!(character.specblocks.get("Config").unwrap().get("Gravity"), Some(&"10".to_string()));
    }

    #[test]
    fn test_integration_projectile_character() {
        let mut parser = CastagneParser::new();

        parser.current_lines = vec![
            ":Character:".to_string(),
            "Name: Fireball".to_string(),
            "Author: Game Dev".to_string(),
            "".to_string(),
            ":Variables:".to_string(),
            "var Damage(Int): 200".to_string(),
            "var Speed(Int): 50".to_string(),
            "var Lifetime(Int): 60".to_string(),
            "var CurrentFrame(Int): 0".to_string(),
            "".to_string(),
            ":Active(Helper):".to_string(),
            "---Init:".to_string(),
            "PlayAnimation(Fireball)".to_string(),
            "CreateHitbox(0, 0, 20, 20, Damage)".to_string(),
            "---Action:".to_string(),
            "MoveForward(Speed)".to_string(),
            "Add(CurrentFrame, 1)".to_string(),
            "If(CurrentFrame, Lifetime)".to_string(),
            "Destroy()".to_string(),
            "EndIf()".to_string(),
            "---Reaction:".to_string(),
            "If(Hit())".to_string(),
            "Destroy()".to_string(),
            "EndIf()".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["fireball.casp".to_string()];

        parser.parse_full_file();
        let result = parser.end_parsing();

        assert!(result.is_some());
        let character = result.unwrap();

        // Check it parsed as a projectile/helper
        assert_eq!(character.metadata.name, "Fireball");
        assert_eq!(character.states.len(), 1);
        assert!(character.states.contains_key("Active"));
        assert_eq!(character.states.get("Active").unwrap().state_type, StateType::Helper);
    }

    #[test]
    fn test_integration_multifile_inheritance() {
        // This test simulates skeleton inheritance
        // In a real scenario, this would load from separate files

        let mut parser = CastagneParser::new();

        // Simulate parent character data
        let parent_metadata = CharacterMetadata {
            name: "BaseCharacter".to_string(),
            author: "Framework".to_string(),
            description: "Base template".to_string(),
            skeleton: None,
            other_fields: HashMap::new(),
        };

        parser.metadata = parent_metadata;

        // Add parent variables
        parser.variables.insert("BaseHealth".to_string(), ParsedVariable {
            name: "BaseHealth".to_string(),
            mutability: VariableMutability::Variable,
            var_type: VariableType::Int,
            subtype: String::new(),
            value: "1000".to_string(),
        });

        // Now parse child that overrides/extends
        parser.current_lines = vec![
            ":Character:".to_string(),
            "Name: DerivedCharacter".to_string(),
            "".to_string(),
            ":Variables:".to_string(),
            "var ChildHealth(Int): 1500".to_string(),
            "".to_string(),
            ":Idle:".to_string(),
            "---Init:".to_string(),
            "Set(x, 1)".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["child.casp".to_string()];

        parser.parse_full_file();
        let result = parser.end_parsing();

        assert!(result.is_some());
        let character = result.unwrap();

        // Should have child metadata
        assert_eq!(character.metadata.name, "DerivedCharacter");

        // Should have both parent and child variables
        assert!(character.variables.contains_key("BaseHealth"));
        assert!(character.variables.contains_key("ChildHealth"));
    }

    #[test]
    fn test_integration_complex_conditionals() {
        let mut parser = CastagneParser::new();

        parser.current_lines = vec![
            ":Test:".to_string(),
            "---Init:".to_string(),
            "If(Health, MaxHealth)".to_string(),
            "Set(Color, Green)".to_string(),
            "Else()".to_string(),
            "If(Health, HalfHealth)".to_string(),
            "Set(Color, Yellow)".to_string(),
            "Else()".to_string(),
            "Set(Color, Red)".to_string(),
            "EndIf()".to_string(),
            "EndIf()".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["conditionals.casp".to_string()];

        parser.parse_states(0);

        let test = parser.states.get("Test").unwrap();
        let actions = test.actions.get("Init").unwrap();

        // Should parse all conditional statements
        assert!(actions.len() >= 8);

        // Check structure
        assert_eq!(actions[0].instruction, "If");
        assert!(actions.iter().any(|a| a.instruction == "Else"));
        assert!(actions.iter().any(|a| a.instruction == "EndIf"));
    }

    #[test]
    fn test_integration_all_features_combined() {
        let mut parser = CastagneParser::new();

        parser.current_lines = vec![
            "# Complete character with all features".to_string(),
            ":Character:".to_string(),
            "Name: Complete Fighter # Full featured".to_string(),
            "Author: Test".to_string(),
            "Description: Uses all parser features".to_string(),
            "".to_string(),
            ":Config:".to_string(),
            "Speed: 100 # Movement speed".to_string(),
            "".to_string(),
            ":Variables:".to_string(),
            "var IntVal(Int): 42".to_string(),
            "var StrVal(Str): Hello".to_string(),
            "var BoolVal(Bool): true".to_string(),
            "var Vec2Val(Vec2): 10, 20".to_string(),
            "var Vec3Val(Vec3): 1, 2, 3".to_string(),
            "def CONSTANT: 100".to_string(),
            "".to_string(),
            ":Idle:".to_string(),
            "---Init: # Setup phase".to_string(),
            "Set(State, 0)".to_string(),
            "Log(\"Starting Idle\")".to_string(),
            "---Action:".to_string(),
            "CheckInput(Attack, Attack)".to_string(),
            "---Reaction:".to_string(),
            "HandleHit()".to_string(),
            "".to_string(),
            ":Attack(Helper, Idle):".to_string(),
            "---Init:".to_string(),
            "CreateHitbox(0, 0, 50, 50, 100)".to_string(),
            "PlayAnimation(Attack)".to_string(),
            "---Action:".to_string(),
            "If(AnimationFinished())".to_string(),
            "ChangeState(Idle)".to_string(),
            "EndIf()".to_string(),
            "".to_string(),
        ];
        parser.line_ids = (1..=parser.current_lines.len()).collect();
        parser.file_paths = vec!["complete.casp".to_string()];

        parser.parse_full_file();
        let result = parser.end_parsing();

        assert!(result.is_some());
        let character = result.unwrap();

        // Metadata
        assert_eq!(character.metadata.name, "Complete Fighter");

        // Variables - all types
        assert_eq!(character.variables.len(), 6);
        assert_eq!(character.variables.get("IntVal").unwrap().var_type, VariableType::Int);
        assert_eq!(character.variables.get("Vec2Val").unwrap().var_type, VariableType::Vec2);

        // Specblocks
        assert!(character.specblocks.contains_key("Config"));

        // States with all features
        assert_eq!(character.states.len(), 2);
        let attack = character.states.get("Attack").unwrap();
        assert_eq!(attack.state_type, StateType::Helper);
        assert_eq!(attack.parent, Some("Idle".to_string()));
        assert!(attack.actions.contains_key("Init"));
        assert!(attack.actions.contains_key("Action"));
    }

    // =========================================================================
    // Basic Template Tests - Verify Rust parser can parse test files
    // =========================================================================

    #[test]
    fn test_basic_template_complete_character_file() {
        // This test parses the real test_character_complete.casp file
        // and verifies expected structure
        let mut parser = CastagneParser::new();
        parser.open_file("test_character_complete.casp");

        if parser.aborting || parser.invalid_file {
            panic!("Failed to load test_character_complete.casp");
        }

        parser.parse_full_file();
        let result = parser.end_parsing();

        assert!(result.is_some(), "Parser should successfully parse test_character_complete.casp");
        let character = result.unwrap();

        // Verify metadata
        assert_eq!(character.metadata.name, "Complete Test Fighter");
        assert_eq!(character.metadata.author, "Parser Development Team");
        assert!(character.metadata.description.contains("comprehensive"));

        // Verify specblocks (2 specblocks: AttackData, PhysicsConfig)
        assert_eq!(character.specblocks.len(), 2);
        assert!(character.specblocks.contains_key("AttackData"));
        assert!(character.specblocks.contains_key("PhysicsConfig"));

        let attack_data = character.specblocks.get("AttackData").unwrap();
        assert_eq!(attack_data.get("LightPunchDamage"), Some(&"5".to_string()));
        assert_eq!(attack_data.get("HeavyPunchDamage"), Some(&"15".to_string()));

        let physics = character.specblocks.get("PhysicsConfig").unwrap();
        assert_eq!(physics.get("Gravity"), Some(&"10".to_string()));
        assert_eq!(physics.get("JumpForce"), Some(&"25".to_string()));

        // Verify variables (12 vars + 2 defs = 14 total)
        assert_eq!(character.variables.len(), 14);

        // Check integer variables
        assert!(character.variables.contains_key("Health"));
        assert_eq!(character.variables.get("Health").unwrap().var_type, VariableType::Int);
        assert_eq!(character.variables.get("Health").unwrap().value, "150");

        // Check string variables
        assert!(character.variables.contains_key("PlayerName"));
        assert_eq!(character.variables.get("PlayerName").unwrap().var_type, VariableType::Str);

        // Check boolean variables
        assert!(character.variables.contains_key("IsGrounded"));
        assert_eq!(character.variables.get("IsGrounded").unwrap().var_type, VariableType::Bool);

        // Check Vec2 variables
        assert!(character.variables.contains_key("Position"));
        assert_eq!(character.variables.get("Position").unwrap().var_type, VariableType::Vec2);

        // Check Var type
        assert!(character.variables.contains_key("Meter"));
        assert_eq!(character.variables.get("Meter").unwrap().var_type, VariableType::Var);

        // Check constants
        assert!(character.variables.contains_key("MAX_COMBO"));
        assert_eq!(character.variables.get("MAX_COMBO").unwrap().mutability, VariableMutability::Define);

        // Verify states (5 states: Idle, Walk, Jump, LightPunch, HeavyPunch)
        assert_eq!(character.states.len(), 5);
        assert!(character.states.contains_key("Idle"));
        assert!(character.states.contains_key("Walk"));
        assert!(character.states.contains_key("Jump"));
        assert!(character.states.contains_key("LightPunch"));
        assert!(character.states.contains_key("HeavyPunch"));

        // Verify Idle state has Init and Action phases
        let idle = character.states.get("Idle").unwrap();
        assert!(idle.actions.contains_key("Init"));
        assert!(idle.actions.contains_key("Action"));
        assert_eq!(idle.state_type, StateType::Normal);

        // Verify actions in Idle Init phase
        let idle_init = idle.actions.get("Init").unwrap();
        assert_eq!(idle_init.len(), 3); // Set AnimationState, Set Velocity, Set IsAttacking
        assert_eq!(idle_init[0].instruction, "Set");
        assert_eq!(idle_init[0].args[0], "AnimationState");

        // Verify LightPunch has Init, Action, and Reaction phases
        let light_punch = character.states.get("LightPunch").unwrap();
        assert!(light_punch.actions.contains_key("Init"));
        assert!(light_punch.actions.contains_key("Action"));
        assert!(light_punch.actions.contains_key("Reaction"));
    }

    #[test]
    fn test_basic_template_basic_character_file() {
        // Test the simplest test_character.casp file
        let mut parser = CastagneParser::new();
        parser.open_file("test_character.casp");

        if parser.aborting || parser.invalid_file {
            panic!("Failed to load test_character.casp");
        }

        parser.parse_full_file();
        let result = parser.end_parsing();

        assert!(result.is_some(), "Parser should successfully parse test_character.casp");
        let character = result.unwrap();

        // Basic character should have metadata
        assert!(!character.metadata.name.is_empty());

        // Should have at least one state or variable
        assert!(character.states.len() > 0 || character.variables.len() > 0);
    }

    #[test]
    fn test_basic_template_advanced_character_file() {
        // Test test_character_advanced.casp
        let mut parser = CastagneParser::new();
        parser.open_file("test_character_advanced.casp");

        if parser.aborting || parser.invalid_file {
            panic!("Failed to load test_character_advanced.casp");
        }

        parser.parse_full_file();
        let result = parser.end_parsing();

        assert!(result.is_some(), "Parser should successfully parse test_character_advanced.casp");
        let character = result.unwrap();

        // Advanced character should have metadata
        assert!(!character.metadata.name.is_empty());

        // Should have multiple states demonstrating advanced features
        assert!(character.states.len() > 0);
    }

    #[test]
    fn test_basic_template_parent_child_skeleton_inheritance() {
        // Test skeleton inheritance with test_parent.casp and test_child.casp

        // First parse parent
        let mut parent_parser = CastagneParser::new();
        parent_parser.open_file("test_parent.casp");

        if parent_parser.aborting || parent_parser.invalid_file {
            // If files don't exist, skip this test gracefully
            return;
        }

        parent_parser.parse_full_file();
        let parent_result = parent_parser.end_parsing();
        assert!(parent_result.is_some(), "Parent should parse successfully");
        let parent = parent_result.unwrap();

        // Then parse child
        let mut child_parser = CastagneParser::new();
        child_parser.open_file("test_child.casp");

        if child_parser.aborting || child_parser.invalid_file {
            // If files don't exist, skip this test gracefully
            return;
        }

        child_parser.parse_full_file();
        let child_result = child_parser.end_parsing();
        assert!(child_result.is_some(), "Child should parse successfully");
        let child = child_result.unwrap();

        // Verify both parsed
        assert!(!parent.metadata.name.is_empty());
        assert!(!child.metadata.name.is_empty());

        // Child might reference parent in skeleton field
        if let Some(skeleton) = &child.metadata.skeleton {
            assert!(skeleton.len() > 0, "Skeleton reference should not be empty");
        }
    }

    #[test]
    fn test_basic_template_all_variable_type_conversions() {
        // Verify type conversion works for all types
        let mut parser = CastagneParser::new();

        parser.variables.insert("IntVar".to_string(), ParsedVariable {
            name: "IntVar".to_string(),
            mutability: VariableMutability::Variable,
            var_type: VariableType::Int,
            subtype: String::new(),
            value: "42".to_string(),
        });

        parser.variables.insert("BoolVar".to_string(), ParsedVariable {
            name: "BoolVar".to_string(),
            mutability: VariableMutability::Variable,
            var_type: VariableType::Bool,
            subtype: String::new(),
            value: "true".to_string(),
        });

        parser.variables.insert("StrVar".to_string(), ParsedVariable {
            name: "StrVar".to_string(),
            mutability: VariableMutability::Variable,
            var_type: VariableType::Str,
            subtype: String::new(),
            value: "Hello".to_string(),
        });

        parser.variables.insert("Vec2Var".to_string(), ParsedVariable {
            name: "Vec2Var".to_string(),
            mutability: VariableMutability::Variable,
            var_type: VariableType::Vec2,
            subtype: String::new(),
            value: "10, 20".to_string(),
        });

        // Verify types are correct (matching GDScript parser behavior)
        // Note: We can't test actual to_variant() calls since they require Godot runtime
        assert_eq!(parser.variables.get("IntVar").unwrap().var_type, VariableType::Int);
        assert_eq!(parser.variables.get("BoolVar").unwrap().var_type, VariableType::Bool);
        assert_eq!(parser.variables.get("StrVar").unwrap().var_type, VariableType::Str);
        assert_eq!(parser.variables.get("Vec2Var").unwrap().var_type, VariableType::Vec2);

        // Verify values can be accessed (as parser would return them)
        assert_eq!(parser.variables.get("IntVar").unwrap().as_int(), Some(42));
        assert_eq!(parser.variables.get("BoolVar").unwrap().as_bool(), Some(true));
        assert_eq!(parser.variables.get("StrVar").unwrap().value, "Hello");
    }

    #[test]
    fn test_basic_template_comment_stripping() {
        // Verify inline comment handling
        let parser = CastagneParser::new();

        // Test cases that should match GDScript behavior
        let test_cases = vec![
            // (input, expected_output)
            ("Set(x, 5) # comment", "Set(x, 5) "),
            ("var Health(Int): 100 # player health", "var Health(Int): 100 "),
            (r#"Log("Text # not comment") # real comment"#, r#"Log("Text # not comment") "#),
            ("# just comment", ""),
            ("NoComment", "NoComment"),
        ];

        for (input, expected) in test_cases {
            let result = parser.strip_inline_comment(input);
            assert_eq!(result, expected,
                "Comment stripping for '{}' should match GDScript behavior", input);
        }
    }

    #[test]
    fn test_basic_template_argument_parsing() {
        // Verify argument parsing works correctly
        let parser = CastagneParser::new();

        // Test cases that should match GDScript behavior
        let test_cases = vec![
            // (input, expected_count, description)
            ("a, b, c", 3, "simple args"),
            ("a", 1, "single arg"),
            ("", 0, "empty args"),
            (r#""string, with, commas", other"#, 2, "string with commas"),
            ("Nested(a, b), c", 2, "nested function call"),
            ("a,b,c", 3, "no spaces"),
            ("a  ,  b  ,  c", 3, "extra spaces"),
        ];

        for (input, expected_count, desc) in test_cases {
            let result = parser.parse_arguments(input);
            assert_eq!(result.len(), expected_count,
                "Argument parsing for '{}' ({}) should match GDScript behavior", input, desc);
        }
    }

    #[test]
    fn test_basic_template_state_header_parsing() {
        // Verify state header parsing works correctly
        let parser = CastagneParser::new();

        // Test various state header formats (without colons - parse_state_header expects preprocessed input)
        let test_cases = vec![
            // (input, (expected_name, expected_type, expected_parent))
            ("Idle", ("Idle", StateType::Normal, None)),
            ("Walk(Idle)", ("Walk", StateType::Normal, Some("Idle"))),
            ("Attack(Helper)", ("Attack", StateType::Helper, None)),
            ("Special(Special, Base)", ("Special", StateType::Special, Some("Base"))),
            ("Projectile(Helper, Base)", ("Projectile", StateType::Helper, Some("Base"))),
        ];

        for (input, expected_data) in test_cases {
            let (exp_name, exp_type, exp_parent) = expected_data;
            let (name, state_type, parent) = parser.parse_state_header(input);
            assert_eq!(name, exp_name, "State name should match for '{}'", input);
            assert_eq!(state_type, exp_type, "State type should match for '{}'", input);
            assert_eq!(parent, exp_parent.map(|s| s.to_string()),
                "Parent should match for '{}'", input);
        }
    }
}
