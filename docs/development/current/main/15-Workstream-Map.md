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
| Now | `phase-90x current-doc/design stale surface hygiene` |
| Front | `90xA1 stale surface inventory lock` |
| Blocker | `none` |
| Next | `90xA2 target split / stop-line freeze` |
| After Next | `90xB1 current/design stale surface cleanup` |

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
- `88x` landed as a no-op archive/deletion rerun
- `89x` selected `90x current-doc/design stale surface hygiene`
- `90x` thins stale wrapper/current wording in current/design docs after the latest recuts

## Successor Corridor

1. `phase-90x current-doc/design stale surface hygiene`
2. `phase-91x top-level .hako wrapper policy review`
3. `phase-92x selfhost proof/compat caller rerun`

## Structural Stop Lines

- `rust-vm`
  - mainline retirement: achieved
  - residual explicit keep: frozen
- `vm-hako`
  - reference/conformance keep

## Reference

- current lane docs:
  - `docs/development/current/main/phases/phase-90x/README.md`
  - `docs/development/current/main/phases/phase-90x/90x-90-current-doc-design-stale-surface-hygiene-ssot.md`
  - `docs/development/current/main/phases/phase-90x/90x-91-task-board.md`
- previous landed lanes:
  - `docs/development/current/main/phases/phase-88x/README.md`
  - `docs/development/current/main/phases/phase-87x/README.md`
  - `docs/development/current/main/phases/phase-86x/README.md`
  - `docs/development/current/main/phases/phase-85x/README.md`
  - `docs/development/current/main/phases/phase-84x/README.md`
  - `docs/development/current/main/phases/phase-83x/README.md`
  - `docs/development/current/main/phases/phase-82x/README.md`
  - `docs/development/current/main/phases/phase-81x/README.md`
