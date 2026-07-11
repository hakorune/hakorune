# 291x-728 Case-A Shape Compat Wrapper Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/loop_scope_shape/case_a_lowering_shape.rs`

## Why

`CaseALoweringShape::detect()` was a deprecated compatibility wrapper that
reconstructed partial `LoopFeatures` from `LoopScopeShape`. No live code called
it anymore; only a unit test kept the wrapper alive.

`CaseALoweringShape::is_recognized()` was also test-only surface. The production
consumer only needs `detect_from_features()` plus `name()` for debug tracing.

## Decision

Keep the structure-first `detect_from_features()` API as the only Case-A shape
detection entry. Do not keep a LoopScopeShape-derived compatibility wrapper that
can lose route facts.

## Changes

- Removed the deprecated `CaseALoweringShape::detect()` wrapper.
- Removed the test-only `is_recognized()` helper.
- Removed unit-test scaffolding that existed only to exercise the deleted
  compatibility wrapper.

## Proof

- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `rg -n "CaseALoweringShape::detect\\(|\\.is_recognized\\(\\)|allow\\(deprecated\\).*CaseALoweringShape|Use detect_from_features" src tests -g '*.rs'`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
