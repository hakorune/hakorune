---
Status: Active
Date: 2026-04-04
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
     - current active lane is `phase-73x emit_mir_mainline blocker follow-up`
     - current active micro task is `73xB1 focused source fix`
     - ranked successor corridor after `68x` is:
       - `69x rust runner product/keep/reference recut`
       - `70x caller-zero archive sweep`
     - `hako.osvm.reserve_bytes_i64` / `commit_bytes_i64` / `decommit_bytes_i64` are already landed
     - boundary audit result: `RuntimeDataBox.delete` is still absent; delete stays on the `MapBox -> RawMap -> nyash.map.delete_hh` lane
     - `phase-29x` cleanup is landed: semantic proof/archive recut, helper deletion, and owner-facade slimming are closed
     - current backend surface is archive/historical-first:
       - `llvm/exe` = `product`
       - `rust-vm` = `historical archive evidence / proof-compat keep`
       - `vm-hako` = `reference/conformance`
       - `wasm` = `experimental`
     - remaining `rust-vm` pressure is explicit compat/proof keep plus archive evidence, not a live owner lane
     - dangerous early flips remain around launcher/default/orchestrator sites
     - `phase-30x` settled ownership and docs/artifact/smoke reading
        - `phase-31x` landed low-blast engineering rehome and shim drain
        - `phase-32x` landed mixed-owner source/smoke split and raw default/token defer
        - `phase-33x` landed and fixed helper-family path truth
        - `phase-34x` landed and fixed stage0 shell residue split
        - `phase-35x` landed and fixed Stage-A compat route thinning
        - `phase-36x` landed and handled selfhost source / stage1 bridge split
        - `phase-37x` landed bootstrap owner split
        - `phase-38x` landed cleanup/archive sweep
     - `phase-39x` landed stage0 vm gate thinning
     - `phase-40x` landed the archive candidate sweep and drained top-level vm-facing shims
     - `phase-41x` landed and hardened the remaining direct/core route
     - `phase-42x` started vm caller starvation and direct/core owner migration and is now landed
     - `phase-43x` is landed and selected the next source lane after handoff
     - `phase-44x` is landed and followed the direct/core owner route
     - `phase-45x` is landed and trimmed the residual vm owner surfaces
     - `phase-46x` is landed and selected `stage0/runtime direct-core finalization`
     - `phase-47x` is landed and its helper-route finalization has been handed off
     - `phase-48x` is landed and the smoke/source cleanup handoff is complete
     - `phase-49x` is landed and the remaining current-doc wording / compat-route narration is closed
     - `phase-51x` is landed and the canonical compat-codegen bucket now lives under the archive path
     - `phase-52x` is landed and polishes archive/historical labeling after source cleanup
     - `phase-53x` is landed, `phase-54x` is landed, `phase-55x` is landed, `phase-56x` is landed, `phase-57x` is landed, `phase-58x` is landed, `phase-59x` is landed, `phase-60x` is landed, `phase-61x` is landed, `phase-62x` is landed, `phase-63x` is landed, `phase-64x` is landed, `phase-65x` is landed, `phase-66x` is landed, `phase-67x` is landed, `phase-68x` is landed, `phase-69x` is landed, and `phase-70x` is active
     - `48xA1` inventory lock is landed; `48xA2` proof-only / compat keep classification is landed
     - `48xB1` smoke script stale-route cleanup is landed; `48xB2` proof-only smoke gate lock is landed; `48xC1` source helper stale-route cleanup is landed; `48xC2` vm.rs / vm_fallback thin keep trim is landed
     - `47xA1` landed, `47xA2` landed, `47xA3` landed, `47xB1` landed, `47xB2` landed, `47xB3` landed, `47xC1` landed, `47xC2` landed, `47xC3` landed, `47xD1` landed, `47xD2` landed, and `47xD3` landed
     - exact order is `50xA1/A2 -> 50xB1/B2 -> 50xC1/C2 -> 50xD1`
     - `42xA1` landed: lock starvation targets and active migration surfaces
     - `42xA2` landed: freeze proof-only VM gates as explicit do-not-grow keeps
     - `42xB1` landed: starve `selfhost_build.sh` downstream callers toward direct/core helper owners
     - `42xB2` landed: trim `run.sh` day-to-day route pressure so it stays route-only facade
     - `42xC1` landed: drain `child.rs` until it owns spawn/capture/timeout/JSON selection only
     - `42xC2` landed: split `vm.rs` preflight/source-prepare ownership out of the broad execution path
     - `42xC3` landed: move shared vm user-factory ownership out of `vm.rs` / `vm_fallback.rs` and drain fallback callers
     - `42xC4` landed: hold `core.hako` compat lane as explicit no-widen while direct/core routes take new work
     - `42xD1` landed: proof / closeout
     - `43xA1` landed: candidate lane shortlist
     - `43xA2` landed: successor lane decision
     - `43xD1` landed: proof / closeout
     - `44xA1` landed: stage-b direct/core target lock
     - `44xA2` landed: `selfhost_build_stageb.sh` direct/core-first cutover
     - `44xB1` landed: `selfhost_run_routes.sh` runtime default cutover
     - `44xB2` landed: `run.sh` direct route fallback explicitization
     - `44xC1` landed: `stage0_capture.rs` route-neutral builder split
     - `44xC2` landed: `stage_a_route.rs` / compat caller switch
     - `44xD1` landed: `run_stageb_compiler_vm.sh` proof-only demotion
     - `44xE1` landed: proof / closeout
      - plain reading: keep `rust-vm` as proof/compat keep, not mainline ownership
      - success condition: keep direct/core routes canonical, keep proof-only VM gates frozen, and stop day-to-day callers from feeding vm routes
        - `39xA1` landed and fixed caller inventory for `selfhost_build.sh` / `run_stageb_compiler_vm.sh` / `run.sh`
        - `39xA2` landed and classifies route ownership
        - `39xB1` landed and selected the direct bootstrap mainline
        - `39xB2` landed and froze the explicit vm keep set
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
   - phase-73x emit_mir_mainline blocker follow-up table:

     | Item | State |
     | --- | --- |
     | Now | `73xB1 focused source fix` |
     | Blocker | `tracked focused residual red on top-level stage1_cli facade` |
     | Next | `73xC1 proof bundle refresh` |
     | After Next | `73xD1 proof / closeout` |
   - rust-vm corridor outcome:

     | Item | State |
     | --- | --- |
     | `mainline retirement` | achieved |
     | `full source retirement` | deferred |
     | `residual explicit keep` | frozen |
   - stage0 shell residue bands:

     | Band | State |
     | --- | --- |
     | Now | `73xB1 focused source fix` |
     | Next | `73xC1 proof bundle refresh` |
     | Later | `73xD1 proof / closeout` |
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
- `phase-39x` stage0 vm gate thinning (landed precursor)
  - `phase-41x` stage0 direct/core route hardening (landed)
     - Active backend surface tasks:
     - `51xA2 proof-only / archive-later classification`
     - Queued backend surface tasks:
     - `55xA2 backend/default/help exposure freeze`
     - `59xA1 route-surface inventory lock`
     - `60xA1 proof/compat keep inventory lock`
     - `60xA2 compat keep boundary freeze`
     - `60x proof/compat keep pruning continuation`
    - `kilo` optimization wave (far future; not the next lane)
- Post-`37xD1` cleanup:
  - drained shim / legacy embedded smoke archive sweep
  - first landed move: `tools/stage1_smoke.sh` -> `tools/archive/legacy-selfhost/stage1_embedded_smoke.sh`
- Parked big tasks:
  - broad widening beyond the current `K2-wide` narrow slices
  - broad `Map` structural expansion
  - `kilo` optimization wave (far future; not the next lane)
- Active small tasks:
  - Map evidence bundle maintenance
  - current `zero-rust` entrypoint sync
- Parked small tasks:
  - warning debt sweep
  - TODO cleanup / ignore triage
  - hotspot cleanup outside the active pilot boundary

## Exact Next

1. keep `phase-47x` exact through stage0/runtime direct-core finalization
2. keep `phase-43x` landed as the successor-lane selection precursor
3. keep `phase-42x` landed as the vm caller starvation / direct-core owner migration precursor
4. keep `phase-41x` landed as the route-hardening precursor
5. keep `phase-40x` landed as the vm archive candidate precursor

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
