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
     - current active lane is `phase-38x cleanup/archive sweep`
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
        - `phase-34x` landed and fixed stage0 shell residue split
        - `phase-35x` landed and fixed Stage-A compat route thinning
        - `phase-36x` landed and handled selfhost source / stage1 bridge split
        - `phase-37x` landed bootstrap owner split
        - `phase-38x` now handles cleanup/archive sweep
        - `34xA1` landed and fixed exact `child.rs` shell/process/capture residue
        - `34xA2` landed and fixed `stage1_cli/core.hako` raw compat residue and dispatch split
        - `34xA3` landed and pinned `core_executor` as the direct `MIR(JSON)` owner
        - `34xB1` landed and split `child.rs` shell residue into route-neutral private helpers
        - `34xC1` landed and fixed raw compat no-widen on `run_program_json`
        - `34xD1` landed and proof-pins the direct `MIR(JSON)` handoff with focused unit tests
        - `35xA1` landed and rehomes captured Stage-A payload resolution into `stage_a_compat_bridge.rs`
        - `35xA2` landed and moves Stage-A child spawn/setup into `stage_a_route.rs`
        - `35xB1` landed and fixes Program(JSON v0) as explicit/no-widen compat only
        - `35xC1` landed and proof-pins the direct-vs-compat Stage-A route
        - `36xA1` landed and moves source extension/read/merge/tmp staging into `source_prepare.rs`
        - `36xA2` landed and fixes `selfhost.rs` as route ordering / macro gate / terminal accept owner
        - `36xB1` landed and moves raw `emit mir-json` glue into `raw_subcommand_emit_mir.hako`
        - `36xB2` landed and moves raw `run` glue into `raw_subcommand_run.hako`
        - `36xC1` landed and fixes the split as evidence instead of reopening compat ownership
        - `37xD1` landed on focused proof: `cargo check`, `git diff --check`, `phase29ci_selfhost_build_exe_consumer_probe.sh`, and `stage1_mainline_smoke.sh`
        - `37xA` prioritizes `selfhost_build.sh` owner split over broader cleanup
        - `37xB` follows with `build.rs` product/engineering split
        - `37xC` freezes explicit engineering keep before caller-drain work
        - `37xD` restores canonical proof after the speed-first split
        - `38xA` archives legacy embedded smoke out of the live surface
        - `38xB` sweeps delete-ready drained shims
        - `38xC` freezes archive-later shims until doc drain lands
        - cleanup rule is `split/rehome/drain -> delete`
        - temporary smoke red is acceptable inside `37xA` / `37xB`; compile/diff checks stay mandatory
     - no-touch-first remains on default/dispatch/selfhost/orchestrator surfaces
     - axis and lane detail is canonical in the SSOTs and backend-lane docs
   - phase-34x stage0 shell residue split table:

     | Item | State |
     | --- | --- |
     | Now | `phase-38x cleanup/archive sweep` |
     | Blocker | `none` |
     | Next | `38xD1 closeout` |
   - stage0 shell residue bands:

     | Band | State |
     | --- | --- |
     | Now | `phase-38x cleanup/archive sweep` |
     | Next | `38xD1 closeout` |
     | Later | `next source lane selection` |
   - stage0 shell residue waves:

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
  - `phase-36x` selfhost source / stage1 bridge split (landed precursor)
  - `phase-37x` bootstrap owner split (landed precursor)
  - `phase-38x` cleanup/archive sweep (active)
- Active backend surface tasks:
  - `phase-38x cleanup/archive sweep`
- Queued backend surface tasks:
  - `next source lane selection`
- Post-`37xD1` cleanup:
  - drained shim / legacy embedded smoke archive sweep
  - first landed move: `tools/stage1_smoke.sh` -> `tools/archive/legacy-selfhost/stage1_embedded_smoke.sh`
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

1. keep `phase-38x` exact through the cleanup/archive sweep
2. keep `phase-37x` landed as the bootstrap-owner precursor
3. keep `phase-36x` landed as the selfhost/stage1 owner split precursor
4. keep `phase-35x` landed as the Stage-A compat precursor
5. keep `phase-34x` landed as the stage0 shell-residue precursor

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
