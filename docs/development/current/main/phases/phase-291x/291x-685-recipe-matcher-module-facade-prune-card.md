---
Status: Landed
Date: 2026-04-29
Scope: prune recipe_tree matcher module public entry
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
---

# 291x-685: Recipe Matcher Module Facade Prune

## Goal

Keep matcher implementation modules behind the `recipe_tree` root facade.

This is BoxShape cleanup. It must not change recipe matching or planner routing
behavior.

## Evidence

External callers already use the root facade type:

- `recipe_tree::RecipeMatcher`

No external caller needs the implementation module path:

- `recipe_tree::matcher`

## Decision

Make `matcher` a private module and keep `RecipeMatcher` re-exported through the
`recipe_tree` root facade.

## Boundaries

- Do not change matcher methods.
- Do not change recipe contracts.
- Do not change planner routing.

## Acceptance

```bash
cargo fmt
cargo test recipe_tree --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- `matcher` is no longer a public recipe-tree module entry.
- Planner callers continue to use `recipe_tree::RecipeMatcher`.
