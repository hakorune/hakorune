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

## Remaining Inventory

These are not immediate deletes without a small inventory card first:

| Surface | Status | Next action |
| --- | --- | --- |
| `if_dry_runner` | HOLD | Live dev caller in runner VM execution path. Keep unless dev route is retired. |
| Stage1/StageB lower-only routes | HOLD | Metadata now truthful as LowerOnly; do not delete while structural lowering probes exist. |
| `condition_pattern` | NEEDS-INVENTORY | Looks self-test heavy, but owns condition vocabulary. Confirm no active route semantics before pruning. |
| `condition_lowering_box` | NEEDS-RECONCILE | Trait exists around `ExprLowerer`; decide whether trait is SSOT or stale abstraction. |
| `condition_to_joinir` facade | HOLD/RECONCILE | Public alias surface is still imported by inline-boundary and expression lowerer paths. |
| `update_env` | NEEDS-INVENTORY | Appears test-only, but overlaps promoted-variable/body-local resolution. Compare with `ScopeManager` before deletion. |
| `JoinValueSpace` extra methods | TEST-SURFACE CANDIDATE | Production uses the core allocator path; narrow unused helpers only after a method-level inventory. |
| `common::dual_value_rewriter` and tiny common helpers | DELETE-CANDIDATE | Verify no current callers, then prune in a focused common-helper card. |

## Next Safe Pass

Start with a read-only inventory card for `condition_pattern` or `update_env`.
Do not mix it with bridge routing cleanup or live LowerOnly route changes.

## Proof

- `rg -n "complex_addend_normalizer|ComplexAddendNormalizer|continue_branch_normalizer|ContinueBranchNormalizer|digitpos_condition_normalizer|DigitPosConditionNormalizer" src tests -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
