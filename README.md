# Castagne-RS 🦀🥊

An **experimental** Rust port of the [Castagne fighting game engine](https://github.com/panthavma/castagne) using [godot-rust](https://godot-rust.github.io/).

## ⚠️ This is a Fun Experiment!

This project is an exploratory port of Castagne from GDScript to Rust. While the **long-term goal** would be to achieve feature parity with the original engine, **we're not expecting to get there**. This is primarily a learning exercise and experiment in porting a complex game engine to Rust.

## What is Castagne?

Castagne is a fighting game engine built on top of Godot that manages the internal logic needed for fighting games, allowing developers to focus on game design rather than low-level mechanics. It features:

- 🎮 Complete fighting game logic system
- 🔄 Rollback netcode for online play
- 🛠️ Powerful editor tools
- 🧩 Modular architecture
- 📦 Examples and templates

Projects using Castagne include **Kronian Titans** and **Molten Winds**.

## Project Structure

```
castagne-rs/
├── castagne/          # Original Castagne engine (git submodule)
├── src/
│   └── lib.rs         # Rust port implementation
├── Cargo.toml
└── README.md
```

## Technology Stack

- **Language**: Rust 🦀
- **Engine Integration**: [godot-rust (gdext)](https://godot-rust.github.io/) - Rust bindings for Godot 4
- **Target Engine**: Godot 4.x
- **Original**: GDScript on Godot

## Current Status

- ✅ Basic project setup
- ✅ Godot-rust integration configured
- ✅ Core engine structure initialized
- ⏳ Engine logic (in progress)
- ⏳ Module system
- ⏳ Editor tools
- ⏳ Networking/Rollback

## Building

Make sure you have Rust and Cargo installed:

```bash
# Build the project
cargo build

# Build for release
cargo build --release
```

## Testing

Run comparison tests to verify parity between Rust and GDScript implementations:

```bash
# One-time setup (downloads Godot)
./scripts/setup-godot.sh

# Run comparison tests
./scripts/run-tests.sh
```

Tests compare behavior between:
- Original Castagne (GDScript)
- castagne-rs (Rust port)

See [TESTING_CLI.md](TESTING_CLI.md) for detailed testing guide.

## Development

The Rust port aims to maintain similar architecture to the original while leveraging Rust's safety and performance characteristics. Key differences:

- **Memory Safety**: Rust's ownership system vs GDScript's reference counting
- **Type System**: Static typing with Rust vs dynamic typing in GDScript
- **Performance**: Compiled Rust code vs interpreted GDScript
- **Godot Integration**: GDExtension API (binary) vs native GDScript

## Contributing

Since this is an experimental project, contributions are welcome but please note:

1. This is **not** intended to replace the original Castagne
2. Focus is on learning and experimentation
3. Don't expect production-ready code
4. Breaking changes may happen frequently

## Original Project

Original Castagne engine: https://github.com/panthavma/castagne

All credit for the engine design and architecture goes to the original Castagne team!

## License

This port is licensed under the MPL-2.0 (Mozilla Public License 2.0) (same as the original Castagne project).

The original Castagne engine is licensed under MPL-2.0. See the `castagne/` submodule for details.

---

**Note**: This is a personal experiment and is not affiliated with or endorsed by the original Castagne project.
