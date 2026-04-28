---
Status: Landed
Date: 2026-04-28
Scope: prune char_map recipe-tree re-export
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
  - src/mir/builder/control_flow/plan/recipe_tree/loop_simple_while_composer.rs
  - src/mir/builder/control_flow/plan/recipe_tree/matcher/patterns.rs
---

# 291x-673: CharMap Recipe Re-Export Prune

## Goal

Remove the char-map builder/type re-export from the recipe-tree root facade.

This is BoxShape cleanup. It must not change char-map recipe construction,
verification, matching, planner acceptance, or lowering behavior.

## Evidence

The char-map builder pair is still re-exported from `recipe_tree/mod.rs`.
Its root-facade consumers are limited to:

- `loop_simple_while_composer.rs`
- `matcher/patterns.rs`

Both can import directly from `char_map_builder`.

## Decision

Move `build_char_map_recipe` and `CharMapRecipe` imports to
`recipe_tree::char_map_builder`.

Remove only this pair from the recipe-tree root facade.

## Boundaries

- Do not move builder implementation.
- Do not touch matcher control flow or recipe verification.
- Do not change char-map facts or contracts.
- Do not prune other recipe-tree builder re-exports in this card.

## Acceptance

```bash
cargo fmt
cargo test char_map --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Char-map recipe callers import the builder pair from its owner module.
- `recipe_tree` root no longer re-exports the char-map builder pair.
- Char-map behavior is unchanged.
