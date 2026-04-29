---
Status: Landed
Date: 2026-04-29
Scope: generic-loop facts cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/generic_loop/facts_types.rs
  - src/mir/builder/control_flow/plan/generic_loop/facts/extract/v0.rs
  - src/mir/builder/control_flow/plan/generic_loop/facts/extract/v1.rs
---

# 291x-704: Generic-Loop Facts Field Prune

## Why

Worker-backed inventory showed that three generic-loop fact fields were written
at extraction time but never read by downstream lowering:

- `GenericLoopV0Facts::cond_profile`
- `GenericLoopV1Facts::shape_id`
- `GenericLoopV1Facts::cond_profile`

Keeping them only widened the facts contract and left dead-code warnings in lib
builds.

## Changes

- removed `cond_profile` from `GenericLoopV0Facts`
- removed `shape_id` and `cond_profile` from `GenericLoopV1Facts`
- trimmed the corresponding extraction-time assignments in `facts/extract/v0.rs`
  and `facts/extract/v1.rs`

## Result

- `cargo build --release` warning count moved from **14** to **12**
- generic-loop extraction now stores only the fields that active lowering still
  consumes

## Proof

```bash
cargo build --release
cargo test --release --lib generic_loop_v1_cleanup_appends_fallthrough_continue -- --nocapture
cargo test --release --lib loop_scan_v0_linear_segment_lowers_simple_exit_allowed_slice -- --nocapture
```
