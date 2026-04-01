---
Status: Accepted (formal-close-synced)
Decision: accepted
Date: 2026-03-25
Scope: `Program(JSON v0)` bootstrap boundary を retire target として固定し、repo-wide external/bootstrap boundary を `MIR(JSON v0)` に統一する separate phase owner。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/selfhost-authority-facade-compat-inventory-ssot.md
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
  - docs/development/current/main/phases/phase-29ch/README.md
  - docs/development/current/main/phases/phase-29ch/29ch-10-mir-direct-bootstrap-unification-checklist.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ci/P1-FUTURE-RETIRE-BRIDGE-DELETE-ORDER.md
  - docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md
  - docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md
  - docs/development/current/main/phases/phase-29ci/P4-MIRBUILDER-ROUTE-SPLIT.md
  - docs/development/current/main/phases/phase-29ci/P5-STAGEB-MALFORMED-PROGRAM-JSON.md
  - docs/development/current/main/phases/phase-29cj/README.md
  - src/stage1/program_json_v0/README.md
  - src/runner/stage1_bridge/README.md
---

# Phase 29ci: Program JSON v0 Bootstrap Boundary Retirement

## Goal

`phase-29ch` で `temporary bootstrap boundary` に縮退した

- `src/stage1/program_json_v0.rs` cluster
- `src/runner/stage1_bridge/**` future-retire lane

を、authority migration と混ぜずに separate phase として retire する。

この phase は `MIR-direct bootstrap unification` ではない。
`phase-29ch` が固定した authority を前提に、bootstrap-only JSON v0 boundary の caller / owner / delete order を扱う。
execution-lane reading では、この phase は stage1 bridge/proof boundary だけを扱い、distribution policy は持たない。

## Current Read

- phase status は `formal-close-synced`
- current accepted scope is complete for boundary retirement + caller-audit under the accepted keep set
- current repo reading is fixed to:
  - `Program(JSON v0)` = compat/internal/bootstrap-only keep + retire target
  - `MIR(JSON v0)` = sole external/bootstrap boundary
- landed scope under this phase:
  - wrapper/helper retirements are landed
  - runner-side JSON artifact split is landed:
    - `src/runner/json_artifact/mir_loader.rs` is the mainline MIR intake owner
    - `src/runner/json_artifact/program_json_v0_loader.rs` is the compat `Program(JSON v0)` loader owner
    - `src/runner/core_executor.rs` is now terminal execution only
  - helper-local shell/test-harness slices are landed through W14
  - smoke-tail caller buckets are landed through W18
  - `phase2044` / `phase2160` thin wrapper families are monitor-only keeps
  - `phase2170` default pack is landed, while legacy `hv1_mircall_*` wrappers remain explicit keeps
- file-level compiler / runner / shell responsibility cleanup is tracked separately in
  `selfhost-authority-facade-compat-inventory-ssot.md`; this phase keeps
  `Program(JSON v0)` in compat quarantine and does not reopen authority placement
- reopen only if:
  - a new exact caller/helper gap appears under boundary-retirement scope
  - or hard delete / broad internal removal explicitly resumes

## Entry Conditions

1. `phase-29ch` の done judgment が green
   - reduced bootstrap proof can be explained without JSON v0 route authority
   - bridge is documented as `temporary bootstrap boundary` only
2. proof bundle is green on the current authority contract
   - Stage1/Stage2 rebuild
   - `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`
   - `tools/selfhost_identity_check.sh --mode {smoke,full} --skip-build`
3. `Program(JSON v0)` retirement work is not mixed back into `phase-29ch`

## Fixed Order

1. reclassify the remaining JSON v0 consumers into `public/deprecate-now`, `internal-compat-keep`, and `delete-ready-later`
2. retire public/bootstrap boundary reading first
3. keep internal compat routes explicit and non-public
4. keep proof bundle green after each retirement slice
5. only after caller inventory is empty, consider deleting the boundary itself

## P0 Inventory

- exact caller / owner matrices live in the P0/P1/P2/P3/P5 docs
- current focus is narrow and operational:
  - wrapper/helper retirement is landed
  - raw direct `stage1_cli.hako emit program-json` is diagnostics-only evidence
  - explicit env-route compat probes and raw compat flags stay alive
  - `stage1_cli.hako` / `launcher.hako` route orchestration thinning is landed
  - `tools/hakorune_emit_mir.sh` helper-local splits are landed: Stage-B Program(JSON) production and direct-emit fallback policy
  - `tools/selfhost/selfhost_build.sh` now keeps the EXE consumer path behind `resolve_emit_exe_context()` + `emit_exe_from_program_json_v0_with_context()`
  - the exact W7.1 proof is `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`
  - raw `selfhost_build.sh --in ...` whole-script routes stay upstream Stage-B source-route diagnostics, not the exact helper-local acceptance line
  - `tools/smokes/v2/lib/test_runner.sh` now keeps the verify-tail policy behind `coerce_verify_builder_emit_result_kind()` + `run_verify_builder_emit_{failure,success}_policy()`
  - the exact W8 proof stays on the phase2044 verify canaries
  - `tools/smokes/v2/lib/test_runner.sh` now also keeps the tagged-stdout contract behind `coerce_phase2160_tagged_stdout_result_kind()` + `run_phase2160_tagged_stdout_repair_policy()`
  - the exact W9 proof is `tools/dev/phase29ci_test_runner_tagged_stdout_probe.sh`
  - heavy `phase2160/builder_min_*` wrappers stay monitor-only for that seam
  - `tools/smokes/v2/lib/test_runner.sh` now also keeps the builder-module env/render seam behind `prepare_builder_module_program_json_runner_context()` + `run_rendered_builder_module_program_json_runner()`
  - the exact W10 proof is `tools/dev/phase29ci_test_runner_builder_envrender_probe.sh`
  - `tools/smokes/v2/lib/test_runner.sh` now also keeps the stdout-file wrapper seam behind `capture_runner_stdout_to_file()` + `select_registry_builder_module_runner()`
  - the exact W11 proof is `tools/dev/phase29ci_test_runner_stdout_file_probe.sh`
  - the phase2160 module-load dehang interrupt is landed behind `IfMirEmitBox`, `CompatMirEmitBox`, and bounded-loop fixes in `lower_return_loop_strlen_sum_box.hako` plus `ParserStmtBox.parse_opt_annotation(...)`
  - the exact dehang proof is `tools/dev/phase2160_mirbuilder_module_load_probe.sh`
  - `phase2160/builder_min_if_compare_intint_canary_vm.sh`, `phase2160/registry_optin_compare_varint_canary_vm.sh`, and `phase2160/registry_optin_canary_vm.sh` are bounded again, but they remain monitor-only and are not the helper-local acceptance line
  - `tools/smokes/v2/lib/test_runner.sh` now also keeps the tagged-stdout caller layer behind `run_stdout_tag_canary_exec_and_repair()`
  - the exact W12 proof is `tools/dev/phase29ci_test_runner_tagged_stdout_caller_probe.sh`
  - `tools/smokes/v2/lib/test_runner.sh` now also keeps the registry-specialized tagged-stdout layer behind `capture_registry_tagged_stdout_snapshot()` + `run_registry_builder_diag_exec_and_contract()`
  - the exact W13 proof is `tools/dev/phase29ci_test_runner_registry_tagged_stdout_probe.sh`
  - `phase2160/registry_optin_method_arraymap_get_diag_canary_vm.sh` stays as the thin diag wrapper check for that layer
  - `tools/smokes/v2/lib/test_runner.sh` now also keeps the method-arraymap fallback synth + token-check layer behind `prepare_registry_method_arraymap_stdout_snapshot()` + `run_registry_method_arraymap_token_policy()`
  - the exact W14 proof is `tools/dev/phase29ci_test_runner_method_arraymap_probe.sh`
  - the W15 reinventory stop-line is landed: `tools/smokes/v2/lib/test_runner.sh` is now treated as near-thin-floor by default, and helper-local work should only reopen on a newly discovered exact seam
  - the W16 first smoke-tail bucket is landed too: uniform raw `verify_program_via_builder_to_core` callers now collapse onto named runner helpers instead of repeating env stacks and rc handling inline
  - the special raw verify keep bucket is landed too: `phase2039/parser_embedded_json_canary.sh` now uses the generic rc wrapper directly, and `phase2043/mirbuilder_internal_new_array_core_exec_canary_vm.sh` now routes through `run_verify_program_via_internal_builder_no_methods_to_core()`
  - the core-primary built-MIR verify-route fix is landed behind `verify_primary_requests_core_v0()`
  - exact W17 proof is `tools/dev/phase29ci_verify_primary_core_route_probe.sh`
  - the `phase2170` default MIR-file verify wrapper pack is landed too: repeated hakovm MIR-call env stacks now live behind `apply_verify_mir_route_env()`, `run_verify_mir_rc_with_env()`, and the named `run_verify_mir_via_hakovm_*` helpers in `tools/smokes/v2/lib/test_runner.sh`
  - the default `phase2170` wrappers now collapse onto `run_verify_mir_canary_and_expect_rc()`, while legacy `hv1_mircall_*` wrappers remain explicit keeps
  - there is no remaining exact implementation leaf under the current boundary-retirement scope
  - keep the already-thin `phase2044` / `phase2160` wrapper families as monitor-only keeps, not reopen triggers
- keep this README as the phase entry point, not the evidence log

## Current Retirement Targets

- public/bootstrap boundary first:
  - wrapper/helper surface `tools/selfhost/run_stage1_cli.sh emit program-json` (landed)
  - wrapper/helper surface `tools/selfhost/selfhost_build.sh --json` (landed)
  - exact smoke/docs that still present those wrappers as live
- raw direct diagnostics pin:
  - `lang/src/runner/stage1_cli.hako` raw `emit program-json` lane is retire-only / diagnostics-only
  - `tools/dev/phase29ch_raw_direct_stage1_cli_probe.sh` pins that retired lane as an absence proof
- raw compat keep after wrapper retirement:
  - CLI `--emit-program-json-v0`
  - CLI `--hako-emit-program-json`
  - CLI `--program-json-to-mir`
  - Stage1 bridge explicit `emit-program-json-v0` route
- compat/internal keep after that:
  - `src/stage1/program_json_v0.rs` cluster
  - `src/runner/json_artifact/program_json_v0_loader.rs`
  - `src/runner/stage1_bridge/program_json/**`
  - `src/runner/stage1_bridge/program_json_entry/**`
  - `.hako` live/bootstrap callers
  - compiled-stage1 / shell callers that still terminate in MIR

## Non-goals

- reopening `phase-29cg` solved reduction buckets
- re-arguing `phase-29ch` authority migration
- widening compat keep or raw direct `stage1-cli` authority

## Acceptance

- retirement work can be explained without mixing authority migration back into `phase-29ch`
- remaining JSON v0 consumers are inventoried with exact owners and boundary class
- public/bootstrap docs and CLI/help read `MIR(JSON)` as the only supported boundary
- at least one compat-only Program(JSON) route remains green and explicitly marked non-public
- wrapper/public helper retirement is pinned by exact smoke and explicit compat probe
- raw direct `stage1_cli.hako` `emit program-json` lane is pinned as retired diagnostics-only evidence
- hard delete is not started in the same wave

## Reopen Pointer

- reopen this phase only when:
  - `docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md`
    shows a new exact caller bucket
  - or hard delete / broad internal removal resumes as a separate wave
