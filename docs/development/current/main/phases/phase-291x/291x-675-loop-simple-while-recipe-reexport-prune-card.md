---
Status: Landed
Date: 2026-04-28
Scope: prune loop_simple_while recipe-tree re-export
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
  - src/mir/builder/control_flow/plan/recipe_tree/loop_simple_while_composer.rs
  - src/mir/builder/control_flow/plan/recipe_tree/matcher/patterns.rs
---

# 291x-675: LoopSimpleWhile Recipe Re-Export Prune

## Goal

Remove the loop-simple-while builder/type re-export from the recipe-tree root
facade.

This is BoxShape cleanup. It must not change loop-simple-while recipe
construction, verification, matching, planner acceptance, or lowering behavior.

## Evidence

The loop-simple-while builder pair is still re-exported from
`recipe_tree/mod.rs`. Its root-facade consumers are limited to:

- `loop_simple_while_composer.rs`
- `matcher/patterns.rs`

Both can import directly from `loop_simple_while_builder`.

## Decision

Move `build_loop_simple_while_recipe` and `LoopSimpleWhileRecipe` imports to
`recipe_tree::loop_simple_while_builder`.

Remove the final builder pair from the recipe-tree root facade and delete the
now-empty compatibility re-export section.

## Boundaries

- Do not move builder implementation.
- Do not touch matcher control flow or recipe verification.
- Do not change loop-simple-while facts or contracts.
- Do not prune unrelated recipe-tree exports.

## Acceptance

```bash
cargo fmt
cargo test loop_simple_while --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Loop-simple-while recipe callers import the builder pair from its owner
  module.
- `recipe_tree` root no longer re-exports recipe builder pairs.
- Loop-simple-while behavior is unchanged.
