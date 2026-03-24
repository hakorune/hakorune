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
- there is no new bootstrap-retire blocker right now.
- `phase-29cu` has completed its formal close sync for the narrow Rune v0 scope.
- there is no active implementation lane right now; next lane selection is pending docs-first promotion.

## Current Priority

1. active implementation lane: `none`
   - lane selection is pending docs-first promotion
   - do not resume coding on any lane until the next active front is explicitly promoted in root/docs pointers
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

- active implementation front: `none`
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

1. keep `phase-29cu` formally closed unless a new exact Rune gap appears
2. keep the Rune lane docs in current-truth reading:
   - declaration-local `attrs.runes`
   - Rust direct MIR carrier
   - `.hako` generic function-rune carrier + real root-entry `Main.main` carrier
   - selected-entry `ny-llvmc` semantics
3. pin the remaining future Rune reopen so `.hako` full support is not forgotten:
   - declaration-local full carrier parity beyond root-entry-only transport
4. keep `phase-29cj` closed unless a new exact disappearing leaf appears
5. choose the next active implementation lane only through docs-first promotion

## Lane Pointers

- Workstream map: `docs/development/current/main/15-Workstream-Map.md`
- Docs mirror: `docs/development/current/main/10-Now.md`
- Mainline phase: `docs/development/current/main/phases/phase-29cj/README.md`
- Bootstrap retire closeout: `docs/development/current/main/phases/phase-29ci/README.md`
- Rune lane: `docs/development/current/main/phases/phase-29cu/README.md`
- Runtime lane: `docs/development/current/main/phases/phase-29y/README.md`
- Substrate lane: `docs/development/current/main/phases/phase-29ct/README.md`
- Execution/artifact policy:
  - `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
  - `docs/development/current/main/design/artifact-policy-ssot.md`

## Archive

- current-task history: `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`
