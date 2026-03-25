---
Status: Active
Date: 2026-03-25
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

1. `phase-29ci`
   - active boundary lane
   - `Program(JSON v0)` boundary retirement / `MIR(JSON v0)` line unification
   - W7 landed: shared shell helper caller audit inside `tools/selfhost/selfhost_build.sh` now covers the Program(JSON)->MIR->EXE consumer path
   - W8 landed: `tools/smokes/v2/lib/test_runner.sh` verify-tail policy split
   - W9 landed: `tools/smokes/v2/lib/test_runner.sh` tagged-stdout contract split
   - W10 landed: `tools/smokes/v2/lib/test_runner.sh` builder-module env/render split
   - W11 landed: `tools/smokes/v2/lib/test_runner.sh` stdout-file wrapper seam split
   - interrupt landed: phase2160 MirBuilder module-load dehang via `IfMirEmitBox`, `CompatMirEmitBox`, and bounded-loop fixes in `lower_return_loop_strlen_sum_box.hako` plus `ParserStmtBox.parse_opt_annotation(...)`
   - W12 landed: `tools/smokes/v2/lib/test_runner.sh` tagged-stdout caller layer split
   - W13 landed: `tools/smokes/v2/lib/test_runner.sh` registry-specialized tagged-stdout layer split
   - W14 landed: `tools/smokes/v2/lib/test_runner.sh` method-arraymap fallback synth + token-check layer split
   - W15 landed: `tools/smokes/v2/lib/test_runner.sh` reinventory stop-line; helper-local work is now treated as near-thin-floor by default
   - W16 landed: first smoke-tail bucket; uniform raw `verify_program_via_builder_to_core` callers now collapse onto named runner helpers
   - W16 exact next: special raw verify keeps with extra env or nonstandard success shape, centered on `phase2039/parser_embedded_json_canary.sh` and `phase2043/mirbuilder_internal_new_array_core_exec_canary_vm.sh`
2. `phase-29cu`
   - formally close-synced
3. `phase-29cj`
   - formally close-synced
4. `phase-29y`
   - parked / monitor-only
5. `phase-29ct`
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

1. keep `phase-29ci` on boundary retirement only
2. keep `stage1_cli.hako` and `launcher.hako` frozen near-thin-floor after the landed W4/W5 route-thinning slices
3. audit shared shell helper keep in fixed order:
   - `tools/hakorune_emit_mir.sh`
   - `tools/selfhost/selfhost_build.sh`
   - `tools/smokes/v2/lib/test_runner.sh`
4. keep explicit env-route compat probes and raw compat flags alive
5. keep internal Program(JSON) routes as compat/test/bootstrap-only keep until caller inventory reaches zero
6. keep `phase-29cu` and `phase-29cj` closed unless exact gaps reappear

## Active Lane

- `phase-29ci` is active again
- active reading:
  - `Program(JSON v0)` public/bootstrap boundary retirement
  - `MIR(JSON v0)` line unification
  - `Program(JSON v0)` hard delete is deferred
- current first-wave targets:
  - wrapper/helper retirements are landed
  - `stage1_cli.hako` and `launcher.hako` route orchestration thinning is landed
  - raw direct `stage1_cli.hako emit program-json` lane is retired as diagnostics-only evidence
  - `tools/hakorune_emit_mir.sh` helper-local splits are landed: Stage-B Program(JSON) production and direct-emit fallback policy
  - `tools/selfhost/selfhost_build.sh` EXE consumer path split is landed; helper-local proof lives in `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`
  - `tools/smokes/v2/lib/test_runner.sh` verify-tail policy split is landed; proof stays on the phase2044 canaries
  - `tools/smokes/v2/lib/test_runner.sh` tagged-stdout contract split is landed; helper-local proof lives in `tools/dev/phase29ci_test_runner_tagged_stdout_probe.sh`
  - heavy `phase2160/builder_min_*` wrappers stay monitor-only for this seam
  - `tools/smokes/v2/lib/test_runner.sh` builder-module env/render split is landed; helper-local proof lives in `tools/dev/phase29ci_test_runner_builder_envrender_probe.sh`
  - `tools/smokes/v2/lib/test_runner.sh` stdout-file wrapper seam split is landed; helper-local proof lives in `tools/dev/phase29ci_test_runner_stdout_file_probe.sh`
  - phase2160 module-load dehang is landed; exact proof lives in `tools/dev/phase2160_mirbuilder_module_load_probe.sh`, and the representative `builder_min_if_compare_intint` / `registry_optin_compare_varint` / `registry_optin` canaries are bounded again as monitor-only checks
  - `tools/smokes/v2/lib/test_runner.sh` tagged-stdout caller layer split is landed; helper-local proof lives in `tools/dev/phase29ci_test_runner_tagged_stdout_caller_probe.sh`
  - `tools/smokes/v2/lib/test_runner.sh` registry-specialized tagged-stdout layer split is landed; helper-local proof lives in `tools/dev/phase29ci_test_runner_registry_tagged_stdout_probe.sh`
  - `tools/smokes/v2/lib/test_runner.sh` method-arraymap fallback synth + token-check layer split is landed; helper-local proof lives in `tools/dev/phase29ci_test_runner_method_arraymap_probe.sh`
  - the `test_runner.sh` reinventory stop-line is landed; helper-local work is now near-thin-floor by default
  - the first smoke-tail caller bucket is also landed: uniform raw `verify_program_via_builder_to_core` callers now route through named helpers instead of open-coded env stacks
  - next caller-audit bucket is the special raw verify keeps with extra env or nonstandard success shape
- guard rails:
  - `Program(JSON v0)` stays no-widen
  - internal `.hako` / host-provider Program(JSON) keep is allowed only as compat that terminates in MIR
  - do not absorb high-level Program(JSON) structure into MIR

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
- `launcher.hako` now keeps top-level route selection behind `LauncherDispatchBox`, so `HakoCli` is down to orchestration only
- `vm-hako` is frozen as monitor-only; throughput probes are archived evidence, not current blockers

## Read Order

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/10-Now.md`
4. active phase README
