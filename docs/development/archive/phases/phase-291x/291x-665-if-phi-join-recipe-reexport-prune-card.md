---
Status: Landed
Date: 2026-04-28
Scope: prune if_phi_join recipe-tree re-export
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
  - src/mir/builder/control_flow/plan/recipe_tree/if_phi_join_composer.rs
  - src/mir/builder/control_flow/plan/recipe_tree/matcher/patterns.rs
---

# 291x-665: IfPhiJoin Recipe Re-Export Prune

## Goal

Remove the if-phi-join builder/type re-export from the recipe-tree root facade.

This is BoxShape cleanup. It must not change if-phi-join recipe construction,
verification, matching, planner acceptance, or lowering behavior.

## Evidence

After `291x-664`, the recipe-tree root facade still exposes builder pairs for
several recipe families. The if-phi-join pair has two root-facade consumers:

- `if_phi_join_composer.rs`
- `matcher/patterns.rs`

Both can import directly from `if_phi_join_builder`.

## Decision

Move `build_if_phi_join_recipe` and `IfPhiJoinRecipe` imports to
`recipe_tree::if_phi_join_builder`.

Remove only this pair from the recipe-tree root facade.

## Boundaries

- Do not move builder implementation.
- Do not touch matcher control flow or recipe verification.
- Do not change if-phi-join facts or recipe contracts.
- Do not prune other recipe-tree builder re-exports in this card.

## Acceptance

```bash
cargo fmt
cargo test if_phi_join --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- If-phi-join recipe callers import the builder pair from its owner module.
- `recipe_tree` root no longer re-exports the if-phi-join builder pair.
- If-phi-join behavior is unchanged.
