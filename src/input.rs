// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! CastagneInput - Input management system
//!
//! Manages input devices (keyboard, controller, AI) and input mapping.
//! Handles device polling, input buffering, and input schema creation.

use godot::prelude::*;
use std::collections::HashMap;

/// Stick direction types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StickDirection {
    Left,
    Right,
    Down,
    Up,
    Back,
    Forward,
    Portside,
    Starboard,
    NeutralH,
    NeutralV,
}

/// Input device types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDeviceType {
    Empty,
    Keyboard,
    Controller,
    AI,
}

/// Physical input types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalInputType {
    Raw,
    Button,
    Axis,
    Stick,
    Combination,
    Any,
}

/// Game input types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameInputType {
    Direct,
    Multiple,
    Derived,
}

/// Derived game input types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameInputDerivedType {
    ButtonPress,
    ButtonRelease,
    Directional,
    DirectionNeutral,
}

/// SOCD (Simultaneous Opposite Cardinal Direction) handling type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocdType {
    Neutral,   // Both inputs cancel out
    Negative,  // Negative direction wins
    Positive,  // Positive direction wins
}

/// Physical input definition
#[derive(Debug, Clone)]
pub struct PhysicalInput {
    pub name: String,
    pub input_type: PhysicalInputType,
    pub game_input_names: Option<Vec<String>>,
    pub keyboard_inputs: Vec<Vec<Vec<i32>>>, // [gameInputID][layoutID][bindings]
    pub controller_inputs: Vec<Vec<Vec<i32>>>, // [gameInputID][layoutID][bindings]
    pub combination: Option<Vec<(usize, usize)>>, // For combination inputs: (physical_input_id, game_input_id)
}

/// Device data
#[derive(Debug, Clone)]
pub struct DeviceData {
    pub name: String,
    pub device_type: InputDeviceType,
    pub display_name: String,
    pub device_action_prefix: String,
    pub bindings_base: usize,
    pub controller_id: Option<usize>,
    pub input_layout: Vec<PhysicalInput>,
    pub input_layout_menu: Vec<PhysicalInput>,
    pub input_map: HashMap<String, InputMapEntry>,
    pub input_map_menu: HashMap<String, InputMapEntry>,
}

/// Input map entry
#[derive(Debug, Clone)]
pub struct InputMapEntry {
    pub game_input_name: String,
    pub action_name: String,
    pub bindings_keyboard: Vec<Vec<i32>>,
    pub bindings_controller: Vec<Vec<i32>>,
}

/// Input schema entry
#[derive(Debug, Clone)]
pub struct InputSchemaEntry {
    pub name: String,
    pub input_type: GameInputType,
    pub combination: Option<Vec<String>>,
    pub combination_any: Option<bool>,
    pub derived_type: Option<GameInputDerivedType>,
    pub target: Option<String>,
    pub targets: Option<Vec<String>>,
    pub dir_id: Option<usize>,
    pub game_input_names: Option<Vec<String>>,
}

/// SOCD entry
type SocdEntry = (String, String, SocdType); // (negative_input, positive_input, socd_type)

/// Input schema
#[derive(Debug, Clone)]
pub struct InputSchema {
    pub inputs: HashMap<String, InputSchemaEntry>,
    pub input_list: Vec<String>,
    pub input_list_by_type: HashMap<GameInputType, Vec<String>>,
    pub socds: Vec<SocdEntry>,
}

/// CastagneInput - Main input management struct
pub struct CastagneInput {
    devices: HashMap<String, DeviceData>,
    devices_list: Vec<String>,
    input_layout: Vec<PhysicalInput>,
    input_layout_menu: Vec<PhysicalInput>,
    input_schema: InputSchema,

    // Config values
    number_of_keyboard_players: usize,
    number_of_controller_players: usize,
    number_of_keyboard_layouts: usize,
    number_of_controller_layouts: usize,
}

impl CastagneInput {
    /// Create a new input manager
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            devices_list: Vec::new(),
            input_layout: Vec::new(),
            input_layout_menu: Vec::new(),
            input_schema: InputSchema {
                inputs: HashMap::new(),
                input_list: Vec::new(),
                input_list_by_type: HashMap::new(),
                socds: Vec::new(),
            },
            number_of_keyboard_players: 2,
            number_of_controller_players: 4,
            number_of_keyboard_layouts: 1,
            number_of_controller_layouts: 1,
        }
    }

    /// Initialize from config data
    pub fn initialize_from_config_data(&mut self, input_layout: Vec<PhysicalInput>, input_layout_menu: Vec<PhysicalInput>) {
        self.input_layout = input_layout.clone();
        self.input_layout_menu = input_layout_menu.clone();
        self.input_schema = self.create_input_schema_from_input_layout(&input_layout);

        // Add default devices
        self.add_device("empty", InputDeviceType::Empty, None);
        self.add_device("ai", InputDeviceType::AI, None);

        // Add keyboard players
        for i in 0..self.number_of_keyboard_players {
            self.add_device(&format!("k{}", i + 1), InputDeviceType::Keyboard, Some(i));
        }

        // Add controller players
        for i in 0..self.number_of_controller_players {
            self.add_device(&format!("c{}", i + 1), InputDeviceType::Controller, Some(i));
        }
    }

    /// Check if device type is null (Empty or AI)
    pub fn is_null_device_type(&self, device_type: InputDeviceType) -> bool {
        device_type == InputDeviceType::Empty || device_type == InputDeviceType::AI
    }

    /// Add a new input device
    pub fn add_device(&mut self, device_name: &str, device_type: InputDeviceType, device_parameter: Option<usize>) {
        if self.devices.contains_key(device_name) {
            godot_error!("[CastagneInput.AddDevice] Device already exists: {}", device_name);
            return;
        }

        let display_name = match device_type {
            InputDeviceType::Empty => "No Input Device".to_string(),
            InputDeviceType::AI => "AI Controlled".to_string(),
            InputDeviceType::Keyboard => format!("Keyboard {}", device_parameter.unwrap_or(0) + 1),
            InputDeviceType::Controller => format!("Controller {}", device_parameter.unwrap_or(0) + 1),
        };

        let mut device_data = DeviceData {
            name: device_name.to_string(),
            device_type,
            display_name,
            device_action_prefix: format!("castagne-{}-", device_name),
            bindings_base: 0,
            controller_id: None,
            input_layout: self.input_layout.clone(),
            input_layout_menu: self.input_layout_menu.clone(),
            input_map: HashMap::new(),
            input_map_menu: HashMap::new(),
        };

        if device_type == InputDeviceType::Keyboard {
            if let Some(param) = device_parameter {
                device_data.bindings_base = param;
            }
        } else if device_type == InputDeviceType::Controller {
            device_data.controller_id = device_parameter;
        }

        device_data.input_map = self.create_input_map_from_input_layout(&self.input_layout);
        device_data.input_map_menu = self.create_input_map_from_input_layout(&self.input_layout_menu);

        self.devices_list.push(device_name.to_string());
        self.devices.insert(device_name.to_string(), device_data);

        // TODO: Create Godot input actions from device
        // This would call InputMap.add_action() and InputMap.action_add_event()
        // but we can't do that easily from Rust without proper Godot bindings
    }

    /// Get list of all devices
    pub fn get_devices_list(&self) -> Vec<String> {
        self.devices_list.clone()
    }

    /// Get list of connected devices (excluding empty and ai)
    pub fn get_connected_devices(&self) -> Vec<String> {
        self.devices_list
            .iter()
            .filter(|d| *d != "empty" && *d != "ai")
            .cloned()
            .collect()
    }

    /// Get device by name
    pub fn get_device(&self, device_name: Option<&str>) -> Option<&DeviceData> {
        let name = device_name.unwrap_or("empty");

        if !self.devices.contains_key(name) {
            godot_error!("[CastagneInput.GetDevice] Device {} not found!", name);
            return None;
        }

        self.devices.get(name)
    }

    /// Poll a device for raw input
    pub fn poll_device(&self, device_name: Option<&str>) -> Option<HashMap<String, bool>> {
        let device = self.get_device(device_name)?;
        let mut input_raw_data = HashMap::new();

        for (input_name, input_map) in &device.input_map {
            let value = if !self.is_null_device_type(device.device_type) {
                // TODO: Actually poll Godot Input.is_action_pressed()
                // For now, return false
                false
            } else {
                false
            };
            input_raw_data.insert(input_name.clone(), value);
        }

        Some(input_raw_data)
    }

    /// Poll a device for menu input (just pressed)
    pub fn poll_device_menu(&self, device_name: Option<&str>) -> Option<HashMap<String, bool>> {
        let device = self.get_device(device_name)?;
        let mut input_data = HashMap::new();

        for (input_name, _input_map) in &device.input_map_menu {
            // Strip "Menu" prefix if present
            let output_name = if input_name.len() >= 4 {
                input_name[4..].to_string()
            } else {
                input_name.clone()
            };

            let value = if !self.is_null_device_type(device.device_type) {
                // TODO: Actually poll Godot Input.is_action_just_pressed()
                false
            } else {
                false
            };
            input_data.insert(output_name, value);
        }

        Some(input_data)
    }

    /// Get the input schema
    pub fn get_input_schema(&self) -> &InputSchema {
        &self.input_schema
    }

    /// Create input data from raw input
    pub fn create_input_data_from_raw_input(&self, raw_input: &HashMap<String, bool>) -> HashMap<String, bool> {
        let schema = &self.input_schema;
        let mut input_data = HashMap::new();

        // Initialize all inputs to false
        for input_name in &schema.input_list {
            input_data.insert(input_name.clone(), false);
        }

        // Start with direct inputs
        if let Some(direct_inputs) = schema.input_list_by_type.get(&GameInputType::Direct) {
            for input_name in direct_inputs {
                if let Some(&value) = raw_input.get(input_name.as_str()) {
                    input_data.insert(input_name.clone(), value);
                }
            }
        }

        // Press associated buttons for pressed combination buttons
        if let Some(combination_inputs) = schema.input_list_by_type.get(&GameInputType::Multiple) {
            for input_name in combination_inputs {
                if let Some(entry) = schema.inputs.get(input_name) {
                    if let (Some(combination), Some(&raw_value)) = (&entry.combination, raw_input.get(input_name)) {
                        if raw_value {
                            let combination_any = entry.combination_any.unwrap_or(false);
                            if combination_any {
                                let mut has_one = false;
                                for c in combination {
                                    if *input_data.get(c).unwrap_or(&false) {
                                        has_one = true;
                                    }
                                }
                                if !has_one && !combination.is_empty() {
                                    input_data.insert(combination[0].clone(), true);
                                }
                            } else {
                                for c in combination {
                                    input_data.insert(c.clone(), true);
                                }
                            }
                            input_data.insert(input_name.clone(), true);
                        }
                    }
                }
            }
        }

        // Handle SOCDs
        for (negative_input, positive_input, socd_type) in &schema.socds {
            let neg = *input_data.get(negative_input).unwrap_or(&false);
            let pos = *input_data.get(positive_input).unwrap_or(&false);

            if !neg || !pos {
                continue;
            }

            match socd_type {
                SocdType::Neutral => {
                    input_data.insert(positive_input.clone(), false);
                    input_data.insert(negative_input.clone(), false);
                }
                SocdType::Negative => {
                    input_data.insert(positive_input.clone(), false);
                }
                SocdType::Positive => {
                    input_data.insert(negative_input.clone(), false);
                }
            }
        }

        // Press the combination button if conditions are met
        if let Some(combination_inputs) = schema.input_list_by_type.get(&GameInputType::Multiple) {
            for input_name in combination_inputs {
                if let Some(entry) = schema.inputs.get(input_name) {
                    if let (Some(combination), Some(&raw_value)) = (&entry.combination, raw_input.get(input_name)) {
                        if !raw_value {
                            let combination_any = entry.combination_any.unwrap_or(false);
                            let value = if combination_any {
                                combination.iter().any(|c| *input_data.get(c).unwrap_or(&false))
                            } else {
                                combination.iter().all(|c| *input_data.get(c).unwrap_or(&false))
                            };
                            input_data.insert(input_name.clone(), value);
                        }
                    }
                }
            }
        }

        input_data
    }

    // -------------------------------------------------------------------------
    // Internal helper methods

    fn create_input_map_from_input_layout(&self, input_layout: &[PhysicalInput]) -> HashMap<String, InputMapEntry> {
        let mut input_map = HashMap::new();

        for physical_input in input_layout {
            let game_input_names = self.physical_input_get_bindable_game_input_names(physical_input);
            let keyboard_bindings = physical_input.keyboard_inputs.clone();
            let controller_bindings = physical_input.controller_inputs.clone();

            for (gi_id, game_input_name) in game_input_names.iter().enumerate() {
                let entry = InputMapEntry {
                    game_input_name: game_input_name.clone(),
                    action_name: game_input_name.clone(),
                    bindings_keyboard: keyboard_bindings.get(gi_id).cloned().unwrap_or_default(),
                    bindings_controller: controller_bindings.get(gi_id).cloned().unwrap_or_default(),
                };

                if input_map.contains_key(game_input_name) {
                    godot_error!("[CastagneInput] Game Input present twice in the input map: {}", game_input_name);
                }
                input_map.insert(game_input_name.clone(), entry);
            }
        }

        input_map
    }

    fn physical_input_get_bindable_game_input_names(&self, physical_input: &PhysicalInput) -> Vec<String> {
        let mut gi_names = self.physical_input_get_game_input_names(physical_input);

        match physical_input.input_type {
            PhysicalInputType::Axis => {
                // Keep only first 2 (Negative, Positive)
                gi_names.truncate(2);
            }
            PhysicalInputType::Stick => {
                // Keep only first 4 (Left, Right, Down, Up)
                gi_names.truncate(4);
            }
            _ => {}
        }

        gi_names
    }

    fn physical_input_get_game_input_names(&self, physical_input: &PhysicalInput) -> Vec<String> {
        let default_names = match physical_input.input_type {
            PhysicalInputType::Axis => vec!["Negative", "Positive", "Neutral"],
            PhysicalInputType::Stick => vec![
                "Left", "Right", "Down", "Up", "Back", "Forward",
                "Portside", "Starboard", "NeutralH", "NeutralV"
            ],
            _ => vec![""],
        };

        if let Some(ref custom_names) = physical_input.game_input_names {
            custom_names.clone()
        } else {
            default_names
                .iter()
                .map(|suffix| format!("{}{}", physical_input.name, suffix))
                .collect()
        }
    }

    fn create_input_schema_from_input_layout(&self, input_layout: &[PhysicalInput]) -> InputSchema {
        let mut schema = InputSchema {
            inputs: HashMap::new(),
            input_list: Vec::new(),
            input_list_by_type: HashMap::new(),
            socds: Vec::new(),
        };

        schema.input_list_by_type.insert(GameInputType::Direct, Vec::new());
        schema.input_list_by_type.insert(GameInputType::Multiple, Vec::new());
        schema.input_list_by_type.insert(GameInputType::Derived, Vec::new());

        for physical_input in input_layout {
            let game_input_names = self.physical_input_get_game_input_names(physical_input);

            // TODO: Implement full input schema creation logic
            // This is simplified - the original has complex logic for different input types

            match physical_input.input_type {
                PhysicalInputType::Raw => {
                    self.add_game_input_to_schema(&mut schema, &game_input_names[0], GameInputType::Direct, None);
                }
                PhysicalInputType::Button => {
                    self.add_game_input_to_schema(&mut schema, &game_input_names[0], GameInputType::Direct, None);
                    // TODO: Add Press and Release derived inputs
                }
                PhysicalInputType::Axis => {
                    // TODO: Add negative and positive inputs, SOCD handling
                }
                PhysicalInputType::Stick => {
                    // TODO: Add directional inputs, derived inputs
                }
                PhysicalInputType::Combination | PhysicalInputType::Any => {
                    // TODO: Add combination input
                }
            }
        }

        schema
    }

    fn add_game_input_to_schema(&self, schema: &mut InputSchema, input_name: &str, input_type: GameInputType, _input_data: Option<HashMap<String, String>>) {
        if schema.inputs.contains_key(input_name) {
            godot_error!("[CastagneInput] Input name {} already exists in the Input Schema!", input_name);
            return;
        }

        let entry = InputSchemaEntry {
            name: input_name.to_string(),
            input_type,
            combination: None,
            combination_any: None,
            derived_type: None,
            target: None,
            targets: None,
            dir_id: None,
            game_input_names: None,
        };

        schema.inputs.insert(input_name.to_string(), entry);
        schema.input_list_by_type.get_mut(&input_type).unwrap().push(input_name.to_string());
        schema.input_list.push(input_name.to_string());
    }
}

impl Default for CastagneInput {
    fn default() -> Self {
        Self::new()
    }
}
