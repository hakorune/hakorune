# Phase 130x: vm public gate final cleanup

- 目的: `--backend vm` の残り public gate surfaces を最終整理し、compat / proof / debug の explicit legacy keep としてだけ残すかを決める。
- 対象:
  - `src/cli/args.rs`
  - `src/runner/dispatch.rs`
  - `src/runner/route_orchestrator.rs`
  - `src/runner/stage1_bridge/direct_route/mod.rs`
- success:
  - public help/docs do not read `--backend vm` as a day-to-day route
  - compat/mainline remain route-first
  - direct-route legacy gate remains explicit and isolated
  - no new caller widens vm back into a default owner path

## Decision Now

- `phase-129x` kept route-first wording and removed the dead raw runtime fallback
- `route_orchestrator.rs` and `stage1_bridge/direct_route/mod.rs` are the remaining explicit legacy gate surfaces
- `phase-130x` decides whether those gates stay as explicit keep/debug only or can be demoted further without breaking compat/proof callers

## Next

1. re-check the remaining `vm` gate wording in `dispatch.rs` / `route_orchestrator.rs` / `direct_route/mod.rs`
2. decide whether `args.rs` should stay default-vm or be demoted later
3. keep the direct-route legacy gate isolated
4. then move to the next cleanup lane only if the remaining explicit gate is stable
