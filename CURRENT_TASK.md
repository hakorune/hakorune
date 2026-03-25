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
- active implementation lane is `phase-29ci`:
  - retire `Program(JSON v0)` from repo-wide external/bootstrap boundary
  - unify public/bootstrap boundary on `MIR(JSON v0)`
  - keep hard delete and broad internal removal for later waves

## Current Priority

1. active implementation lane: `phase-29ci`
   - status: `reopen W14 active`
   - scope: `Program(JSON v0)` boundary retirement + `MIR(JSON v0)` line unification
   - working rule:
     - public/bootstrap surfaces move to MIR-first now
     - `Program(JSON v0)` stays compat/internal keep only
     - hard delete is later
   - exact W14 target:
     - keep `launcher.hako` W5 thinning landed and frozen near-thin-floor
     - keep the first two `tools/hakorune_emit_mir.sh` helper-local splits landed: Stage-B Program(JSON) production and direct-emit fallback policy
     - keep `tools/selfhost/selfhost_build.sh` EXE consumer path landed behind `resolve_emit_exe_context()` + `emit_exe_from_program_json_v0_with_context()`
     - pin that exact leaf via `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`
     - raw `selfhost_build.sh --in ...` whole-script routes remain upstream Stage-B source-route diagnostics and are not the W7.1 acceptance line
     - keep `tools/smokes/v2/lib/test_runner.sh` verify-tail policy landed behind `coerce_verify_builder_emit_result_kind()` + `run_verify_builder_emit_{failure,success}_policy()`
     - keep `phase2044` canaries as the exact proof line for that helper-local slice
     - keep `tools/smokes/v2/lib/test_runner.sh` tagged-stdout contract landed behind `coerce_phase2160_tagged_stdout_result_kind()` + `run_phase2160_tagged_stdout_repair_policy()`
     - keep the exact W9 proof on `tools/dev/phase29ci_test_runner_tagged_stdout_probe.sh`
     - keep heavy `phase2160/builder_min_*` wrapper canaries as monitor-only while the tagged-stdout helper-local seam is pinned directly
     - keep `tools/smokes/v2/lib/test_runner.sh` builder-module env/render seam landed behind `prepare_builder_module_program_json_runner_context()` + `run_rendered_builder_module_program_json_runner()`
     - keep the exact W10 proof on `tools/dev/phase29ci_test_runner_builder_envrender_probe.sh`
     - keep `tools/smokes/v2/lib/test_runner.sh` stdout-file wrapper seam landed behind `capture_runner_stdout_to_file()` + `select_registry_builder_module_runner()`
     - keep the exact W11 proof on `tools/dev/phase29ci_test_runner_stdout_file_probe.sh`
     - keep the phase2160 module-load dehang interrupt landed behind `IfMirEmitBox`, `CompatMirEmitBox`, and the bounded-loop fixes in `lower_return_loop_strlen_sum_box.hako` and `ParserStmtBox.parse_opt_annotation(...)`
     - keep the exact dehang proof on `tools/dev/phase2160_mirbuilder_module_load_probe.sh`
     - keep `phase2160/builder_min_if_compare_intint`, `phase2160/registry_optin_compare_varint`, and `phase2160/registry_optin` bounded again as monitor-only canaries
     - keep `tools/smokes/v2/lib/test_runner.sh` tagged-stdout caller layer landed behind `run_stdout_tag_canary_exec_and_repair()`
     - keep the exact W12 proof on `tools/dev/phase29ci_test_runner_tagged_stdout_caller_probe.sh`
     - keep `tools/smokes/v2/lib/test_runner.sh` registry-specialized tagged-stdout layer landed behind `capture_registry_tagged_stdout_snapshot()` + `run_registry_builder_diag_exec_and_contract()`
     - keep the exact W13 proof on `tools/dev/phase29ci_test_runner_registry_tagged_stdout_probe.sh`
     - keep `phase2160/registry_optin_method_arraymap_get_diag_canary_vm.sh` as the thin diag wrapper check for that registry-specialized layer
     - keep `tools/smokes/v2/lib/test_runner.sh` method-arraymap fallback synth + token-check layer landed behind `prepare_registry_method_arraymap_stdout_snapshot()` + `run_registry_method_arraymap_token_policy()`
     - keep the exact W14 proof on `tools/dev/phase29ci_test_runner_method_arraymap_probe.sh`
     - treat `test_runner.sh` as reinventory-ready / near-thin-floor before promoting any new helper-local leaf
     - keep explicit env-route compat probes, raw compat flags, and the retired raw direct `stage1_cli.hako emit program-json` diagnostics pin alive
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

1. keep `phase-29ci` as the current boundary lane
2. reclassify every remaining `Program(JSON v0)` consumer into:
   - `public/deprecate-now`
   - `internal-compat-keep`
   - `delete-ready-later`
3. keep `stage1_cli.hako` and `launcher.hako` frozen near-thin-floor after their route thinning slices
4. move the next live caller-audit bucket to shared shell helper keep:
   - first landed target: `tools/hakorune_emit_mir.sh`
   - second landed target: `tools/selfhost/selfhost_build.sh`
   - current exact bucket: `tools/smokes/v2/lib/test_runner.sh`
   - landed exact leaves:
     - verify-tail policy behind `handle_verify_builder_emit_result(...)`
     - tagged-stdout contract behind `ensure_phase2160_tagged_stdout_contract(...)`
     - builder-module env/render seam behind `run_program_json_via_builder_module_vm_with_env()`
     - stdout-file wrapper seam behind `run_builder_module_vm_to_stdout_file()` + `run_registry_builder_module_vm_to_stdout_file()`
     - phase2160 module-load dehang behind `IfMirEmitBox`, `CompatMirEmitBox`, and the bounded-loop fixes in `lower_return_loop_strlen_sum_box.hako` and `ParserStmtBox.parse_opt_annotation(...)`
     - tagged-stdout caller layer behind `run_stdout_tag_canary_exec_and_repair()`
     - registry-specialized tagged-stdout layer behind `capture_registry_tagged_stdout_snapshot()` + `run_registry_builder_diag_exec_and_contract()`
   - current exact bucket: reinventory `tools/smokes/v2/lib/test_runner.sh` for near-thin-floor vs smoke-tail boundary; do not promote the 43-file tail in the same slice
5. keep explicit env-route compat probes and raw compat flags alive:
   - CLI flags
   - stage1 bridge/program-json explicit route
   - compat probe helpers
6. keep internal Program(JSON) routes only where they terminate in MIR and are not public API
7. keep `phase-29cu` / `phase-29cj` formally closed unless an exact gap reappears

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
