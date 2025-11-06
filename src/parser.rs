// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! CastagneParser - Parses .casp character files
//!
//! This is a minimal v0 implementation of the Castagne parser.
//! The original GDScript version is ~2279 lines of complex parsing logic.
//! This version provides the basic structure with TODOs for full implementation.

use godot::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Phases that can have events
const PHASES_BASE: &[&str] = &[
    "Init", "Action", "Reaction", "Freeze", "Manual", "AI", "Subentity", "Halt",
];

/// Variable mutability types
#[derive(Debug, Clone, PartialEq)]
pub enum VariableMutability {
    Variable,
    Define,
    Internal,
}

/// Variable types
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub enum StateType {
    Normal,
    BaseState,
    Helper,
    Special,
    Specblock,
}

/// Parsed variable definition
#[derive(Debug, Clone)]
pub struct ParsedVariable {
    pub name: String,
    pub mutability: VariableMutability,
    pub var_type: VariableType,
    pub subtype: String,
    pub value: String, // TODO: Should be Variant or enum for typed values
}

/// Parsed state information
#[derive(Debug, Clone)]
pub struct ParsedState {
    pub name: String,
    pub state_type: StateType,
    pub parent: Option<String>,
    pub actions: HashMap<String, Vec<ParsedAction>>, // Phase -> Actions
}

/// A parsed action/instruction
#[derive(Debug, Clone)]
pub struct ParsedAction {
    pub instruction: String,
    pub args: Vec<String>,
    pub line_number: usize,
}

/// Character metadata
#[derive(Debug, Clone)]
pub struct CharacterMetadata {
    pub name: String,
    pub author: String,
    pub description: String,
    pub skeleton: Option<String>,
    pub other_fields: HashMap<String, String>,
}

/// Full parsed character data
#[derive(Debug, Clone)]
pub struct ParsedCharacter {
    pub metadata: CharacterMetadata,
    pub variables: HashMap<String, ParsedVariable>,
    pub states: HashMap<String, ParsedState>,
    pub subentities: HashMap<String, CharacterMetadata>,
    pub transformed_data: HashMap<String, HashMap<String, String>>,
}

/// CastagneParser - Main parser struct
///
/// Parses .casp files to create Castagne characters.
/// This is a minimal implementation - the original is much more complex!
pub struct CastagneParser {
    logs_active: bool,
    errors: Vec<String>,

    // Parsing state
    current_lines: Vec<String>,
    line_ids: Vec<usize>,
    file_paths: Vec<String>,
    current_file: usize,

    // Parsed data
    metadata: CharacterMetadata,
    variables: HashMap<String, ParsedVariable>,
    states: HashMap<String, ParsedState>,
    specblock_defines: HashMap<String, ParsedVariable>,

    // Flags
    aborting: bool,
    invalid_file: bool,
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
            specblock_defines: HashMap::new(),
            aborting: false,
            invalid_file: false,
        }
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
        self.specblock_defines.clear();
        self.aborting = false;
        self.invalid_file = false;

        self.open_file(file_path);
    }

    fn end_parsing(&mut self) -> Option<ParsedCharacter> {
        if self.aborting || self.invalid_file {
            return None;
        }

        Some(ParsedCharacter {
            metadata: self.metadata.clone(),
            variables: self.variables.clone(),
            states: self.states.clone(),
            subentities: HashMap::new(), // TODO: Implement subentity parsing
            transformed_data: HashMap::new(), // TODO: Implement data transformation
        })
    }

    fn open_file(&mut self, file_path: &str) {
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

    fn parse_full_file(&mut self) {
        if self.aborting {
            return;
        }

        self.log(">>> Starting to parse the full file.");

        // Step 1: Parse metadata
        self.parse_metadata(0);

        // TODO: If metadata has skeleton, open that file too and recurse

        // Step 2: Parse specblocks
        self.parse_specblocks(0);

        // Step 3: Parse variables
        self.parse_variables(0);

        // Step 4: Parse states
        self.parse_states(0);

        // TODO: Step 5: Optimize
        self.log(">>> Parsing complete!");
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
                // Parse metadata fields
                if let Some(colon_pos) = line.find(':') {
                    let key = line[..colon_pos].trim();
                    let value = line[colon_pos + 1..].trim().to_string();

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

            i += 1;
        }

        self.log(&format!("Parsed metadata: Name={}", self.metadata.name));
        &self.metadata
    }

    fn parse_specblocks(&mut self, _file_id: usize) -> HashMap<String, String> {
        // TODO: Parse :SpecblockName: blocks
        // These define constants/values used in the character

        HashMap::new()
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
                self.parse_variable_line(&line);
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
                let state_name = line[1..line.len() - 1].to_string();

                // Skip special blocks we've already handled
                if state_name != "Character" && state_name != "Variables" {
                    self.parse_state(state_name, &mut i);
                }
            }

            i += 1;
        }

        self.log(&format!("Parsed {} states", self.states.len()));
    }

    fn parse_state(&mut self, state_name: String, i: &mut usize) {
        self.log(&format!("Parsing state: {}", state_name));

        let mut state = ParsedState {
            name: state_name.clone(),
            state_type: StateType::Normal,
            parent: None,
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
            // Parse action line
            else if !line.is_empty() && !line.starts_with('#') {
                if let Some(ref phase) = current_phase {
                    if let Some(action) = self.parse_action_line(line, *i) {
                        state.actions
                            .entry(phase.clone())
                            .or_insert_with(Vec::new)
                            .push(action);
                    }
                }
            }

            *i += 1;
        }

        self.states.insert(state_name, state);
        *i -= 1; // Back up one so the outer loop doesn't skip a line
    }

    fn parse_action_line(&self, line: &str, line_number: usize) -> Option<ParsedAction> {
        // Parse function call: FunctionName(Arg1, Arg2, ...)
        // or simple instruction: FunctionName

        if let Some(open_paren) = line.find('(') {
            if let Some(close_paren) = line.rfind(')') {
                let instruction = line[..open_paren].trim().to_string();
                let args_str = &line[open_paren + 1..close_paren];

                // Parse arguments (simple split by comma for now)
                let args: Vec<String> = if args_str.trim().is_empty() {
                    Vec::new()
                } else {
                    args_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                };

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

    // -------------------------------------------------------------------------
    // Instruction execution (for runtime)

    /// Standard parse function (used by modules to register functions)
    pub fn standard_parse_function(&self, function_name: &str, args: &[String]) -> Vec<String> {
        // TODO: Parse function arguments according to function signature
        // For now, just return the args as-is
        args.to_vec()
    }

    /// Execute an instruction of type I (Integer operations)
    pub fn instruction_i(&self, _args: &[String], _state_handle: &crate::state_handle::CastagneStateHandle) {
        // TODO: Implement integer instruction execution
        // Examples: Set, Add, Mul, etc.
    }

    /// Execute an instruction of type F (Flag operations)
    pub fn instruction_f(&self, _args: &[String], _state_handle: &crate::state_handle::CastagneStateHandle) {
        // TODO: Implement flag instruction execution
        // Examples: Flag, Unflag, IfFlag, etc.
    }

    /// Execute an instruction of type S (String operations)
    pub fn instruction_s(&self, _args: &[String], _state_handle: &crate::state_handle::CastagneStateHandle) {
        // TODO: Implement string instruction execution
    }

    /// Execute an instruction of type L (caLl / function call)
    pub fn instruction_l(&self, _args: &[String], _state_handle: &crate::state_handle::CastagneStateHandle) {
        // TODO: Implement function call instruction
    }

    /// Execute an instruction of type V (adVanced operations)
    pub fn instruction_v(&self, _args: &[String], _state_handle: &crate::state_handle::CastagneStateHandle) {
        // TODO: Implement advanced instruction execution
    }

    /// Execute an instruction of type P (Parser operations)
    pub fn instruction_p(&self, _args: &[String], _state_handle: &crate::state_handle::CastagneStateHandle) {
        // TODO: Implement parser instruction execution
    }

    /// Execute an instruction of type R (bRanch / conditional)
    pub fn instruction_r(&self, _args: &[String], _state_handle: &crate::state_handle::CastagneStateHandle) {
        // TODO: Implement branch instruction execution
        // Examples: If, Else, EndIf
    }

    /// Execute a branch instruction with a condition
    pub fn instruction_branch(&self, _args: &[String], _state_handle: &crate::state_handle::CastagneStateHandle, _condition: bool) {
        // TODO: Implement conditional branching
    }

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
}
