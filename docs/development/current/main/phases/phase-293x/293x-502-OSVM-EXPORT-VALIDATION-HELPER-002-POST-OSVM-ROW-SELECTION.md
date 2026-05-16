# 293x-502 OSVM-EXPORT-VALIDATION-HELPER-002 Post-OSVM Row Selection

Status: landed
Date: 2026-05-17

## Decision

`OSVM-EXPORT-VALIDATION-HELPER-001` closed the OSVM export validation helper
cleanup.

Select exactly one next cleanup row:

```text
GENERIC-METHOD-ROUTE-SPLIT-001:
  move generic collection read route matchers out of the root
  generic_method_route_plan.rs facade
```

## Why This Row

The remaining large-file cleanup candidates include:

```text
numeric_substrate.rs
generic_method_route_plan.rs
global_call_route_plan/generic_string_body_analysis.rs
```

`generic_method_route_plan.rs` already has submodule seams for write routes,
model, origin inference, and JSON route helpers. The safest next BoxShape slice
is to move collection read route matchers (`has`, `get`, `length`/`len`,
`keys`) behind a dedicated owner while keeping the root file as the route
refresh facade.

## Selected Row

```text
row:
  GENERIC-METHOD-ROUTE-SPLIT-001
owner:
  src/mir/generic_method_route_plan/collection_read_routes.rs
scope:
  move collection read route matcher functions and their local helper policy
  out of src/mir/generic_method_route_plan.rs
stop_line:
  no accepted route changes
  no route kind/proof/value-demand/return-shape spelling changes
  no origin-inference or write-route changes
  no allocator/provider behavior
evidence:
  cargo test -q generic_method_route_plan
  bash tools/checks/current_state_pointer_guard.sh
  tools/checks/dev_gate.sh quick
  git diff --check
```

## Stop Lines

- Do not add or remove accepted generic method routes.
- Do not change MIR JSON field spelling, route ids, proofs, value demands,
  return shapes, or core method tags.
- Do not touch write routes, scalar map proof, origin inference behavior,
  allocator behavior, provider activation, hooks, host allocator replacement,
  or `#[global_allocator]`.

## Closeout

This row closes when `GENERIC-METHOD-ROUTE-SPLIT-001` has a selected current
card with owner, scope, stop lines, and evidence.
