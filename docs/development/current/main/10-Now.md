---
Status: SSOT
Date: 2026-04-01
Scope: main ラインの current summary と正本リンクだけを置く薄い mirror/dashboard。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md
---

# Self Current Task — Now (main)

## Purpose

- この文書は docs 側の薄い mirror/dashboard。
- current summary、next exact read、正本リンクだけを置く。
- 長い landed history、acceptance detail、phase ledger は owner doc に逃がす。

## Root Anchors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen map: `docs/development/current/main/15-Workstream-Map.md`
- layout contract: `docs/development/current/main/DOCS_LAYOUT.md`

## Immediate Resume

- current lane is `phase-29x backend owner cutover prep`
- exact current order is owned by `CURRENT_TASK.md` and `15-Workstream-Map.md`
- axis details are canonical in:
  - `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
  - `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
  - `docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md`

## Current Read

- `K2-core` is closed as the accepted `RawArray first truthful substrate` stop-line.
- `K2-wide` boundary-shrink lock-down is closed enough for handoff.
- `zero-rust default operationalization` is landed; `hako.osvm.reserve_bytes_i64` / `commit_bytes_i64` / `decommit_bytes_i64` are already landed and `page_size` stays parked.
- `stage2plus entry / first optimization wave` is accepted; the active front has moved to `phase-29x backend owner cutover prep`.
- boundary audit result: `RuntimeDataBox` remains facade-only and delete stays on `MapBox` / `RawMap`.
- current active step is `phase-29x backend owner cutover prep`; the phase29x LLVM-only daily gate is green and the next exact focus is the backend-owner cutover prep lane.
- current LLVM follow-up is organized separately from `K2-wide`; see backend lane docs for the live lane names.
- landed rows are tracked in `CURRENT_TASK.md` and the technical SSOTs below.
- portability split stays explicit:
  - `.hako` owns capability facades
  - final OS VM / TLS / atomic / GC leaf glue stays native keep
- artifact reading stays:
  - current reality: `target/release/hakorune`, `target/selfhost/hakorune`, `lang/bin/hakorune`
  - target contract: `target/k0|k1/`, `artifacts/k0|k1/`, `dist/k2/<channel>/<triple>/bundle/`
  - migration tasks do not live in artifact roots
- folder structure and smoke taxonomy docs are synced; the next optimization wave can read the current layout without extra prep

## Exact Links

- rough order / next slices:
  - `docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md`
- axis / artifact / placement:
  - `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
  - `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
- current `K2-wide` technical detail:
  - `docs/development/current/main/design/raw-map-substrate-ssot.md`
  - `docs/development/current/main/design/raw-map-truthful-native-seam-inventory.md`
  - `docs/development/current/main/design/gc-tls-atomic-capability-ssot.md`
  - `docs/development/current/main/design/atomic-tls-gc-truthful-native-seam-inventory.md`
  - `docs/development/current/main/design/final-metal-split-ssot.md`
  - `docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md`
- folder / smoke layout:
  - `lang/README.md`
  - `docs/development/current/main/design/smoke-taxonomy-and-discovery-ssot.md`
  - `docs/development/testing/smoke-tests-v2.md`
  - `docs/how-to/smokes.md`
- current phase detail:
  - `docs/development/current/main/phases/phase-29x/README.md`
  - `docs/development/current/main/phases/phase-29bq/README.md`
  - `docs/development/current/main/design/backend-owner-cutover-ssot.md`
  - `docs/development/current/main/design/runtime-decl-manifest-v0.toml`

## Restart Reminder

1. read `CURRENT_TASK.md`
2. read `15-Workstream-Map.md`
3. read the current SSOT for the active slice
4. run `tools/checks/dev_gate.sh quick`
5. if working on the blocker, inspect `lang/src/compiler/entry/compiler_stageb.hako` and the latest Stage-B gate log
