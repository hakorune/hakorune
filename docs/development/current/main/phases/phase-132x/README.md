# Phase 132x: vm default backend decision

- 目的: `args.rs` から default `vm` を外し、mainline/default と explicit vm keep を分離する。
- 対象:
  - `src/cli/args.rs`
  - `src/runner/dispatch.rs`
  - `src/runner/stage1_bridge/direct_route/mod.rs`
  - `tools/archive/legacy-selfhost/stage1-cli/stage1_minimal.sh`（archived）
  - `tools/archive/legacy-selfhost/stage1-cli/stage1_debug.sh`（archived）
  - `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_direct_emit_dominance_block_vm.sh`
- success:
  - caller bucketization is complete
  - `--backend vm` is no longer the default backend
  - explicit vm / vm-hako proof-debug callers still work
  - help/docs wording matches the final decision
  - phase-131x migration lands cleanly into the default-removal implementation

## Decision Now

- the legacy `vm` contract smoke is already archived
- default child path is backend-hint free
- direct-route selection is narrowed
- the legacy emit-mode special-case has been removed from `route_orchestrator.rs`
- decision fixed: remove `vm` from the default backend now
- explicit vm / vm-hako proof-debug / compat callers remain explicit keep

## Caller Buckets

- move to mainline / route-first candidates:
  - `tools/using_e2e_smoke.sh`
- keep-now explicit vm / vm-hako proof-debug / compat:
  - `tools/using_unresolved_smoke.sh`
  - `tools/using_resolve_smoke.sh`
  - `tools/using_strict_path_fail_smoke.sh`
  - `tools/selfhost_read_tmp_dev_smoke.sh`
  - `tools/archive/legacy-selfhost/engineering/ny_selfhost_inline.sh`
  - `tools/selfhost/proof/run_stageb_compiler_vm.sh`
  - `tools/selfhost/proof/selfhost_vm_smoke.sh`
  - `tools/selfhost/proof/bootstrap_selfhost_smoke.sh`
  - `tools/selfhost/proof/selfhost_smoke.sh`
  - `tools/plugins/plugin_v2_smoke.sh`
  - `tools/hako_check.sh`
  - `tools/dev_stagea.sh`
  - `tools/dev_stageb.sh`
  - `tools/dev/hako_debug_run.sh`
  - `tools/phi_trace_bridge_try.sh`
  - `tools/modules_smoke.sh`
  - `tools/exe_first_smoke.sh`
  - `tools/exe_first_runner_smoke.sh`
  - `tools/selfhost_stage2_bridge_smoke.sh`
  - phase29x / phase29bq / phase29y / phase29z / phase29cc / phase21_5 explicit vm suites
- archived after phase-29cv P43 stale validation:
  - `tools/archive/manual-smokes/selfhost_json_guard_smoke.sh`
  - `tools/archive/manual-smokes/selfhost_parser_json_smoke.sh`
  - `tools/archive/manual-smokes/ny_stage2_new_method_smoke.sh`
- delete/archive candidate:
  - none remaining in the active tree; archive-only surfaces already live under `tools/archive/**`

## Next

1. change `args.rs` default away from `vm`
2. align help / error wording with mainline default + explicit keep callers
3. update the last route-first candidate only where semantics do not depend on vm-family behavior

## Current Result

- `src/cli/args.rs` now defaults `--backend` to `mir`
- `src/runner/dispatch.rs` unknown-backend wording now points to `mir` as mainline/default
- `README.md` / `README.ja.md` no longer describe raw CLI ingress as defaulting to `vm`
- `tools/using_e2e_smoke.sh` now uses `tools/selfhost/run.sh --runtime --runtime-route mainline`
- `phase21_5_perf_direct_emit_dominance_block_vm.sh` now pins its first two legacy assertions to explicit `--backend vm`
- `tools/stage1_minimal.sh` and `tools/stage1_debug.sh` are retired from
  active root tools; use `tools/selfhost/compat/run_stage1_cli.sh` for the
  current Stage1 CLI compatibility wrapper.
- The former root ny selfhost inline helper is retired from active root tools;
  historical evidence lives under
  `tools/archive/legacy-selfhost/engineering/ny_selfhost_inline.sh`.
- The former root selfhost JSON guard, selfhost parser JSON smoke, and Stage-2
  new/method smoke are retired from active root tools after P43 stale
  validation; historical evidence lives under `tools/archive/manual-smokes/`.
