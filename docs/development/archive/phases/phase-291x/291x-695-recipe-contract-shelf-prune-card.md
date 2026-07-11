---
Status: Landed
Date: 2026-04-29
Scope: recipe-tree cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/contracts.rs
  - src/mir/builder/control_flow/plan/recipe_tree/matcher/mod.rs
  - src/mir/builder/control_flow/plan/single_planner/rules.rs
---

# 291x-695: Recipe Contract Shelf Prune

## Why

`recipe_tree/contracts.rs` still carried Phase-A contract shelves that no longer
had any owner-path readers. The active recipe-first planner only consumes
`RecipeContractKind::LoopWithExit`, so the extra variants and metadata fields
were dead surface.

## Changes

- reduced `RecipeContractKind` to the active `LoopWithExit` variant only
- removed dead contract metadata types and fields:
  - `ExitRequirement`
  - `StmtConstraint`
  - `RecipeContract.required_exits`
  - `RecipeContract.allowed_stmts`
- trimmed matcher construction and `recipe_tree/mod.rs` exports accordingly
- removed the fossil fallback arm in `single_planner/rules.rs` that became
  unreachable after the contract shelf prune

## Result

- `cargo build --release` warning count moved from **36** to **32**
- the recipe contract surface now matches the active planner/recipe path

## Proof

```bash
cargo build --release
cargo test --release recipe_only_rule_is_single_plan_contract_only -- --nocapture
```
