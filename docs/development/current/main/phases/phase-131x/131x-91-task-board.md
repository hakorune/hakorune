# Phase 131x Task Board

## A. Inventory

- [x] `tools/smokes/v2/profiles/integration/apps/archive/phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh`
- [x] `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_observability_vm.sh`
- [x] `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_strict_dev_priority_vm.sh`
- [x] `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_derust_strict_default_route_vm.sh`
- [x] `src/runner/stage1_bridge/stub_child.rs`
- [x] `src/runner/stage1_bridge/plan.rs`
- [x] `src/runner/route_orchestrator.rs`
- [x] `src/cli/args.rs`

## B. Migration Order

- [x] archive the explicit legacy contract smoke
- [ ] confirm the default child path stays backend-hint free
- [ ] narrow the direct-route selection path in `plan.rs`
- [ ] isolate or remove `emit-mode-force-rust-vm-keep`
- [ ] only then re-evaluate `args.rs` default-vm

## C. Proof

- [ ] `bash tools/selfhost/mainline/stage1_mainline_smoke.sh`
- [ ] `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh`
- [ ] `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
- [x] `bash tools/smokes/v2/profiles/integration/apps/archive/phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh`

## D. Closeout

- [ ] update `CURRENT_TASK.md`
- [ ] update `05-Restart-Quick-Resume.md`
- [ ] update `10-Now.md`
- [ ] update `15-Workstream-Map.md`
