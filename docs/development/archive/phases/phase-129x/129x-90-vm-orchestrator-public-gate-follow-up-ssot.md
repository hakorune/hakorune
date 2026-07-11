# Phase 129x SSOT

- Current lane: `vm orchestrator/public gate follow-up`
- Scope:
  - `src/cli/args.rs`
  - `src/runner/dispatch.rs`
  - `src/runner/route_orchestrator.rs`
  - `tools/selfhost/run.sh`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - `src/runner/stage1_bridge/direct_route/mod.rs`

## Working Hypothesis

- `phase-128x` already narrowed the backend-hint chain and removed default child backend forwarding
- the remaining work is public gate wording / gate placement, not compat temp-MIR plumbing
- the binary-only direct-route `vm` gate is an explicit legacy contract and should stay isolated while public surfaces are reviewed

## Exit Criteria

- `--backend vm` no longer reads as a day-to-day route in public help/docs
- explicit compat / proof / debug callers remain intentional and documented
- the direct-route legacy gate stays isolated from the route-first shell surfaces
- no new caller reintroduces a default `vm` owner path
