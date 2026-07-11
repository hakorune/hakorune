---
Status: Landed
Date: 2026-04-29
Scope: prune direct recipe_tree contracts module imports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
  - src/mir/builder/control_flow/plan/recipe_tree/contracts.rs
---

# 291x-682: Recipe Contract Module Facade Prune

## Goal

Route recipe contract type users through the `recipe_tree` root facade and make
the `contracts` module private.

This is BoxShape cleanup. It must not change planner matching, recipe
verification, or lowering behavior.

## Evidence

`RecipeMatcher` is active, but `contracts.rs` still carried a stale
"not wired into routing yet" comment and external callers imported contract
types through `recipe_tree::contracts`.

That left the contract vocabulary with a module-level public entrypoint instead
of a single root facade.

## Decision

Re-export contract vocabulary from `recipe_tree` root:

- `RecipeContract`
- `RecipeContractKind`
- `StmtConstraint`

Make `contracts` private and update callers to use the root facade.

## Boundaries

- Do not change contract variants or fields.
- Do not change matcher routing order.
- Do not change planner outcome semantics.

## Acceptance

```bash
cargo fmt
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Contract vocabulary has one external recipe-tree import path.
- `contracts` is no longer a public recipe-tree module entry.
- Stale contract wiring comments are removed.
