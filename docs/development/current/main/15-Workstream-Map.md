---
Status: Active
Date: 2026-04-03
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
        - `phase-31x engineering lane isolation` (landed)
        - `phase-32x product / engineering split`
   - current read:
     - `K2-core` is closed
     - `K2-wide` boundary-shrink lock-down is closed enough for handoff
     - `zero-rust default operationalization` is landed
     - `stage2plus entry / first optimization wave` is accepted
     - `phase-30x backend surface simplification` is landed
     - current active lane is `phase-34x stage0 shell residue split`
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
        - `phase-31x` landed low-blast engineering rehome and shim drain
        - `phase-32x` landed mixed-owner source/smoke split and raw default/token defer
        - `phase-33x` landed and fixed helper-family path truth
        - `phase-34x` now handles stage0 shell residue split
        - `34xA1` landed and fixed exact `child.rs` shell/process/capture residue
        - `34xA2` landed and fixed `stage1_cli/core.hako` raw compat residue and dispatch split
        - `34xA3` is active and pins `core_executor` as the direct `MIR(JSON)` owner
        - current active micro task is `34xA3 core_executor takeover seam lock`
        - cleanup rule is `split/rehome/drain -> delete`
     - no-touch-first remains on default/dispatch/selfhost/orchestrator surfaces
     - axis and lane detail is canonical in the SSOTs and backend-lane docs
   - phase-34x stage0 shell residue split table:

     | Item | State |
     | --- | --- |
     | Now | `phase-34x stage0 shell residue split` |
     | Blocker | `none` |
     | Next | `34xB1 child runner thinning` |
   - shared helper family bands:

     | Band | State |
     | --- | --- |
     | Now | `34xA3 core_executor takeover seam lock` |
     | Next | `34xB1 child runner thinning` |
     | Later | `raw backend default/token follow-up lane` |
   - shared helper family waves:

     | Wave | Status | Read as |
     | --- | --- | --- |
     | `33xA helper family inventory` | landed | fix exact keep/rehome/shim-only reading |
     | `33xB hako_check family path truth` | landed | move family-local smoke helpers under `tools/hako_check/**` |
     | `33xC emit_mir thin wrapper path truth` | landed | fix thin wrappers as route-preset shims before touching broad helper keep |
     | `33xD closeout/docs cleanup` | landed | close the helper-family recut after broad keep truth is fixed |
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
  - `phase-31x` engineering lane isolation (landed precursor)
  - `phase-32x` product / engineering split (landed precursor)
  - `phase-34x` stage0 shell residue split
- Active backend surface tasks:
  - `34xA3 core_executor takeover seam lock`
- Queued backend surface tasks:
  - `34xB1 child runner thinning`
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

1. keep `phase-34x` exact through the stage0 shell residue split
2. keep `phase-33x` landed as the helper-family precursor
3. keep `phase-32x` landed as the mixed-owner split precursor
4. keep `phase-31x` landed as the engineering rehome precursor
5. keep `phase-30x` landed as the ownership-flip precursor

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
