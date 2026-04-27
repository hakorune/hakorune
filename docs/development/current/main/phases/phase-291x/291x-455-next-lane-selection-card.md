---
Status: Landed
Date: 2026-04-27
Scope: Select next GenericMethodRoute cleanup lane
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/291x-454-generic-method-route-decision-closeout-card.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/map_lookup_fusion_plan.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-455: Next Lane Selection

## Goal

Select the next BoxShape-only compiler-cleanliness lane after
`GenericMethodRouteDecision`.

## Selected Lane

`GenericMethodRoute` still keeps observed route evidence as flat fields:

- `receiver_origin_box`
- `key_route`

These fields are not raw surface compatibility and not decided backend metadata.
They are MIR-observed evidence used by route consumers and JSON emitters.

The next lane is:

```text
GenericMethodRouteEvidence record split
```

## Constraints

- BoxShape-only.
- Do not change route matching.
- Do not change JSON field names or values.
- Do not change helper symbols, lowering tiers, or `.inc` behavior.
- Keep MapLookup fusion as a consumer of route evidence, not a re-deriver.

## Next

Inventory all `receiver_origin_box` / `key_route` GenericMethodRoute consumers
before editing code.
