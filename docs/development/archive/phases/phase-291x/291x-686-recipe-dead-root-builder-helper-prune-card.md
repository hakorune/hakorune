---
Status: Landed
Date: 2026-04-29
Scope: prune unused recipe_tree root builder helpers
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/mod.rs
---

# 291x-686: Recipe Dead Root Builder Helper Prune

## Goal

Keep the `recipe_tree` root facade limited to helpers that still have callers.

This is BoxShape cleanup. It must not change recipe structure, matching,
verification, or lowering behavior.

## Evidence

Repo-wide search showed these helpers were only referenced by their own
definitions:

- `build_if_v2_join_root`
- `build_arena_and_if_v2_join_root_from_single_if_stmt`
- `build_arena_and_loop_v0_root_from_single_loop_stmt`
- `build_arena_and_loop_v0_root_from_nested_stmt_only`

The live external root helper is still `build_stmt_only_block`, used by
`facts::stmt_view` and recipe builders.

## Decision

Remove the unused root builder helpers and their now-unused imports.

## Boundaries

- Keep `build_stmt_only_block`.
- Do not change route-specific recipe builders.
- Do not change matcher/verification/lowering behavior.

## Acceptance

```bash
cargo fmt
cargo test recipe_tree --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Unused root builder helpers are removed.
- `recipe_tree` root now exposes fewer construction entrypoints.
