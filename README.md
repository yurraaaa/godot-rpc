# godot-rpc

A lightweight Discord Rich Presence (RPC) GDExtension for Godot 4, written in Rust.

Shows your active Godot project and elapsed time directly on your Discord profile while using the editor.

## Installation

1. Copy the `addons/godot-rpc` folder into your Godot project's `addons/` directory.
2. Restart or open your project in Godot 4.2+.

## Configuration

By default, it uses a built-in Discord application. You can set a custom Application ID in your Godot project:

- Go to **Project -> Project Settings**.
- Find `editor/godot_rpc/application_id` and enter your Discord Application ID.
- Restart the editor.

## Building from Source

Prerequisites: Rust toolchain (2024 edition).

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

Copy the resulting compiled library from `target/` into `addons/godot-rpc/bin/` with the appropriate name as configured in `rpc.gdextension`.
