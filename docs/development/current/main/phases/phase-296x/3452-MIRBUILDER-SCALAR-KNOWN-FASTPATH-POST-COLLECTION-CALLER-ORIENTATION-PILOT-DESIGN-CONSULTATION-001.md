# 3452 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-COLLECTION-CALLER-ORIENTATION-PILOT-DESIGN-CONSULTATION-001

## Status

Queued design consultation after a green 3451 rerun. Do not implement this
frontier early.

## Consultation Boundary

Read caller-orientation authority would cover MapLoad, String, and Collection.
The next unresolved semantic boundary is mutation-bearing non-Delete Write:

```text
MapStoreI64 = ScalarI64 write boundary
ArrayAppendAny = Any push boundary
MapStoreAny = Any write boundary
DeleteSurfacePolicy = retired and still parked
```

## Consultation Question

Choose the first mutation-bearing proof axis:

```text
A: MapStoreI64-only caller contract authority pilot
B: ArrayAppendAny-only caller contract authority pilot
C: MapStoreAny-only Any-write pilot
D: full non-Delete Write three-row authority island
E: park caller orientation and return to Source Selfhost route selection
```

The answer must define mutation authority versus metadata authority, exhaustive
scope, source authority, fail-fast boundary, fallback prohibition, and
promotion conditions. Runtime mutation, publication execution, backend
lowering, Delete, wide, and Source Selfhost authority remain zero until then.
