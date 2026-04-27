---
Status: Landed
Date: 2026-04-28
Scope: move recipes composer compat callers to direct plan owners and delete the facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-575-plan-compat-residue-inventory-card.md
  - src/mir/builder/control_flow/plan/composer/mod.rs
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
  - src/mir/builder/control_flow/recipes/mod.rs
---

# 291x-582: Recipes Composer Compat Prune

## Goal

Delete `recipes::composer_compat` after moving its callers to the direct
`plan::composer` and `plan::recipe_tree::RecipeComposer` owner paths.

This is BoxShape-only cleanup. It does not move implementation ownership or
change composer behavior.

## Evidence

`recipes/composer_compat.rs` was a pure facade over:

- `plan::composer::{compose_match_return_branchn, shadow_pre_plan_guard_error,
  strict_nested_loop_guard, try_compose_core_loop_v2_nested_minimal,
  MatchReturnPlan}`
- `plan::recipe_tree::RecipeComposer`

All live call sites only consumed those re-exports and did not rely on any
recipes-owned behavior.

## Boundaries

- Rewrite imports and namespace calls only.
- Delete the recipes-side facade after all callers move.
- Do not change plan composer or recipe-tree implementations.

## Acceptance

- No `recipes::composer_compat` re-export users remain.
- No `control_flow::recipes::{...RecipeComposer...}` compat imports remain.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Moved the eight live caller files to direct plan owner paths.
- Removed `recipes/composer_compat.rs`.
- Removed the dead re-export block from `recipes/mod.rs`.

## Verification

```bash
rg -n "compose_match_return_branchn|shadow_pre_plan_guard_error|strict_nested_loop_guard|try_compose_core_loop_v2_nested_minimal|MatchReturnPlan|RecipeComposer" src tests -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune
cargo fmt -- --check
git diff --check
```
