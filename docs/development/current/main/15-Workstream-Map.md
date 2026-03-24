---
Status: Active
Date: 2026-03-24
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

1. `phase-29cj`
   - mainline
   - status: `formal-close-sync-ready`
2. `phase-29cu`
   - secondary active lane
3. `phase-29y`
   - parked / monitor-only
4. `phase-29ct`
   - stop-line reached

## Mainline Now

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
  - `LauncherArtifactIoBox`

## Exact Next

1. re-inventory near-thin-floor owners:
   - `lang/src/mir/builder/MirBuilderBox.hako`
   - `lang/src/runner/stage1_cli_env.hako`
   - `lang/src/runner/stage1_cli.hako`
   - `lang/src/runner/launcher.hako`
2. choose one exact disappearing leaf only if it is visible
3. otherwise freeze `phase-29cj` as formally close-synced
4. after that, return implementation focus to `phase-29cu`

## Secondary Active Lane

- `phase-29cu` remains active
- fixed order:
  1. Rust parser gate
  2. `.hako` parser parity
  3. declaration-local `attrs.runes` -> direct MIR carrier
  4. verifier / consumer activation
  5. `ny-llvmc` selected-entry consumer
- guard rails:
  - `Program(JSON v0)` stays no-widen
  - `llvmlite` stays noop/compat

## Parked / Stop-Line

- `phase-29y`
  - parked
  - reopen only on exact runtime gate / bootstrap-proof failure
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
- `MirBuilderBox.hako` is close to a pure route-sequencing owner
- `stage1_cli_env.hako`, `stage1_cli.hako`, and `launcher.hako` now keep payload/input/I/O side effects behind same-file helpers
- `vm-hako` is frozen as monitor-only; throughput probes are archived evidence, not current blockers

## Read Order

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/10-Now.md`
4. active phase README
