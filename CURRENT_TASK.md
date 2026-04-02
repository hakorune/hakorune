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
7. `phase-29x backend owner cutover prep` (landed)
8. `phase-30x backend surface simplification`

- `K-axis` stays `K0 / K1 / K2` and is read as a build/runtime stage axis, not a task axis.
- current stage progression reads as `K0 -> K1 -> K2`.
- `K2-core` / `K2-wide` are task packs inside `K2`.
- `K2-core` is closed.
- `K2-wide` boundary-shrink lock-down is landed enough to hand off; `zero-rust` default operationalization is landed, `stage2plus entry / first optimization wave` is accepted, `phase-29x backend owner cutover prep` is landed, and current active lane is `phase-30x backend surface simplification`.

## Immediate Handoff

- Restart handoff: landed `K2-wide` / `zero-rust` rows stay accepted, `stage2plus` acceptance bundle is complete, `phase-29x` cleanup is closed, and the current active front is `phase-30x backend surface simplification`.
- Active lane: `phase-30x-backend-surface-simplification`
- Axis and lane detail is canonical in:
  - `docs/development/current/main/phases/phase-29x/README.md`
  - `docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md`
  - `docs/development/current/main/phases/phase-29x/29x-91-task-board.md`
  - `docs/development/current/main/phases/phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md`
  - `docs/development/current/main/phases/phase-29x/29x-97-compare-bridge-retirement-prep-ssot.md`
  - `docs/development/current/main/phases/phase-29x/29x-98-legacy-route-retirement-investigation-ssot.md`
  - `docs/development/current/main/phases/phase-29x/29x-99-structure-recut-wave-plan-ssot.md`
  - `docs/development/current/main/phases/phase-30x/README.md`
  - `docs/development/current/main/phases/phase-30x/30x-90-backend-surface-simplification-ssot.md`
  - `docs/development/current/main/phases/phase-30x/30x-91-task-board.md`
  - `docs/development/current/main/design/backend-owner-cutover-ssot.md`
  - `docs/development/current/main/design/runtime-decl-manifest-v0.toml`
- Current read:
  - `K2-core` is closed
  - `K2-wide` lock-down is closed enough for handoff
  - `stage2plus entry / first optimization wave` is accepted
  - current active lane is `phase-30x backend surface simplification`
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

- Active next: `phase-30x backend surface simplification`
- Current blocker: `none`
- Exact focus: `30xC3 rust-vm smoke/test inventory`
  - `phase-29x` W4/W5/W6 is landed; explicit helper deletion and path-truth cleanup are closed
  - current backend reading is role-first:
    - `llvm/exe` = `product`
    - `rust-vm` = `engineering/bootstrap`
    - `vm-hako` = `reference/conformance`
    - `wasm` = `experimental`
  - `rust-vm` is still deep in bootstrap/selfhost/plugin/macro/smoke paths; do not force-remove it before inventory and smoke split
  - current docs mostly use the label `rust-vm`, not `vm-rust`
  - dangerous early flips are launcher/default/orchestrator sites such as `src/cli/args.rs`, `src/runner/dispatch.rs`, `src/runner/modes/common_util/selfhost/child.rs`, `lang/src/runner/stage1_cli/core.hako`, `tools/selfhost/run.sh`, and `tools/plugin_v2_smoke.sh`
  - `30xA1`, `30xA2`, `30xB1-30xB4`, `30xC1`, and `30xC2` are landed
  - `30xC2` grouped plugin/macro/tooling pressure into `engineering/tooling keep` and `manual residue watch`
  - active micro task is `30xC3 rust-vm smoke/test inventory`
  - next queued micro task is `30xC4 rust-vm docs/help inventory`
  - `phase29cc_wsm` families are read as `experimental`, not product-mainline or co-main
  - `compat/llvmlite-monitor-keep` is compat/probe keep only, not `llvm/exe` product evidence
  - `tools/smokes/v2/configs/matrix.conf` now reads `vm/llvm` as engineering/product only; `vm-hako` and `wasm` stay outside the matrix axis
  - `30xC1` fixed bootstrap/selfhost keep surfaces and found no archive/delete candidate in that bucket
- Exact read order:
  1. `docs/development/current/main/15-Workstream-Map.md`
  2. `docs/development/current/main/phases/phase-30x/README.md`
  3. `docs/development/current/main/phases/phase-30x/30x-90-backend-surface-simplification-ssot.md`
  4. `docs/development/current/main/phases/phase-30x/30x-91-task-board.md`
  5. `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
  6. `docs/development/current/main/design/artifact-policy-ssot.md`
  7. `docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md`
- backend surface table:

  | Item | State |
  | --- | --- |
  | Now | `phase-30x backend surface simplification` |
  | Blocker | `none` |
  | Next | `30xC3 rust-vm smoke/test inventory` |
- Exact implementation rule:
  - keep `RuntimeDataBox` facade-only
  - boundary audit result: `RuntimeDataBox.delete` does not exist; delete stays on `MapBox` / `RawMap` only
  - keep `K2-wide` widening on capability modules, not ad hoc native escape hatches
  - keep `hako_alloc` closed until a concrete backend-private consumer appears
- LLVM task rule:
  - keep backend lane follow-up in the backend lane docs
  - do not mix keep-lane notes into `K2-wide` implementation notes

## Backend Surface Bands

| Band | State | Read as |
| --- | --- | --- |
| Now | `30xC3 rust-vm smoke/test inventory` | group vm-backed smoke/test orchestrators separately |
| Next | `30xC4 rust-vm docs/help inventory` | group stale main-narrative docs/help pressure |
| Later | `30xD-30xF` | dangerous flip lock, user-facing main switch, backend default gate |

## Backend Surface Waves

| Wave | Status | Read as |
| --- | --- | --- |
| `30xA role taxonomy lock` | landed | lock the role-first reading in root docs and phase docs |
| `30xB smoke taxonomy split` | landed | separate product / engineering / reference / experimental smoke reading |
| `30xC rust-vm dependency inventory` | active | map internal `--backend vm` pressure before any flip |
| `30xD dangerous-early-flip lock` | queued | freeze launcher/default/orchestrator sites that must not move early |
| `30xE user-facing main switch prep` | queued | move README/help/examples toward `llvm/exe` first without flipping defaults |
| `30xF backend default decision gate` | queued | decide the raw CLI default only after the previous slices land |

## Phase-30x Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `30xA1` | landed | root mirrors use the same role-first labels |
| `30xA2` | landed | design role SSOTs use the same role-first labels |
| `30xB1` | landed | `vm-hako` reference smoke lock |
| `30xB2` | landed | `wasm` experimental smoke lock |
| `30xB3` | landed | `llvm/exe` product vs `llvmlite` probe boundary lock |
| `30xB4` | landed | matrix/guide smoke taxonomy cleanup |
| `30xC1` | landed | bootstrap/selfhost `rust-vm` pressure inventory |
| `30xC2` | landed | plugin/macro/tooling `rust-vm` pressure inventory |
| `30xC3` | active | smoke/test `rust-vm` pressure inventory |
| `30xC4` | queued | docs/help `rust-vm` pressure inventory |
| `30xD1-30xD3` | queued | dangerous early flips are frozen explicitly |
| `30xE1-30xE4` | queued | user-facing main switch is prepared without a raw default flip |
| `30xF1-30xF2` | queued | backend default decision gate stays last |

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
  - `docs/development/current/main/phases/phase-30x/README.md`
  - `docs/development/current/main/phases/phase-30x/30x-90-backend-surface-simplification-ssot.md`
  - `docs/development/current/main/phases/phase-30x/30x-91-task-board.md`
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
