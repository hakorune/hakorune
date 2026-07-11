---
Status: Landed
Date: 2026-04-29
Scope: plan helper cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/common_init.rs
  - src/mir/builder/control_flow/plan/features/loop_cond_continue_with_return_phi_materializer.rs
  - src/mir/builder/control_flow/plan/route_prep_pipeline.rs
---

# 291x-702: Plan Helper Shelf Prune

## Why

Two plan-side helpers had become pure dead surface:

- `LoopCondContinueWithReturnPhiMaterializer::current_bindings_mut()`
- `CommonPatternInitializer::check_carrier_updates_allowed()`

Neither had owner-path callers beyond their own definition or stale comment
examples, so keeping them only widened the plan API and left dead-code noise in
lib builds.

## Changes

- removed the unused mutable bindings accessor from the continue-with-return phi
  materializer
- removed the dead carrier-update gate helper from `common_init.rs`
- left the live read-only bindings accessor and route-prep initialization path
  unchanged

## Result

- `cargo build --release` warning count moved from **18** to **16**
- the affected plan helpers now expose only live entry points

## Proof

```bash
cargo build --release
cargo test --release --lib prepare_rebinds_header_phis_and_close_uses_them_as_final_values -- --nocapture
cargo test --release --lib test_context_carrier_count -- --nocapture
```
