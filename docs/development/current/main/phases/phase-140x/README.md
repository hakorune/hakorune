# Phase 140x: map owner pilot

- Status: Landed
- 目的: `MapCoreBox` / `MapStateCoreBox` を visible semantics owner として固定し、Rust 側を thin map facade / observer shim / raw substrate / compat forwarding に限定する second pilot を source-backed に詰める。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
  - `lang/src/runtime/collections/map_core_box.hako`
  - `lang/src/runtime/collections/map_state_core_box.hako`
  - `lang/src/runtime/substrate/raw_map/raw_map_core_box.hako`
  - `crates/nyash_kernel/src/plugin/map_aliases.rs`
  - `crates/nyash_kernel/src/plugin/map_substrate.rs`
  - `crates/nyash_kernel/src/plugin/map_runtime_facade.rs`
  - `crates/nyash_kernel/src/plugin/map_probe.rs`
  - `crates/nyash_kernel/src/plugin/map_slot_load.rs`
  - `crates/nyash_kernel/src/plugin/map_slot_store.rs`
- success:
  - `Map owner` seam is source-backed
  - `.hako` owner responsibilities are explicit
  - Rust thin facade and observer shim are explicit
  - Rust compat/runtime forwarding is marked shrink-only
  - next lane is `phase-141x string semantic boundary review`

## Decision Now

- `.hako` owner:
  - `map_core_box.hako`
  - `map_state_core_box.hako`
- substrate below owner:
  - `raw_map_core_box.hako`
- Rust thin facade:
  - `map_aliases.rs`
- Rust observer shim:
  - `map_substrate.rs`
- Rust compat/runtime forwarding:
  - `map_runtime_facade.rs`
- Rust accelerators:
  - `map_probe.rs`
  - `map_slot_load.rs`
  - `map_slot_store.rs`

## Fresh Read

- the pilot is not about moving raw probe/load/store out of Rust
- the pilot is about making `MapBox.{get,set,has,len,length,size}` policy/fallback/key-normalization/state definitively `.hako`-owned
- `map_runtime_facade.rs` should remain compat/runtime forwarding only
- `map_probe.rs` / `map_slot_load.rs` / `map_slot_store.rs` stay native/raw leaves and must not become owners

## Next

1. lock exact owner/substrate/facade/forwarding/accelerator boundaries
2. mark `map_runtime_facade.rs` shrink-only
3. hand off to `phase-141x string semantic boundary review`
