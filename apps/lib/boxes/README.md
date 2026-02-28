# apps/lib/boxes

Shared `.hako` box library sources.

## Runtime status (phase29cc)

- `array_std.hako`, `console_std.hako`, `map_std.hako`, `string_std.hako`:
  standard library boxes used by current app/runtime paths.
- `wasm_canvas_box.hako`, `wasm_display_box.hako`:
  reserved facades for future WasmBox-first route.
  Current `nyash-wasm` g4 path uses marker-driven prebuilt fixtures and JS draw hooks,
  so these two files are intentionally not on the active compile route yet.

When promoting WasmBox-first route, update this file and the phase SSOT docs in
`docs/development/current/main/phases/phase-29cc/`.
