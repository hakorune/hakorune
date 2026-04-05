# Phase 129x: vm orchestrator/public gate follow-up

- 目的: `--backend vm` の public gate / orchestrator surface を再点検し、日常 route と explicit legacy keep/debug を分ける。
- 対象:
  - `src/cli/args.rs`
  - `src/runner/dispatch.rs`
  - `src/runner/route_orchestrator.rs`
  - `tools/selfhost/run.sh`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - `src/runner/stage1_bridge/direct_route/mod.rs`
- success:
  - public help/docs do not read `--backend vm` as a day-to-day route
  - compat/mainline remain route-first
  - `selfhost/route` child tags use canonical `mainline|compat` while alias inputs remain accepted
  - binary-only direct-route vm gate remains explicit legacy and isolated
  - no new caller widens vm back into a default owner path

## Decision Now

- `phase-128x` is landed with the backend-hint chain narrowed
- default `stage1_cli_env.hako` child path no longer forwards backend hints
- runtime child route tags now emit canonical `mainline|compat`
- binary-only direct-route vm gate remains an explicit legacy contract
- `phase-129x` follows the remaining public vm gate / orchestrator surfaces

## Next

1. inventory remaining public `vm` wording in CLI/help/docs
2. decide whether any public `--backend vm` callsites can be demoted without breaking explicit legacy keep/debug callers
3. keep the direct-route legacy gate isolated
4. then move to the next vm public-gate cleanup lane
