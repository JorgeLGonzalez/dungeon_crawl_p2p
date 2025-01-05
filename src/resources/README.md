# Resources

## Assets

Asset management relies on the [bevy_asset_loader](https://crates.io/crates/bevy_asset_loader) crate. We should be able to handle sprite images, audio and hopefully ron/json files the same way?

The loading is configured in `main` in the `App` via the "add_loading_state" method.

NB: The asset folder is relative to the project root. WASM builds auto-magically handle this (at least in local server) and if you run `cargo run`, cargo automatically sets the `CARGO_MANIFEST_DIR` env var to the dir containing `Cargo.toml`, but the VS Debugger needs to have this variably explicitly set.
