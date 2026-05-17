# Route Fixpoint Owner SSOT

Status: Active
Date: 2026-05-18

## Purpose

Route metadata refresh is a compiler-owned convergence system, not an incidental
ordering detail inside semantic refresh.

The owner is:

```text
src/mir/route_fixpoint.rs
```

It is invoked by:

```text
src/mir/semantic_refresh.rs
```

## Owned Inputs

RouteFixpoint may coordinate these module-level route families:

```text
generic_method_routes
global_call_routes
user_box_method_routes
route-published value_types
typed object field value_types needed by route planning
map lookup fusion refreshes that depend on global-call target shapes
```

## Owned Output

The output is still the existing MIR metadata:

```text
functions[].metadata.generic_method_routes
functions[].metadata.global_call_routes
functions[].metadata.user_box_method_routes
functions[].metadata.lowering_plan
functions[].metadata.value_types
```

RouteFixpoint does not create a second MIR dialect.

## Contract

- The semantic refresh orchestrator calls RouteFixpoint once.
- RouteFixpoint owns the bounded module-level route convergence sequence.
- Family-specific planners keep their local materialization rules.
- Backend consumers continue to read `lowering_plan`.
- Pure-first preflight continues to read existing route metadata.

## Stop Lines

- Do not add new route acceptance shapes here.
- Do not add new proof vocabulary here.
- Do not add Python preflight reason vocabulary here.
- Do not add backend `.inc` app/name matchers here.
- Do not change source syntax or allocator behavior here.
- Do not hide unsupported routes behind fallback.

## Future Work

Future rows may add:

```text
RouteFixpointReport
RouteDiagnostic vocabulary
RouteLedger view
nyllvmc progress events
```

Those are separate rows. `ROUTE-FIXPOINT-001` only makes the owner explicit.
