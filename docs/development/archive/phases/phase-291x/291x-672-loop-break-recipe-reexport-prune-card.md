---
Status: Landed
Date: 2026-04-28
Scope: prune loop_break recipe-tree re-export
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
  - src/mir/builder/control_flow/plan/recipe_tree/loop_break_composer.rs
  - src/mir/builder/control_flow/plan/recipe_tree/matcher/patterns.rs
---

# 291x-672: LoopBreak Recipe Re-Export Prune

## Goal

Remove the loop-break builder/type re-export from the recipe-tree root facade.

This is BoxShape cleanup. It must not change loop-break recipe construction,
verification, matching, planner acceptance, or lowering behavior.

## Evidence

The loop-break builder pair is still re-exported from `recipe_tree/mod.rs`.
Its root-facade consumers are limited to:

- `loop_break_composer.rs`
- `matcher/patterns.rs`

Both can import directly from `loop_break_builder`.

## Decision

Move `build_loop_break_recipe` and `LoopBreakRecipe` imports to
`recipe_tree::loop_break_builder`.

Remove only this pair from the recipe-tree root facade.

## Boundaries

- Do not move builder implementation.
- Do not touch matcher control flow or recipe verification.
- Do not change loop-break facts or contracts.
- Do not prune other recipe-tree builder re-exports in this card.

## Acceptance

```bash
cargo fmt
cargo test loop_break --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Loop-break recipe callers import the builder pair from its owner module.
- `recipe_tree` root no longer re-exports the loop-break builder pair.
- Loop-break behavior is unchanged.
