# C Core ABI (Design Stage) — Phase 22.2

Status: design-stage shim; defaults OFF; behavior unchanged.

Purpose
- Define a minimal, stable C ABI boundary to enable future replacement of selected Rust runtime paths when Hakorune is compiled to native EXE via LLVM.
- Keep Rust path authoritative while stubbing C calls behind env+feature gates; ON/OFF parity must hold.

Conventions
- Encoding: UTF‑8, null-terminated (const char*). No ownership transfer.
- Return: `int` (0 = success; negative values reserved for future detailed errors).
- Threading: functions must be reentrant; stateful access goes via (type_id, instance_id).

Functions (initial)
- `int ny_core_probe_invoke(const char* target, const char* method, int32_t argc)`
  - No-op probe for diagnostics; safe to call for any target/method pair.
- `int ny_core_map_set(int32_t type_id, uint32_t instance_id, const char* key, const char* val)`
  - Design stub for MapBox.set. Current implementation is no-op; Rust path performs the actual mutation.

Gates & Features
- Build: `cargo build --release -p nyash-rust --features c-core`
- Env:
  - `HAKO_C_CORE_ENABLE=1` — enable c-core probe routing
  - `HAKO_C_CORE_TARGETS=MapBox.set,ArrayBox.push` — limit targets (default: MapBox.set)
  - Tags: `[c-core:invoke:<Box>.<method>]`

Call Sites (Rust)
- PluginLoaderV2 (enabled): `src/runtime/plugin_loader_v2/enabled/ffi_bridge.rs`
  - When gated ON + targeted, call C shim then continue with the original Rust path (parity preserved).

Validation
- Parity canaries compare ON/OFF outputs (and rc) for MapBox.set; later for ArrayBox.push/get/size.
- Failure/unavailable paths must fall back immediately to the Rust path.

Roadmap
- Expand ArrayBox (push → get → size) with the same staged approach.
- Formalize error codes and minimal state API only after parity is stable.
