---
Status: SSOT
Date: 2026-04-04
Scope: current lane / blocker / next pointer だけを置く薄い mirror。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Self Current Task — Now (main)

## Current

- lane: `phase-88x archive/deletion rerun`
- current front: `88xA1 archive-ready inventory lock`
- blocker: `none`
- recent landed:
  - `phase-86x phase index / current mirror hygiene`
  - `phase-87x embedded snapshot / wrapper repoint rerun`

## Current Read

- `launcher.hako emit_mir_mainline` is green
- `stage1_mainline_smoke.sh` is green
- `83x` froze top-level selfhost wrappers as explicit public/front-door keeps
- current work is rerunning archive/delete-ready inventory after the latest repoints

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-88x/README.md`
3. `docs/development/current/main/phases/phase-88x/88x-91-task-board.md`
