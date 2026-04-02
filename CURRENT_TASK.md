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
8. `phase-30x backend surface simplification` (landed)
9. `phase-31x engineering lane isolation` (landed)
10. `phase-32x product / engineering split` (landed)
11. `phase-33x shared helper family recut`

- `K-axis` stays `K0 / K1 / K2` and is read as a build/runtime stage axis, not a task axis.
- current stage progression reads as `K0 -> K1 -> K2`.
- `K2-core` / `K2-wide` are task packs inside `K2`.
- `K2-core` is closed.
- `K2-wide` boundary-shrink lock-down is landed enough to hand off; `zero-rust` default operationalization is landed, `stage2plus entry / first optimization wave` is accepted, `phase-29x backend owner cutover prep` is landed, `phase-30x backend surface simplification` is landed, `phase-31x engineering lane isolation` is landed, `phase-32x product / engineering split` is landed, and current active lane is `phase-33x shared helper family recut`.

## Immediate Handoff

- Restart handoff: landed `K2-wide` / `zero-rust` rows stay accepted, `stage2plus` acceptance bundle is complete, `phase-29x` cleanup is closed, `phase-30x` ownership flip is landed, `phase-31x` engineering rehome sweep is landed, `phase-32x` mixed-owner split is landed, and the current active front is `phase-33x shared helper family recut`.
- Active lane: `phase-33x-shared-helper-family-recut`
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
  - `docs/development/current/main/phases/phase-31x/README.md`
  - `docs/development/current/main/phases/phase-31x/31x-90-engineering-lane-isolation-ssot.md`
  - `docs/development/current/main/phases/phase-31x/31x-91-task-board.md`
  - `docs/development/current/main/phases/phase-33x/README.md`
  - `docs/development/current/main/phases/phase-33x/33x-90-shared-helper-family-recut-ssot.md`
  - `docs/development/current/main/phases/phase-33x/33x-91-task-board.md`
  - `docs/development/current/main/phases/phase-32x/README.md`
  - `docs/development/current/main/phases/phase-32x/32x-90-product-engineering-split-ssot.md`
  - `docs/development/current/main/phases/phase-32x/32x-91-task-board.md`
  - `docs/development/current/main/design/backend-owner-cutover-ssot.md`
  - `docs/development/current/main/design/runtime-decl-manifest-v0.toml`
- Current read:
  - `K2-core` is closed
  - `K2-wide` lock-down is closed enough for handoff
  - `stage2plus entry / first optimization wave` is accepted
  - `phase-30x backend surface simplification` is landed
  - `phase-32x product / engineering split` is landed
  - current active lane is `phase-33x shared helper family recut`
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

- Active next: `phase-33x shared helper family recut`
- Current blocker: `none`
- Exact focus: `33xC3 hakorune_emit_mir top-level keep gate`
  - `phase-32x` is landed; mixed-owner source/smoke split and raw default/token defer are fixed
  - current next cleanup is helper-family path truth, not `vm.rs` deletion
  - current backend reading stays role-first:
    - `llvm/exe` = `product`
    - `rust-vm` = `engineering(stage0/bootstrap + tooling keep)`
    - `vm-hako` = `reference/conformance`
    - `wasm` = `experimental`
  - current cleanup rule is `split/rehome/drain -> delete`
  - landed in `32xA-G`: `build.rs` / `phase2100` split, top-level orchestrator rehome, `core_executor` direct-MIR seam, shared helper gate, and raw default/token defer
  - active in `33xA1`: helper family caller inventory is fixed for `hako_check` and `emit_mir`
  - landed in `33xB1`: `tools/hako_check/deadblocks_smoke.sh` is the canonical deadblocks family home and the old top-level path is shim-only
  - landed in `33xC1`: thin `emit_mir` wrappers have a low-blast caller inventory
  - landed in `33xC2`: thin `emit_mir` wrappers stay as top-level route-preset compatibility shims, and routing truth stays in `tools/smokes/v2/lib/emit_mir_route.sh`
  - active in `33xC3`: `tools/hakorune_emit_mir.sh` broad top-level keep is being gated after wrapper role truth was fixed
  - raw backend default still stays deferred; no-touch-first remains on `src/cli/args.rs`, `src/runner/dispatch.rs`, `tools/selfhost/run.sh`, and `tools/selfhost/selfhost_build.sh`
- Exact read order:
  1. `docs/development/current/main/15-Workstream-Map.md`
  2. `docs/development/current/main/phases/phase-33x/README.md`
  3. `docs/development/current/main/phases/phase-33x/33x-90-shared-helper-family-recut-ssot.md`
  4. `docs/development/current/main/phases/phase-33x/33x-91-task-board.md`
  5. `docs/development/current/main/phases/phase-32x/README.md`
- product / engineering split table:

  | Item | State |
  | --- | --- |
  | Now | `phase-33x shared helper family recut` |
  | Blocker | `none` |
  | Next | `33xC3 hakorune_emit_mir top-level keep gate` |
- Exact implementation rule:
  - keep `RuntimeDataBox` facade-only
  - boundary audit result: `RuntimeDataBox.delete` does not exist; delete stays on `MapBox` / `RawMap` only
  - keep `K2-wide` widening on capability modules, not ad hoc native escape hatches
  - keep `hako_alloc` closed until a concrete backend-private consumer appears
- LLVM task rule:
  - keep backend lane follow-up in the backend lane docs
  - do not mix keep-lane notes into `K2-wide` implementation notes

## Shared Helper Family Bands

| Band | State | Read as |
| --- | --- | --- |
| Now | `33xC3 hakorune_emit_mir top-level keep gate` | lock broad live integration after wrapper role is fixed |
| Next | `33xD1 closeout/docs cleanup` | close the helper-family recut after broad keep truth is fixed |
| Later | `raw backend default/token follow-up lane` | keep token/default truthification deferred until deeper source/smoke split requires it |

## Shared Helper Family Waves

| Wave | Status | Read as |
| --- | --- | --- |
| `33xA helper family inventory` | landed | fix exact keep/rehome/shim-only reading |
| `33xB hako_check family path truth` | active | move family-local smoke helpers under `tools/hako_check/**` |
| `33xC emit_mir thin wrapper path truth` | active | truthify thin wrappers as route-preset shims before broad helper keep |
| `33xD top-level keep gate` | queued | lock broad keep conditions for `hako_check.sh` and `hakorune_emit_mir.sh` |

## Phase-33x Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `33xA1` | landed | helper family caller inventory |
| `33xB1` | landed | `hako_check_deadblocks_smoke` family-home rehome |
| `33xB2` | queued | `hako_check.sh` top-level keep gate |
| `33xC1` | landed | `emit_mir` thin wrapper caller inventory |
| `33xC2` | landed | `emit_mir` thin wrapper route-preset lock |
| `33xC3` | active | `hakorune_emit_mir.sh` top-level keep gate |
| `33xD1` | queued | closeout/docs cleanup |

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
  - `docs/development/current/main/phases/phase-33x/README.md`
  - `docs/development/current/main/phases/phase-33x/33x-90-shared-helper-family-recut-ssot.md`
  - `docs/development/current/main/phases/phase-33x/33x-91-task-board.md`
  - `docs/development/current/main/phases/phase-32x/README.md`
  - `docs/development/current/main/phases/phase-32x/32x-90-product-engineering-split-ssot.md`
  - `docs/development/current/main/phases/phase-32x/32x-91-task-board.md`
  - `docs/development/current/main/phases/phase-31x/README.md`
  - `docs/development/current/main/phases/phase-31x/31x-90-engineering-lane-isolation-ssot.md`
  - `docs/development/current/main/phases/phase-31x/31x-91-task-board.md`
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
