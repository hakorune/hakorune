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

- lane: `phase-102 real-app read_quoted loop regression (VM + LLVM EXE)`
- current front: `MiniJsonLoader.read_quoted_from 最小抽出 fixture を VM/LLVM EXE parity で固定する`
- blocker: `none`
- recent landed:
  - `phase-100 Pinned Read-Only Captures`
  - `phase-99 Trim/escape 実コード寄り強化（VM+LLVM EXE）`
  - `phase-98 Plugin loader fail-fast + LLVM parityの持続化`
  - `phase-97 LLVM EXE parity for MiniJsonLoader fixtures`

## Current Read

- `phase-95` fixture E2E is green on VM (`apps/tests/phase95_json_loader_escape_min.hako`)
- `phase-96` fixture E2E is green on VM (`apps/tests/phase96_json_loader_next_non_ws_min.hako`)
- `stage1_mainline_smoke.sh` is green
- top-level wrappers remain public/front-door keep unless caller audit proves otherwise
- `phase-97` fixed LLVM EXE parity for `phase95/96` fixtures under `compat replay=harness`
- `phase-98` fixed plugin loader strict/best-effort runtime proof and kept LLVM EXE parity green
- `phase-99` trailing-backslash fixture is already green on both VM and LLVM EXE
- `phase-100` landed with pinned read-only captures and accumulator proof locked
- `phase-102` is the current lane: real-app `read_quoted_from` loop parity on VM and LLVM EXE

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-102/README.md`
