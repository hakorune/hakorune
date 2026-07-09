# 3404 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001
```

## Purpose

Materialize the scoped `.hako` route-decision authority pilot for
`SetSurfacePolicy / MapStoreI64`.

This is the selected implementation card from 3403.

## Expected Scope

```text
surface:
  SetSurfacePolicy/MapStoreI64

authority source:
  WRITE_SET_MAPSTORE_I64_HAKO_POLICY

Rust role:
  oracle / compat checker retained

mismatch:
  fail-fast

still Rust-owned:
  runtime mutation authority
  publication execution
  backend lowering
  ABI
  caller orientation
```

## Expected Implementation Shape

```text
1. Add a MapStoreI64 `.hako` route-decision authority helper.
2. Construct the live decision from WRITE_SET_MAPSTORE_I64_HAKO_POLICY.
3. Construct the Rust oracle decision separately.
4. Compare all fields fail-fast.
5. Return the `.hako` decision on match.
6. Switch the MapStoreI64 branch in write_routes.rs to the authority helper.
7. Do not add runtime fallback.
```

## Non-Claims Until Implemented

```text
write_set_mapstore_i64_hako_route_decision_authority_pilot = 0
write_surface_authority_pilot = 0
mapstore_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
scalar_known_hako_runtime_route_authority = 0
source_selfhost_claim = 0
```
