---
Status: Landed
Date: 2026-04-27
Scope: Select next GenericMethodRoute cleanup lane
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/291x-458-generic-method-route-evidence-closeout-card.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/map_lookup_fusion_plan.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-459: Next Lane Selection

## Goal

Select the next BoxShape-only compiler-cleanliness lane after
`GenericMethodRouteEvidence`.

## Selected Lane

`GenericMethodRoute` now has separate records for surface, observed evidence,
and decided backend metadata. The remaining flat fields are:

- site: `block`, `instruction_index`
- operands: `receiver_value`, `key_value`, `result_value`

The next lane is:

```text
GenericMethodRouteSite / GenericMethodRouteOperands record split
```

## Constraints

- BoxShape-only.
- Do not change route matching.
- Do not change JSON field names or values.
- Do not change helper symbols, lowering tiers, or `.inc` behavior.
- Keep MapLookup fusion as a consumer of route site/operands, not a re-deriver.

## Next

Inventory all site and operand consumers before editing code.
