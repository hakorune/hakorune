# 291x-755 Cleanup ExitKind Vocabulary Reconcile Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `CleanupFacts`
- `CanonicalLoopFacts::cleanup_kinds_present`
- `coreloop-exitmap-composition-ssot.md`
- `CURRENT_STATE.toml`

## Why

The remaining `CleanupKindFacts::Return` slot duplicated the ExitKind vocabulary.
The active SSOT already says cleanup presence is expressed in ExitKind terms, so
keeping a separate one-variant enum made the structural hold look like a
half-implemented second truth source.

## Decision

Keep `CleanupFacts` as the cleanup presence slot, but store `ExitKindFacts`
directly.

Do not add cleanup producers in this card. This is a vocabulary reconciliation
only; release behavior stays unchanged.

## Landed

- Removed the duplicate `CleanupKindFacts` enum.
- Changed `CleanupFacts::kinds_present` to `BTreeSet<ExitKindFacts>`.
- Changed `CanonicalLoopFacts::cleanup_kinds_present` to use `ExitKindFacts`.
- Updated the canonical projection unit test to assert cleanup through
  `ExitKindFacts::Return`.
- Updated the CoreLoop ExitMap SSOT to forbid a separate cleanup-kind enum.

## Remaining Queue Impact

The CleanupKindFacts item is closed. Remaining structural cleanup is now:

- `LoopFacts::condition_shape`
- `SkeletonKind::{If2,BranchN}`
- bridge strict/env/LowerOnly semantics
- `condition_lowering_box` / `condition_to_joinir` / `update_env`

## Proof

- `rg -n "CleanupKindFacts" src/mir/builder/control_flow -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
