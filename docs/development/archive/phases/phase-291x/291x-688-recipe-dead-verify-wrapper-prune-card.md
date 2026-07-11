---
Status: Landed
Date: 2026-04-29
Scope: prune unused recipe verified wrapper
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/verified.rs
---

# 291x-688: Recipe Dead Verify Wrapper Prune

## Goal

Keep RecipeTree verification entrypoints limited to live callers.

This is BoxShape cleanup. It must not change verification behavior or lowering
behavior.

## Evidence

`verify_block_contract` in `recipe_tree::verified` was a thin wrapper around
`verify_block_contract_with_pre(..., None)`, but repo-wide search showed no live
caller.

The live creation path is `verify_block_contract_with_pre`, called by
`parts::entry` so the lower entry can seed PortSig with current bindings.

## Decision

Remove the unused `verify_block_contract` wrapper and keep
`verify_block_contract_with_pre` as the creation entry.

## Boundaries

- Do not change `verify_block_contract_with_pre`.
- Do not change contract checks.
- Do not change PortSig construction.

## Acceptance

```bash
cargo fmt
cargo test recipe_tree --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- The unused verified wrapper is removed.
- RecipeTree verification has one live verified-block creation entry.
