---
Status: Landed
Date: 2026-04-28
Scope: prune loop_continue_only recipe-tree re-export
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
  - src/mir/builder/control_flow/plan/recipe_tree/loop_continue_only_composer.rs
  - src/mir/builder/control_flow/plan/recipe_tree/matcher/patterns.rs
---

# 291x-664: LoopContinueOnly Recipe Re-Export Prune

## Goal

Remove the `loop_continue_only` builder/type re-export from the recipe-tree
root facade.

This is BoxShape cleanup. It must not change loop-continue-only recipe
construction, verification, matching, planner acceptance, or lowering behavior.

## Evidence

`recipe_tree/mod.rs` still re-exported many builder functions and recipe types
for flattened-builder compatibility. The `loop_continue_only` pair had only two
root-facade consumers:

- `loop_continue_only_composer.rs`
- `matcher/patterns.rs`

Both can import directly from `loop_continue_only_builder`.

## Decision

Move `build_loop_continue_only_recipe` and `LoopContinueOnlyRecipe` imports to
`recipe_tree::loop_continue_only_builder`.

Remove only this pair from the recipe-tree root facade. Leave the broader
recipe-tree facade thinning for separate cards.

## Boundaries

- Do not move builder implementation.
- Do not touch recipe matcher logic.
- Do not change loop-continue-only facts or recipe contracts.
- Do not prune other recipe-tree builder re-exports in this card.

## Acceptance

```bash
cargo fmt
cargo test loop_continue_only --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Loop-continue-only recipe callers import the builder pair from its owner
  module.
- `recipe_tree` root no longer re-exports the loop-continue-only builder pair.
- Loop-continue-only behavior is unchanged.
