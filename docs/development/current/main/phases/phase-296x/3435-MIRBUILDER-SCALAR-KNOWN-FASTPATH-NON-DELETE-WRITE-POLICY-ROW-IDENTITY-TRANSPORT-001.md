# 3435 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-NON-DELETE-WRITE-POLICY-ROW-IDENTITY-TRANSPORT-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-NON-DELETE-WRITE-POLICY-ROW-IDENTITY-TRANSPORT-001
```

## Purpose

Carry the exact policy row identity through the generated typed artifacts for
the three already-materialized non-Delete Write surfaces:

```text
map_store_i64_set_surface
array_append_any_push_surface
map_store_any_set_surface
```

This is transport metadata only. Existing route/oracle semantics remain the
authority and caller-orientation contracts remain live-consumer-free.

## Required Delta

1. Add `policy_row_id` to each generated typed Write policy artifact.
2. Regenerate all three artifacts and refresh their caller-contract fixture
   provenance.
3. Add exact row identity and freshness guards for the closed three-row set.

## Non-Claims

```text
caller_orientation_runtime_path = 0
route_selection_authority_switch = 0
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

Stop and consult Pro if identity transport requires route/effect/mutation/value
semantics, a runtime consumer, Delete inclusion, or an authority switch. The
next packet item is:

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-SET-MAPSTORE-I64-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001
```

Proceed only after the three-row transport guard is green.
