---
Status: Landed
Date: 2026-04-27
Scope: Prune GenericMethodRoute component records from root crate::mir re-export
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-472-generic-method-route-root-export-inventory-card.md
  - src/mir/mod.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
---

# 291x-473: GenericMethodRoute Root Export Prune

## Goal

Thin the root `crate::mir` surface for generic method routing.

`GenericMethodRoute` remains a legitimate root export because `MirFunction`
metadata stores route records. Component construction records and route-local
metadata enums should be imported from their owner module:

```text
crate::mir::generic_method_route_plan
```

This is BoxShape-only. It must not change route detection, route metadata JSON,
helper symbols, lowering tiers, or `.inc` behavior.

## Change

- Keep root exports for `GenericMethodRoute` and route refresh functions.
- Remove component construction records from the root `crate::mir` re-export:
  - `GenericMethodRouteDecision`
  - `GenericMethodRouteEvidence`
  - `GenericMethodRouteKind`
  - `GenericMethodRouteOperands`
  - `GenericMethodRouteProof`
  - `GenericMethodRouteSite`
  - `GenericMethodRouteSurface`
- Update the JSON fixture to import those construction records from
  `crate::mir::generic_method_route_plan`.

## Acceptance

- `cargo check -q` passes.
- Generic method route JSON fixture still passes.
- Generic method route and map lookup fusion tests still pass.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `tools/checks/dev_gate.sh quick` passes.

## Result

Landed.

- Root `crate::mir` now re-exports only `GenericMethodRoute` plus the route
  refresh functions from `generic_method_route_plan`.
- Generic method route component construction records are imported from
  `crate::mir::generic_method_route_plan` in the JSON fixture.
- Route semantics, JSON field names, helper symbols, lowering tiers, and `.inc`
  behavior are unchanged.

Verification:

```bash
cargo check -q
cargo test -q build_mir_json_root_emits_generic_method_routes
cargo test -q generic_method_route
cargo test -q map_lookup_fusion
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

Note: quick gate still reports the existing chip8 release artifact sync warning,
then completes with `[dev-gate] profile=quick ok`.
