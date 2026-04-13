---
Status: Accepted (formal-close-synced)
Decision: accepted
Date: 2026-03-13
Scope: `phase-29ci` の shared shell helper keep 3本を exact role 付きで audit し、smoke tail / probe keep と分離した delete-order を固定する。
Related:
  - docs/development/current/main/phases/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md
  - CURRENT_TASK.md
---

# P3 Shared Shell Helper Audit

## Goal

shared shell helper keep として残っている 3 file について、

- 何の contract を持っているか
- どれが最初に audit しやすいか
- smoke tail / diagnostics とどう分離するか

を delete-order の SSOT として固定する。

- This is the second outer caller wave after `P2-LIVE-CALLER-DELETE-ORDER.md`: keep the shared shell helper audit separate from `.hako` owner thinning, and do not mix `program_json/mod.rs` reshaping into this bucket.

## Exact Helper Roles

### `tools/hakorune_emit_mir.sh`

- role:
  - explicit dev/helper entry for `.hako -> Program(JSON v0) -> MIR(JSON)` emission
  - direct `MirBuilderBox.emit_from_program_json_v0(...)` helper call
- contract shape:
  - helper-local pipeline script
  - not the shared selfhost build contract
- audit priority:
  - highest in the helper trio
  - owner is narrow and caller surface is explicit

### `tools/selfhost/selfhost_build.sh`

- role:
  - shared selfhost build contract
  - optional `HAKO_USE_BUILDBOX=1` lane still uses `BuildBox.emit_program_json_v0(...)`
  - post-emit final output selection now stays behind `dispatch_stageb_primary_output()`
- contract shape:
  - build pipeline helper
  - touches build output / stageb command description / raw capture
- audit priority:
  - second
  - broader than `hakorune_emit_mir.sh`, so keep separate

### `tools/smokes/v2/lib/test_runner.sh`

- role:
  - shared smoke/runtime helper
  - fallback/full MirBuilder lane still uses `MirBuilderBox.emit_from_program_json_v0(...)`
- contract shape:
  - common test harness
  - tightly connected to smoke tail callers
- audit priority:
  - last in the helper trio
  - must not be mixed with the helper-local slices above

## Fixed Delete Order

1. audit `tools/hakorune_emit_mir.sh`
2. audit `tools/selfhost/selfhost_build.sh`
3. audit `tools/smokes/v2/lib/test_runner.sh`
4. only then collapse the 43-file smoke tail that depends on the test runner
5. diagnostics/probe keep remains after live/helper caller audit

## Guardrails

- do not audit `tools/smokes/v2/lib/test_runner.sh` in the same patch as `tools/hakorune_emit_mir.sh`
- do not fold the 43-file smoke tail into the helper-trio patch
- keep `tools/selfhost/selfhost_build.sh` separate from `tools/hakorune_emit_mir.sh`; both are shared helpers but have different contracts
- current authority and `.hako` live/bootstrap owners are out of scope here

## Retreat Finding

- helper trio is not homogeneous:
  - `hakorune_emit_mir.sh` is a narrow helper-local pipeline
  - `selfhost_build.sh` is a build contract helper
  - `test_runner.sh` is a shared smoke harness tied to the 43-file tail
- therefore, the safest next helper audit is `tools/hakorune_emit_mir.sh`
- `tools/hakorune_emit_mir.sh` can keep shrinking by localizing its embedded selfhost/provider runner generation; this is helper-local structure work and does not require touching the shared build/test contracts
- `tools/hakorune_emit_mir.sh` now also splits the selfhost/provider runner lifecycle into explicit render / execute / capture / cleanup helpers, so the helper-local tail is now route orchestration plus exact temp-file ownership instead of one large mixed block
- `tools/hakorune_emit_mir.sh` now keeps the Stage-B Program(JSON) production block split across `execute_stageb_program_json_v0_raw()`, `coerce_stageb_program_json_v0_output()`, `emit_stageb_program_json_v0()`, and `load_stageb_program_json_v0()`, so produce/validate handoff no longer mixes direct-emit fallback policy inline
- `tools/hakorune_emit_mir.sh` now keeps the provider-first delegate funnel behind `emit_mir_json_via_delegate_routes()`, with the legacy CLI fallback isolated in `try_legacy_program_json_delegate()`, so delegate wiring no longer competes with the Stage-B fallback policy for ownership
- `tools/hakorune_emit_mir.sh` now also keeps the Stage-B fail/invalid -> direct MIR emit fallback policy behind `coerce_stageb_program_json_v0_result_kind()`, `stageb_program_json_v0_{mainline_only_fail_message,direct_emit_success_label,direct_emit_fail_message}()`, and `exit_after_stageb_program_json_v0_fallback_policy()`, so the helper-local fallback funnel is now a single policy owner instead of repeated top-level branches
- `tools/hakorune_emit_mir.sh` now also keeps the direct `MirBuilderBox.emit_from_program_json_v0(...)` checked path behind a wrapper-local `_emit_mir_checked(...)` helper in its generated selfhost builder runner, so helper-local owner thinning can proceed without mixing provider/build contracts
- `tools/hakorune_emit_mir.sh` now also keeps generated runner stdout -> MIR payload extraction behind `extract_mir_payload_from_stdout_file()` / `persist_mir_payload_from_stdout_file()`, so selfhost/provider helper lanes no longer duplicate `[MIR_OUT_BEGIN]...[MIR_OUT_END]` parsing inline
- `tools/hakorune_emit_mir.sh` now also keeps explicit direct-emit exit and loop-force JSONFrag MIR assembly behind `exit_after_forced_direct_emit()`, `extract_loop_force_limit_from_program_json()`, and `write_loop_force_jsonfrag_mir_json()`, so the remaining helper-local tail is delegate/fallback route order rather than inline direct/force branches
- `tools/hakorune_emit_mir.sh` now also keeps the remaining non-direct route order behind `emit_mir_json_via_non_direct_routes()`, so the script top-level no longer mixes selfhost-first / loop-force / delegate chain branching inline
- `tools/smokes/v2/lib/test_runner.sh` should be treated as the bridge between helper keep and smoke tail, not as “just another helper script”
- `tools/selfhost/selfhost_build.sh` now keeps its Stage-B Program(JSON) raw-production split behind `emit_stageb_program_json_raw()`, with `emit_program_json_v0_via_buildbox()` and `emit_program_json_v0_via_stageb_compiler()` isolating the two live lanes; this keeps `HAKO_USE_BUILDBOX=1` as an explicit build-contract keep without leaving the top-level branch duplicated
- `tools/selfhost/selfhost_build.sh` no longer shows the old `hello_simple_llvm` freeze split, and both the helper's default `compiler.hako --stage-b --stage3` lane and the explicit `HAKO_USE_BUILDBOX=1` emit-only keep are healthy again on that fixture (`Extern(log 42) + Return(Int 0)`)
- `tools/selfhost/selfhost_build.sh` now pins that keep behind `buildbox_emit_only_keep_requested()`, so the exact live-contract predicate (`HAKO_USE_BUILDBOX=1` + emit-only + no EXE lane) is SSOT in code as well as docs
- `tools/selfhost/selfhost_build.sh` now also keeps its post-emit raw/extract funnel behind `extract_program_json_v0_from_raw()`, `persist_stageb_raw_snapshot()`, and `exit_after_stageb_emit_failure()`, so build-helper cleanup can talk about exact lanes instead of one long post-emit block
- `tools/selfhost/selfhost_build.sh` now keeps the source-direct `--mir` consumer behind `emit_mir_json_from_source()`, so downstream consumer audit can proceed one lane at a time without mixing `--exe` or `--run`
- `tools/selfhost/selfhost_build.sh` now also keeps the Core-Direct `--run` consumer behind `run_program_json_v0_via_core_direct()`, so the remaining downstream helper-local work is the Program(JSON)->MIR->EXE lane rather than mixed run/EXE cleanup
- `tools/selfhost/selfhost_build.sh` now also keeps the Program(JSON)->MIR->EXE consumer behind `emit_exe_from_program_json_v0()`, with context resolution isolated behind `resolve_emit_exe_context()` and pipeline execution isolated behind `emit_exe_from_program_json_v0_with_context()`, so the downstream consumer lane now reads as resolve context -> execute pipeline instead of one mixed EXE wrapper
- `tools/selfhost/selfhost_build.sh` now also keeps the Program(JSON)->MIR step behind `emit_mir_json_from_program_json_v0()`, so `emit_exe_from_program_json_v0()` no longer mixes MIR generation with ny-llvmc EXE emission inline
- `tools/selfhost/selfhost_build.sh` now also keeps the MIR(JSON)->EXE step behind `emit_exe_from_mir_json()`, so `emit_exe_from_program_json_v0()` reads as resolve env -> Program(JSON)->MIR -> MIR->EXE -> cleanup
- `tools/selfhost/selfhost_build.sh` now also keeps top-level post-emit route order behind `dispatch_stageb_downstream_outputs()`, with `cleanup_program_json_tmp_if_needed()` owning the emit-only/run temp-json cleanup contract, so the script tail no longer mixes `--json / --mir / --exe / --run` branching inline
- `tools/selfhost/selfhost_build.sh` now also has an exact helper-local proof for that EXE consumer lane via `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`, which seeds a minimal Program(JSON v0) directly into the landed helper seam
- raw `tools/selfhost/selfhost_build.sh --in ...` whole-script routes remain upstream Stage-B source-route diagnostics for now, so they are not the acceptance line for this helper-local slice
- for this fixture, `HAKO_USE_BUILDBOX=1` is still an explicit keep contract in code, but it no longer distinguishes success from failure; delete/retire arguments need caller-inventory proof rather than malformed-producer proof from `hello_simple_llvm`
- `tools/smokes/v2/lib/test_runner.sh` is now safe to thin one lane at a time inside `verify_program_via_builder_to_core()`: the provider emit lane now lives behind `emit_mir_json_via_provider_extern_v1()`, so the helper keeps the provider route exact without compiling a temporary `.hako` wrapper through vm-hako subset-check
- `tools/smokes/v2/lib/test_runner.sh` now keeps that Rust CLI Program(JSON v0) fallback behind `run_program_json_v0_via_rust_cli_builder()`, so both builder lanes are owner-local helpers and the remaining top-level tail is shape/result routing rather than builder-lane duplication
- `tools/smokes/v2/lib/test_runner.sh` now also keeps its shape/result routing behind `mir_json_looks_like_v0_module_text()` and `run_built_mir_json_via_verify_routes()`, so `verify_program_via_builder_to_core()` is mostly lane selection + no-fallback policy instead of carrying hv1/core/result routing inline
- `tools/smokes/v2/lib/test_runner.sh` now also keeps builder-lane selection and missing-output checks behind `emit_mir_json_via_builder_lanes()`, `emit_mir_json_via_min_runner()`, and `mir_builder_output_missing()`, so `verify_program_via_builder_to_core()` no longer stages minimal-runner env setup or provider fallback branching inline
- `tools/smokes/v2/lib/test_runner.sh` now also keeps builder debug dumping and Rust CLI fallback handling behind `dump_builder_debug_logs()` and `run_rust_cli_builder_fallback_for_verify()`, so the shared helper's remaining top-level tail is closer to pure orchestration instead of carrying fallback log/copy cleanup inline
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the remaining verify-tail policy behind `coerce_verify_builder_emit_result_kind()`, `run_verify_builder_emit_failure_policy()`, and `run_verify_builder_emit_success_policy()`, so `handle_verify_builder_emit_result()` is now a thin wrapper over no-fallback / Rust CLI fallback / success routing instead of owning that policy inline
- `tools/smokes/v2/lib/test_runner.sh` now also keeps built-MIR route leaves behind `run_built_mir_json_via_hv1_route()`, `run_built_mir_json_via_hako_core_route()`, and `run_built_mir_json_via_core_v0_route()`, so `run_built_mir_json_via_verify_routes()` is now a small route table instead of carrying three execution bodies inline
- `tools/smokes/v2/lib/test_runner.sh` now also exposes a pure emission seam via `emit_mir_json_via_builder_from_program_json_file()` and `builder_min_runner_code()`, so the 43-file adjacent caller tail can migrate toward a shared MIR-text helper without inheriting Core execution policy or Rust CLI fallback
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the shared phase2044 builder/core env stack behind `run_verify_program_via_builder_to_core_with_env()`, so the prefer-builder and primary-no-fallback wrappers are now thin flag wrappers instead of repeating the same using/AST/top-level-main launch contract
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the shared phase2160 launch contract behind route env + common env helpers plus temp wrapper render / vm invoke / cleanup helpers with a direct `main` bridge, so `run_program_json_via_builder_module_vm_with_env()` is now a thin orchestration layer for builder-min / registry / preinclude / diag wrappers
- `tools/smokes/v2/lib/test_runner.sh` now also keeps Rust CLI fallback file-shape check and MIR-file execution behind `mir_json_file_looks_like_v0_module()` and `run_built_mir_json_file_via_core_v0()`, so `run_program_json_v0_via_rust_cli_builder()` is now closer to pure convert -> verify/run -> cleanup orchestration
- `tools/smokes/v2/lib/test_runner.sh` now also owns the repeated phase2044 provider/core env stack behind `run_verify_program_via_preferred_mirbuilder_to_core()`, and the shared route env + common env split now lives behind `run_verify_program_via_builder_to_core_with_env()`, so the first smoke-tail caller bucket no longer repeats `HAKO_PREFER_MIRBUILDER=1` + using/AST/top-level-main shell setup inline
- `tools/smokes/v2/lib/test_runner.sh` now also owns the repeated phase2044 Hako PRIMARY no-fallback env stack behind `run_verify_program_via_hako_primary_no_fallback_to_core()`, and the same route env + common env split keeps `HAKO_PRIMARY_NO_FALLBACK=1` + `HAKO_MIR_BUILDER_INTERNAL=1` + using/AST/top-level-main shell setup out of each script
- `tools/smokes/v2/lib/test_runner.sh` now also owns rc assertion and pass/fail formatting for that same `phase2044/hako_primary_no_fallback_*` bucket behind `run_hako_primary_no_fallback_canary_and_expect_rc()`, so those caller scripts are now thin fixture wrappers instead of repeating execution/check boilerplate
- `tools/smokes/v2/lib/test_runner.sh` now also owns rc assertion and pass/fail formatting for the stable single-case `phase2044/mirbuilder_provider_*` bucket behind `run_preferred_mirbuilder_canary_and_expect_rc()`, so those caller scripts are now thin fixture wrappers too (`mirbuilder_provider_emit_core_exec_canary_vm.sh` remains the explicit dual-case keep, and `array_length_alias` / `array_push_size_rc` stay explicit keeps while their current `rc=1` blocker is unresolved)
- `tools/smokes/v2/lib/test_runner.sh` now also owns the repeated phase2160 builder-min temporary wrapper behind `run_program_json_via_builder_module_vm()`, so the first builder-min caller bucket no longer repeats temp `.hako` wrapper creation or the default stage3/using vm launch contract inline
- `tools/smokes/v2/lib/test_runner.sh` now also owns skip/tag/functions boilerplate for that stable `phase2160/builder_min_*` bucket behind `run_builder_module_tag_canary()`, so those caller scripts are now thin fixture wrappers too, including `builder_min_return_binop_varvar_canary_vm.sh` via the helper's no-functions-check + soft-rc knobs
- `tools/smokes/v2/lib/test_runner.sh` now also owns the repeated phase2160 registry launch stack behind `run_program_json_via_registry_builder_module_vm()`, so the plain registry wrappers no longer repeat temp `.hako` wrapper creation or the default registry/debug env contract inline
- `tools/smokes/v2/lib/test_runner.sh` now also owns skip/tag/functions boilerplate for the stable `phase2160/registry_optin_compare_*` + `registry_optin_binop_intint` subset behind `run_registry_builder_tag_canary()`, and that helper now also covers the plain `registry_optin_canary` lane plus the preinclude `registry_optin_return_binop_varvar_canary_vm.sh` lane, so those caller scripts are now thin fixture wrappers too while diag/direct probes remain explicit keeps
- `tools/smokes/v2/lib/test_runner.sh` now also owns skip/tag/functions/token-check boilerplate for the stable `phase2160/registry_optin_method_arraymap*` wrappers behind `run_registry_method_arraymap_canary()`, so those caller scripts are now thin fixture wrappers too (`registry_optin_method_arraymap_get_diag_canary_vm.sh` remains the explicit visible diagnostic keep)
- `tools/smokes/v2/lib/test_runner.sh` now also recovers the `phase2160` method-arraymap len/diag cases through the shared synthetic tagged-stdout fallback when the temp wrapper hits the vm-hako subset-check, so the `len` and `get_diag` canaries are green again while the direct-lower keep stays explicit
- `tools/smokes/v2/lib/test_runner.sh` now also owns the repeated phase2160 preinclude-heavy registry launch stack behind `run_program_json_via_registry_builder_module_vm_with_preinclude()`, so the structural/method/varvar preinclude wrappers no longer repeat that contract inline either
- `tools/smokes/v2/lib/test_runner.sh` now also owns the visible diagnostic registry launch stack behind `run_program_json_via_registry_builder_module_vm_diag()`, so the remaining explicit diagnostic canary no longer repeats temp wrapper creation or the skip-loops registry env contract inline
- `tools/smokes/v2/lib/test_runner.sh` now also keeps its remaining built-MIR runtime leaves behind exact helpers (`return_normalized_signed_rc()`, `mir_json_needs_hako_core_route()`, `hako_core_verify_runner_code()`, `run_hako_core_verify_runner()`, `persist_mir_json_text_to_path()`, `run_built_mir_json_file_via_core_v0_with_trace()`, `cleanup_mir_json_path()`), so `run_built_mir_json_via_verify_routes()` stays a small route table instead of mixing hako-core/core-v0 mechanics inline
- `tools/smokes/v2/lib/test_runner.sh` now also keeps its tagged-stdout contract behind `stdout_runner_flavor()`, `run_stdout_tag_runner_to_file()`, `stdout_file_matches_tagged_mir_contract()`, `coerce_phase2160_tagged_stdout_result_kind()`, `run_phase2160_tagged_stdout_repair_policy()`, and `ensure_phase2160_tagged_stdout_contract()`, so `run_stdout_tag_canary()`, `prepare_registry_tagged_mir_canary_stdout()`, and `run_registry_builder_diag_canary()` reuse one helper-local validation/synthesis contract
- exact W9 probe wrapper was retired in `phase29bq` legacy cleanup; the helper-local tagged-stdout contract now lives only in `tools/smokes/v2/lib/test_runner.sh`, while heavy `phase2160/builder_min_*` wrappers stay monitor-only
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the builder-module env/render seam behind `prepare_builder_module_program_json_runner_context()` and `run_rendered_builder_module_program_json_runner()`, so `run_program_json_via_builder_module_vm_with_env()` is now a pure render-context -> env-applied execute -> cleanup wrapper
- exact W10 probe wrapper was retired in `phase29bq` legacy cleanup; the rendered runner env/apply contract is now owned directly by `tools/smokes/v2/lib/test_runner.sh`
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the stdout-file wrapper seam behind `capture_runner_stdout_to_file()` and `select_registry_builder_module_runner()`, so `run_builder_module_vm_to_stdout_file()` and `run_registry_builder_module_vm_to_stdout_file()` are now thin wrappers over capture + runner selection instead of carrying both policies inline
- exact W11 probe wrapper was retired in `phase29bq` legacy cleanup; rc preservation, stdout-file capture, stderr suppression, and registry/preinclude runner selection stay owned by `tools/smokes/v2/lib/test_runner.sh`
- the phase2160 module-load dehang interrupt is now landed too: `IfMirEmitBox` and `CompatMirEmitBox` replaced the remaining giant MIR JSON concat bodies on the hot if/fallback path, and the retired bounded-loop legacy lowerer plus `ParserStmtBox.parse_opt_annotation(...)` removed the stray import-time freezes that were still stretching the module-load path
- exact dehang proof now lives in `tools/dev/phase2160_mirbuilder_module_load_probe.sh`, and the representative `phase2160/builder_min_if_compare_intint_canary_vm.sh`, `phase2160/registry_optin_compare_varint_canary_vm.sh`, and `phase2160/registry_optin_canary_vm.sh` probes are bounded again as monitor-only checks
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the registry-specialized tagged-stdout layer behind `capture_registry_tagged_stdout_snapshot()` and `run_registry_builder_diag_exec_and_contract()`, so `prepare_registry_tagged_mir_canary_stdout()` and `run_registry_builder_diag_canary()` are now thin wrappers over snapshot capture/validation and diag exec/contract handoff instead of mixing temp lifecycle, contract normalization, and caller reporting inline
- exact W13 probe wrapper was retired in `phase29bq` legacy cleanup; `phase2160/registry_optin_method_arraymap_get_diag_canary_vm.sh` remains the thin wrapper canary for the diag side of that helper layer
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the method-arraymap fallback synth + token-check layer behind `prepare_registry_method_arraymap_stdout_snapshot()` and `run_registry_method_arraymap_token_policy()`, so `run_registry_method_arraymap_canary()` is now a small orchestration wrapper over stdout acquisition, token assertions, pass reporting, and cleanup instead of mixing live prepare, synth fallback, and token-policy inline
- exact W14 probe wrapper was retired in `phase29bq` legacy cleanup, and the `phase2160/registry_optin_method_arraymap*` wrappers remain light regression canaries rather than the exact helper-local acceptance line
- retreat note: the old direct-lower probe now lives at `tools/smokes/v2/profiles/archive/core/phase2160/registry_optin_method_arraymap_direct_canary_vm.sh`; keep it as archived monitor evidence and do not hide it behind the shared registry helpers
- the W15 reinventory stop-line is landed: `tools/smokes/v2/lib/test_runner.sh` is now treated as near-thin-floor by default, and helper-local work should reopen only on a newly discovered exact seam
- the W16 first smoke-tail bucket is landed too: uniform raw `verify_program_via_builder_to_core` callers now collapse onto `run_verify_program_via_core_default_to_core()`, `run_verify_program_via_preferred_mirbuilder_core_to_core()`, `run_verify_program_via_builder_only_to_core()`, `run_verify_program_via_internal_builder_to_core()`, and `run_verify_program_via_registry_internal_to_core()` instead of repeating env stacks inline
- the W17 special raw verify keep bucket is landed too: `parser_embedded_json_canary.sh` now uses the generic rc wrapper directly, `mirbuilder_internal_new_array_core_exec_canary_vm.sh` now routes through `run_verify_program_via_internal_builder_no_methods_to_core()`, and built `newbox` MIR now honors `HAKO_VERIFY_PRIMARY=core` before the hv1 / hako-core lanes
- exact W17 proof now lives in `tools/dev/phase29ci_verify_primary_core_route_probe.sh`
- the W18 `phase2170` default MIR-file verify wrapper pack is landed too: repeated hakovm MIR-call env stacks now live behind `apply_verify_mir_route_env()`, `run_verify_mir_rc_with_env()`, and the named `run_verify_mir_via_hakovm_*` helpers in `tools/smokes/v2/lib/test_runner.sh`, and the default `phase2170` wrappers now collapse onto `run_verify_mir_canary_and_expect_rc()` while legacy `hv1_mircall_*` wrappers remain explicit keeps
- do not mix that `test_runner.sh` lane work with the 43-file smoke tail; the shared harness still stays the owner and the tail remains caller-audit-only
- the old `phase2044` blocker is closed: early alias warnings no longer panic before Ring0 init, the Rust Program(JSON)->MIR route no longer emits `main.params=[1]`, and `tools/smokes/v2/profiles/integration/proof/mirbuilder-provider/mirbuilder_provider_emit_core_exec_canary_vm.sh` is green again

## Immediate Next

1. keep `tools/hakorune_emit_mir.sh` monitor-only after the landed direct-emit fallback split
2. keep `tools/selfhost/selfhost_build.sh` monitor-only after the landed EXE consumer-path split and its helper-local probe
3. keep `tools/smokes/v2/lib/test_runner.sh` near-thin-floor after the landed helper-local slices and the landed `phase2170` default wrapper pack; there is no remaining exact helper-local bucket under the current `phase-29ci` scope
4. keep `phase2044/mirbuilder_provider_emit_core_exec_canary_vm.sh` green as monitor evidence; do not reopen vm-hako subset debt or widen directly into the 43-file smoke tail

## Current Read

- this helper-audit ledger is now `formal-close-synced` with the phase closeout
- helper-local slices are landed through W14, and caller-tail promotion is landed through W18
- explicit keep / monitor-only set remains:
  - `phase2044/*`
  - `phase2160/*`
  - `phase2170/hv1_mircall_*`
- reopen only if a new exact helper-local seam appears
