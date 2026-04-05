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

- `tools/selfhost/lib/selfhost_run_routes.sh`
  - `runtime-route compat` still shells into raw `--backend vm`
- `src/runner/stage1_bridge/direct_route/mod.rs`
  - binary-only direct run still requires backend `vm`
- `src/runner/route_orchestrator.rs`
  - still carries `emit-mode-force-rust-vm-keep`
- `src/runner/dispatch.rs`
  - still exposes public `backend=vm`
- `src/cli/args.rs`
  - `--backend` still defaults to `vm`
