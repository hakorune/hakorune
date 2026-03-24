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

1. `phase-29cu`
   - active implementation lane
2. `phase-29cj`
   - formally close-synced
3. `phase-29y`
   - parked / monitor-only
4. `phase-29ct`
   - stop-line reached

## Bootstrap-Retire Now

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

1. keep `phase-29cj` closed unless a new exact disappearing leaf appears
2. keep `phase-29cu` active for the next exact Rune leaf
3. land selected-entry transport shim retirement with a canonical non-shim root-entry carrier
4. pin the remaining future Rune reopen:
   - `.hako` declaration-local full carrier parity beyond root-entry transport
5. choose the next active implementation lane only after the Rune shim-retirement cut is pinned

## Active Lane

- `phase-29cu` remains active
- landed:
  - Rust parser gate + declaration-local attrs
  - `.hako` parser grammar/arg-shape parity
  - `.hako` statement/program-route invalid-placement fail-fast
  - Rust function-target placement / ABI-facing verifier contract
  - `.hako` selected-entry shim value-contract parity for `CallConv("c")` / `Ownership(owned|borrowed|shared)`
  - Rust direct MIR `attrs.runes` carrier
  - `.hako` source-route selected-entry transport shim (transitional keep)
  - `.hako` compiler/mirbuilder generic function-rune carrier from `defs[].attrs.runes`
  - `.hako` compiler/mirbuilder function-name attrs injection
  - `ny-llvmc` selected-entry `Symbol` / `CallConv` consumer
- lane state:
  - `reopen W1 landed`
- next exact leaf:
  - retire the selected-entry transport shim
  - replace it with a canonical non-shim root-entry carrier
- planned future reopen after that:
  - `.hako` full declaration-local MIR parity beyond root-entry transport
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
- `MirBuilderBox.hako` is now treated as a near-thin-floor route-sequencing owner
- `stage1_cli_env.hako`, `stage1_cli.hako`, and `launcher.hako` now keep payload/input/I/O side effects behind same-file helpers, and the last raw subcmd / checked payload leaves are landed
- `vm-hako` is frozen as monitor-only; throughput probes are archived evidence, not current blockers

## Read Order

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/10-Now.md`
4. active phase README
