---
Status: Landed
Date: 2026-04-29
Scope: loop parts cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/parts/loop_/body_block.rs
  - src/mir/builder/control_flow/plan/parts/loop_/mod.rs
  - src/mir/builder/control_flow/plan/loop_scan_v0/pipeline.rs
  - src/mir/builder/control_flow/plan/features/generic_loop_body/v1.rs
---

# 291x-701: Loop Body-Block Wrapper Prune

## Why

`parts/loop_/body_block.rs` still carried two thin wrapper entries that had no
owner-path callers outside their own definition/re-export shelf:

- `lower_loop_with_body_block_with_break_phi_dsts`
- `lower_loop_with_exit_only_body_block`

The live lowering routes already use `lower_loop_with_body_block()` or the
verified entry helpers directly, so keeping the extra wrappers only widened the
surface and added dead-code noise.

## Changes

- removed `lower_loop_with_body_block_with_break_phi_dsts()`
- removed `lower_loop_with_exit_only_body_block()`
- narrowed `parts::loop_` re-exports to the single live body-block entry

## Result

- `cargo build --release` warning count moved from **20** to **18**
- the loop parts facade now exposes only the body-block lowering entry that has
  real production callers

## Proof

```bash
cargo build --release
cargo test --release --lib loop_scan_v0_linear_segment_lowers_simple_exit_allowed_slice -- --nocapture
cargo test --release --lib generic_loop_v1_cleanup_appends_fallthrough_continue -- --nocapture
```
