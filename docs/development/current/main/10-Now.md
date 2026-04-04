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

- lane: `phase-95 json_loader escape loop E2E lock`
- current front: `json_loader escape loop fixture / strict VM proof`
- blocker: `none`
- recent landed:
  - `phase-94 escape route P5b ch reassignment E2E`
  - `phase-93x archive-later engineering helper sweep`
  - `phase-92x selfhost proof/compat caller rerun`
  - `phase-91x top-level .hako wrapper policy review`
  - `phase-90x current-doc/design stale surface hygiene`

## Current Read

- `phase-94` fixture E2E is green (`tools/selfhost/test_pattern5b_escape_minimal.hako`)
- `stage1_mainline_smoke.sh` is green
- `93x` finished moving archive-later engineering helpers into `tools/archive/legacy-selfhost/engineering/`
- top-level wrappers remain public/front-door keep unless caller audit proves otherwise
- next fixture corridor is `95 -> 97`; `vm-hako` interpreter recut is parked until after optimization

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-95/README.md`
