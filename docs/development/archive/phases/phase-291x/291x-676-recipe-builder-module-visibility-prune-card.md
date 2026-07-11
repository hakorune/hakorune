---
Status: Landed
Date: 2026-04-28
Scope: prune recipe_tree builder module visibility
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
---

# 291x-676: Recipe Builder Module Visibility Prune

## Goal

Make recipe-tree builder modules private to `recipe_tree` after removing the
root builder re-export facade.

This is BoxShape cleanup. It must not change recipe construction,
verification, matching, planner acceptance, or lowering behavior.

## Evidence

The remaining builder module consumers are all `recipe_tree` descendants:

- route composers
- `matcher/patterns.rs`

No consumer outside `recipe_tree` needs direct module access after the
291x-664 through 291x-675 re-export pruning sequence.

## Decision

Change the builder module declarations from
`pub(in crate::mir::builder) mod ..._builder` to private `mod ..._builder`.

Keep builder item visibility unchanged for now; the module boundary now owns
the external surface.

## Boundaries

- Do not move builder implementations.
- Do not change builder function/type visibility in this card.
- Do not touch composer or matcher logic.
- Do not prune non-builder recipe-tree modules.

## Acceptance

```bash
cargo fmt
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Recipe builder modules are private to `recipe_tree`.
- External recipe-tree builder access remains closed.
- Runtime behavior is unchanged.
