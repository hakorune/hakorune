# Phase 129x Task Board

## A. Inventory

- [x] `src/cli/args.rs`
- [x] `src/runner/dispatch.rs`
- [ ] `src/runner/route_orchestrator.rs`
- [x] `tools/selfhost/run.sh`
- [ ] `tools/selfhost/lib/selfhost_run_routes.sh`
- [ ] `src/runner/stage1_bridge/direct_route/mod.rs`

## B. Public Gate Follow-up

- [x] inventory remaining public `vm` wording in CLI/help/docs
- [ ] decide whether any public `--backend vm` callsites can be demoted without breaking explicit legacy keep/debug callers
- [x] keep the direct-route legacy gate isolated

## C. Proof

- [ ] `bash tools/selfhost/mainline/stage1_mainline_smoke.sh`
- [ ] `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh`
- [ ] `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
- [ ] `bash tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh`

## D. Closeout

- [ ] update `CURRENT_TASK.md`
- [ ] update `05-Restart-Quick-Resume.md`
- [ ] update `10-Now.md`
- [ ] update `15-Workstream-Map.md`
