# Phase 137x: Main Kilo Reopen Selection

Status: Active
Scope: compact observe-only guardrail entry for phase-137x.
Moved full ledger to: `archive/README-full-ledger-2026-05-18.md`
Related:
- docs/development/current/main/CURRENT_STATE.toml
- docs/development/current/main/phases/phase-137x/137x-current.md
- docs/development/current/main/phases/phase-137x/137x-91-task-board.md
- docs/development/current/main/design/perf-owner-first-optimization-ssot.md

## Current Role

Phase 137x is not the active implementation lane. It remains an observe-only
optimization guardrail while `active_lane` in `CURRENT_STATE.toml` points
elsewhere.

Use `CURRENT_STATE.toml` for the current active lane and blocker. Use
`137x-current.md` for the compact phase-137x dashboard, and use the archived
full ledger only when historical optimization evidence is needed.

## Guard Tokens

- current-state active lane: read `active_lane` in `CURRENT_STATE.toml`
- current-state blocker: read `current_blocker_token` in `CURRENT_STATE.toml`
- phase-137x lane: `phase-137x observe-only guardrail`
- active taskboard: `137x-91-task-board.md`
- optimization return lane: `phase-137x observe-only guardrail`

## Stop Lines

- Do not append landed history here.
- Do not reopen kilo optimization work from this file alone.
- Do not treat phase-137x as a prerequisite for the current implementation row unless
  `CURRENT_STATE.toml` reopens a real blocker.
