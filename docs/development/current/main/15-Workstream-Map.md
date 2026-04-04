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
| Now | `phase-81x caller-zero archive rerun` |
| Front | `81xA1 caller inventory rerun` |
| Blocker | `none` |
| Next | `81xA2 keep/archive candidate classification` |
| After Next | `81xB1 archive-ready sweep or no-op proof` |

## Current Read

- `phase-79x` closed the focused launcher blocker
- `launcher.hako emit_mir_mainline` is green
- `stage1_mainline_smoke.sh` is green
- `80x` is landed; pointer docs are thin again
- `81x` reruns caller-zero facts after the recuts settled

## Successor Corridor

1. `phase-81x caller-zero archive rerun`

## Structural Stop Lines

- `rust-vm`
  - mainline retirement: achieved
  - residual explicit keep: frozen
- `vm-hako`
  - reference/conformance keep

## Reference

- current lane docs:
  - `docs/development/current/main/phases/phase-79x/README.md`
  - `docs/development/current/main/phases/phase-81x/README.md`
  - `docs/development/current/main/phases/phase-81x/81x-90-caller-zero-archive-rerun-ssot.md`
  - `docs/development/current/main/phases/phase-81x/81x-91-task-board.md`
- previous landed lanes:
  - `docs/development/current/main/phases/phase-80x/README.md`
  - `docs/development/current/main/phases/phase-79x/README.md`
