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

- lane: `phase-83x selfhost top-level facade/archive decision`
- current front: `83xA1 top-level facade inventory lock`
- blocker: `none`
- recent landed:
  - `phase-81x caller-zero archive rerun`
  - `phase-82x next source lane selection`

## Current Read

- `launcher.hako emit_mir_mainline` is green
- `stage1_mainline_smoke.sh` is green
- `82x` selected the top-level selfhost facade/archive decision lane
- current work is classifying top-level `tools/selfhost/*` wrappers into keep vs archive-ready

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-83x/README.md`
3. `docs/development/current/main/phases/phase-83x/83x-91-task-board.md`
