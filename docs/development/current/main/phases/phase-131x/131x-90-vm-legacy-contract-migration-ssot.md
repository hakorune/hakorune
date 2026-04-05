# Phase 131x SSOT: vm legacy contract migration

- Status: SSOT
- Date: 2026-04-05
- Scope: remaining explicit legacy `vm` contract caller surfaces only.

## Decision Frame

- `vm` remains an explicit legacy keep/debug family until the caller surfaces are migrated.
- `vm-hako` remains a reference/conformance lane.
- the explicit legacy contract smoke is archived; the remaining work starts from backend-hint chain and direct-route gates.
- `route_orchestrator.rs`, `stage1_bridge/direct_route/mod.rs`, and `stage1_bridge/plan.rs` are still behavior gates.
- `stub_child.rs` and `stage1_bridge/env/stage1_aliases.rs` are the next source seams because they still carry backend-hint chain behavior.
- `args.rs` default-vm decision is the last decision, and it now lives in phase-132x.

## Safe Order

1. remove backend-hint forwarding from the default child path
2. narrow direct-route selection in the stage1 bridge plan
3. isolate or remove `emit-mode-force-rust-vm-keep`
4. only then hand off `--backend vm` default behavior to phase-132x

## Success Criteria

- explicit legacy contract smoke is archived/retired
- default child path stays backend-hint free
- direct-route legacy gate remains isolated
- `args.rs` default-vm is not changed until the caller migration is done and the phase-132x decision is recorded

## Not In Scope

- vm-hako interpreter recut
- product/native route work
- unrelated cleanup lanes
