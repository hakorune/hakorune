---
Status: Landed
Date: 2026-04-27
Scope: Isolate GenericMethodRoute JSON fixture construction behind owner test support
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-481-generic-method-route-fixture-construction-inventory-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-482: GenericMethodRoute Fixture Construction Isolation

## Goal

Move JSON fixture route construction details behind the GenericMethodRoute owner
module so runner JSON tests assert metadata output without owning route internals.

## Result

- Added `generic_method_route_plan::test_support` behind `#[cfg(test)]`.
- Added one fixture builder per JSON route shape currently asserted by the
  runner test.
- Replaced direct imports of `GenericMethodRoute*` construction records in the
  runner JSON test with owner-provided fixture builders.
- Preserved route order and JSON assertions.

## Boundaries

- BoxShape-only.
- Preserve route order and all JSON assertions.
- Do not change route detection, emitted JSON fields, helper symbols, proof
  tags, route ids, `.inc` behavior, or lowering tiers.

## Verification

Ran:

```bash
cargo check -q
cargo test -q build_mir_json_root_emits_generic_method_routes
cargo test -q generic_method_route
```
