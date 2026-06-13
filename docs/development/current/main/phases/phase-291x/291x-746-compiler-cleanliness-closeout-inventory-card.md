# 291x-746 Compiler Cleanliness Closeout Inventory Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `docs/development/current/main/CURRENT_STATE.toml`
- this closeout inventory card

## Why

The latest cleanup burst removed the clearly dead JoinIR lowering and bridge
shelves through 291x-745. The remaining items are no longer simple dead-shelf
deletes; they need ownership or test-surface decisions before code changes.

This card records that boundary so the next pass does not restart from a broad
repo scan.

## Landed In This Burst

- Removed the direct FuncScanner append-defs lowerer shelf.
- Removed ExitMetaBuilder and gated InlineBoundaryBuilder as test-only.
- Removed unused JoinIR method-return type inference and exec-route helper
  surfaces.
- Synced `JOINIR_TARGETS` metadata with actual Exec vs LowerOnly behavior.
- Removed three unused AST rewrite normalizers:
  - complex addend
  - continue branch
  - DigitPos condition

## Current State

The lane is at a clean checkpoint:

- release lib-warning backlog remains zero
- `cargo test --lib --no-run` is warning-free for this lane
- quick gate is green, aside from the known chip8 release-artifact sync warning
- no uncommitted code cleanup is pending

## Supersession Note

This card is the historical inventory boundary after 291x-746, not the current
deletion queue. Later cards 291x-747 through 291x-775 worked through this
inventory, and `CURRENT_STATE.toml` owns the live latest-card pointer and next
blocker token.

## Remaining Inventory

These are not immediate deletes without a small inventory card first:

| Surface | Status | Next action |
| --- | --- | --- |
| `if_dry_runner` | HOLD | Live dev caller in runner VM execution path. Keep unless dev route is retired. |
| Stage1/StageB lower-only routes | HOLD | Metadata now truthful as LowerOnly; do not delete while structural lowering probes exist. |
| `condition_pattern` | RETIRED | Removed by 291x-748 after usage inventory showed no production caller; condition vocabulary now belongs to active route facts / `condition_lowerer` / `ExprLowerer`. |
| `condition_lowering_box` | RETIRED | Removed after usage inventory showed only the test-only trait harness. |
| `condition_to_joinir` facade | RETIRED | Removed after direct `condition_lowerer` tests covered the old facade surface. |
| `update_env` | RETIRED | Removed after promoted-variable/body-local resolution was covered through `ScopeManager` / `condition_lowerer` / `expr_lowerer` tests. |
| `JoinValueSpace` extra methods | RETIRED | Unused `alloc_join_param` / `alloc_join_local` wrappers removed; callers use `alloc_param` / `alloc_local` directly. |
| `common::dual_value_rewriter` | RETIRED | Already removed; no current source hits remain. |
| `common.rs` tiny helpers | HOLD | Current helper functions have active callers in target-specific lowerers / if-select / if-merge. |

## Next Safe Pass

Start with a read-only inventory card for another live surface.
Do not mix it with bridge routing cleanup or live LowerOnly route changes.

## Proof

- `rg -n "complex_addend_normalizer|ComplexAddendNormalizer|continue_branch_normalizer|ContinueBranchNormalizer|digitpos_condition_normalizer|DigitPosConditionNormalizer" src tests -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
