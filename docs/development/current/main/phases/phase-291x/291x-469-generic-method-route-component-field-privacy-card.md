---
Status: Landed
Date: 2026-04-27
Scope: Make GenericMethodRoute component fields private
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-468-generic-method-route-component-field-inventory-card.md
  - src/mir/generic_method_route_plan.rs
---

# 291x-469: GenericMethodRoute Component Field Privacy

## Goal

Make `GenericMethodRoute` component records constructor-only from outside the
route-plan module.

This is BoxShape-only. It must not change route matching, JSON output, helper
selection, `.inc` behavior, or lowering tiers.

## Change

- Make fields private on:
  - `GenericMethodRouteSurface`
  - `GenericMethodRouteSite`
  - `GenericMethodRouteEvidence`
  - `GenericMethodRouteOperands`
  - `GenericMethodRouteDecision`
- Keep the existing public constructors unchanged.
- Keep `GenericMethodRoute` accessors as the public read surface.

## Verification

```bash
cargo test -q generic_method_route
cargo test -q build_mir_json_root_emits_generic_method_routes
cargo test -q map_lookup_fusion
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Result

PASS.

- `GenericMethodRouteSurface`, `GenericMethodRouteSite`,
  `GenericMethodRouteEvidence`, `GenericMethodRouteOperands`, and
  `GenericMethodRouteDecision` fields are private.
- Existing public constructors remain unchanged.
- `GenericMethodRoute` accessors remain the public read surface.
- Route matching, JSON field names, helper symbols, `.inc` behavior, and
  lowering tiers are unchanged.
- `tools/checks/dev_gate.sh quick` passed. The existing chip8 release-artifact
  sync warning remained informational because the quick gate finished with
  `[dev-gate] profile=quick ok`.
