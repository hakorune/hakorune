# 291x-753 ScanDirection Test Helper Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/builder/control_flow/plan/domain.rs`
- `src/mir/builder/control_flow/plan/mod.rs`
- `src/mir/builder/control_flow/plan/composer/coreloop_single_entry.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

`ScanDirection` and `scan_direction_from_step_lit` were cfg-test-only helpers
used by the old coreloop single-entry composer. The active check only needed to
know whether `step_lit` was `1` or `-1`; the enum did not carry additional
contract information.

## Decision

Remove the test-only helper and inline the small predicate at the old composer
test surface.

Keep `LoopBreakStepPlacement` and other domain vocabulary because they are live
in loop-break facts and recipe lowering.

## Landed

- Removed `ScanDirection`.
- Removed `scan_direction_from_step_lit`.
- Removed the cfg-test re-export from `plan/mod.rs`.
- Replaced the old composer gate with `matches!(scan.step_lit, 1 | -1)`.

## Remaining Queue Impact

The standalone scan-direction helper shelf is closed. Remaining planner cleanup
is structural:

- `LoopFacts::condition_shape`
- `CleanupKindFacts::Return`
- `SkeletonKind::{If2,BranchN}`
- old coreloop v0/v1 composer test surface

## Proof

- `rg -n "ScanDirection|scan_direction_from_step_lit" src/mir/builder/control_flow/plan -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
