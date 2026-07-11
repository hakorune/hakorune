---
Status: Landed
Date: 2026-04-29
Scope: narrow recipe matcher utility visibility
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/matcher/utils.rs
---

# 291x-680: Recipe Matcher Utils Visibility Prune

## Goal

Make recipe matcher utility functions visible only inside the `matcher` module.

This is BoxShape cleanup. It must not change matcher verification behavior,
planner acceptance, or lowering behavior.

## Evidence

`matcher/utils.rs` exposed helper functions as `pub(crate)`, but their callers
are limited to sibling matcher modules:

- `matcher/patterns.rs`
- `matcher/loop_cond.rs`
- `matcher/loop_scan.rs`

No caller outside `recipe_tree::matcher` needs these helpers.

## Decision

Change matcher utility functions from `pub(crate)` to `pub(super)`.

## Boundaries

- Do not change utility logic.
- Do not change matcher routing order.
- Do not change verifier entrypoints.
- Do not touch facts extraction or lowering.

## Acceptance

```bash
cargo fmt
cargo test recipe_tree --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Matcher helpers are no longer visible crate-wide.
- `RecipeMatcher` remains the route verification entrypoint.
