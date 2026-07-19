---
Status: LOOP0-I0a closed; LOOP0-I0b is next
Date: 2026-07-19
Parent: callable-result-i64-site0-r0-expression-spine-loop0-task-2026-07-18.md
Scope: shared Loop route selection/execution split
---

# Callable-result LOOP0-I0a route-selection closeout

## Decision

`LOOP0-I0a` is closed as a behavior-neutral BoxShape refactor.

```text
one ordered ENTRIES table
  -> RecipeFirstRouteSelectionV1
       -> raw execution order
       -> legacy diagnostic-effective order
  -> raw selected-route executor
```

`registry/selection.rs` is the sole owner of predicate evaluation and candidate
suppression. It has no Builder, composer, PlanLowerer, ledger, claim batch, or
source-location authority.

## Preserved split

Raw execution and legacy diagnostics intentionally use different projections.

```text
raw execution:
  suppression + predicate matches in ENTRIES order
  each selected route runs in order
  Ok(None) continues; Err propagates

diagnostics:
  raw execution candidates
  minus the historical GenericLoopV1 diagnostic filter for char-map,
  simple-while, and nested-minimal observations
```

The diagnostic filter is not allowed to change the raw execution list.

## Located preparation

`VerifiedLocatedGenericLoopV1SelectionV1` is a non-Clone token issued only
when the raw execution list is exactly `[GenericLoopV1]`. Missing, non-Generic,
or overlapping selections reject before a future located composer/Builder
consumer exists. The token is preparatory only: production located roots remain
zero in I0a.

## Evidence

```text
cargo test -q route_entry::registry --lib
cargo test -q generic_loop --lib
cargo check --all-targets
bash tools/checks/coreplan_planner_required_route_exhaustion_guard.sh
python3 tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0.py .
```

Focused tests prove actual located-fixture GenericLoopV1 selection without a
Builder, diagnostic/raw projection separation, raw `Ok(None)` continuation,
and raw `Err` propagation. The existing public expression-spine guard imports
the private I0a structural guard.

## Next: `LOOP0-I0b`

Thread one stack-scoped effect-emission port through core/body/block/Loop
lowering. It must preserve all raw callers and behavior, keep plan/view/site/
ledger/claim authority out of `MirBuilder`, and provide the one future selected
claim-emission handoff. It must not connect a production located root, retry a
selected located failure, or use raw function/method spelling as selected-call
authority.
