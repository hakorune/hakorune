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
| Now | `phase-84x runner wrapper/source contract thinning` |
| Front | `84xB1 wrapper/source thinning` |
| Blocker | `none` |
| Next | `84xC1 proof refresh` |
| After Next | `84xD1 closeout` |

## Current Read

- `launcher.hako emit_mir_mainline` is green
- `stage1_mainline_smoke.sh` is green
- `80x` is landed; pointer docs are thin again
- `81x` closed with a no-op archive sweep
- `83x` closed as an explicit keep proof for top-level selfhost wrappers
- `84x` thins the remaining top-level `.hako` wrapper/source pressure

## Successor Corridor

1. `phase-84x runner wrapper/source contract thinning`

## Structural Stop Lines

- `rust-vm`
  - mainline retirement: achieved
  - residual explicit keep: frozen
- `vm-hako`
  - reference/conformance keep

## Reference

- current lane docs:
  - `docs/development/current/main/phases/phase-84x/README.md`
  - `docs/development/current/main/phases/phase-84x/84x-90-runner-wrapper-source-contract-thinning-ssot.md`
  - `docs/development/current/main/phases/phase-84x/84x-91-task-board.md`
- previous landed lanes:
  - `docs/development/current/main/phases/phase-83x/README.md`
  - `docs/development/current/main/phases/phase-82x/README.md`
  - `docs/development/current/main/phases/phase-81x/README.md`
  - `docs/development/current/main/phases/phase-80x/README.md`
