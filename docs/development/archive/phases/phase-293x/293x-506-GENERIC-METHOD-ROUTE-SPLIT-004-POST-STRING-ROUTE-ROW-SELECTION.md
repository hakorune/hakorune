# 293x-506 GENERIC-METHOD-ROUTE-SPLIT-004 Post-String-Route Row Selection

Status: landed
Date: 2026-05-17

## Decision

`GENERIC-METHOD-ROUTE-SPLIT-003` closed the generic method string route matcher
split.

Select exactly one next cleanup row:

```text
GLOBAL-STRING-BODY-ANALYSIS-SPLIT-001:
  split the generic string global-call body analysis owner into smaller
  analysis phase modules
```

## Why This Row

`generic_method_route_plan.rs` is now a small facade after collection read and
string matcher splits. The next large cleanup candidate from the current
inventory is `src/mir/global_call_route_plan/generic_string_body_analysis.rs`,
which still owns value-class transfer, method-call acceptance, extern/global
call acceptance, and return-contract checks in one 899-line file.

## Selected Row

```text
row:
  GLOBAL-STRING-BODY-ANALYSIS-SPLIT-001
owner:
  src/mir/global_call_route_plan/generic_string_body_analysis/
scope:
  split generic string body analysis by analysis phase while preserving the
  public generic_pure_string_instruction_reject_reason entry point
stop_line:
  no accepted global-call route changes
  no value-class or reject-reason spelling changes
  no backend emission changes
  no allocator/provider behavior
evidence:
  cargo test -q global_call_route_plan
  bash tools/checks/current_state_pointer_guard.sh
  tools/checks/dev_gate.sh quick
  git diff --check
```

## Stop Lines

- Do not add or remove accepted global-call route shapes.
- Do not change generic string value-class semantics, reject reasons, route
  proof strings, return contracts, or backend emission.
- Do not touch generic method routes, allocator behavior, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.

## Closeout

This row closes when `GLOBAL-STRING-BODY-ANALYSIS-SPLIT-001` has a selected
current card with owner, scope, stop lines, and evidence.
