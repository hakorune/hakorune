---
Status: Landed
Date: 2026-04-28
Scope: migrate small plan/facts wrapper callers to facts owner paths
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-575-plan-compat-residue-inventory-card.md
  - src/mir/builder/control_flow/facts/expr_bool.rs
  - src/mir/builder/control_flow/facts/no_exit_block.rs
  - src/mir/builder/control_flow/facts/stmt_view.rs
  - src/mir/builder/control_flow/plan/facts/mod.rs
---

# 291x-578: Small Plan Facts Owner-Path Migration

## Goal

Move the small live `plan::facts` callers for `expr_bool`, `no_exit_block`, and
`stmt_view` onto the facts owner modules, then delete the obsolete wrappers.

This is still BoxShape-only cleanup. It does not change the facts contracts or
accept new shapes.

## Evidence

Before the migration:

- `plan::facts::expr_bool` had one live caller
- `plan::facts::no_exit_block` had seven live callers
- `plan::facts::stmt_view` had nine live callers

The caller set was local to plan-owned features, parts, generic-loop facts, and
recipe-tree helpers, so the migration stayed mechanical.

## Cleaner Boundary

```text
facts/expr_bool.rs
facts/no_exit_block.rs
facts/stmt_view.rs
  own the reusable Facts contracts

plan/facts/*
  no longer mirrors those modules
```

## Boundaries

- Change owner paths only.
- Delete wrappers only after all live callers are migrated.
- Do not change contract semantics, recipe structure, or lowering policy.
- Leave broader `plan::extractors::common_helpers` and `cond_block_view`
  migrations to their own cards.

## Acceptance

- No `plan::facts::expr_bool` references remain.
- No `plan::facts::no_exit_block` references remain.
- No `plan::facts::stmt_view` references remain.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Migrated the remaining small live callers to facts owner paths.
- Removed `plan/facts/expr_bool.rs`.
- Removed `plan/facts/no_exit_block.rs`.
- Removed `plan/facts/stmt_view.rs`.
- Removed the deleted wrapper module declarations from `plan/facts/mod.rs`.

## Verification

```bash
rg -n "control_flow::plan::facts::expr_bool|plan::facts::expr_bool|control_flow::plan::facts::no_exit_block|plan::facts::no_exit_block|control_flow::plan::facts::stmt_view|plan::facts::stmt_view" src tests -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune
cargo fmt -- --check
git diff --check
```
