# Phase 131x: vm legacy contract migration

- 目的: 残っている explicit legacy `vm` contract smoke は archive 済みにして、backend-hint chain と direct-route gate を順番に畳む。
- 対象:
  - `tools/smokes/v2/profiles/integration/apps/archive/phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh`
  - `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_observability_vm.sh`
  - `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_strict_dev_priority_vm.sh`
  - `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_derust_strict_default_route_vm.sh`
  - `src/runner/stage1_bridge/stub_child.rs`
  - `src/runner/stage1_bridge/env/stage1_aliases.rs`
  - `src/runner/stage1_bridge/plan.rs`
  - `src/runner/route_orchestrator.rs`
  - `src/cli/args.rs`
- success:
  - explicit legacy contract smoke is archived and does not look like day-to-day ownership
  - default child path stays backend-hint free
  - backend-hint child env only survives on explicit override paths
  - direct-route legacy gate remains isolated until its caller contract is migrated
  - `args.rs` default-vm is only re-evaluated after the caller surfaces are cut

## Decision Now

- worker inventory confirmed that wording-only is no longer enough
- the remaining `vm` gate is behavior, not just docs
- safe order is:
  1. narrow the backend-hint chain in `stub_child.rs` / `env/stage1_aliases.rs`
  2. narrow direct-route selection in `plan.rs`
  3. isolate or remove `emit-mode-force-rust-vm-keep` in `route_orchestrator.rs`
  4. only then re-evaluate `args.rs` default-vm

## Next

1. confirm the default child path stays backend-hint free
2. narrow the direct-route selection path only after the caller contract is isolated
3. keep the `--backend vm` default as a last-step decision
