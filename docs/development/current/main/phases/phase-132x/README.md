# Phase 132x: vm default backend decision

- 目的: `args.rs` の default backend が `vm` のままでよいかを、omitted-backend caller inventory を根拠に最後に決める。
- 対象:
  - `src/cli/args.rs`
  - `src/runner/dispatch.rs`
  - `src/runner/stage1_bridge/direct_route/mod.rs`
  - `tools/stage1_minimal.sh`
  - `tools/stage1_debug.sh`
  - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_direct_emit_dominance_block_vm.sh`
- success:
  - omitted-backend caller inventory is complete
  - `--backend vm` default is either retained as an explicit legacy keep/debug default or moved only after all dependent callers are updated
  - help/docs wording matches the final decision
  - phase-131x migration lands cleanly into this decision lane

## Decision Now

- the legacy `vm` contract smoke is already archived
- default child path is backend-hint free
- direct-route selection is narrowed
- the legacy emit-mode special-case has been removed from `route_orchestrator.rs`
- the last open question is whether `args.rs` should keep `vm` as the default backend or move to an explicit legacy-only surface

## Caller Buckets

- move to mainline / route-first candidates:
  - `tools/selfhost_json_guard_smoke.sh`
  - `tools/selfhost_parser_json_smoke.sh`
  - `tools/using_unresolved_smoke.sh`
  - `tools/using_resolve_smoke.sh`
  - `tools/using_e2e_smoke.sh`
  - `tools/using_strict_path_fail_smoke.sh`
  - `tools/selfhost_read_tmp_dev_smoke.sh`
  - `tools/ny_selfhost_inline.sh`
- keep-now explicit vm / vm-hako proof-debug / compat:
  - `tools/selfhost/proof/run_stageb_compiler_vm.sh`
  - `tools/selfhost/proof/selfhost_vm_smoke.sh`
  - `tools/selfhost/proof/bootstrap_selfhost_smoke.sh`
  - `tools/selfhost/proof/selfhost_smoke.sh`
  - `tools/plugins/plugin_v2_smoke.sh`
  - `tools/hako_check.sh`
  - `tools/dev_stagea.sh`
  - `tools/dev_stageb.sh`
  - `tools/dev/hako_debug_run.sh`
  - `tools/ny_stage2_new_method_smoke.sh`
  - `tools/phi_trace_bridge_try.sh`
  - `tools/modules_smoke.sh`
  - `tools/exe_first_smoke.sh`
  - `tools/exe_first_runner_smoke.sh`
  - `tools/selfhost_stage2_bridge_smoke.sh`
  - phase29x / phase29bq / phase29y / phase29z / phase29cc / phase21_5 explicit vm suites
- delete/archive candidate:
  - none in the active tree; archive-only surfaces already live under `tools/archive/**`

## Next

1. inventory callers that omit `--backend`
2. decide whether `vm` should remain the default backend
3. update help/docs/callers in one shot after the decision
