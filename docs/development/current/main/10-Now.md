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

- lane: `phase-131x vm legacy contract migration`
- current front: explicit legacy `vm` contract smoke は archive 済み。backend-hint chain を順に畳む
- blocker: `src/runner/stage1_bridge/stub_child.rs` がまだ legacy contract を支えている
- recent landed:
  - `phase-130x vm public gate final cleanup`
  - `phase-127x compat route raw vm cut prep`
  - `phase-125x vm bridge/backend gate follow-up`
  - `phase-124x vm public docs/manual demotion`
  - `phase-123x proof gate shrink follow-up`
  - `phase-122x vm compat route exit plan`
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
- current work is the stage1 bridge vm gate softening lane:
  - compat boundary smoke is route-first and green
  - compat temp-MIR handoff is green again with the parser-EXE preference env applied internally
  - the default `stage1_cli_env.hako` child path no longer forwards backend hints
  - the next source seam is the remaining public gate / orchestrator wording and selection plumbing

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-131x/README.md`
