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

- lane: `phase-105 digit OR-chain LLVM parity regression`
- current front: `long digit OR-chain under loop(true) を LLVM EXE parity で戻す`
- blocker: `LLVM EXE で long digit OR-chain が 0/1 に潰れる`
- recent landed:
  - `phase-104 loop(true) + break-only digits（read_digits 系）`
  - `phase-103 if-only regression baseline（VM + LLVM EXE）`
  - `phase-102 real-app read_quoted loop regression (VM + LLVM EXE)`
  - `phase-100 Pinned Read-Only Captures`
  - `phase-99 Trim/escape 実コード寄り強化（VM+LLVM EXE）`

## Current Read

- `phase-95` fixture E2E is green on VM (`apps/tests/phase95_json_loader_escape_min.hako`)
- `phase-96` fixture E2E is green on VM (`apps/tests/phase96_json_loader_next_non_ws_min.hako`)
- `stage1_mainline_smoke.sh` is green
- top-level wrappers remain public/front-door keep unless caller audit proves otherwise
- `phase-97` fixed LLVM EXE parity for `phase95/96` fixtures under `compat replay=harness`
- `phase-98` fixed plugin loader strict/best-effort runtime proof and kept LLVM EXE parity green
- `phase-99` trailing-backslash fixture is already green on both VM and LLVM EXE
- `phase-100` landed with pinned read-only captures and accumulator proof locked
- `phase-102` landed with real-app `read_quoted_from` loop parity on VM and LLVM EXE
- `phase-103` landed with if-only merge / early return parity on VM and LLVM EXE
- `phase-104` landed with a sequential-flag workaround for loop(true)+break-only digits parity
- `phase-105` is the current lane for the original long digit OR-chain parity regression
- after `phase-105`, execution naming cleanup is queued:
  - vocabulary split: `stage / route / backend override / lane / kernel`
  - route rename direction: `runtime-mode exe` -> `runtime-route mainline`
  - VM family lane names: `rust-vm-keep / vm-hako-reference / vm-compat-fallback`
  - `kernel` reserved for `nyash_kernel`; `lang/src/vm` treated as VM/reference cluster

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-105/README.md`
