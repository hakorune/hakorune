---
Status: Landed
Date: 2026-04-29
Scope: clean stale recipe_tree root comments
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
---

# 291x-678: RecipeTree Stale Comment Cleanup

## Goal

Refresh stale comments in `recipe_tree/mod.rs` so the root module describes the
current compiler surface.

This is docs/comment cleanup. It must not change recipe construction,
verification, matching, planner acceptance, or lowering behavior.

## Evidence

`recipe_tree/mod.rs` still described RecipeTree as an `M1 scaffold` with
`no calls from existing pipeline`.

That is no longer true: route handlers use `RecipeComposer`, matcher
verification is active, and the builder re-export facade has been closed.

## Decision

Update the module comments to describe the current shape:

- RecipeTree owns the recursive lowering vocabulary.
- Builder modules are private.
- Composer modules expose behavior through `RecipeComposer`.
- Shared block helpers are local recipe construction helpers.

## Boundaries

- Do not change Rust item visibility.
- Do not move modules.
- Do not change recipe construction or verifier behavior.

## Acceptance

```bash
cargo fmt
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Stale scaffold comments are removed.
- The RecipeTree root now describes the active compiler surface.
