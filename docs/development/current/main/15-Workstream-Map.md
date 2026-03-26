---
Status: Active
Date: 2026-03-26
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

1. `phase-29bq`
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
2. `phase-29ck`
   - active follow-up / route-ready
   - `Stage0 = llvmlite` keep lane / `Stage1 = ny-llvmc(boundary pure-first)` mainline split is fixed
   - current exact blocker: `none` for the current kilo entry
   - current exact front: `P16-STAGE1-CANONICAL-MIR-CUTOVER.md`
   - current reading: route correction is landed, and next work is benchmark-guided leaf optimization on `ny-llvmc(boundary)`
   - `.hako` remains the preferred Stage1 canonical MIR authority and Rust stays a thin seam target
   - current `vm-hako` LLVM/exe proof is manual monitor only, not active acceptance
3. `phase-29ci`
   - formally close-synced
   - `Program(JSON v0)` boundary retirement / `MIR(JSON v0)` line unification is complete for the accepted keep set
   - helper-local slices through W14 are landed
   - smoke-tail caller buckets through W18 are landed
   - `phase2044` / `phase2160` thin wrapper families are monitor-only keeps
   - `phase2170` default MIR-file verify wrappers are landed
   - legacy `hv1_mircall_*` wrappers remain explicit keeps
   - reopen only on a new exact gap or explicit hard-delete resumption
4. `phase-29cu`
   - formally close-synced
5. `phase-29cj`
   - formally close-synced
6. `phase-29y`
   - parked / monitor-only
7. `phase-29ct`
   - stop-line reached

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

1. keep `phase-29bq` active as failure-driven / blocker-none lane
2. keep `phase-29ck` focused on the Stage1 MIR dialect cutover until the `.hako` producer stop-line is cleared and Rust is reduced to a thin materializer seam
3. keep `phase-29ci` / `phase-29cu` / `phase-29cj` closed unless an exact gap reappears
4. treat `phase2044` / `phase2160` thin wrapper families and `phase2170/hv1_mircall_*` as explicit keeps, not active caller-debt buckets

## Active Lane

- `phase-29bq` is active again
- active reading:
  - selfhost `.hako` migration remains `mirbuilder first / parser later`
  - current blocker is `none`
  - promotion is failure-driven only
- current lane rule:
  - use `29bq-90/91/92/113/114/115` as the operational SSOT set
  - keep the lane blocker-none until the next exact blocker is captured
  - do not promote a broader leaf without first pinning the next blocker
- guard rails:
  - keep compiler-expressivity-first policy
  - keep selfhost migration docs-first / failure-driven
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
4. active phase README
