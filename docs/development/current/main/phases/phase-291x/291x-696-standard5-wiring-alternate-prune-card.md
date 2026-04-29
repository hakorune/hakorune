---
Status: Landed
Date: 2026-04-29
Scope: plan steps cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/steps/loop_wiring_standard5.rs
  - src/mir/builder/control_flow/plan/parts/wiring_tests.rs
---

# 291x-696: Standard5 Wiring Alternate Prune

## Why

`loop_wiring_standard5.rs` still carried the older full-wiring helpers
`build_standard5_wiring` and `build_standard5_frag`, but the active owner-path
already uses short-circuit header branches plus `build_standard5_internal_wires`
directly. The extra helpers were dead surface.

## Changes

- removed dead alternate helpers:
  - `build_standard5_wiring`
  - `build_standard5_frag`
- trimmed now-unused imports in `loop_wiring_standard5.rs`
- kept the active surface unchanged:
  - `build_standard5_internal_wires`
  - `empty_carriers_args`

## Result

- `cargo build --release` warning count moved from **32** to **30**
- the Standard5 steps surface now matches the internal-wire-only loop path

## Proof

```bash
cargo build --release
cargo test --release --lib then_only_loop_uses_join_dst_for_carrier -- --nocapture
```
