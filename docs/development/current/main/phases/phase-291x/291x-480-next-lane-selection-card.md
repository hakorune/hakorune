---
Status: Landed
Date: 2026-04-27
Scope: Select next compiler-cleanliness lane after GenericMethodRoute visibility prune
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-478-generic-method-route-visibility-closeout-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-480: Next Lane Selection

## Selected Lane

GenericMethodRoute JSON fixture construction isolation.

## Why This Lane

The previous lane made GenericMethodRoute construction records crate-private and
kept JSON emission on stable route accessors. One seam remains too wide:

```text
src/runner/mir_json_emit/tests/generic_method_routes.rs
  imports GenericMethodRoute construction records directly
  constructs route internals outside the MIR owner module
```

That is acceptable for a test, but it keeps runner JSON fixtures coupled to the
route owner's internal construction shape. The cleaner boundary is:

```text
src/mir/generic_method_route_plan.rs
  owns route construction details
  exposes cfg(test) fixture builders

src/runner/mir_json_emit/tests/*
  consumes ready-made GenericMethodRoute fixtures
  asserts JSON shape only
```

## Boundaries

- BoxShape-only.
- Do not change route detection or route metadata refresh.
- Do not change JSON field names, helper symbols, proof tags, or route order.
- Do not change `.inc` lowering behavior.
- Do not add hot lowering, MapGet specialization, or new CoreMethod rows.

## Next

Inventory GenericMethodRoute fixture construction consumers and then isolate
runner JSON fixtures behind owner-provided `#[cfg(test)]` builders.
