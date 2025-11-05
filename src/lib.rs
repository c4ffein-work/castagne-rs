// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Castagne-RS - Experimental Rust port of the Castagne fighting game engine
//!
//! This is a Rust port of the Castagne engine using godot-rust.
//! Castagne is a modular fighting game engine for Godot with rollback netcode support.

use godot::prelude::*;
use std::collections::HashMap;

// Module declarations
pub mod config;
pub mod engine;
pub mod memory;
pub mod module;
pub mod modules;
pub mod state_handle;

use crate::config::CastagneConfig;
use crate::engine::CastagneEngineCore;

struct CastagneRsExtension;

#[gdextension]
unsafe impl ExtensionLibrary for CastagneRsExtension {}

/// Castagne Engine - Godot Node wrapper
///
/// This is the main Godot node that wraps the Castagne engine core.
/// It can be added to a Godot scene and provides the same interface as the original.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct CastagneEngine {
    base: Base<Node>,
    #[allow(dead_code)]
    engine_core: Option<CastagneEngineCore>,
    init_on_ready: bool,
}

#[godot_api]
impl INode for CastagneEngine {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Castagne-RS: Initializing Castagne Engine (Rust Port)");

        Self {
            base,
            engine_core: None,
            init_on_ready: true,
        }
    }

    fn ready(&mut self) {
        if self.init_on_ready {
            godot_print!("Castagne-RS: Engine ready! Initializing core...");
            self.initialize_engine();
        }
    }
}

#[godot_api]
impl CastagneEngine {
    /// Get the version string
    #[func]
    pub fn get_version(&self) -> GString {
        GString::from("0.1.0-experimental")
    }

    /// Initialize the engine core
    #[func]
    pub fn initialize_engine(&mut self) {
        godot_print!("Castagne-RS: Creating engine core...");

        // Create config and engine
        let config = CastagneConfig::new();
        let mut engine = CastagneEngineCore::new(config);

        // Initialize the engine
        let battle_init_data = HashMap::new();
        match engine.init(battle_init_data) {
            Ok(_) => {
                godot_print!("Castagne-RS: Engine core initialized successfully!");
                self.engine_core = Some(engine);
            }
            Err(e) => {
                godot_error!("Castagne-RS: Failed to initialize engine: {}", e);
            }
        }
    }

    /// Execute a single frame (for testing)
    #[func]
    pub fn execute_frame(&mut self) -> bool {
        if let Some(ref mut engine) = self.engine_core {
            match engine.execute_frame() {
                Ok(_) => true,
                Err(e) => {
                    godot_error!("Castagne-RS: Frame execution failed: {}", e);
                    false
                }
            }
        } else {
            godot_error!("Castagne-RS: Engine not initialized!");
            false
        }
    }

    /// Get info about the engine state
    #[func]
    pub fn get_info(&self) -> GString {
        if self.engine_core.is_some() {
            GString::from("Castagne-RS Engine v0.1.0 - Core initialized")
        } else {
            GString::from("Castagne-RS Engine v0.1.0 - Not initialized")
        }
    }
}
