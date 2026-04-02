---
Status: Active
Date: 2026-04-02
Scope: current mainline / secondary lanes / parked lanes の one-screen map。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md
---

# Workstream Map

## Purpose

- current lane の順番と残りの見通しだけを 1 画面で読む。
- canonical rough task order is owned by `design/kernel-implementation-phase-plan-ssot.md`.
- this file is the operational one-screen mirror, not a competing SSOT.

## Current Order

1. `policy-refresh`
   - active docs/policy lane
   - active sequence:
     - `stage / docs / naming`
     - `K1 done-enough stop-line`
     - `K2-core accepted stop-line`
     - `zero-rust default operationalization` (landed)
     - `stage2plus entry / first optimization wave` (accepted)
     - `phase-29x backend owner cutover prep` (landed)
     - `phase-30x backend surface simplification`
   - current read:
     - `K2-core` is closed
     - `K2-wide` boundary-shrink lock-down is closed enough for handoff
     - `zero-rust default operationalization` is landed
     - `stage2plus entry / first optimization wave` is accepted
     - current active lane is `phase-30x backend surface simplification`
     - `hako.osvm.reserve_bytes_i64` / `commit_bytes_i64` / `decommit_bytes_i64` are already landed
     - boundary audit result: `RuntimeDataBox.delete` is still absent; delete stays on the `MapBox -> RawMap -> nyash.map.delete_hh` lane
     - `phase-29x` cleanup is landed: semantic proof/archive recut, helper deletion, and owner-facade slimming are closed
     - current backend surface is role-first:
       - `llvm/exe` = `product`
       - `rust-vm` = `engineering/bootstrap`
       - `vm-hako` = `reference/conformance`
       - `wasm` = `experimental`
     - `rust-vm` internal pressure is still deep in bootstrap/selfhost, plugin/macro/dev tooling, smoke/test, and docs/help
     - dangerous early flips remain around launcher/default/orchestrator sites
     - `30xA1`, `30xA2`, `30xB1-30xB4`, and `30xC1` are landed
     - `30xC2` grouped plugin/macro/tooling pressure into `engineering/tooling keep` plus `manual residue watch`
     - `30xC3` grouped smoke/test pressure into `engineering smoke keep`, `mixed orchestrator keep`, and `manual residue watch`
     - `30xC4` grouped docs/help pressure into `rewrite in 30xE`, `engineering docs keep`, and `stale help snapshot watch`
     - `30xD1` froze raw CLI default token and central dispatch as no-touch-first surfaces
     - `30xD2` froze selfhost/stage1 wrappers as no-touch-first bootstrap surfaces
     - current active micro task is `30xD3 plugin/smoke orchestrator freeze`
     - next queued micro task is `30xE1 README/README.ja prep`
     - `phase29cc_wsm` families are experimental smoke lanes, not product-mainline evidence
     - `compat/llvmlite-monitor-keep` is compat/probe keep only, not `llvm/exe` product evidence
     - `tools/smokes/v2/configs/matrix.conf` now reads `vm/llvm` as engineering/product only
     - `30xC1` found no archive/delete candidate in bootstrap/selfhost
     - review intake detail stays in `phase-30x`
     - axis and lane detail is canonical in the SSOTs and backend-lane docs
   - phase-30x backend surface simplification table:

     | Item | State |
     | --- | --- |
     | Now | `phase-30x backend surface simplification` |
     | Blocker | `none` |
     | Next | `30xD3 plugin/smoke orchestrator freeze` |
   - cleanup bands:

     | Band | State |
     | --- | --- |
     | Now | `30xD3 plugin/smoke orchestrator freeze` |
     | Next | `30xE1 README/README.ja prep` |
     | Later | `30xE2-30xF` |
   - cleanup waves:

     | Wave | Status | Read as |
     | --- | --- | --- |
     | `30xA role taxonomy lock` | landed | root docs and phase docs use the same backend roles |
     | `30xB smoke taxonomy split` | landed | role-first smoke/gate reading |
     | `30xC rust-vm dependency inventory` | active | internal `--backend vm` pressure map |
     | `30xD dangerous-early-flip lock` | queued | launcher/default/orchestrator freeze |
     | `30xE user-facing main switch prep` | queued | `llvm/exe` first docs/help/examples |
     | `30xF backend default decision gate` | queued | decide raw CLI default last |
2. `phase-29bq`
   - active selfhost lane
   - `mirbuilder first / parser later`
   - current blocker: `none`
   - failure-driven steady-state
3. `phase-29x`
   - landed precursor lane
   - keep exact detail in phase README and backend-owner SSOT
4. `phase-29ck`
   - active follow-up / docs-first exact front
5. `phase-29ci`
   - close-synced
6. `phase-29cu`
   - close-synced
7. `phase-29cj`
   - close-synced
8. `phase-29y`
   - parked / monitor-only
9. `phase-29ct`
   - stop-line reached

## Next Horizon Inventory

- Active big tasks:
  - `stage / docs / naming` fixation
  - `zero-rust` default operationalization (landed)
  - `stage2plus` entry / first optimization wave (accepted)
  - `phase-30x` backend surface simplification
- Active backend surface tasks:
  - `30xA role taxonomy lock`
  - `30xB smoke taxonomy split`
  - `30xC rust-vm dependency inventory`
- Parked big tasks:
  - broad widening beyond the current `K2-wide` narrow slices
  - broad `Map` structural expansion
- Active small tasks:
  - Map evidence bundle maintenance
  - current `zero-rust` entrypoint sync
- Parked small tasks:
  - warning debt sweep
  - TODO cleanup / ignore triage
  - hotspot cleanup outside the active pilot boundary

## Exact Next

1. keep `phase-30x` exact and docs-first
2. keep `phase-29x` landed as the precursor lane
3. keep `phase-29bq` active as failure-driven / blocker-none lane
4. keep closed lanes closed unless a new exact gap appears

## Active Lane

- `policy-refresh` is active
- current lane rules:
  - keep `stage0/stage1/stage2-mainline/stage2+` as build/distribution vocabulary
  - keep `K0/K1/K2` as build/runtime stage vocabulary
  - keep task packs separate from `K-axis`
  - keep backend surfaces role-first: `product / engineering / reference / experimental`
  - keep `RuntimeDataBox` facade-only
  - keep `.hako` capability facades distinct from native keep leaf glue
  - keep artifact roots binary/bundle only; migration task notes stay in root/docs/phase owners
  - keep backend lane vocabulary in the backend-lane docs

## Parked / Stop-Line

- `phase-29y`
  - reopen only on exact runtime gate / bootstrap-proof failure
- `phase-29ct`
  - docs/task lane only until explicit reopen
- `phase-21_5` perf
  - parked reopen
- `phase-29cs`
  - parked naming cleanup

## Recently Landed

- `K2-core` accepted stop-line is closed
  - `K2-wide` landed rows now include:
    - `RawMap` first slice
    - `RawMap.clear`
    - `RawMap.remove/delete`
    - `hako.atomic.fence_i64`
    - `hako.tls.last_error_text_h`
    - `hako.gc.write_barrier_i64`
    - `hako.osvm.reserve_bytes_i64` / `commit_bytes_i64` / `decommit_bytes_i64`
    - `hako_alloc` handle reuse and GC trigger policy rows
- current follow-up is boundary-shrink lock-down

## Read Order

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/10-Now.md`
4. `docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md`
5. active phase README
