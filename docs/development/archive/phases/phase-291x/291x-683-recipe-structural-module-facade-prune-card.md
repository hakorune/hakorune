---
Status: Landed
Date: 2026-04-29
Scope: prune recipe_tree structural module public entries
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
  - src/mir/builder/control_flow/plan/parts/dispatch/if_join.rs
---

# 291x-683: Recipe Structural Module Facade Prune

## Goal

Keep RecipeTree structural vocabulary behind the `recipe_tree` root facade.

This is BoxShape cleanup. It must not change recipe structure, verification, or
lowering behavior.

## Evidence

`block` was public even though its structural types are already re-exported
from `recipe_tree` root.

`join_scope` exposed one helper through a public module path:

- `recipe_tree::join_scope::collect_branch_local_vars_from_block_recursive`

## Decision

Make `block` and `join_scope` private modules.

Re-export `collect_branch_local_vars_from_block_recursive` through the
`recipe_tree` root facade and update its caller.

## Boundaries

- Do not change RecipeBlock/RecipeItem/BodyId definitions.
- Do not change join-scope collection logic.
- Do not touch lowering behavior.

## Acceptance

```bash
cargo fmt
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- `block` and `join_scope` are no longer public recipe-tree module entries.
- Structural recipe vocabulary is reached through the root facade.
