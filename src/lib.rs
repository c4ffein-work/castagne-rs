// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Castagne-RS - Rust Parser for Castagne Fighting Game Engine
//!
//! This provides a high-performance Rust parser for .casp files that integrates
//! with the Castagne GDScript engine via GDExtension.
//!
//! Architecture:
//! - Parser: Fast Rust implementation for parsing .casp character files
//! - Engine: GDScript (original Castagne, ported to Godot 4.5)
//! - Integration: GDExtension interface for seamless interop

use godot::prelude::*;

// Module declarations
pub mod parser;
pub mod test_runner;

struct CastagneRsExtension;

#[gdextension]
unsafe impl ExtensionLibrary for CastagneRsExtension {}
