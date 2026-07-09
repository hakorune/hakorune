# 3436 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001
```

## Purpose

Connect the existing compiler-side caller-orientation assertion boundary to
the MapStoreI64 policy lookup. The consumer accepts only the transported
policy row ID and returns `Unit`; it does not select or execute a route.

## Required Delta

1. Add a MapStoreI64 assertion helper that consumes the generated
   metadata-only contract and exact `policy_row_id`.
2. Call it after the existing MapStoreI64 policy/oracle lookup.
3. Add fail-fast tests and a guard proving no route, mutation, backend, or
   publication data crosses the caller-orientation boundary.

## Non-Claims

```text
caller_selected_route_authority = 0
caller_runtime_dispatch_authority = 0
caller_orientation_runtime_path = 0
hako_runtime_route_authority = 0
backend_lowering_authority = 0
write_mutation_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
delete_hako_route_decision_authority_pilot = 0
write_wide_authority = 0
scalar_known_wide_authority = 0
source_selfhost_claim = 0
```

## Stop Conditions

Stop and consult Pro if the consumer needs route kind, core operation,
receiver/value/effect/mutation metadata, a non-Unit result, runtime/backend
dispatch, Delete, or fallback behavior. The next packet item is 3437 only
after the MapStoreI64 live assertion guard is green.
