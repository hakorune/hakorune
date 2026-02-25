Plugins Autoload Sample

How to run (from repo root):
- Build core: `cargo build --release`
- Enable autoload and run:
  `NYASH_USING_DYLIB_AUTOLOAD=1 ./target/release/hakorune examples/plugins/autoload_sample/main.hako`

Notes
- The plugin path in nyash.toml points to `plugins/nyash-counter-plugin/*`. Ensure the shared library exists.
- If missing, build plugins with `cargo build --release -p nyash-counter-plugin`.
- Autoload is guarded; respects `NYASH_DISABLE_PLUGINS=1`.

