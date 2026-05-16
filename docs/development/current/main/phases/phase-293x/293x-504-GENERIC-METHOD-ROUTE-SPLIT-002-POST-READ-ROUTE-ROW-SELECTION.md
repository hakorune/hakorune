# 293x-504 GENERIC-METHOD-ROUTE-SPLIT-002 Post-Read-Route Row Selection

Status: landed
Date: 2026-05-17

## Decision

`GENERIC-METHOD-ROUTE-SPLIT-001` closed the collection read route matcher
split.

Select exactly one next cleanup row:

```text
GENERIC-METHOD-ROUTE-SPLIT-003:
  move generic string route matchers out of the root generic_method_route_plan.rs
  facade
```

## Why This Row

After `GENERIC-METHOD-ROUTE-SPLIT-001`, the root generic method route facade is
much smaller, but it still owns string-specific route matchers for
`substring`, `indexOf`, `lastIndexOf`, and `contains`. Those routes share a
clear owner boundary and can move without changing route behavior.

## Selected Row

```text
row:
  GENERIC-METHOD-ROUTE-SPLIT-003
owner:
  src/mir/generic_method_route_plan/string_routes.rs
scope:
  move string route matcher functions and their local string helper policy out
  of src/mir/generic_method_route_plan.rs
stop_line:
  no accepted route changes
  no route kind/proof/value-demand/return-shape spelling changes
  no collection read/write route changes
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
  return shapes, publication policy, or core method tags.
- Do not touch collection read routes, write routes, scalar map proof,
  typed-object origin inference behavior, allocator behavior, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.

## Closeout

This row closes when `GENERIC-METHOD-ROUTE-SPLIT-003` has a selected current
card with owner, scope, stop lines, and evidence.
