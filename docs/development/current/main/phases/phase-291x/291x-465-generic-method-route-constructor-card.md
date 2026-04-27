---
Status: Landed
Date: 2026-04-27
Scope: Add GenericMethodRoute constructor SSOT and remove direct route literals
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-464-generic-method-route-constructor-inventory-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-465: GenericMethodRoute Constructor

## Goal

Make `GenericMethodRoute` assembly flow through one constructor instead of
public struct literals.

This is BoxShape-only. It must not change route matching, JSON output, helper
selection, `.inc` behavior, or lowering tiers.

## Change

- Add `GenericMethodRoute::new(site, surface, evidence, operands, decision)`.
- Make `GenericMethodRoute` fields private.
- Replace matcher route literals with the constructor.
- Replace JSON fixture route literals with the constructor.
- Keep existing accessors as the public read surface.

## Verification

```bash
rg -n "GenericMethodRoute \\{" src/mir src/runner -g '*.rs'
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

- `GenericMethodRoute::new(...)` is the route record assembly entry.
- `GenericMethodRoute` fields are private; external consumers read through the
  existing accessors.
- Direct route struct literals were removed from matcher code and JSON fixture
  construction.
- Route matching, JSON field names, helper symbols, `.inc` behavior, and
  lowering tiers are unchanged.
- `rg -n "GenericMethodRoute \\{" src/mir src/runner -g '*.rs'` now reports
  only the struct definition, impl block, and type-signature references.
- `tools/checks/dev_gate.sh quick` passed. The existing chip8 release-artifact
  sync warning remained informational because the quick gate finished with
  `[dev-gate] profile=quick ok`.
