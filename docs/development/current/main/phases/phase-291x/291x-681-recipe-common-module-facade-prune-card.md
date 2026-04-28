---
Status: Landed
Date: 2026-04-29
Scope: prune direct recipe_tree common module imports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
  - src/mir/builder/control_flow/plan/recipe_tree/common.rs
---

# 291x-681: Recipe Common Module Facade Prune

## Goal

Route `ExitKind` and `IfMode` users through the `recipe_tree` root facade and
make the `common` module private.

This is BoxShape cleanup. It must not change recipe shape, verification,
planner acceptance, or lowering behavior.

## Evidence

`recipe_tree/mod.rs` already re-exports `ExitKind` and `IfMode`, but callers
still imported them from `recipe_tree::common`.

That left two public entry paths for the same vocabulary:

- `recipe_tree::{ExitKind, IfMode}`
- `recipe_tree::common::{ExitKind, IfMode}`

## Decision

Move callers to `recipe_tree::{ExitKind, IfMode}` and make `common` private.

## Boundaries

- Do not change `ExitKind` or `IfMode`.
- Do not move `common.rs`.
- Do not touch lowering or verifier logic.

## Acceptance

```bash
cargo fmt
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- `common` is no longer a public recipe-tree module entry.
- `ExitKind` and `IfMode` have a single public import path.
