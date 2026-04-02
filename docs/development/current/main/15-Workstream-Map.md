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
     - `phase-30x backend surface simplification` (landed)
     - `phase-31x engineering lane isolation`
   - current read:
     - `K2-core` is closed
     - `K2-wide` boundary-shrink lock-down is closed enough for handoff
     - `zero-rust default operationalization` is landed
     - `stage2plus entry / first optimization wave` is accepted
     - `phase-30x backend surface simplification` is landed
     - current active lane is `phase-31x engineering lane isolation`
     - `hako.osvm.reserve_bytes_i64` / `commit_bytes_i64` / `decommit_bytes_i64` are already landed
     - boundary audit result: `RuntimeDataBox.delete` is still absent; delete stays on the `MapBox -> RawMap -> nyash.map.delete_hh` lane
     - `phase-29x` cleanup is landed: semantic proof/archive recut, helper deletion, and owner-facade slimming are closed
     - current backend surface is role-first:
       - `llvm/exe` = `product`
       - `rust-vm` = `engineering(stage0/bootstrap + tooling keep)`
       - `vm-hako` = `reference/conformance`
       - `wasm` = `experimental`
     - `rust-vm` internal pressure is still deep in bootstrap/selfhost, plugin/macro/dev tooling, smoke/test, and docs/help
     - dangerous early flips remain around launcher/default/orchestrator sites
     - `phase-30x` settled ownership and docs/artifact/smoke reading
     - `phase-31x` now handles actual source/smoke cleanup
     - `31xA` landed and fixed `tools/engineering/**` as the engineering home
     - `31xB1` landed and moved `run_vm_stats.sh`
     - `31xB2` landed and moved `parity.sh`
     - `31xC` landed and fixed the shared helper family as `keep here`
     - current active micro task is `31xD1 orchestrator keep vs rehome split`
     - next queued micro task is `31xD2 docs and live path repoint`
     - cleanup rule is `rehome -> shim -> drain -> delete`
     - no-touch-first remains on default/dispatch/selfhost/orchestrator surfaces
     - axis and lane detail is canonical in the SSOTs and backend-lane docs
   - phase-31x engineering lane isolation table:

     | Item | State |
     | --- | --- |
     | Now | `phase-31x engineering lane isolation` |
     | Blocker | `none` |
     | Next | `31xD1 orchestrator keep vs rehome split` |
   - engineering isolation bands:

     | Band | State |
     | --- | --- |
     | Now | `31xD orchestrator isolation prep` |
     | Next | `31xE shim drain and legacy sweep` |
     | Later | `shared helper follow-up` |
   - engineering isolation waves:

     | Wave | Status | Read as |
     | --- | --- | --- |
     | `31xA engineering home lock` | landed | switch active lane and fix `tools/engineering/**` as canonical home |
     | `31xB low-blast tool rehome` | landed | move low-blast engineering tools off the top-level front |
     | `31xC shared helper family inventory` | landed | decide keep / rehome / archive for helper family |
     | `31xD orchestrator isolation prep` | active | split no-touch-first orchestrators into keep vs later rehome |
     | `31xE shim drain and legacy sweep` | queued | delete/archive after moved paths are drained |
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
  - `phase-30x` backend surface simplification (landed precursor)
  - `phase-31x` engineering lane isolation
- Active backend surface tasks:
  - `31xD orchestrator isolation prep`
- Queued backend surface tasks:
  - `31xE shim drain and legacy sweep`
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

1. keep `phase-31x` exact and orchestrator-first after the shared-helper keep decision
2. keep `phase-30x` landed as the ownership-flip precursor
3. keep `phase-29x` landed as the backend-owner precursor lane
4. keep `phase-29bq` active as failure-driven / blocker-none lane

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
