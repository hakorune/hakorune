---
Status: Active
Date: 2026-04-04
Scope: keep archive / historical labeling minimal and explicit after the rust-vm source cleanup; rewrite archive README / wrapper wording so legacy traces read as historical evidence only.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-51x/README.md
  - tools/archive/legacy-selfhost/README.md
  - tools/archive/legacy-selfhost/compat-codegen/README.md
  - tools/archive/legacy-selfhost/compat-codegen/run_compat_pure_pack.sh
---

# Phase 52x: Archive Historical Labeling Polish

## Goal

- inventory archive-only rust-vm and historical wording surfaces
- rewrite archive README / wrapper text so they read as historical evidence only
- keep canonical historical traces minimal and explicit
- avoid widening archive evidence into a live owner lane

## Plain Reading

- active source cleanup already finished in phase-50x / phase-51x
- phase-52x only polishes archive/historical labels
- it does not restore any live owner lane

## Success Conditions

- archive README surfaces say historical / archived / proof-only only
- active source tree remains free of rust-vm wording
- canonical historical evidence remains available
- no new daily callers are introduced

## Failure Patterns

- archive docs start reading like live routes again
- historical evidence gets duplicated or widened
- compat/proof keep semantics are accidentally restated as current ownership

## Big Tasks

1. `52xA inventory historical evidence`
   - `52xA1` archive historical evidence inventory lock
   - `52xA2` archive README / wrapper wording rewrite
2. `52xB archive pack wording cleanup`
   - `52xB1` archive pack orchestrator wording cleanup
3. `52xC proof / closeout`
   - `52xC1` proof / closeout

## Boundaries

- leave canonical historical rust-vm evidence in archive
- no new live callers
- keep active source tree free of rust-vm wording
