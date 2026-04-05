# Phase 131x SSOT: vm legacy contract migration

- Status: SSOT
- Date: 2026-04-05
- Scope: remaining explicit legacy `vm` contract surfaces only.

## Decision Frame

- `vm` remains an explicit legacy keep/debug family until the caller surfaces are migrated.
- `vm-hako` remains a reference/conformance lane.
- `route_orchestrator.rs`, `stage1_bridge/direct_route/mod.rs`, and `stage1_bridge/plan.rs` are still behavior gates.
- `stub_child.rs` is the next source seam because it still carries backend-hint chain behavior.
- `args.rs` default-vm is the last decision, not the first.

## Safe Order

1. migrate or retire the explicit legacy contract smoke
2. remove backend-hint forwarding from the default child path
3. narrow direct-route selection in the stage1 bridge plan
4. isolate or remove `emit-mode-force-rust-vm-keep`
5. only then re-evaluate `--backend vm` default behavior

## Success Criteria

- explicit legacy contract smoke is route-first or retired
- default child path stays backend-hint free
- direct-route legacy gate remains isolated
- `args.rs` default-vm is not changed until the caller migration is done

## Not In Scope

- vm-hako interpreter recut
- product/native route work
- unrelated cleanup lanes
