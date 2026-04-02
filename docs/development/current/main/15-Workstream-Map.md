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
        - `phase-31x engineering lane isolation` (landed)
        - `phase-32x product / engineering split`
   - current read:
     - `K2-core` is closed
     - `K2-wide` boundary-shrink lock-down is closed enough for handoff
     - `zero-rust default operationalization` is landed
     - `stage2plus entry / first optimization wave` is accepted
     - `phase-30x backend surface simplification` is landed
     - current active lane is `phase-32x product / engineering split`
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
        - `phase-32x` now handles mixed-owner source/smoke split
        - `32xA1` landed and fixed `build.rs` as the first mixed-owner source target
        - `32xA2` landed and fixed `phase2100/run_all.sh` as the first thick smoke-aggregator target
        - `32xB2` landed and thinned `src/runner/build.rs` by helper-first extraction
        - `32xC1` landed and fixed the exact `phase2100` role buckets
        - `32xC2` landed and made `phase2100/run_all.sh` a thin meta-runner over role sub-runners
        - current active micro task is `32xD1 bootstrap_selfhost_smoke caller drain map`
        - next queued micro task is `32xD2 plugin_v2_smoke caller drain map`
        - cleanup rule is `split/rehome/drain -> delete`
     - no-touch-first remains on default/dispatch/selfhost/orchestrator surfaces
     - axis and lane detail is canonical in the SSOTs and backend-lane docs
   - phase-32x product / engineering split table:

     | Item | State |
     | --- | --- |
     | Now | `phase-32x product / engineering split` |
     | Blocker | `none` |
     | Next | `32xD1 bootstrap_selfhost_smoke caller drain map` |
   - product / engineering split bands:

     | Band | State |
     | --- | --- |
     | Now | `32xD1 bootstrap_selfhost_smoke caller drain map` |
     | Next | `32xD2 plugin_v2_smoke caller drain map` |
     | Later | `32xD1/D2 top-level orchestrator rehome prep` |
   - product / engineering split waves:

     | Wave | Status | Read as |
     | --- | --- | --- |
     | `32xA mixed-owner inventory` | landed | inventory exact mixed-owner source/smoke targets |
     | `32xB build.rs split plan` | landed | split product build and engineering build ownership |
     | `32xC phase2100 role split plan` | landed | split the thick smoke aggregator by role |
     | `32xD top-level orchestrator rehome prep` | active | drain callers before moving remaining top-level keeps |
     | `32xE direct-route takeover prep` | queued | reduce shell-based `--backend vm` residues behind dedicated seams |
     | `32xF shared helper follow-up gate` | queued | reopen helper-family recut only on a dedicated lane |
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
  - `phase-32x` product / engineering split
- Active backend surface tasks:
  - `32xD1 bootstrap_selfhost_smoke caller drain map`
- Queued backend surface tasks:
  - `32xD2 plugin_v2_smoke caller drain map`
  - `32xD2 plugin_v2_smoke caller drain map`
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

1. keep `phase-32x` exact through the mixed-owner split plan
2. keep `phase-31x` landed as the engineering rehome precursor
3. keep `phase-30x` landed as the ownership-flip precursor
4. keep `phase-29x` landed as the backend-owner precursor lane
5. keep `phase-29bq` active as failure-driven / blocker-none lane

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
