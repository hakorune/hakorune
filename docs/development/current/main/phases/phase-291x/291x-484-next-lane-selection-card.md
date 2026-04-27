---
Status: Landed
Date: 2026-04-27
Scope: Select next compiler-cleanliness lane after GenericMethodRoute fixture isolation
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-483-generic-method-route-fixture-construction-closeout-card.md
  - src/mir/map_lookup_fusion_plan.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/map_lookup_fusion_routes.rs
---

# 291x-484: Next Lane Selection

## Selected Lane

MapLookupFusionRoute public field and JSON fixture boundary cleanup.

## Why This Lane

`MapLookupFusionRoute` is MIR-owned metadata derived from
`generic_method_routes`, but its full record shape is still public:

```text
src/mir/map_lookup_fusion_plan.rs
  pub struct MapLookupFusionRoute { pub ... }

src/runner/mir_json_emit/root.rs
  reads fields directly for JSON emission

src/runner/mir_json_emit/tests/map_lookup_fusion_routes.rs
  manually constructs the public field record
```

That keeps route internals as the consumer contract. The cleaner boundary is
the same shape used for GenericMethodRoute:

```text
MapLookupFusionRoute
  -> stable read accessors
  -> owner-provided cfg(test) fixture builder

runner JSON emitter/tests
  -> consume accessors/fixture only
```

## Boundaries

- BoxShape-only.
- Do not change MapLookup fusion detection or refresh ordering.
- Do not change JSON field names, values, effects, route ids, or lowering tiers.
- Do not change `.inc` metadata readers or const-fold lowering behavior.
- Do not add new MapGet/MapHas proof or hot lowering.

## Next

Inventory external field readers and fixture construction before pruning the
public field surface.
