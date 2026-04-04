---
Status: SSOT
Date: 2026-04-05
Scope: current lane / blocker / next pointer だけを置く薄い mirror。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Self Current Task — Now (main)

## Current

- lane: `phase-94 escape route P5b “完全E2E” のための ch 再代入対応`
- current front: `TBD`
- blocker: `none`
- recent landed:
  - `phase-93x archive-later engineering helper sweep`
  - `phase-92x selfhost proof/compat caller rerun`
  - `phase-91x top-level .hako wrapper policy review`
  - `phase-90x current-doc/design stale surface hygiene`
  - `phase-89x next source lane selection`
  - `phase-88x archive/deletion rerun`

## Current Read

- `launcher.hako emit_mir_mainline` is green
- `stage1_mainline_smoke.sh` is green
- `93x` finished moving archive-later engineering helpers into `tools/archive/legacy-selfhost/engineering/`
- current cleanup is thin; the next lane is the existing `phase-94` task

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-94/README.md`
