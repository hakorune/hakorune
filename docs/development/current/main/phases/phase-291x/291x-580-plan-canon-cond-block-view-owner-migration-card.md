---
Status: Landed
Date: 2026-04-28
Scope: migrate CondBlockView callers to the facts canon owner path and delete the plan-side wrapper
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-575-plan-compat-residue-inventory-card.md
  - src/mir/builder/control_flow/facts/canon/cond_block_view.rs
  - src/mir/builder/control_flow/plan/canon/mod.rs
---

# 291x-580: Plan Canon CondBlockView Owner Migration

## Goal

Move all remaining `plan::canon::cond_block_view::CondBlockView` users to the
facts canon owner path, then delete the obsolete plan-side wrapper.

This is a BoxShape-only owner migration. It does not change the
analysis-only `CondBlockView` contract.

## Evidence

The plan-side file was a pure re-export:

```text
plan/canon/cond_block_view.rs
  -> facts/canon/cond_block_view.rs::CondBlockView
```

All live call sites used the same symbol and needed the same owner-path
rewrite. No plan-owned behavior remained in the wrapper.

## Boundaries

- Rewrite `CondBlockView` owner paths only.
- Delete the wrapper file only after all live callers migrate.
- Do not change `CondBlockView` fields, constructors, or canon behavior.
- Leave remaining queue items (`ssa` wrappers and later boundary reviews) to
  their own cards.

## Acceptance

- No `plan::canon::cond_block_view::CondBlockView` references remain in `src/`
  or `tests/`.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Migrated all live `CondBlockView` callers to
  `facts::canon::cond_block_view::CondBlockView`.
- Deleted `plan/canon/cond_block_view.rs`.
- Removed the dead `plan::canon::cond_block_view` module declaration.

## Verification

```bash
rg -n "control_flow::plan::canon::cond_block_view::CondBlockView|plan::canon::cond_block_view::CondBlockView" src tests -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune
cargo fmt -- --check
git diff --check
```
