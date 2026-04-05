# Phase 122x: vm compat route exit plan

- 目的: `runtime-route compat` / `runtime-mode stage-a-compat` / raw `--backend vm` の依存を、どの順で外すかを具体化する。
- 対象:
  - `tools/selfhost/run.sh`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - `src/runner/stage1_bridge/direct_route/mod.rs`
  - `src/runner/route_orchestrator.rs`
  - compat route を説明している current/docs surface
- success:
  - compat route の exit plan が `shell alias -> bridge direct route -> backend gate` の順で読める
  - `phase-123x proof gate shrink follow-up` に渡す具体順が固定される

## First-pass exit order

1. shell compat surface
   - `tools/selfhost/run.sh`
   - `tools/selfhost/lib/selfhost_run_routes.sh`
2. Stage1 direct bridge
   - `src/runner/stage1_bridge/direct_route/mod.rs`
3. backend/lane gate
   - `src/runner/route_orchestrator.rs`
   - `src/runner/dispatch.rs`
