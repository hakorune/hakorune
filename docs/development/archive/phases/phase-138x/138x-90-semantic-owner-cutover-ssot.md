# Phase 138x-90: semantic owner cutover SSOT

## Goal

Turn the landed four-bucket split into a stable final owner model before reopening perf work.

## Final Reading

### Permanent owners

1. `Rust host microkernel`
2. `.hako semantic kernel`
3. `native accelerators`

### Auxiliary surfaces

- `ABI facade`
- `compat quarantine`

## Constraints

- keep the `phase-134x` split as the refactor seam
- do not restart broad perf work before the owner graph is fixed
- do not broaden `.hako` migration into hot leaf substrate
- do not let `compat quarantine` become a permanent owner

## First Concrete Cutover

1. `Array owner`
2. `Map owner`
3. `String` semantic boundary review

## Array Pilot Seam

- visible owner:
  - `lang/src/runtime/collections/array_core_box.hako`
  - `lang/src/runtime/collections/array_state_core_box.hako`
- substrate below the owner:
  - `lang/src/runtime/substrate/raw_array/raw_array_core_box.hako`
  - `lang/src/runtime/substrate/ptr/ptr_core_box.hako`
- Rust ABI facade:
  - `crates/nyash_kernel/src/plugin/array_substrate.rs`
- Rust compat/runtime forwarding:
  - `crates/nyash_kernel/src/plugin/array_runtime_facade.rs`
- Rust accelerators:
  - `crates/nyash_kernel/src/plugin/array_handle_cache.rs`
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs`

The pilot moves visible semantics only. Raw slot implementation and cache/fast
path remain Rust-native.

## Success Condition

- current docs read `semantic owner cutover`, not `perf reopen`
- Rust permanent zones are locked
- `.hako` semantic owner corridor is fixed
- `phase-139x array owner pilot` is the next implementation lane
