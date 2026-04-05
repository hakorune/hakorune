# Phase 131x: vm legacy contract migration

- 目的: 残っている explicit legacy `vm` contract smoke を route-first contract へ移し、backend-hint chain と direct-route gate を順番に畳む。
- 対象:
  - `tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh`
  - `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_observability_vm.sh`
  - `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_strict_dev_priority_vm.sh`
  - `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_derust_strict_default_route_vm.sh`
  - `src/runner/stage1_bridge/stub_child.rs`
  - `src/runner/stage1_bridge/plan.rs`
  - `src/runner/route_orchestrator.rs`
  - `src/cli/args.rs`
- success:
  - explicit legacy contract smoke does not look like day-to-day ownership
  - default child path stays backend-hint free
  - direct-route legacy gate remains isolated until its caller contract is migrated
  - `args.rs` default-vm is only re-evaluated after the caller surfaces are cut

## Decision Now

- worker inventory confirmed that wording-only is no longer enough
- the remaining `vm` gate is behavior, not just docs
- safe order is:
  1. migrate explicit legacy contract smoke to route-first or retire it
  2. narrow the backend-hint chain in `stub_child.rs`
  3. narrow direct-route selection in `plan.rs`
  4. isolate or remove `emit-mode-force-rust-vm-keep` in `route_orchestrator.rs`
  5. only then re-evaluate `args.rs` default-vm

## Next

1. move the explicit legacy contract smoke to the new route-first contract
2. confirm the default child path stays backend-hint free
3. narrow the direct-route selection path only after the caller contract is isolated
4. keep the `--backend vm` default as a last-step decision
