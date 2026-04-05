# Phase 138x: nyash_kernel semantic owner cutover

- Status: Landed
- 目的: `phase-134x` で landed した4層 split を中間形として固定したうえで、最終アーキテクチャを `Rust host microkernel` / `.hako semantic kernel` / `native accelerators` に定義し直す。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
  - `docs/development/current/main/phases/README.md`
  - `crates/nyash_kernel/src/entry.rs`
  - `crates/nyash_kernel/src/ffi/**`
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/plugin/array_substrate.rs`
  - `crates/nyash_kernel/src/plugin/map_aliases.rs`
  - `crates/nyash_kernel/src/plugin/module_string_dispatch/**`
- success:
  - current no longer reads like direct perf reopen
  - final owner graph is source-backed
  - `Array owner` pilot is the next concrete implementation lane
  - `phase-137x main kilo reopen selection` stays alive as a successor, not as current work

## Decision Now

- `phase-134x` four-bucket split stays landed as the refactor seam
- but the final architecture is no longer read as four permanent layers
- current permanent reading is:
  1. `Rust host microkernel`
  2. `.hako semantic kernel`
  3. `native accelerators`
- auxiliary surfaces:
  - `ABI facade`
  - `compat quarantine`

## Fresh Read

- `entry.rs` / `ffi/**` / `future.rs` / `invoke_core.rs` / `hako_forward_bridge.rs` are Rust keep by owner, not by accident
- `string.rs` / `array_substrate.rs` / `map_aliases.rs` are thin ABI facade candidates, not semantic owners
- `module_string_dispatch/**` should be treated as quarantine, not as a final backend owner
- `Array -> Map -> String` is the clean migration order for semantic ownership
- `Array owner` seam is source-backed:
  - owner: `array_core_box.hako` / `array_state_core_box.hako`
  - substrate: `raw_array_core_box.hako` / `ptr_core_box.hako`
  - ABI facade: `array_substrate.rs`
  - compat/runtime forwarders: `array_runtime_facade.rs`
  - native accelerators: `array_handle_cache.rs` / `array_string_slot.rs`

## Next

1. hand off to `phase-139x array owner pilot`
2. keep `phase-137x` as perf successor
