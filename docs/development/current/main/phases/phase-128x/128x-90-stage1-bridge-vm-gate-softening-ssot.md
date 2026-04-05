# Phase 128x SSOT

- Current lane: `stage1 bridge vm gate softening`
- Scope:
  - `stage1_bridge` backend-hint chain
  - direct bridge `backend=vm` gate
  - compat/direct bridge caller path

## Working hypothesis

- `compat` route is already route-first and green again
- the remaining `vm` pressure lives in `stage1_bridge` and its direct-route helpers
- the next safe cut is to inventory the bridge helpers before touching `dispatch` / `route_orchestrator`

## Exit criteria

- explicit compat fallback remains opt-in
- public docs do not read `--backend vm` as a day-to-day route
- bridge helpers no longer hide a mandatory `backend=vm` dependence for route-first callers
