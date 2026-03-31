---
Status: Active
Date: 2026-03-31
Scope: current mainline / secondary lanes / parked lanes の one-screen map。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/phases/phase-29cj/README.md
  - docs/development/current/main/phases/phase-29cu/README.md
  - docs/development/current/main/phases/phase-29y/README.md
  - docs/development/current/main/phases/phase-29ct/README.md
---

# Workstream Map

## Purpose

- current lane の順番と残りの見通しだけを 1 画面で読む。
- 実装 detail や長い履歴は phase README に逃がす。
- `CURRENT_TASK.md` は root anchor、この文書は docs 側の作業順 map。

## Current Order

1. `policy-refresh`
   - active docs/policy lane
   - visible sequence: `Rune lane (parallel)` -> `K0 -> K-migration` -> `K2-core acceptance lock` -> `RawMap deferred in K2-wide`
   - keep `stage` vocabulary fixed and add compressed `K-axis` as the current operational reading
   - keep `Rune lane` docs (`@rune` canonical surface / `attrs.runes`) synchronized as a parallel compiler-contract lane
   - `K2-core acceptance lock` is the next structural step; `RawArray` remains first and `RuntimeData facade-only` stays fixed
   - `K2-core` smoke/evidence gate is the existing `nyash_kernel` RawArray contract test set, not a new benchmark lane
   - semantic `MapBox` work is already `K1 done-enough`; map perf stays regression/evidence, not the next structural lane
   - execution order is `Rune lane (parallel)` plus `K0 -> K-migration`; `K2-core acceptance lock` sits before `RawMap` in `K2-wide`
2. `phase-29bq`
   - active selfhost lane
   - `mirbuilder first / parser later`
   - current blocker: `none`
   - latest landed blocker: `phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_blockexpr_min.hako`
   - operation mode: failure-driven / blocker-none steady-state
   - current exact implementation leaf: `none while blocker=none`
   - active read order:
     - `29bq-90-selfhost-checklist.md`
     - `29bq-91-mirbuilder-migration-progress-checklist.md`
     - `29bq-92-parser-handoff-checklist.md`
     - `29bq-113-hako-recipe-first-migration-lane.md`
     - `29bq-114-hako-cleanup-integration-prep-lane.md`
     - `29bq-115-selfhost-to-go-checklist.md`
3. `phase-29ck`
   - active follow-up / docs-first exact front
   - `Stage0 = llvmlite` keep lane / `Stage1 = ny-llvmc(boundary pure-first)` mainline split is fixed
   - current route-correction blocker: retired for the current kilo entry
   - current exact front: `P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md`
   - current reading: route correction is landed, and next work is docs-first live-route debug bundle + semantic window proof before another array leaf attempt on `ny-llvmc(boundary)`
   - `.hako` remains the preferred Stage1 canonical MIR authority and Rust stays a thin seam target
   - current `vm-hako` LLVM/exe proof is manual monitor only, not active acceptance
4. `phase-29ci`
   - formally close-synced
   - `Program(JSON v0)` boundary retirement / `MIR(JSON v0)` line unification is complete for the accepted keep set
   - helper-local slices through W14 are landed
   - smoke-tail caller buckets through W18 are landed
   - `phase2044` / `phase2160` thin wrapper families are monitor-only keeps
   - `phase2170` default MIR-file verify wrappers are landed
   - legacy `hv1_mircall_*` wrappers remain explicit keeps
   - reopen only on a new exact gap or explicit hard-delete resumption
5. `phase-29cu`
   - formally close-synced
6. `phase-29cj`
   - formally close-synced
7. `phase-29y`
   - parked / monitor-only
8. `phase-29ct`
   - stop-line reached

## Next Horizon Inventory

- Active big tasks:
  - `Rune lane (parallel, compiler-contract side)`
  - `K-migration` with `K2-core acceptance lock`
  - policy stabilization for the compressed `K-axis`
  - zero-rust default operationalization for daily/distribution
- Parked big tasks:
  - `K2-wide` follow-up (`RawMap`, capability widening, metal review)
  - broad `Map` structural expansion
- Active small tasks:
  - docs ladder sync
  - Rune docs/tag sync
  - Map evidence bundle maintenance
  - lane-local cleanup candidates only:
    - Rune lane: `src/parser/runes.rs`, `src/parser/statements/helpers.rs`, `src/stage1/program_json_v0.rs`, `src/macro/ast_json/roundtrip.rs`
    - RawArray lane: `crates/nyash_kernel/src/plugin/handle_cache.rs`, `crates/nyash_kernel/src/plugin/runtime_data_array_dispatch.rs`, `crates/nyash_kernel/src/plugin/array_slot_load.rs`, `crates/nyash_kernel/src/plugin/array_slot_store.rs`, `crates/nyash_kernel/src/plugin/array_slot_append.rs`
- Parked small tasks:
  - warning debt sweep
  - TODO cleanup / ignore triage
  - code-hotspot cleanup outside the active pilot boundary

## Boundary-Retire Snapshot

- live stop-line: `src/host_providers/mir_builder.rs::module_to_mir_json(...)`
- latest landed Rust cuts:
  - `Stage1ProgramJsonInput`
  - `Stage1ProgramJsonValue`
  - `Stage1ProgramJsonModuleHandoff`
  - `Stage1FinalizedMirModule`
  - `SourceProgramJsonAuthority`
  - `SourceProgramJsonOutputHandoff`
- latest landed `.hako` cuts:
  - `BuilderProgramJsonInputContractBox`
  - `BuilderFuncDefsGateBox`
  - `BuilderLoopForceRouteBox`
  - `BuilderUnsupportedTailBox`
  - `Stage1MirPayloadContractBox`
  - `Stage1CliProgramJsonInputBox`
  - `Stage1CliRawSubcommandInputBox`
  - `LauncherArtifactIoBox`
  - `LauncherPayloadContractBox`
- frozen near-thin-floor owners:
  - `MirBuilderBox.hako`
  - `stage1_cli_env.hako`
  - `stage1_cli.hako`
  - `launcher.hako`

## Exact Next

1. keep `policy-refresh` first until compressed `K-axis` / `zero-rust by default` / `plugin=cold lane only` wording is synced
2. keep `phase-29bq` active as failure-driven / blocker-none lane
3. keep `phase-29ck` focused on `P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md` until the live-route debug bundle and semantic `array_rmw_window` proof are fixed
4. keep `phase-29ci` / `phase-29cu` / `phase-29cj` closed unless an exact gap reappears
5. treat `phase2044` / `phase2160` thin wrapper families and `phase2170/hv1_mircall_*` as explicit keeps, not active caller-debt buckets

## Active Lane

- `policy-refresh` is active
- active reading:
  - keep `stage0/stage1/stage2-mainline/stage2+` as build/distribution vocabulary
  - keep `K0/K1/K2(core|wide)` as replacement-progress vocabulary
  - keep `Rune` visible as a parallel compiler-contract lane, not a serial step inside `K-axis`
  - freeze current collection wave as `K1 done-enough`
  - prep `K2-core RawArray` as the next structural target
- current lane rule:
  - read `kernel-replacement-axis-ssot.md` first
  - keep `Map` perf as regression/evidence; do not promote it into the next structural replacement lane
  - keep `plugin` as the cold dynamic loader noun only
- guard rails:
  - keep stage/build vocabulary and replacement vocabulary separate
  - keep selfhost migration docs-first / failure-driven on the secondary lane
  - do not reopen `phase-29ci` helper-local work without a new exact gap

## Parked / Stop-Line

- `phase-29y`
  - parked
  - reopen only on exact runtime gate / bootstrap-proof failure
  - `vm-hako` stays monitor/debug/bootstrap-proof only; any future interpreter lane must reopen separately
- `phase-29ct`
  - stop-line reached
  - docs/task lane only until explicit reopen
- `phase-21_5` perf
  - parked reopen
- `phase-29cs`
  - parked naming cleanup

## Recently Landed

- `build_surrogate.rs` is down to a typed dispatch shim
- `src/host_providers/mir_builder.rs` is now a façade above the Rust stop-line
- `MirBuilderBox.hako` is now treated as a near-thin-floor route-sequencing owner
- `stage1_cli_env.hako`, `stage1_cli.hako`, and `launcher.hako` now keep payload/input/I/O side effects behind same-file helpers, and the last raw subcmd / checked payload leaves are landed
- `launcher.hako` now keeps top-level route selection behind `LauncherDispatchBox`, so `HakoCli` is down to orchestration only
- `vm-hako` is frozen as monitor-only; throughput probes are archived evidence, not current blockers

## Read Order

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/10-Now.md`
4. `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
5. active phase README
