# 291x-752 SplitScanFacts Shape Slot Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/builder/control_flow/plan/facts/loop_types.rs`
- `src/mir/builder/control_flow/plan/facts/loop_split_scan.rs`
- `src/mir/builder/control_flow/plan/facts/scan_shapes.rs`
- old cfg-test coreloop composer fixtures that constructed split-scan facts
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

Worker inventory classified `SplitScanFacts::shape` as a test-only candidate.
Fresh usage confirmed the field was always `SplitScanShape::Minimal`; current
release routing only needs `split_scan: Some(...)` and the variables inside the
fact. The separate shape enum did not add production information.

## Decision

Remove the split-scan shape slot and the single-variant `SplitScanShape` enum.

Do not change split-scan acceptance or lowering semantics.

## Landed

- Removed `SplitScanFacts::shape`.
- Removed `SplitScanShape`.
- Removed the old cfg-test gate that checked `SplitScanShape::Minimal`.
- Updated split-scan fixtures/tests to construct the smaller fact.

## Remaining Queue Impact

`SplitScanFacts::shape` is no longer in the structural/test-surface queue.
Remaining planner facts:

- `LoopFacts::condition_shape`
- `CleanupKindFacts::Return`
- `SkeletonKind::{If2,BranchN}`
- old coreloop v0/v1 composer test surface

## Proof

- `rg -n "SplitScanShape|split_scan\\.shape|shape: SplitScanShape::Minimal" src/mir/builder/control_flow/plan -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
