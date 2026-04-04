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
| Now | `phase-85x next source lane selection` |
| Front | `85xA2 candidate lane ranking` |
| Blocker | `none` |
| Next | `85xB1 successor lane decision` |
| After Next | `85xD1 proof / closeout` |

## Current Read

- `launcher.hako emit_mir_mainline` is green
- `stage1_mainline_smoke.sh` is green
- `80x` is landed; pointer docs are thin again
- `81x` closed with a no-op archive sweep
- `83x` closed as an explicit keep proof for top-level selfhost wrappers
- `84x` landed after repointing Stage1 build/default entry contracts to canonical `entry/*`
- `85x` selects the next structural source lane

## Successor Corridor

1. `phase-85x next source lane selection`

## Structural Stop Lines

- `rust-vm`
  - mainline retirement: achieved
  - residual explicit keep: frozen
- `vm-hako`
  - reference/conformance keep

## Reference

- current lane docs:
  - `docs/development/current/main/phases/phase-85x/README.md`
  - `docs/development/current/main/phases/phase-85x/85x-90-next-source-lane-selection-ssot.md`
  - `docs/development/current/main/phases/phase-85x/85x-91-task-board.md`
- previous landed lanes:
  - `docs/development/current/main/phases/phase-84x/README.md`
  - `docs/development/current/main/phases/phase-83x/README.md`
  - `docs/development/current/main/phases/phase-82x/README.md`
  - `docs/development/current/main/phases/phase-81x/README.md`
