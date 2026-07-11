---
Status: Landed
Date: 2026-04-29
Scope: prune recipe root check-only verification export
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
  - src/mir/builder/control_flow/plan/recipe_tree/verified.rs
---

# 291x-689: Recipe Check Contract Root Prune

## Goal

Keep check-only recipe verification internal to RecipeTree.

This is BoxShape cleanup. It must not change contract checking behavior or
planner routing behavior.

## Evidence

`check_block_contract` was exported from the `recipe_tree` root facade, but all
callers were inside `recipe_tree` composer/matcher modules.

External lowering callers use `VerifiedRecipeBlock` creation through
`verify_block_contract_with_pre`; they do not need the check-only helper.

## Decision

Remove `check_block_contract` from the root facade and let RecipeTree internals
call `verified::check_block_contract` directly.

## Boundaries

- Do not change `check_block_contract` behavior.
- Do not change matcher/composer acceptance.
- Do not change verified-block creation.

## Acceptance

```bash
cargo fmt
cargo test recipe_tree --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- `check_block_contract` is no longer exported from `recipe_tree` root.
- Check-only verification remains an internal RecipeTree implementation detail.
