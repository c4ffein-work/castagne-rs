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

        self.line_ids.push(0);
        self.file_paths.push(file_path.to_string());
        self.current_lines.push(String::new());

        // TODO: Actually read the file
        // For now, we'll mark it as invalid if we can't open it
        match File::open(file_path) {
            Ok(_) => {
                // File exists, good to go
            }
            Err(_) => {
                self.fatal_error(&format!("File {} does not exist.", file_path));
            }
        }
    }

    fn parse_full_file(&mut self) {
        if self.aborting {
            return;
        }

        // TODO: Full implementation
        // 1. Parse metadata and follow skeleton chain
        // 2. Parse specblocks in reverse order
        // 3. Parse variables in reverse order
        // 4. Parse states in reverse order
        // 5. Optimize the code

        self.log(">>> Starting to parse the full file.");

        // Step 1: Parse metadata
        self.parse_metadata(0);

        // TODO: If metadata has skeleton, open that file too and recurse

        // TODO: Step 2: Parse specblocks

        // TODO: Step 3: Parse variables

        // TODO: Step 4: Parse states

        // TODO: Step 5: Optimize
    }

    fn parse_metadata(&mut self, _file_id: usize) -> &CharacterMetadata {
        // TODO: Read file and parse :Character: block
        // This should parse lines like:
        //   Name: Character Name
        //   Author: Author Name
        //   Description: Character description
        //   Skeleton: path/to/parent.casp

        self.log("Parsing metadata...");

        // For now, return empty metadata
        // TODO: Implement actual parsing
        &self.metadata
    }

    fn parse_specblocks(&mut self, _file_id: usize) -> HashMap<String, String> {
        // TODO: Parse :SpecblockName: blocks
        // These define constants/values used in the character

        HashMap::new()
    }

    fn parse_variables(&mut self, _file_id: usize) {
        // TODO: Parse :Variables: block
        // Format:
        //   var VariableName(Type): DefaultValue
        //   def ConstantName: Value

        self.log("Parsing variables...");
    }

    fn parse_states(&mut self, _file_id: usize) {
        // TODO: Parse state blocks
        // Format:
        //   :StateName:
        //   ---Init:
        //   FunctionCall(Args)
        //   ---Action:
        //   AnotherFunction(Args)

        self.log("Parsing states...");
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
