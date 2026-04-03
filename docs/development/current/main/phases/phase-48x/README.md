---
Status: Active
Date: 2026-04-04
Scope: clean up the remaining rust-vm-facing smoke/source surfaces, classify proof-only keeps and compat keeps, and remove stale day-to-day `--backend vm` references from docs and scripts.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-48x/48x-90-smoke-source-cleanup-ssot.md
  - docs/development/current/main/phases/phase-48x/48x-91-task-board.md
---

# Phase 48x: Smoke/Source Cleanup

## Goal

- inventory the remaining rust-vm-facing smoke/source routes
- keep proof-only and compat routes explicit and non-growing
- remove stale day-to-day `--backend vm` wording from scripts, source comments, and examples
- preserve `cargo check --bin hakorune` and the explicit proof gates

## Plain Reading

- `phase-47x` finished direct/core finalization.
- `48xA1` locked the inventory of remaining VM-facing smoke/source surfaces.
- the remaining `--backend vm` references are mostly proof-only or compat keeps, plus stale doc/example noise.
- this lane classifies what stays, what is proof-only, and what should be archived or rewritten.

## Success Conditions

- day-to-day smoke/source paths no longer imply `--backend vm` defaults
- proof-only and compat keeps are explicit
- docs/examples match the current direct/core + compat split
- `cargo check --bin hakorune` stays green

## Failure Patterns

- deleting proof/compat keeps before replacement or classification
- letting docs/examples drift behind code
- widening compat keeps while cleaning

## Big Tasks

1. `48xA` inventory and classify
   - `48xA1` residual vm surface inventory lock (landed)
   - `48xA2` proof-only / compat keep classification (landed)
2. `48xB` smoke cleanup
   - `48xB1` smoke script stale-route cleanup (landed)
   - `48xB2` proof-only smoke gate lock (landed)
3. `48xC` source cleanup
   - `48xC1` source helper stale-route cleanup (landed)
   - `48xC2` vm.rs / vm_fallback thin keep trim (active)
4. `48xD` docs/examples cleanup
   - `48xD1` README/example command cleanup
   - `48xD2` stale `--backend vm` commentary cleanup
5. `48xE` proof / closeout
   - `48xE1` proof / closeout
