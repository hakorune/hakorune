# Phase 126x: vm public gate shrink decision

- 目的: `--backend vm` を public explicit gate からさらに縮められるかを、active compat/proof/debug contracts 付きで判断する。
- 対象:
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - `tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
  - `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh`
  - `src/runner/stage1_bridge/plan.rs`
  - `src/runner/stage1_bridge/args.rs`
  - `src/runner/stage1_bridge/env/stage1_aliases.rs`
  - `src/runner/stage1_bridge/direct_route/mod.rs`
  - `src/config/env/stage1.rs`
  - `src/runner/route_orchestrator.rs`
  - `src/runner/dispatch.rs`
  - `src/cli/args.rs`
- success:
  - shell compat, bridge direct route, backend gate のどこが hard blocker でどこが soft blocker かを言い切る
  - `phase-127x compat route raw vm cut prep` に渡す decision-now が固定される

## Decision Buckets

### hard blockers

- `tools/selfhost/lib/selfhost_run_routes.sh`
  - `runtime-route compat` still shells into raw `--backend vm`
- `tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
  - explicit fallback success still asserts `vm-route/pre-dispatch` and `lane=vm-compat-fallback`
- `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh`
  - `runtime-route compat` is still the positive explicit fallback lane

### source-side blockers

- `src/runner/stage1_bridge/plan.rs`
  - `backend_cli_hint().is_some()` still selects `BinaryOnlyRunDirect` when run-direct is enabled
- `src/runner/stage1_bridge/args.rs`
  - stage1 child args still materialize `run --backend <hint> <source>`
- `src/runner/stage1_bridge/env/stage1_aliases.rs`
  - child env still propagates `NYASH_STAGE1_BACKEND` / `STAGE1_BACKEND`
- `src/config/env/stage1.rs`
  - backend hint and binary-only direct toggles are still live SSOT
- `src/runner/stage1_bridge/direct_route/mod.rs`
  - binary-only direct run still rejects any backend except `vm`

### route-agnostic keep

- `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_route_smoke_vm.sh`
  - `mode=pipeline-entry`
  - `runtime_route=compat|mainline`
  - `runtime_mode=stage-a-compat|exe`
  - these assertions can survive a compat MIR-handoff route
- `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh`
  - route label and semantic parity checks can survive a compat MIR-handoff route
  - only the explicit fallback gate/env expectation needs care

### soft blockers

- `src/runner/route_orchestrator.rs`
  - `emit-mode-force-rust-vm-keep` should be revisited only after compat route changes
- `src/runner/dispatch.rs`
  - public `backend=vm` branch is still needed while compat/proof/debug remain live
- `src/cli/args.rs`
  - `--backend` default/help surface is last-cut only

## Decision Now

- do not shrink public `--backend vm` yet
- first cut must be compat route contract, not CLI/default wording
- proof gates stay explicit and optional
- after compat contract softens, the next source seam is the Stage1 bridge backend-hint chain, not `args.rs`

## Minimal Change Before Raw-VM Exit

1. switch compat branch in `tools/selfhost/lib/selfhost_run_routes.sh`
   - from raw `--backend vm`
   - to temp-MIR handoff
2. replace `phase29x_vm_route_non_strict_compat_boundary_vm.sh` assertions
   - remove raw `vm-route/pre-dispatch`
   - remove raw `lane=vm-compat-fallback`
   - keep explicit compat gate / fallback-env contract
3. keep `phase29bq` runtime route/parity smokes
   - route labels stay valid
   - parity checks stay valid
4. then cut the Stage1 bridge backend-hint chain
   - `src/runner/stage1_bridge/plan.rs`
   - `src/runner/stage1_bridge/args.rs`
   - `src/runner/stage1_bridge/env/stage1_aliases.rs`
   - `src/config/env/stage1.rs`
   - `src/runner/stage1_bridge/direct_route/mod.rs`
5. revisit orchestrator / dispatch / CLI default only after compat shell and Stage1 bridge no longer reintroduce raw `--backend vm`
