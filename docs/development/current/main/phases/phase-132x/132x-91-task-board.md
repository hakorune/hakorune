# Phase 132x Task Board

## A. Inventory

- move to mainline / route-first candidates
  - `tools/selfhost_json_guard_smoke.sh`
  - `tools/selfhost_parser_json_smoke.sh`
  - `tools/using_unresolved_smoke.sh`
  - `tools/using_resolve_smoke.sh`
  - `tools/using_e2e_smoke.sh`
  - `tools/using_strict_path_fail_smoke.sh`
  - `tools/selfhost_read_tmp_dev_smoke.sh`
  - `tools/ny_selfhost_inline.sh`

- keep now as explicit vm / vm-hako proof-debug / compat
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
  - `tools/smokes/v2/profiles/integration/phase29x/**`
  - `tools/smokes/v2/profiles/integration/selfhost/phase29bq/**`
  - `tools/smokes/v2/profiles/integration/apps/phase29y_*/phase29z_*/phase29cc_*/phase21_5_perf_direct_emit_dominance_block_vm.sh`

- delete/archive candidate
  - none in the active tree

## B. Decision

- [ ] omitted-backend caller inventory complete
- [ ] remove `vm` from the default backend
- [ ] align help/docs with mainline default + explicit legacy keep/debug wording
- [ ] keep explicit vm / vm-hako proof-debug callers alive
- [ ] move route-first candidates out of `--backend vm` calls where semantics do not change

## C. Proof

- [ ] `cargo test --lib route_orchestrator::tests --quiet`
- [ ] `cargo test --lib stage1_bridge::env::tests --quiet`
- [ ] `cargo test --lib stage1_bridge::plan::tests --quiet`
- [ ] `cargo test --lib stage1_bridge::stub_child::tests --quiet`
- [ ] `bash tools/selfhost/mainline/stage1_mainline_smoke.sh`
- [ ] `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_direct_emit_dominance_block_vm.sh`

## D. Closeout

- [ ] update `CURRENT_TASK.md`
- [ ] update `05-Restart-Quick-Resume.md`
- [ ] update `10-Now.md`
- [ ] update `15-Workstream-Map.md`
