# Phase 142x: array owner cutover implementation

- Status: Active
- 目的: `phase-139x` で固定した Array seam を宣言で終わらせず、`.hako` 側を visible owner implementation に寄せ、Rust 側を thin ABI facade / compat forwarding / accelerator に押し戻す。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
  - `lang/src/runtime/collections/array_core_box.hako`
  - `lang/src/runtime/collections/array_state_core_box.hako`
  - `lang/src/runtime/substrate/raw_array/raw_array_core_box.hako`
  - `crates/nyash_kernel/src/plugin/array_substrate.rs`
  - `crates/nyash_kernel/src/plugin/array_runtime_aliases.rs`
  - `crates/nyash_kernel/src/plugin/array_runtime_any.rs`
  - `crates/nyash_kernel/src/plugin/array_runtime_facade.rs`
  - `crates/nyash_kernel/src/plugin/array_runtime_substrate.rs`
  - `crates/nyash_kernel/src/plugin/runtime_data_array_dispatch.rs`
  - `crates/nyash_kernel/src/plugin/array_handle_cache.rs`
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs`
- success:
  - Array visible owner behavior is implemented on the `.hako` side, not only declared
  - historical runtime aliases are isolated from the forwarding core
  - RuntimeData-style any-key array routes are isolated from the slot/substrate forwarding core
  - mainline substrate-side append/capacity/string-slot forwarding is isolated from compat/runtime facade code
  - `RuntimeDataBox` array branch is isolated from the top-level dispatch shell
  - `array_runtime_facade.rs` is index-forwarding-only and shrink-only
  - `array_substrate.rs` stays thin ABI facade
  - accelerator leaves remain Rust-owned
  - next lane is `phase-143x map owner cutover implementation`

## Decision Now

- `.hako` owner implementation:
  - `array_core_box.hako`
  - `array_state_core_box.hako`
- substrate below owner:
  - `raw_array_core_box.hako`
  - `ptr_core_box.hako`
- Rust thin facade:
  - `array_substrate.rs`
- Rust compat alias surface:
  - `array_runtime_aliases.rs`
- Rust any-key runtime shell:
  - `array_runtime_any.rs`
- Rust substrate forwarding shell:
  - `array_runtime_substrate.rs`
- Rust compat/runtime forwarding:
  - `array_runtime_facade.rs`
- Rust RuntimeData array branch:
  - `runtime_data_array_dispatch.rs`
- Rust accelerators:
  - `array_handle_cache.rs`
  - `array_string_slot.rs`

## Fresh Read

- this lane is not about moving cache/string-slot leaves out of Rust
- this lane is about making `.hako` the actual owner of visible Array semantics
- Rust should retain capability, forwarding core, and isolated compat alias surfaces only

## Next

1. cut visible Array owner behavior over to `.hako`
2. keep historical runtime aliases out of `array_runtime_facade.rs`
3. hand off to `phase-143x map owner cutover implementation`
