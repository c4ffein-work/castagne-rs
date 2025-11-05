use godot::prelude::*;

struct CastagneRsExtension;

#[gdextension]
unsafe impl ExtensionLibrary for CastagneRsExtension {}

/// Castagne Engine Core
///
/// This is an experimental Rust port of the Castagne fighting game engine.
/// The original engine is written in GDScript for Godot.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct CastagneEngine {
    base: Base<Node>,
}

#[godot_api]
impl INode for CastagneEngine {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Castagne-RS: Initializing Castagne Engine (Rust Port)");

        Self {
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("Castagne-RS: Engine ready!");
    }
}

#[godot_api]
impl CastagneEngine {
    #[func]
    pub fn get_version(&self) -> GString {
        GString::from("0.1.0-experimental")
    }
}
