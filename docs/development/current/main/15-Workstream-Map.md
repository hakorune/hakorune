---
Status: Active
Date: 2026-04-04
Scope: current mainline / next lane / parked corridor の one-screen map。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Workstream Map

## Current Lane

| Item | State |
| --- | --- |
| Now | `phase-83x selfhost top-level facade/archive decision` |
| Front | `83xA1 top-level facade inventory lock` |
| Blocker | `none` |
| Next | `83xA2 keep/archive decision freeze` |
| After Next | `83xB1 archive-ready sweep or explicit keep proof` |

## Current Read

- `launcher.hako emit_mir_mainline` is green
- `stage1_mainline_smoke.sh` is green
- `80x` is landed; pointer docs are thin again
- `81x` closed with a no-op archive sweep
- `82x` selected the top-level selfhost facade/archive decision lane
- `83x` classifies top-level selfhost wrappers into explicit keeps vs archive-ready aliases

## Successor Corridor

1. `phase-83x selfhost top-level facade/archive decision`

## Structural Stop Lines

- `rust-vm`
  - mainline retirement: achieved
  - residual explicit keep: frozen
- `vm-hako`
  - reference/conformance keep

## Reference

- current lane docs:
  - `docs/development/current/main/phases/phase-83x/README.md`
  - `docs/development/current/main/phases/phase-83x/83x-90-selfhost-top-level-facade-archive-decision-ssot.md`
  - `docs/development/current/main/phases/phase-83x/83x-91-task-board.md`
- previous landed lanes:
  - `docs/development/current/main/phases/phase-82x/README.md`
  - `docs/development/current/main/phases/phase-81x/README.md`
  - `docs/development/current/main/phases/phase-80x/README.md`
