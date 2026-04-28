---
Status: Landed
Date: 2026-04-28
Scope: prune loop-cond feature pipeline re-exports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/features/mod.rs
  - src/mir/builder/control_flow/plan/normalizer/mod.rs
  - src/mir/builder/control_flow/plan/recipe_tree/loop_cond_composer.rs
---

# 291x-662: Loop-Cond Feature Pipeline Re-Export Prune

## Goal

Remove backwards-compat re-exports for flattened loop-cond pipeline entrypoints
from `plan::features`.

This is BoxShape cleanup. It must not change loop-cond normalization,
recipe composition, PHI handling, carrier handling, or lowering behavior.

## Evidence

Worker inventory found two re-exports in `plan/features/mod.rs`:

- `lower_loop_cond_break_continue`
- `lower_loop_cond_continue_only`

Only `PlanNormalizer` and `loop_cond_composer` used the flattened facade paths.
Both can call the owning modules directly:

- `features::loop_cond_bc::lower_loop_cond_break_continue`
- `features::loop_cond_co_pipeline::lower_loop_cond_continue_only`

## Decision

Update callers to use the owning pipeline module paths and remove the
backwards-compat re-exports from `features/mod.rs`.

## Boundaries

- Do not move pipeline functions.
- Do not change facts-to-lower contracts.
- Do not change recipe verification or loop-cond route behavior.
- Do not mix this with recipe-tree facade cleanup.

## Acceptance

```bash
cargo fmt
cargo test loop_cond --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- `PlanNormalizer` and `loop_cond_composer` now call loop-cond pipeline owners
  directly.
- `plan::features` no longer re-exports the flattened pipeline entrypoints.
- Loop-cond lowering behavior is unchanged.
