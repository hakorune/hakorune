# Phase 130x Task Board

## A. Inventory

- [ ] `src/cli/args.rs`
- [ ] `src/runner/dispatch.rs`
- [ ] `src/runner/route_orchestrator.rs`
- [ ] `src/runner/stage1_bridge/direct_route/mod.rs`

## B. Final Cleanup

- [ ] decide whether public `--backend vm` can be demoted later without breaking explicit legacy keep/debug callers
- [ ] keep the direct-route legacy gate isolated
- [ ] make sure `compat` and `mainline` remain route-first in wording

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
