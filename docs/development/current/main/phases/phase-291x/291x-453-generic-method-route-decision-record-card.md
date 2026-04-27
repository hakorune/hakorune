---
Status: Landed
Date: 2026-04-27
Scope: Split GenericMethodRoute decided metadata into a named decision record
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-452-generic-method-route-decision-inventory-card.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/map_lookup_fusion_plan.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-453: GenericMethodRoute Decision Record

## Goal

Make `GenericMethodRoute` structurally distinguish decided backend-facing
metadata from surface compatibility, evidence, and operand values.

This is BoxShape-only. It must not change route matching, JSON output,
helper selection, `.inc` behavior, or lowering tiers.

## Change

- Add `GenericMethodRouteDecision`.
- Replace flat `route_kind`, `proof`, `core_method`, `return_shape`,
  `value_demand`, and `publication_policy` fields on `GenericMethodRoute` with
  `decision`.
- Keep thin accessors so JSON output and route consumers can read the same
  values without treating the struct layout as policy.
- Keep existing JSON field names unchanged.

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

PASS. `GenericMethodRoute` now keeps decided backend metadata in
`GenericMethodRouteDecision`. Route matching, JSON field names, helper symbols,
and lowering tiers remain unchanged.

`tools/checks/dev_gate.sh quick` passed. The existing chip8 release-artifact
sync warning remains informational in this profile.
