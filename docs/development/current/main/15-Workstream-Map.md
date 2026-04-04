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
| Now | `phase-88x archive/deletion rerun` |
| Front | `88xA1 archive-ready inventory lock` |
| Blocker | `none` |
| Next | `88xA2 keep/archive/delete-ready classification` |
| After Next | `88xB1 focused archive/deletion sweep` |

## Current Read

- `launcher.hako emit_mir_mainline` is green
- `stage1_mainline_smoke.sh` is green
- `80x` is landed; pointer docs are thin again
- `81x` closed with a no-op archive sweep
- `83x` closed as an explicit keep proof for top-level selfhost wrappers
- `84x` landed after repointing Stage1 build/default entry contracts to canonical `entry/*`
- `85x` selected `86x` as the next structural source lane
- `86x` landed with a thinner phase index/current mirror surface
- `87x` landed after refreshing snapshot-pinned runner paths to canonical `facade/*` and `entry/*`
- `88x` reruns archive/delete-ready inventory after the latest repoints

## Successor Corridor

1. `phase-88x archive/deletion rerun`

## Structural Stop Lines

- `rust-vm`
  - mainline retirement: achieved
  - residual explicit keep: frozen
- `vm-hako`
  - reference/conformance keep

## Reference

- current lane docs:
  - `docs/development/current/main/phases/phase-88x/README.md`
  - `docs/development/current/main/phases/phase-88x/88x-90-archive-deletion-rerun-ssot.md`
  - `docs/development/current/main/phases/phase-88x/88x-91-task-board.md`
- previous landed lanes:
  - `docs/development/current/main/phases/phase-87x/README.md`
  - `docs/development/current/main/phases/phase-86x/README.md`
  - `docs/development/current/main/phases/phase-85x/README.md`
  - `docs/development/current/main/phases/phase-84x/README.md`
  - `docs/development/current/main/phases/phase-83x/README.md`
  - `docs/development/current/main/phases/phase-82x/README.md`
  - `docs/development/current/main/phases/phase-81x/README.md`
