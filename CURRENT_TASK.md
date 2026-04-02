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
9. `phase-31x engineering lane isolation`

- `K-axis` stays `K0 / K1 / K2` and is read as a build/runtime stage axis, not a task axis.
- current stage progression reads as `K0 -> K1 -> K2`.
- `K2-core` / `K2-wide` are task packs inside `K2`.
- `K2-core` is closed.
- `K2-wide` boundary-shrink lock-down is landed enough to hand off; `zero-rust` default operationalization is landed, `stage2plus entry / first optimization wave` is accepted, `phase-29x backend owner cutover prep` is landed, `phase-30x backend surface simplification` is landed, and current active lane is `phase-31x engineering lane isolation`.

## Immediate Handoff

- Restart handoff: landed `K2-wide` / `zero-rust` rows stay accepted, `stage2plus` acceptance bundle is complete, `phase-29x` cleanup is closed, `phase-30x` ownership flip is landed, and the current active front is `phase-31x engineering lane isolation`.
- Active lane: `phase-31x-engineering-lane-isolation`
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
  - `docs/development/current/main/design/backend-owner-cutover-ssot.md`
  - `docs/development/current/main/design/runtime-decl-manifest-v0.toml`
- Current read:
  - `K2-core` is closed
  - `K2-wide` lock-down is closed enough for handoff
  - `stage2plus entry / first optimization wave` is accepted
  - `phase-30x backend surface simplification` is landed
  - current active lane is `phase-31x engineering lane isolation`
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

- Active next: `phase-31x engineering lane isolation`
- Current blocker: `none`
- Exact focus: `31xD1 orchestrator keep vs rehome split`
  - `phase-30x` is landed; ownership flip is complete and `phase-31x` is the first source/smoke rehome lane
  - current backend reading stays role-first:
    - `llvm/exe` = `product`
    - `rust-vm` = `engineering(stage0/bootstrap + tooling keep)`
    - `vm-hako` = `reference/conformance`
    - `wasm` = `experimental`
  - current cleanup rule is `rehome -> shim -> drain -> delete`
  - landed in `31xA`: phase switch and `tools/engineering/**` home lock
  - landed in `31xB`: `tools/engineering/run_vm_stats.sh` and `tools/engineering/parity.sh`; old top-level paths are compatibility shims only
  - landed in `31xC`: `tools/hako_check.sh`, `tools/hako_check_deadcode_smoke.sh`, and `tools/hakorune_emit_mir.sh` are fixed as shared-helper `keep here`
  - active in `31xD1`: keep-vs-rehome split for bootstrap/plugin/selfhost orchestrators
  - raw backend default remains deferred; no-touch-first still includes `src/cli/args.rs`, `src/runner/dispatch.rs`, `tools/selfhost/run.sh`, `tools/selfhost/selfhost_build.sh`, and `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`
- Exact read order:
  1. `docs/development/current/main/15-Workstream-Map.md`
  2. `docs/development/current/main/phases/phase-31x/README.md`
  3. `docs/development/current/main/phases/phase-31x/31x-90-engineering-lane-isolation-ssot.md`
  4. `docs/development/current/main/phases/phase-31x/31x-91-task-board.md`
  5. `docs/development/current/main/phases/phase-30x/README.md`
- engineering isolation table:

  | Item | State |
  | --- | --- |
  | Now | `phase-31x engineering lane isolation` |
  | Blocker | `none` |
  | Next | `31xD1 orchestrator keep vs rehome split` |
- Exact implementation rule:
  - keep `RuntimeDataBox` facade-only
  - boundary audit result: `RuntimeDataBox.delete` does not exist; delete stays on `MapBox` / `RawMap` only
  - keep `K2-wide` widening on capability modules, not ad hoc native escape hatches
  - keep `hako_alloc` closed until a concrete backend-private consumer appears
- LLVM task rule:
  - keep backend lane follow-up in the backend lane docs
  - do not mix keep-lane notes into `K2-wide` implementation notes

## Engineering Isolation Bands

| Band | State | Read as |
| --- | --- | --- |
| Now | `31xD orchestrator isolation prep` | keep-vs-rehome split for no-touch-first orchestrators |
| Next | `31xE shim drain and legacy sweep` | delete/archive only after shim drain is explicit |
| Later | `shared helper follow-up` | only reopen on a dedicated helper-local cleanup lane |

## Engineering Isolation Waves

| Wave | Status | Read as |
| --- | --- | --- |
| `31xA engineering home lock` | landed | switch active lane and fix `tools/engineering/**` as canonical home |
| `31xB low-blast tool rehome` | landed | move low-blast engineering tools out of the top-level front |
| `31xC shared helper family inventory` | landed | decide keep / rehome / archive for helper family |
| `31xD orchestrator isolation prep` | active | split no-touch-first orchestrators into keep vs later rehome |
| `31xE shim drain and legacy sweep` | queued | delete/archive only after moved paths are drained |

## Phase-31x Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `31xA1` | landed | root/current mirrors and phase index switch to `phase-31x` |
| `31xA2` | landed | `tools/engineering/**` is the canonical home |
| `31xB1` | landed | `run_vm_stats.sh` actual move + shim |
| `31xB2` | landed | `parity.sh` actual move + shim |
| `31xC1` | landed | inventory shared helper family |
| `31xC2` | landed | choose shared helper disposition |
| `31xD1` | active | orchestrator keep vs rehome split |
| `31xD2` | queued | docs/live path repoint for moved orchestrators |
| `31xE1` | queued | delete drained compatibility shims |
| `31xE2` | queued | archive stale top-level wrappers |

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
