# Phase 131x Task Board

## A. Inventory

- [x] `tools/smokes/v2/profiles/integration/apps/archive/phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh`
- [x] `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_observability_vm.sh`
- [x] `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_strict_dev_priority_vm.sh`
- [x] `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_derust_strict_default_route_vm.sh`
- [x] `src/runner/stage1_bridge/stub_child.rs`
- [x] `src/runner/stage1_bridge/env/stage1_aliases.rs`
- [x] `src/runner/stage1_bridge/plan.rs`
- [x] `src/runner/route_orchestrator.rs`
- [x] `src/cli/args.rs`

## B. Migration Order

- [x] archive the explicit legacy contract smoke
- [x] confirm the default child path stays backend-hint free
- [x] narrow the direct-route selection path in `plan.rs`
- [x] isolate or remove `emit-mode-force-rust-vm-keep`
- [x] hand off `args.rs` default-vm decision to phase-132x

## C. Proof

- [x] `bash tools/selfhost/mainline/stage1_mainline_smoke.sh`
- [x] `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh`
- [x] `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
- [x] `bash tools/smokes/v2/profiles/integration/apps/archive/phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh`

## D. Closeout

- [x] update `CURRENT_TASK.md`
- [x] update `05-Restart-Quick-Resume.md`
- [x] update `10-Now.md`
- [x] update `15-Workstream-Map.md`
