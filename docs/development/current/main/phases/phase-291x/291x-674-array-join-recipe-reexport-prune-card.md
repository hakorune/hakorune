---
Status: Landed
Date: 2026-04-28
Scope: prune array_join recipe-tree re-export
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
  - src/mir/builder/control_flow/plan/recipe_tree/loop_simple_while_composer.rs
  - src/mir/builder/control_flow/plan/recipe_tree/matcher/patterns.rs
---

# 291x-674: ArrayJoin Recipe Re-Export Prune

## Goal

Remove the array-join builder/type re-export from the recipe-tree root facade.

This is BoxShape cleanup. It must not change array-join recipe construction,
verification, matching, planner acceptance, or lowering behavior.

## Evidence

The array-join builder pair is still re-exported from `recipe_tree/mod.rs`.
Its root-facade consumers are limited to:

- `loop_simple_while_composer.rs`
- `matcher/patterns.rs`

Both can import directly from `array_join_builder`.

## Decision

Move `build_array_join_recipe` and `ArrayJoinRecipe` imports to
`recipe_tree::array_join_builder`.

Remove only this pair from the recipe-tree root facade.

## Boundaries

- Do not move builder implementation.
- Do not touch matcher control flow or recipe verification.
- Do not change array-join facts or contracts.
- Do not prune other recipe-tree builder re-exports in this card.

## Acceptance

```bash
cargo fmt
cargo test array_join --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Array-join recipe callers import the builder pair from its owner module.
- `recipe_tree` root no longer re-exports the array-join builder pair.
- Array-join behavior is unchanged.
