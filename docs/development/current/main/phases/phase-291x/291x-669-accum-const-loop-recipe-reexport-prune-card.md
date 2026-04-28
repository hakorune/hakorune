---
Status: Landed
Date: 2026-04-28
Scope: prune accum_const_loop recipe-tree re-export
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
  - src/mir/builder/control_flow/plan/recipe_tree/accum_const_loop_composer.rs
  - src/mir/builder/control_flow/plan/recipe_tree/matcher/patterns.rs
---

# 291x-669: AccumConstLoop Recipe Re-Export Prune

## Goal

Remove the accum-const-loop builder/type re-export from the recipe-tree root
facade.

This is BoxShape cleanup. It must not change accum-const-loop recipe
construction, verification, matching, planner acceptance, or lowering behavior.

## Evidence

The accum-const-loop builder pair is still re-exported from
`recipe_tree/mod.rs`. Its root-facade consumers are limited to:

- `accum_const_loop_composer.rs`
- `matcher/patterns.rs`

Both can import directly from `accum_const_loop_builder`.

## Decision

Move `build_accum_const_loop_recipe` and `AccumConstLoopRecipe` imports to
`recipe_tree::accum_const_loop_builder`.

Remove only this pair from the recipe-tree root facade.

## Boundaries

- Do not move builder implementation.
- Do not touch matcher control flow or recipe verification.
- Do not change accum-const-loop facts or contracts.
- Do not prune other recipe-tree builder re-exports in this card.

## Acceptance

```bash
cargo fmt
cargo test accum_const_loop --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Accum-const-loop recipe callers import the builder pair from its owner
  module.
- `recipe_tree` root no longer re-exports the accum-const-loop builder pair.
- Accum-const-loop behavior is unchanged.
