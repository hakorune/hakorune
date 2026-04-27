---
Status: Landed
Date: 2026-04-28
Scope: migrate plan-side common helper imports to facts owners and delete the wrapper shelf
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-575-plan-compat-residue-inventory-card.md
  - src/mir/builder/control_flow/facts/extractors/common_helpers/mod.rs
  - src/mir/builder/control_flow/facts/stmt_walk.rs
  - src/mir/builder/control_flow/plan/mod.rs
---

# 291x-579: Plan Extractors Common Helpers Owner Migration

## Goal

Move the remaining `plan::extractors::common_helpers` callers to their facts
owners, then delete the obsolete compatibility wrapper shelf.

This stays BoxShape-only. It does not change extractor behavior or route
acceptance.

## Evidence

All remaining plan-side imports were pure wrapper calls:

- extractor helpers from `facts::extractors::common_helpers`
- statement walkers from `facts::stmt_walk`

No plan-owned helper implementations remained behind the wrapper.

## Owner Mapping

```text
plan::extractors::common_helpers::{count_control_flow, ControlFlowDetector,
ControlFlowCounts, is_true_literal, extract_loop_increment_plan,
has_continue_statement, has_return_statement}
  -> facts::extractors::common_helpers::*

plan::extractors::common_helpers::{flatten_stmt_list, walk_stmt_list,
strip_trailing_continue_view}
  -> facts::stmt_walk::*
```

## Boundaries

- Rewrite imports only.
- Delete the wrapper shelf only after all callers move.
- Do not change helper semantics or route contracts.
- Leave `cond_block_view` and broader remaining compat seams to later cards.

## Acceptance

- No `plan::extractors::common_helpers` references remain in `src/` or `tests/`.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Migrated all remaining helper imports to facts owners.
- Deleted the now-unused `plan::extractors` wrapper shelf.
- Removed the obsolete `plan::extractors` module declaration from `plan/mod.rs`.

## Verification

```bash
rg -n "plan::extractors::common_helpers|control_flow::plan::extractors::common_helpers" src tests -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune
cargo fmt -- --check
git diff --check
```
