# Phase 140x-90: map owner pilot SSOT

## Goal

Make `Map` the second source-backed `.hako` semantic owner pilot without moving raw probe/load/store implementation out of Rust.

## Owner Graph

- `.hako` visible owner:
  - `lang/src/runtime/collections/map_core_box.hako`
  - `lang/src/runtime/collections/map_state_core_box.hako`
- `.hako` substrate hop:
  - `lang/src/runtime/substrate/raw_map/raw_map_core_box.hako`
- Rust thin facade:
  - `crates/nyash_kernel/src/plugin/map_aliases.rs`
- Rust observer shim:
  - `crates/nyash_kernel/src/plugin/map_substrate.rs`
- Rust compat/runtime forwarding:
  - `crates/nyash_kernel/src/plugin/map_runtime_data.rs`
  - `crates/nyash_kernel/src/plugin/map_compat.rs`
- Rust native accelerators:
  - `crates/nyash_kernel/src/plugin/map_probe.rs`
  - `crates/nyash_kernel/src/plugin/map_slot_load.rs`
  - `crates/nyash_kernel/src/plugin/map_slot_store.rs`

## Stop Lines

- do not move `nyash.map.slot_*` / `probe_*` / `entry_count_i64` implementation out of Rust in this lane
- do not move raw probe/load/store leaves out of Rust
- do not let `map_runtime_data.rs` / `map_compat.rs` grow new owner logic
- keep `MapCoreBox` / `MapStateCoreBox` as the visible semantics owner

## Success Condition

- the Map semantic owner seam is stable and explicit
- Rust raw substrate remains capability-only
- Rust compat/runtime forwarding is shrink-only
- `phase-141x string semantic boundary review` is ready
