---
Status: Landed
Date: 2026-04-29
Scope: prune recipe_tree verified module public entry
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
  - src/mir/builder/control_flow/plan/recipe_tree/verified.rs
  - src/mir/builder/control_flow/plan/parts/entry.rs
  - src/mir/builder/control_flow/plan/parts/verify.rs
  - src/mir/builder/control_flow/plan/parts/wiring_tests.rs
---

# 291x-684: Recipe Verified Module Facade Prune

## Goal

Keep the recipe verification implementation behind the `recipe_tree` root facade.

This is BoxShape cleanup. It must not change recipe verification, PortSig
semantics, route composition, or lowering behavior.

## Evidence

`verified` is the RecipeTree acceptance-gate implementation, but callers reached
through the module path directly:

- `recipe_tree::verified::check_block_contract`
- `recipe_tree::verified::VerifiedRecipeBlock`
- `recipe_tree::verified::ObligationState`
- `recipe_tree::verified::PortType`
- `recipe_tree::verified::verify_block_contract_with_pre`

That leaves two visible entrances: the root facade and the implementation
module.

## Decision

Make `verified` a private module and re-export the required gate surface through
`recipe_tree` root.

## Boundaries

- Do not change `check_block_contract` behavior.
- Do not change `verify_block_contract_with_pre` behavior.
- Do not change `VerifiedRecipeBlock` or PortSig semantics.
- Do not change lowering behavior.

## Acceptance

```bash
cargo fmt
cargo test recipe_tree --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- `verified` is no longer a public recipe-tree module entry.
- Verification callers now enter through the `recipe_tree` root facade.
