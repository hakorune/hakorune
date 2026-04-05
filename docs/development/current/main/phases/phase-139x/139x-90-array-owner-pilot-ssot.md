# Phase 139x-90: array owner pilot SSOT

## Goal

Make `Array` the first source-backed `.hako` semantic owner pilot without moving raw slot implementation out of Rust.

## Owner Graph

- `.hako` visible owner:
  - `lang/src/runtime/collections/array_core_box.hako`
  - `lang/src/runtime/collections/array_state_core_box.hako`
- `.hako` substrate hop:
  - `lang/src/runtime/substrate/raw_array/raw_array_core_box.hako`
  - `lang/src/runtime/substrate/ptr/ptr_core_box.hako`
- Rust ABI facade:
  - `crates/nyash_kernel/src/plugin/array_substrate.rs`
- Rust compat/runtime forwarding:
  - `crates/nyash_kernel/src/plugin/array_runtime_facade.rs`
- Rust native accelerators:
  - `crates/nyash_kernel/src/plugin/array_handle_cache.rs`
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs`

## Stop Lines

- do not move `nyash.array.slot_*` implementation out of Rust in this lane
- do not move cache/fast-path leaves out of Rust
- do not let `array_runtime_facade.rs` grow new owner logic
- keep `ArrayCoreBox` / `ArrayStateCoreBox` as the visible semantics owner

## Success Condition

- the Array semantic owner seam is stable and explicit
- Rust raw substrate remains capability-only
- Rust compat/runtime forwarding is shrink-only
- `phase-140x map owner pilot` is ready
