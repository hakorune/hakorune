---
Status: Landed
Date: 2026-04-28
Scope: prune recipe_tree builder item visibility
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/recipe_tree/*_builder.rs
---

# 291x-677: Recipe Builder Item Visibility Prune

## Goal

Make recipe builder entry functions and recipe carrier types visible only within
`recipe_tree`.

This is BoxShape cleanup. It must not change recipe construction,
verification, matching, planner acceptance, or lowering behavior.

## Evidence

After `291x-676`, builder modules are private to `recipe_tree`, but many
builder functions and recipe carrier types still declare broader
`pub(in crate::mir::builder)` or plain `pub` visibility.

Their consumers are only sibling `recipe_tree` modules:

- route composers
- `matcher/patterns.rs`

## Decision

Change recipe builder entry functions and recipe carrier structs to
`pub(super)`.

Keep recipe carrier fields as-is in this card; the type visibility now owns the
external surface.

## Boundaries

- Do not move builder implementations.
- Do not change recipe fields or recipe construction.
- Do not touch composer or matcher logic.
- Do not prune non-builder items.

## Acceptance

```bash
cargo fmt
cargo test recipe_tree --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Recipe builder entry functions are visible only inside `recipe_tree`.
- Recipe carrier types are visible only inside `recipe_tree`.
- Runtime behavior is unchanged.
