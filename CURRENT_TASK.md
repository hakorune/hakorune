# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-02
Scope: repo root から current order / current blocker / next exact read に最短で戻るための restart anchor。詳細の進捗・履歴・設計本文は `docs/development/current/main/` 側を正本とする。

## Purpose

- root から最短で current lane と next structural step に戻る。
- 本ファイルは薄い入口に保ち、長い ledger / appendix / landed history は phase README と design SSOT に逃がす。
- artifact roots (`target/**`, `artifacts/**`, `dist/**`) are for binaries/bundles only; migration task order stays in design SSOT and phase docs.

## Quick Restart Pointer

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
4. `git status -sb`
5. `tools/checks/dev_gate.sh quick`

## Order At A Glance

1. `stage / docs / naming` fixation
2. `K1 done-enough` stop-line fixation
3. `K2-core` accepted stop-line
4. `K2-wide` boundary-shrink lock-down (closed)
5. `zero-rust` default operationalization (landed)
6. `stage2plus entry / first optimization wave` (accepted)
7. `phase-29x backend owner cutover prep`

- `K-axis` stays `K0 / K1 / K2` and is read as a build/runtime stage axis, not a task axis.
- current stage progression reads as `K0 -> K1 -> K2`.
- `K2-core` / `K2-wide` are task packs inside `K2`.
- `K2-core` is closed.
- `K2-wide` boundary-shrink lock-down is landed enough to hand off; `zero-rust` default operationalization is landed, `stage2plus entry / first optimization wave` is accepted, and current active lane is `phase-29x backend owner cutover prep`.

## Immediate Handoff

- Restart handoff: landed `K2-wide` / `zero-rust` rows stay accepted, `stage2plus` acceptance bundle is complete, and the current active front is `phase-29x backend owner cutover prep`.
- Active lane: `phase-29x-backend-owner-cutover`
- Axis and lane detail is canonical in:
  - `docs/development/current/main/phases/phase-29x/README.md`
  - `docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md`
  - `docs/development/current/main/phases/phase-29x/29x-91-task-board.md`
  - `docs/development/current/main/phases/phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md`
  - `docs/development/current/main/phases/phase-29x/29x-97-compare-bridge-retirement-prep-ssot.md`
  - `docs/development/current/main/phases/phase-29x/29x-98-legacy-route-retirement-investigation-ssot.md`
  - `docs/development/current/main/phases/phase-29x/29x-99-structure-recut-wave-plan-ssot.md`
  - `docs/development/current/main/design/backend-owner-cutover-ssot.md`
  - `docs/development/current/main/design/runtime-decl-manifest-v0.toml`
- Current read:
  - `K2-core` is closed
  - `K2-wide` lock-down is closed enough for handoff
  - `stage2plus entry / first optimization wave` is accepted
  - current active lane is `phase-29x backend owner cutover prep`
- landed rows already accepted:
  - `RawMap` first slice
  - `RawMap.clear`
  - `RawMap.remove/delete`
  - `hako.atomic.fence_i64`
  - `hako.tls.last_error_text_h`
  - `hako.gc.write_barrier_i64`
  - `hako.osvm.reserve_bytes_i64` / `commit_bytes_i64` / `decommit_bytes_i64` (first live osvm rows)
  - `hako_alloc` handle reuse policy
  - `hako_alloc` GC trigger threshold policy
- Portability rule:
  - `.hako` owns capability facades.
  - final OS VM / TLS / atomic / GC platform glue stays native keep for Linux, Windows (`WSL`/`cmd.exe`), and macOS portability.

## Immediate Next Task

- Active next: `phase-29x backend owner cutover prep`
- Current blocker: `none`
- Exact focus: `29x-99 W4 Hako-side caller drain prep / 99P1 compat selfhost payload demotion`
  - phase2120 pure and proof buckets are now physically recut into `integration/compat/pure-keep`, `archive/pure-historical`, `integration/proof/vm-adapter-legacy`, and `integration/proof/native-reference`; the legacy cluster orchestrator is runner-only
  - phase2044 has been physically recut into `integration/compat/llvmlite-monitor-keep`, `integration/proof/hako-primary-no-fallback`, and `integration/proof/mirbuilder-provider`; the llvmlite trio is monitor-only keep and the proof buckets are runner-only
  - inside the llvmlite trio, nothing is archive-ready; `compare_branch` / `const42` are merge-later only
  - phase2111 and phase251 archive proofs are now grouped under one replay-evidence suite
  - compat selfhost wrapper stays archive-later; `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` and `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako` are not daily owners, and the driver now lives in the compat bucket instead of `tools/selfhost/examples/`
  - selfhost compat stack wording is now locked as `payload -> transport wrapper -> pack orchestrator`
  - root-first proof candidate inventory is now pinned: the compat selfhost wrapper only has the separate `phase29ck_vmhako_llvm_backend_runtime_proof` lane as a non-drop-in candidate, while `extern_provider.hako` now has one exact proof lane under `integration/compat/extern-provider-stop-line-proof`
  - direct live callers are fixed at 5 surfaces: `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako`, `lang/src/vm/hakorune-vm/extern_provider.hako`, `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs`, `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs`, and `src/runtime/plugin_loader_v2/enabled/extern_functions.rs`
  - `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh` and `tools/compat/legacy-codegen/run_compat_pure_pack.sh` are wrappers/orchestrators, not direct `emit_object` callers
  - `29x-98` still owns helper deletion and exact stop-line; caller demotion is now visible, but helper deletion stays closed
  - W5 prep has started: codegen receiver bodies are now split into dedicated modules, but the chokepoint collapse itself is still pending after `W4`
  - `29x-99` now owns beauty-first cleanup planning, with `W4 Hako-side caller drain prep` active and `W3 smoke/proof filesystem recut` landed
  - current active micro task is `99P1 compat selfhost payload demotion`
  - next queued micro task is `99P2 extern_provider compat codegen caller demotion`
  - review intake owner is `29x-99`; mirror docs only carry the open deltas, not the full intake table
- Exact read order:
  1. `docs/development/current/main/15-Workstream-Map.md`
  2. `docs/development/current/main/phases/phase-29x/README.md`
  3. `docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md`
  4. `docs/development/current/main/phases/phase-29x/29x-91-task-board.md`
  5. `docs/development/current/main/phases/phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md`
  6. `docs/development/current/main/phases/phase-29x/29x-97-compare-bridge-retirement-prep-ssot.md`
  7. `docs/development/current/main/phases/phase-29x/29x-98-legacy-route-retirement-investigation-ssot.md`
  8. `docs/development/current/main/phases/phase-29x/29x-99-structure-recut-wave-plan-ssot.md`
  9. `docs/development/current/main/design/backend-owner-cutover-ssot.md`
  10. `docs/development/current/main/design/runtime-decl-manifest-v0.toml`
- K2-wide lock-down table:

  | Item | State |
  | --- | --- |
  | Now | `phase-29x backend owner cutover prep` |
  | Blocker | `none` |
  | Next | `29x-99` W4 caller demotion lane -> `29x-98` stop-line stays fixed until `99P1-99P3` land |
- Exact implementation rule:
  - keep `RuntimeDataBox` facade-only
  - boundary audit result: `RuntimeDataBox.delete` does not exist; delete stays on `MapBox` / `RawMap` only
  - keep `K2-wide` widening on capability modules, not ad hoc native escape hatches
  - keep `hako_alloc` closed until a concrete backend-private consumer appears
- LLVM task rule:
  - keep backend lane follow-up in the backend lane docs
  - do not mix keep-lane notes into `K2-wide` implementation notes

## Cleanup Bands

| Band | State | Read as |
| --- | --- | --- |
| Now | `99P1 compat selfhost payload demotion` | the exact `vm-hako` proof is green, so the first direct `.hako` caller can move off the bridge |
| Next | `99P2-99P3 Hako-side caller demotion` | demote the provider caller next, then make the bridge archive-only |
| Later | `src/host_providers/llvm_codegen/legacy_mir_front_door.rs::emit_object_from_mir_json(...)` / Rust dispatch residues | delete only after caller inventory reaches zero |

## Cleanup Waves

| Wave | Status | Read as |
| --- | --- | --- |
| `W1 docs-first path-truth pass` | landed | lock target buckets, names, and move order |
| `W2 mixed-file split pass` | landed | split owner-looking mixed files before behavior change |
| `W3 smoke/proof filesystem recut` | landed | phase-number homes become semantic homes |
| `W4 Hako-side caller drain prep` | active | exact replacement proof is green; caller demotion is now in progress |
| `W5 Rust compat receiver collapse` | pending-after-W4 | reduce legacy receiver spread to one chokepoint |
| `W6 final delete/archive sweep` | pending-after-W5 | delete helpers only after inventory reaches zero |

## Cleanup Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `99E split-target inventory lock` | landed | freeze target split homes before any move |
| `99F file-move / shim order lock` | landed | define move-first, shim-second, delete-last order |
| `99G-99J mixed-file split targets` | landed | `extern_provider.hako`, `llvm_codegen.rs`, `LlvmBackendBox`, compat boxes |
| `99K-99M smoke/proof filesystem recut` | landed | `phase2044`, `phase2120`, archive evidence bundle |
| `99N1-99N3 compat wrapper contract/gap lock` | landed | drop-in contract and proof gap are fixed for the compat selfhost wrapper stack |
| `99O1-99O3 extern_provider contract/proof-target/prereq lock` | landed | compat codegen stub contract, exact proof target, and demotion order are fixed |
| `99O4 minimal root-first lowering proof smoke` | landed | exact `vm-hako` proof is green for the `extern_provider` stop-line |
| `99P1 compat selfhost payload demotion` | active | move the compat selfhost payload off `CodegenBridgeBox.emit_object_args(...)` |
| `99P2 extern_provider compat codegen caller demotion` | pending-after-P1 | move the gated provider caller off `CodegenBridgeBox.emit_object_args(...)` |
| `99P3 make CodegenBridgeBox.emit_object_args(...) archive-only` | pending-after-P2 | remaining direct Hako callers are zero or archive-only |
| `99Q1-99S1 Rust compat receiver collapse` | pending-after-W4 | reduce Rust legacy codegen acceptance to one chokepoint |

- `phase2044` llvmlite trio is monitor-only keep under `integration/compat/llvmlite-monitor-keep`.
- `phase2120` pure canaries stay split: `array_set_get` / `loop_count` keep via `compat/pure-keep`, archive-backed historical pins via `archive/pure-historical`.

## Canonical Owners

### Restart / Mirrors

- root anchor: `CURRENT_TASK.md`
- quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- one-screen order mirror: `docs/development/current/main/15-Workstream-Map.md`
- thin docs mirror/dashboard: `docs/development/current/main/10-Now.md`

### Rough Order / Axis Vocabulary

- canonical rough task order:
  - `docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md`
- `K-axis` / artifact / task placement:
  - `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
  - `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`

### Current Technical SSOTs

- `K2-wide` capability / metal keep:
  - `docs/development/current/main/design/gc-tls-atomic-capability-ssot.md`
  - `docs/development/current/main/design/atomic-tls-gc-truthful-native-seam-inventory.md`
  - `docs/development/current/main/design/final-metal-split-ssot.md`
- `hako_alloc` rows:
  - `docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md`
- current phase-order context:
  - `docs/development/current/main/phases/phase-29x/README.md`
  - `docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md`
  - `docs/development/current/main/phases/phase-29x/29x-91-task-board.md`
  - `docs/development/current/main/design/backend-owner-cutover-ssot.md`
  - `docs/development/current/main/design/runtime-decl-manifest-v0.toml`
  - `docs/development/current/main/phases/phase-29bq/README.md`

## Notes

- `target/**`, `artifacts/**`, and `dist/**` are artifact roots only.
- migration tasks stay in `CURRENT_TASK.md`, `15-Workstream-Map.md`, `design/kernel-implementation-phase-plan-ssot.md`, and phase docs.
- do not create a new "rough order" SSOT unless `kernel-implementation-phase-plan-ssot.md` becomes structurally overloaded.
