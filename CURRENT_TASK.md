# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-01
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
4. `K2-wide` next structural follow-up
5. `zero-rust` default operationalization

- `K-axis` stays `K0 / K1 / K2` and is read as a build/runtime stage axis, not a task axis.
- current stage progression reads as `K0 -> K1 -> K2`.
- `K2-core` / `K2-wide` are task packs inside `K2`.
- `K2-core` is closed.
- `K2-wide` is the active structural lane.

## Immediate Handoff

- Restart handoff: commit `6d56898ea`, worktree clean, `RawMap.clear` is landed, next slice is `RawMap.remove/delete`.
- Active lane: `policy-refresh`
- LLVM lane split:
  - `llvmlite` = stage0 / compat / probe keep lane
  - `ny-llvm` / `ny-llvmc` = daily mainline AOT lane
  - `llvmlite` work stops at keep/probe parity; mainline follow-up work belongs to `ny-llvm`
- Current read:
  - `K0 = all-Rust hakorune`
  - `K1 = .hako kernel migration stage`
  - `K2 = .hako kernel mainline / zero-rust daily-distribution stage`
  - `K2-core = RawArray first`
  - `K2-wide = RawMap second + capability widening + metal review`
- current `K2-wide` focus is RawMap remove/delete / boundary-shrink planning
- landed rows already accepted:
  - `RawMap` first slice
  - `RawMap` clear
  - `hako.atomic`
  - `hako.tls`
  - `hako.gc`
  - `hako.osvm`
  - `hako_alloc` handle reuse policy
  - `hako_alloc` GC trigger threshold policy
- Portability rule:
  - `.hako` owns capability facades.
  - final OS VM / TLS / atomic / GC platform glue stays native keep for Linux, Windows (`WSL`/`cmd.exe`), and macOS portability.

## Immediate Next Task

- Active next: `K2-wide` RawMap remove/delete / boundary-shrink planning
- Exact read order:
  1. `docs/development/current/main/15-Workstream-Map.md`
  2. `docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md`
  3. `docs/development/current/main/design/atomic-tls-gc-truthful-native-seam-inventory.md`
  4. `docs/development/current/main/design/final-metal-split-ssot.md`
- Exact implementation rule:
  - keep `RuntimeDataBox` facade-only
  - keep `K2-wide` widening on capability modules, not ad hoc native escape hatches
  - keep `hako_alloc` closed until a concrete backend-private consumer appears
- LLVM task rule:
  - keep `llvmlite` as compat/probe only
  - queue `ny-llvm` follow-up slices separately, by bucket
  - do not mix `llvmlite` keep work into `ny-llvm` mainline implementation notes

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
  - `docs/development/current/main/phases/phase-29bq/README.md`

## Notes

- `target/**`, `artifacts/**`, and `dist/**` are artifact roots only.
- migration tasks stay in `CURRENT_TASK.md`, `15-Workstream-Map.md`, `design/kernel-implementation-phase-plan-ssot.md`, and phase docs.
- do not create a new "rough order" SSOT unless `kernel-implementation-phase-plan-ssot.md` becomes structurally overloaded.
