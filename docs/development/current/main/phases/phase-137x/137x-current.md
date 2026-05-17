# Phase 137x Current Dashboard

Status: Active
Scope: compact dashboard for the observe-only phase-137x optimization lane.
Moved full ledger to: `archive/137x-current-full-ledger-2026-05-18.md`
Related:
- docs/development/current/main/phases/phase-137x/README.md
- docs/development/current/main/phases/phase-137x/137x-91-task-board.md
- docs/development/current/main/design/perf-owner-first-optimization-ssot.md

## Current State

- phase-137x is observe-only unless `CURRENT_STATE.toml` reopens a real app or
  optimization blocker.
- active implementation work remains in `phase-293x mimalloc blueprint lane`.
- exact historical evidence and rejected optimization details live in the
  archived full ledger.

## Use This File For

- remembering that phase-137x is parked / observe-only;
- finding the archived evidence ledger;
- linking the perf owner-first SSOT and taskboard.

## Do Not Use This File For

- active mimalloc row selection;
- landed-history accumulation;
- reopening optimization work without current-state evidence.
