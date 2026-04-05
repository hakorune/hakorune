# Phase 125x: vm bridge/backend gate follow-up

- 目的: docs/manual demotion 後に残る compat bridge / backend gate blockers を source-backed に絞り込む。
- 対象:
  - `src/runner/stage1_bridge/direct_route/mod.rs`
  - `src/runner/route_orchestrator.rs`
  - `src/runner/dispatch.rs`
  - `src/cli/args.rs`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
- success:
  - shell compat surface, bridge direct route, backend/lane gate の blocker が再確認できる
  - `phase-126x vm public gate shrink decision` に渡す cut order が読める

## Follow-up Order

1. Stage1 direct bridge
2. route/backend gate
3. CLI default/help surface

## Known Blockers

- proof-gate keep remains public
  - `tools/selfhost/proof/run_stageb_compiler_vm.sh`
  - `tools/selfhost/proof/selfhost_vm_smoke.sh`
  - top-level facades and `make smoke-selfhost` still expose these as intentional proof/debug front doors
  - this lane does not demote/delete them; it only clarifies the source blockers behind raw `--backend vm`
- `tools/selfhost/lib/selfhost_run_routes.sh`
  - `runtime-route compat` still shells into raw `--backend vm`
  - `run.sh --direct` still shells into `tools/selfhost/proof/run_stageb_compiler_vm.sh`
- `src/runner/stage1_bridge/direct_route/mod.rs`
  - binary-only direct run still requires backend `vm`
- `src/runner/route_orchestrator.rs`
  - still carries `emit-mode-force-rust-vm-keep`
- `src/runner/dispatch.rs`
  - still exposes public `backend=vm`
- `src/cli/args.rs`
  - `--backend` still defaults to `vm`

## Caller Buckets

### shell / proof callers

- `tools/selfhost/lib/selfhost_run_routes.sh`
- `tools/selfhost/proof/selfhost_smoke.sh`
- `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
- `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_box_from_min_vm.sh`
- `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_lambda_literal_pair_min_vm.sh`
- `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_method_boundary_min_vm.sh`
- `tools/smokes/v2/profiles/integration/selfhost/phase29cc_selfhost_stageb_funcscanner_typed_params_implements_min_vm.sh`
- `tools/smokes/v2/profiles/integration/parser/parser_rune_decl_local_attrs_selected_entry_trace.sh`

### bridge / backend gate blockers

- `src/runner/stage1_bridge/direct_route/mod.rs`
- `src/runner/route_orchestrator.rs`
- `src/runner/dispatch.rs`
- `src/cli/args.rs`

## Cut Order Decision

1. keep proof gates public but optional
   - no new callers
   - no new broad docs/manual examples
2. cut shell compat dependence before backend gate changes
   - otherwise `runtime-route compat` still reintroduces raw `--backend vm`
3. cut direct bridge dependence next
   - otherwise binary-only direct route still requires backend `vm`
4. touch backend gate and CLI defaults last
   - only after shell and bridge no longer need raw `--backend vm`
