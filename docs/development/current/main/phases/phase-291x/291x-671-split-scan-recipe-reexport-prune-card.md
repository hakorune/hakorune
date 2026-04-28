---
Status: Landed
Date: 2026-04-28
Scope: prune split_scan recipe-tree re-export
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
  - src/mir/builder/control_flow/plan/recipe_tree/split_scan_composer.rs
  - src/mir/builder/control_flow/plan/recipe_tree/matcher/patterns.rs
---

# 291x-671: SplitScan Recipe Re-Export Prune

## Goal

Remove the split-scan builder/type re-export from the recipe-tree root facade.

This is BoxShape cleanup. It must not change split-scan recipe construction,
verification, matching, planner acceptance, or lowering behavior.

## Evidence

The split-scan builder pair is still re-exported from `recipe_tree/mod.rs`.
Its root-facade consumers are limited to:

- `split_scan_composer.rs`
- `matcher/patterns.rs`

Both can import directly from `split_scan_builder`.

## Decision

Move `build_split_scan_recipe` and `SplitScanRecipe` imports to
`recipe_tree::split_scan_builder`.

Remove only this pair from the recipe-tree root facade.

## Boundaries

- Do not move builder implementation.
- Do not touch matcher control flow or recipe verification.
- Do not change split-scan facts or contracts.
- Do not prune other recipe-tree builder re-exports in this card.

## Acceptance

```bash
cargo fmt
cargo test split_scan --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Split-scan recipe callers import the builder pair from its owner module.
- `recipe_tree` root no longer re-exports the split-scan builder pair.
- Split-scan behavior is unchanged.
