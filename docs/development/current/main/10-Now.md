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

- lane: `phase-122x vm compat route exit plan`
- current front: `runtime-route compat / stage-a-compat / raw --backend vm の依存をどの順で外すかを具体化する`
- blocker: `none`
- recent landed:
  - `phase-121x vm backend retirement gate decision`
  - `phase-120x vm route retirement decision refresh`
  - `phase-119x vm debug/observability surface review`
  - `phase-118x proof wrapper surface review`
  - `phase-117x vm compat/proof env hardening`
  - `phase-116x execution surface alias pruning`
  - `phase-115x vm route retirement planning`
  - `phase-114x execution surface wording closeout`
  - `phase-113x kernel vs vm-reference cluster wording correction`
  - `phase-112x vm-family lane naming hardening`
  - `phase-111x selfhost runtime route naming cleanup`
  - `phase-110x selfhost execution vocabulary SSOT`
  - `phase-105 digit OR-chain LLVM parity regression`
  - `phase-104 loop(true) + break-only digits（read_digits 系）`
  - `phase-103 if-only regression baseline（VM + LLVM EXE）`
  - `phase-102 real-app read_quoted loop regression (VM + LLVM EXE)`

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
- `phase-104` landed with loop(true)+break-only digits parity on VM and LLVM EXE
- `phase-105` restored the original long digit OR-chain parity on VM and LLVM EXE
- current work is the vm compat route exit plan lane:
  - vocabulary split: `stage / route / backend override / lane / kernel`
  - selfhost mainline already reads as `runtime-route mainline`
  - raw `--backend vm` is now treated as compat/proof/debug only
  - current buckets are `compat route`, `proof wrappers`, `debug/observability`
  - current target is how compat route exits raw `--backend vm` step by step

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-122x/README.md`
