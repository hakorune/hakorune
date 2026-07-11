# Phase 130x SSOT: vm public gate final cleanup

- Status: SSOT
- Date: 2026-04-05
- Scope: `--backend vm` / vm-family public gate surfaces の最終整理だけを置く。

## Decision Frame

- `vm` remains an explicit legacy keep/debug family.
- `vm-hako` remains a reference/conformance lane.
- `route_orchestrator.rs` and `stage1_bridge/direct_route/mod.rs` are the remaining explicit legacy gate surfaces.
- `args.rs` default-vm policy is still a later decision; do not change it casually.

## Success Criteria

- public help/docs do not make `--backend vm` look like day-to-day ownership
- compat/mainline stay route-first
- direct-route legacy gate stays isolated
- no caller widens vm back into a default owner path

## Not In Scope

- vm-hako interpreter recut
- product/native route work
- unrelated cleanup lanes
