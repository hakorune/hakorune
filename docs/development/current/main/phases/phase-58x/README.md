---
Status: Landed
Date: 2026-04-04
Scope: select the successor source lane after phase-57x closed without a broad rust-vm deletion wave.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-57x/README.md
  - docs/development/current/main/phases/phase-57x/57x-90-rust-vm-delete-ready-audit-removal-wave-ssot.md
  - docs/development/current/main/phases/phase-57x/57x-91-task-board.md
---

# Phase 58x: Next Source Lane Selection

## Goal

- turn the phase-57x audit result into an explicit successor lane
- choose the next highest-leverage source lane without reopening rust-vm as a live owner
- keep `vm-hako` outside this corridor as reference/conformance keep

## Plain Reading

- `phase-57x` proved that the remaining rust-vm source surfaces are still explicit keeps.
- no broad rust-vm deletion wave landed.
- `phase-58x` now decides what to do next instead of forcing a fake removal.

## Candidate Directions

- rust-vm route-surface retirement continuation
- proof/compat keep pruning continuation
- successor selection closeout if no source deletion lane is justified yet

## First Reading

- route-surface continuation is leading because explicit backend affordances and the `stage-a` compat branch still form the largest remaining re-growth surface.
- keep-pruning continuation stays second because its target set is narrower and already explicit.
- delete-ready rerun stays third because `phase-57x` produced no new caller-zero target.

## Decision

- `phase-59x rust-vm route-surface retirement continuation` is the selected successor lane.

## Outcome

- successor inventory and ranking are locked
- `phase-59x rust-vm route-surface retirement continuation` is selected as the highest-leverage next lane

## Success Conditions

- the successor lane is ranked and selected
- the selection is consistent with the `keep-now / archive-later / delete-ready` outcome from `phase-57x`
- `cargo check --bin hakorune` and `git diff --check` stay green

## Big Tasks

1. inventory and rank the successor lane
   - `58xA1` successor lane inventory lock
   - `58xA2` candidate lane ranking
2. decide the next lane
   - `58xB1` successor lane decision
3. prove and close
   - `58xD1` proof / closeout
