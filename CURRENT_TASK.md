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
10. `phase-32x product / engineering split`

- `K-axis` stays `K0 / K1 / K2` and is read as a build/runtime stage axis, not a task axis.
- current stage progression reads as `K0 -> K1 -> K2`.
- `K2-core` / `K2-wide` are task packs inside `K2`.
- `K2-core` is closed.
- `K2-wide` boundary-shrink lock-down is landed enough to hand off; `zero-rust` default operationalization is landed, `stage2plus entry / first optimization wave` is accepted, `phase-29x backend owner cutover prep` is landed, `phase-30x backend surface simplification` is landed, `phase-31x engineering lane isolation` is landed, and current active lane is `phase-32x product / engineering split`.

## Immediate Handoff

- Restart handoff: landed `K2-wide` / `zero-rust` rows stay accepted, `stage2plus` acceptance bundle is complete, `phase-29x` cleanup is closed, `phase-30x` ownership flip is landed, `phase-31x` engineering rehome sweep is landed, and the current active front is `phase-32x product / engineering split`.
- Active lane: `phase-32x-product-engineering-split`
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
  - current active lane is `phase-32x product / engineering split`
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

- Active next: `phase-32x product / engineering split`
- Current blocker: `none`
- Exact focus: `32xE1 child.rs / stage1_cli direct-route gap inventory`
  - `phase-31x` is landed; low-blast engineering rehome and shim drain are complete
  - current next cleanup is not `vm.rs` deletion; it is mixed-owner source/smoke split
  - current backend reading stays role-first:
    - `llvm/exe` = `product`
    - `rust-vm` = `engineering(stage0/bootstrap + tooling keep)`
    - `vm-hako` = `reference/conformance`
    - `wasm` = `experimental`
  - current cleanup rule is `split/rehome/drain -> delete`
  - landed in `32xA`: `build.rs` and `phase2100` mixed-owner inventory
  - landed in `32xB1`: shared vs product vs engineering target split for `src/runner/build.rs`
  - landed in `32xB2`: helper-first extraction inside `src/runner/build.rs`
  - landed in `32xC1`: role buckets for `phase2100/run_all.sh` are fixed
  - landed in `32xC2`: `phase2100/run_all.sh` is now a thin meta-runner over role sub-runners
  - landed in `32xD1`: `tools/selfhost/bootstrap_selfhost_smoke.sh` is the canonical bootstrap smoke home and the old top-level path is shim-only
  - landed in `32xD2`: `tools/plugins/plugin_v2_smoke.sh` is the canonical plugin smoke home and the old top-level path is shim-only
  - active in `32xE1`: direct `--backend vm` shell residue inventory for `child.rs` and `stage1_cli`
  - raw backend default remains deferred; no-touch-first still includes `src/cli/args.rs`, `src/runner/dispatch.rs`, `tools/selfhost/run.sh`, `tools/selfhost/selfhost_build.sh`, and the public `phase2100/run_all.sh` path
- Exact read order:
  1. `docs/development/current/main/15-Workstream-Map.md`
  2. `docs/development/current/main/phases/phase-32x/README.md`
  3. `docs/development/current/main/phases/phase-32x/32x-90-product-engineering-split-ssot.md`
  4. `docs/development/current/main/phases/phase-32x/32x-91-task-board.md`
  5. `docs/development/current/main/phases/phase-31x/README.md`
- product / engineering split table:

  | Item | State |
  | --- | --- |
  | Now | `phase-32x product / engineering split` |
  | Blocker | `none` |
  | Next | `32xE1 child.rs / stage1_cli direct-route gap inventory` |
- Exact implementation rule:
  - keep `RuntimeDataBox` facade-only
  - boundary audit result: `RuntimeDataBox.delete` does not exist; delete stays on `MapBox` / `RawMap` only
  - keep `K2-wide` widening on capability modules, not ad hoc native escape hatches
  - keep `hako_alloc` closed until a concrete backend-private consumer appears
- LLVM task rule:
  - keep backend lane follow-up in the backend lane docs
  - do not mix keep-lane notes into `K2-wide` implementation notes

## Product / Engineering Split Bands

| Band | State | Read as |
| --- | --- | --- |
| Now | `32xE1 child.rs / stage1_cli direct-route gap inventory` | inventory direct shell residues before core takeover |
| Next | `32xE2 core_executor takeover seam lock` | lock the direct MIR/core takeover seam |
| Later | `32xF1 shared helper follow-up gate` | keep helper-family recut on a dedicated lane |

## Product / Engineering Split Waves

| Wave | Status | Read as |
| --- | --- | --- |
| `32xA mixed-owner inventory` | landed | inventory exact mixed-owner source/smoke targets |
| `32xB build.rs split plan` | landed | split product build and engineering build ownership |
| `32xC phase2100 role split plan` | landed | split the thick smoke aggregator by role |
| `32xD top-level orchestrator rehome prep` | landed | drain callers before moving remaining top-level keeps |
| `32xE direct-route takeover prep` | active | reduce shell-based `--backend vm` residues behind dedicated seams |
| `32xF shared helper follow-up gate` | queued | reopen helper-family recut only on a dedicated lane |

## Phase-32x Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `32xA1` | landed | `build.rs` mixed ownership inventory |
| `32xA2` | landed | `phase2100` mixed aggregator inventory |
| `32xB1` | landed | `build.rs` split target lock |
| `32xB2` | landed | `build.rs` implementation slice order |
| `32xC1` | landed | `phase2100` role bucket lock |
| `32xC2` | landed | `phase2100` thin meta-runner plan |
| `32xD1` | landed | `bootstrap_selfhost_smoke` caller drain map |
| `32xD2` | landed | `plugin_v2_smoke` caller drain map |
| `32xE1` | active | `child.rs` / `stage1_cli` direct-route gap inventory |
| `32xE2` | queued | `core_executor` takeover seam lock |
| `32xF1` | queued | shared helper follow-up gate |
| `32xG1` | deferred | raw backend default/token remains last |

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
