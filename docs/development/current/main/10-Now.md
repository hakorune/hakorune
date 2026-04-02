---
Status: SSOT
Date: 2026-04-02
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
- current active step is `phase-29x backend owner cutover prep`; the phase29x LLVM-only daily gate is green, `LLVMEmitBox` stays compat/proof keep, `CodegenBridgeBox` has no daily dependency, `phase2111` explicit emit/link proofs are archived, `phase251` lowering canaries are quarantined, `phase2044` has been physically recut into `integration/compat/llvmlite-monitor-keep`, `integration/proof/hako-primary-no-fallback`, and `integration/proof/mirbuilder-provider`, the llvmlite trio is suite-locked monitor-only keep and that dedicated suite is now the final live keep bucket, `phase2120` pure and proof buckets are now physically recut into `integration/compat/pure-keep`, `archive/pure-historical`, `integration/proof/vm-adapter-legacy`, and `integration/proof/native-reference`, `phase2111` / `phase251` archive proofs share one replay-evidence suite, the selfhost compat stack wording is locked as `payload -> transport wrapper -> pack orchestrator`, the root-first proof candidate inventory is pinned, and docs-first cleanup planning now lives in `29x-99` with `W4 Hako-side caller drain prep` active and `W3 smoke/proof filesystem recut` landed.
- direct live callers are fixed at 5 surfaces; `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` and `tools/compat/legacy-codegen/run_compat_pure_pack.sh` are wrapper/orchestrator layers only, not direct `emit_object` callers.
- `29x-98` still owns delete-readiness and stop-line; caller demotion is now visible, but helper deletion stays closed.
- `29x-99` owns macro cleanup waves and micro tasks; `99N1-99N3`, `99O1-99O3`, and `99O4` are landed, and the active micro task is `99P1 compat selfhost payload demotion`.
- next queued micro task is `99P2 extern_provider compat codegen caller demotion`; Rust-side collapse starts at `99Q1` only after `99P1-99P3` land.
- W5 prep is partially landed: codegen receiver bodies now live in dedicated modules, but the one-chokepoint collapse itself is still pending after W4.
- review intake lives in `29x-99`; this mirror only carries the open deltas.
- immediate cleanup order is `compat selfhost wrapper archive conditions -> archive proof bundle -> directory semantic recut`.
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

## Cleanup Bands

| Band | State | Read as |
| --- | --- | --- |
| Now | `99P1 compat selfhost payload demotion` | the exact `vm-hako` proof is green, so the first direct `.hako` caller can move off the bridge |
| Next | `99P2-99P3 Hako-side caller demotion` | demote the provider caller next, then make the bridge archive-only |
| Later | `src/host_providers/llvm_codegen/legacy_mir_front_door.rs::emit_object_from_mir_json(...)` / Rust dispatch residues | delete only after caller inventory reaches zero |

## Cleanup Waves

| Wave | Status | Read as |
| --- | --- | --- |
| `W1 docs-first path-truth pass` | landed | target buckets and move order |
| `W2 mixed-file split pass` | landed | split owner-looking mixed files |
| `W3 smoke/proof filesystem recut` | landed | semantic homes replace phase-number homes |
| `W4 Hako-side caller drain prep` | active | exact replacement proof is green; caller demotion is underway |
| `W5 Rust compat receiver collapse` | pending-after-W4 | one compat receiver chokepoint |
| `W6 final delete/archive sweep` | pending-after-W5 | helper deletion after zero callers |

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
  - `docs/development/current/main/phases/phase-29x/29x-99-structure-recut-wave-plan-ssot.md`
  - `docs/development/current/main/phases/phase-29bq/README.md`
  - `docs/development/current/main/design/backend-owner-cutover-ssot.md`
  - `docs/development/current/main/design/runtime-decl-manifest-v0.toml`

## Restart Reminder

1. read `CURRENT_TASK.md`
2. read `15-Workstream-Map.md`
3. read the current SSOT for the active slice
4. run `tools/checks/dev_gate.sh quick`
5. if working on the active blocker-free lane, inspect `docs/development/current/main/phases/phase-29x/29x-98-legacy-route-retirement-investigation-ssot.md`
