---
Status: Landed
Date: 2026-04-29
Scope: narrow recipe matcher verifier function visibility
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/matcher/patterns.rs
  - src/mir/builder/control_flow/plan/recipe_tree/matcher/loop_cond.rs
  - src/mir/builder/control_flow/plan/recipe_tree/matcher/loop_scan.rs
---

# 291x-679: Recipe Matcher Verifier Visibility Prune

## Goal

Make route-specific recipe verifier functions visible only to the parent
`matcher` module.

This is BoxShape cleanup. It must not change recipe matching, verification,
planner acceptance, or lowering behavior.

## Evidence

Route verifier functions in `matcher/patterns.rs`, `matcher/loop_cond.rs`, and
`matcher/loop_scan.rs` were plain `pub fn`.

Their only consumer is `matcher/mod.rs`, which imports child verifier modules
and calls them from `RecipeMatcher::try_match_loop`.

## Decision

Change route verifier functions from `pub fn` to `pub(super) fn`.

Keep matcher utilities unchanged in this card.

## Boundaries

- Do not change verifier logic.
- Do not change matcher routing order.
- Do not change `RecipeMatcher` visibility.
- Do not touch lowering or facts extraction.

## Acceptance

```bash
cargo fmt
cargo test recipe_tree --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Route verifier functions no longer expose a crate-wide public surface.
- `RecipeMatcher` remains the route verification entrypoint.
