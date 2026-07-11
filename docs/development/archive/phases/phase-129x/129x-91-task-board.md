# Phase 129x Task Board

## A. Inventory

- [x] `src/cli/args.rs`
- [x] `src/runner/dispatch.rs`
- [x] `src/runner/route_orchestrator.rs`
- [x] `tools/selfhost/run.sh`
- [x] `tools/selfhost/lib/selfhost_run_routes.sh`
- [x] `src/runner/stage1_bridge/direct_route/mod.rs`

## B. Public Gate Follow-up

- [x] inventory remaining public `vm` wording in CLI/help/docs
- [x] decide whether any public `--backend vm` callsites can be demoted without breaking explicit legacy keep/debug callers
- [x] keep the direct-route legacy gate isolated

## C. Proof

- [x] `bash tools/selfhost/mainline/stage1_mainline_smoke.sh`
- [x] `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh`
- [x] `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
- [x] `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh`

## D. Closeout

- [x] update `CURRENT_TASK.md`
- [x] update `05-Restart-Quick-Resume.md`
- [x] update `10-Now.md`
- [x] update `15-Workstream-Map.md`
