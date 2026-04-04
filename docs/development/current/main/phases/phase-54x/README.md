---
Status: Active
Date: 2026-04-04
Scope: choose the next source lane after `phase-53x` landed so archive/historical cleanup does not become the de facto live default again.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-53x/README.md
  - docs/development/current/main/phases/phase-53x/53x-90-residual-vm-source-audit-ssot.md
  - docs/development/current/main/phases/phase-53x/53x-91-task-board.md
---

# Phase 54x: Next Source Lane Selection

## Goal

- inventory the candidate next source lanes after `phase-53x`
- rank the candidates by leverage, not by doc symmetry
- choose the next source lane without reopening rust-vm as a live owner

## Plain Reading

- `phase-53x` is landed and handed off.
- `rust-vm` is no longer the day-to-day owner, and `vm-hako` stays reference/conformance only.
- the current job is to decide the next source lane cleanly before any new work starts.
- `kilo` remains far-future; this phase is about the nearer next source focus, not a delayed optimization wave.

## Success Conditions

- candidate next lanes are inventoried and compared
- a concrete successor is selected with rationale
- current docs point at the new active lane instead of the landed audit lane
- `cargo check --bin hakorune` and `git diff --check` stay green

## Failure Patterns

- selecting a lane for doc symmetry instead of leverage
- reopening `--backend vm` / rust-vm as a day-to-day default
- leaving current mirrors pointed at a landed lane after handoff
- treating proof-only or compat keeps as if they were the next default producer

## Big Tasks

1. inventory candidate next source lanes
   - `54xA1` successor lane inventory lock
   - `54xA2` candidate lane ranking
2. select the successor lane
   - `54xB1` successor lane decision
3. prove and close the selection lane
   - `54xD1` proof / closeout
