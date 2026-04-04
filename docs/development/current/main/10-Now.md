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

- lane: `phase-97 LLVM EXE parity for MiniJsonLoader fixtures`
- current front: `escape / next_non_ws fixture parity under LLVM EXE`
- blocker: `compile pin is in place; LLVM EXE runtime still returns wrong output for phase95/96 fixtures`
- recent landed:
  - `phase-96 MiniJsonLoader next_non_ws loop E2E lock`
  - `phase-95 json_loader escape loop E2E lock`
  - `phase-94 escape route P5b ch reassignment E2E`
  - `phase-93x archive-later engineering helper sweep`
  - `phase-92x selfhost proof/compat caller rerun`

## Current Read

- `phase-95` fixture E2E is green on VM (`apps/tests/phase95_json_loader_escape_min.hako`)
- `phase-96` fixture E2E is green on VM (`apps/tests/phase96_json_loader_next_non_ws_min.hako`)
- `stage1_mainline_smoke.sh` is green
- `93x` finished moving archive-later engineering helpers into `tools/archive/legacy-selfhost/engineering/`
- top-level wrappers remain public/front-door keep unless caller audit proves otherwise
- `phase-97` compile blocker is narrowed to `compat replay=harness`; remaining blocker is LLVM EXE runtime parity

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-97/README.md`
