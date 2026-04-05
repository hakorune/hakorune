# Phase 139x: array owner pilot

- Status: Active
- 目的: `ArrayCoreBox` / `ArrayStateCoreBox` を visible semantics owner として固定し、Rust 側を `ABI facade` / `raw substrate` / `native accelerators` に限定する first pilot を source-backed に詰める。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
  - `lang/src/runtime/collections/array_core_box.hako`
  - `lang/src/runtime/collections/array_state_core_box.hako`
  - `lang/src/runtime/substrate/raw_array/raw_array_core_box.hako`
  - `lang/src/runtime/substrate/ptr/ptr_core_box.hako`
  - `crates/nyash_kernel/src/plugin/array_substrate.rs`
  - `crates/nyash_kernel/src/plugin/array_runtime_facade.rs`
  - `crates/nyash_kernel/src/plugin/array_handle_cache.rs`
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs`
- success:
  - `Array owner` seam is source-backed
  - `.hako` owner responsibilities are explicit
  - Rust compat/runtime forwarding is marked shrink-only
  - Rust raw substrate and accelerators stay non-owning
  - next lane is `phase-140x map owner pilot`

## Decision Now

- `.hako` owner:
  - `array_core_box.hako`
  - `array_state_core_box.hako`
- substrate below owner:
  - `raw_array_core_box.hako`
  - `ptr_core_box.hako`
- Rust ABI facade:
  - `array_substrate.rs`
- Rust compat/runtime forwarding:
  - `array_runtime_facade.rs`
- Rust accelerators:
  - `array_handle_cache.rs`
  - `array_string_slot.rs`

## Fresh Read

- the pilot is not about moving raw `slot_*` implementation out of Rust
- the pilot is about making `ArrayBox.{get,set,push,len,length,size}` policy/fallback/state definitively `.hako`-owned
- `array_runtime_facade.rs` should remain compat/runtime forwarding only
- `array_handle_cache.rs` and `array_string_slot.rs` stay native accelerators and must not become owners

## Next

1. lock exact owner/substrate/facade/accelerator boundaries
2. mark compat/runtime forwarding shrink-only
3. hand off to `phase-140x map owner pilot`
