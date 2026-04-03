---
Status: Landed
Date: 2026-04-03
Scope: choose the next source lane after `phase-45x` so VM residual cleanup does not become the de facto live default again.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-45x/README.md
  - docs/development/current/main/phases/phase-45x/45x-90-vm-residual-cleanup-ssot.md
  - docs/development/current/main/phases/phase-45x/45x-91-task-board.md
---

# Phase 46x: Next Source Lane Selection

## Goal

- inventory the remaining live VM pressure after `phase-45x`
- rank candidate next lanes by leverage, not by doc hygiene
- choose the next source lane without reopening direct/core mainline ownership

## Plain Reading

- `phase-45x` is landed; the residual rust-vm cleanup lane has been handed off.
- the current repo still has a few live VM defaults in helper routes, but most broad cleanup is already done.
- this phase selected `phase-47x stage0/runtime direct-core finalization` as the next highest-leverage lane.
- `vm core tail shrink` stays later, after the last helper-route defaults are drained.

## Success Conditions

- remaining VM surfaces are inventoried and compared against each other
- candidate lanes are ranked with a concrete recommendation
- `phase-47x stage0/runtime direct-core finalization` is selected cleanly
- current docs stay honest about what is active now versus what is far-future

## Failure Patterns

- treating proof-only VM gates as if they are the next default producer
- choosing a lane for doc symmetry instead of route ownership leverage
- leaving the current mirrors pointed at a landed lane while the active front has already moved

## Big Tasks

1. inventory remaining live VM surfaces and caller edges
2. shortlist candidate next lanes
3. select the successor lane with the highest leverage
4. close out the selection lane cleanly
