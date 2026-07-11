---
Status: Landed
Date: 2026-04-29
Scope: condition-env helper cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/condition_env_builder.rs
  - src/mir/builder/control_flow/plan/route_prep_pipeline.rs
---

# 291x-703: ConditionEnvBuilder Test-Surface Prune

## Why

`ConditionEnvBuilder` no longer had any production owner-path callers. The only
remaining uses were its own self-tests inside `condition_env_builder.rs`, so
keeping the builder type and constructors live in lib builds only added dead-code
noise.

## Changes

- gated `ConditionEnvBuilder` and its supporting imports behind `#[cfg(test)]`
- kept the self-tests intact so the helper logic still has direct coverage
- left surrounding route-prep docs and comments unchanged

## Result

- `cargo build --release` warning count moved from **16** to **14**
- lib builds now keep only the live planning surface while the helper shelf
  remains available to its dedicated tests

## Proof

```bash
cargo build --release
cargo test --release --lib test_build_for_break_condition_v2_uses_param_region -- --nocapture
cargo test --release --lib test_build_loop_param_only_v2 -- --nocapture
```
