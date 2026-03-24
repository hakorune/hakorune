# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-25
Scope: repo root の再起動入口。詳細の status/phase 進捗は `docs/development/current/main/` を正本とする。

## Purpose

- root から最短で current blocker / active lane / next fixed order に到達する。
- 本ファイルは薄い入口に保ち、長い phase 履歴や retired lane detail は phase README / design SSOT へ逃がす。
- naming cleanup lane の単一正本は `docs/development/current/main/phases/phase-29cs/README.md`。
- substrate capability ladder lane の単一正本は `docs/development/current/main/phases/phase-29ct/README.md`。
- current-task history archive の単一正本は `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`。

## Quick Restart Pointer

- `docs/development/current/main/05-Restart-Quick-Resume.md`
- `docs/development/current/main/15-Workstream-Map.md`
- `git status -sb`
- `tools/checks/dev_gate.sh quick`

## Current blocker (SSOT)

- runtime lane is parked/monitor-only again; there is no active `vm-hako` throughput blocker.
- `phase-29cj` has completed its near-thin-floor reinventory and formal close sync.
- `phase-29cu` has completed its formal close sync for the narrow Rune v0 scope.
- next active implementation lane is `phase-29ci` reopen:
  - retire `Program(JSON v0)` from repo-wide external/bootstrap boundary
  - unify public/bootstrap boundary on `MIR(JSON v0)`
  - keep hard delete and broad internal removal for later waves

## Current Priority

1. active implementation lane: `phase-29ci`
   - status: `reopen W2 active`
   - scope: `Program(JSON v0)` boundary retirement + `MIR(JSON v0)` line unification
   - working rule:
     - public/bootstrap surfaces move to MIR-first now
     - `Program(JSON v0)` stays compat/internal keep only
     - hard delete is later
   - exact W2 target:
     - retire public wrapper/helper Program(JSON) surfaces
     - keep raw compat flags and explicit compat probes alive
2. close-synced Rune lane: `phase-29cu`
   - status: `formal-close-synced`
   - accepted narrow-scope current truth:
     - declaration-local `attrs.runes`
     - Rust direct MIR carrier
     - `.hako` source-route root-entry carrier via a real `defs[].Main.main.attrs.runes` entry
     - `.hako` compiler/mirbuilder generic function-rune carrier from `defs[].attrs.runes`
     - selected-entry `ny-llvmc` `Symbol` / `CallConv` semantics
   - future reopen only if `.hako` declaration-local full carrier parity resumes
3. close-synced mainline lane: `phase-29cj`
   - status: `formal-close-synced`
   - reopen only if a new exact disappearing leaf appears above the Rust stop-line or if deletion-prep explicitly resumes
4. parked / stop-line
   - `phase-29y`: parked monitor-only
   - `phase-29ct`: stop-line reached
   - `phase-21_5` perf reopen: parked
   - `phase-29cs`: parked
- runtime lane: `phase-29y / parked`. current blocker: `none`.

## Unified Vocabulary

- `Stage1InputContractBox`: shared input/env contract
- `Stage1ProgramJsonMirCallerBox`: checked Program(JSON)->MIR handoff
- `Stage1ProgramJsonTextGuardBox`: Program(JSON) text guard
- `Stage1SourceMirAuthorityBox`: source authority
- `Stage1ProgramResultValidationBox`: Program JSON validation
- `Stage1MirResultValidationBox`: shared MIR validation
- `Stage1ProgramJsonCompatBox`: compat quarantine
- `nyash.plugin.invoke_by_name_i64`: compat-only plugin dispatch export

## Main Workstream

- active implementation front: `phase-29ci`
- active boundary rule:
  - `MIR(JSON v0)` is the only supported external/bootstrap interchange boundary
  - `Program(JSON v0)` is compat/internal keep and retire target
- close-synced Rune lane: `phase-29cu`
- close-synced bootstrap-retire lane: `phase-29cj`
- live Rust stop-line:
  - `src/host_providers/mir_builder.rs`
  - `src/host_providers/mir_builder/handoff.rs`
  - `src/host_providers/mir_builder/decls.rs`
- latest landed cuts above the same stop-line:
  - Rust: source-route authority / output projection split
  - `.hako`: `BuilderProgramJsonInputContractBox`, `BuilderFuncDefsGateBox`, `BuilderLoopForceRouteBox`, `BuilderUnsupportedTailBox`
  - runner locals: `Stage1MirPayloadContractBox`, `Stage1CliProgramJsonInputBox`, `Stage1CliRawSubcommandInputBox`, `LauncherArtifactIoBox`, `LauncherPayloadContractBox`
- frozen near-thin-floor owners:
  - `src/stage1/program_json_v0/authority.rs`
  - `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
  - `src/runner/stage1_bridge/program_json/**`
  - `src/runner/stage1_bridge/program_json_entry/**`
  - `lang/src/mir/builder/MirBuilderBox.hako`
  - `lang/src/runner/stage1_cli_env.hako`
  - `lang/src/runner/stage1_cli.hako`
  - `lang/src/runner/launcher.hako`

## Next Task

1. reopen `phase-29ci` as the current boundary lane
2. reclassify every remaining `Program(JSON v0)` consumer into:
   - `public/deprecate-now`
   - `internal-compat-keep`
   - `delete-ready-later`
3. retire public wrapper/helper Program(JSON) surfaces now:
   - `run_stage1_cli.sh emit program-json`
   - `selfhost_build.sh --json`
   - exact smoke/docs that still treat those wrappers as live
4. keep raw compat routes explicit:
   - CLI flags
   - stage1 bridge/program-json explicit route
   - compat probe helpers
5. keep internal Program(JSON) routes only where they terminate in MIR and are not public API
6. keep `phase-29cu` / `phase-29cj` formally closed unless an exact gap reappears

## Lane Pointers

- Workstream map: `docs/development/current/main/15-Workstream-Map.md`
- Docs mirror: `docs/development/current/main/10-Now.md`
- Mainline phase: `docs/development/current/main/phases/phase-29cj/README.md`
- Active boundary retire lane: `docs/development/current/main/phases/phase-29ci/README.md`
- Rune lane: `docs/development/current/main/phases/phase-29cu/README.md`
- Runtime lane: `docs/development/current/main/phases/phase-29y/README.md`
- Substrate lane: `docs/development/current/main/phases/phase-29ct/README.md`
- Execution/artifact policy:
  - `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
  - `docs/development/current/main/design/artifact-policy-ssot.md`

## Archive

- current-task history: `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`
