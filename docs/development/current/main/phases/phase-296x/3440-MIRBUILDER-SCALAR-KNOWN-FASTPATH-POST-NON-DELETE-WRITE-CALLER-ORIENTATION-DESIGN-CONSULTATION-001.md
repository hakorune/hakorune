# 3440 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-NON-DELETE-WRITE-CALLER-ORIENTATION-DESIGN-CONSULTATION-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-NON-DELETE-WRITE-CALLER-ORIENTATION-DESIGN-CONSULTATION-001
```

## Status

Design consultation stop. Do not implement code, runtime path, backend path,
authority switch, Delete revival, or ScalarKnown-wide claim in this card.

## Evidence

The read eight-row and non-Delete Write three-row caller-orientation packets
are assertion-only, policy-row-ID-only, fail-fast consumers. Existing Hako
route-decision policy and Rust oracle compatibility remain the authority.

## Pro Question

Which next proof axis is authorized?

```text
A: authority-bearing caller orientation for one already-authoritative surface
B: formally scoped ScalarKnown-wide caller-orientation basis, still runtime=0
C: Delete revival as a separately guarded authority pilot
D: park caller orientation and return to wider Source Selfhost route selection
E: park the entire caller-orientation lane
```

Please specify the first permitted claim, its required non-claims, the
exhaustive surface set, and the fail-fast boundary. Until that answer exists,
`caller_orientation_runtime_path`, `hako_runtime_route_authority`,
`backend_lowering_authority`, `runtime_mutation_authority`,
`publication_execution`, `delete_hako_route_decision_authority_pilot`,
`scalar_known_wide_authority`, and `source_selfhost_claim` remain `0`.

## Decision

Adopt option A. The first authority-bearing caller-orientation pilot is
MapLoadScalarI64Routes only.

```text
selected_next_card = MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-AUTHORITY-PILOT-001
authority_scope = policy_row_id_contract_only
consumer_input = PolicyRowIdOnly
consumer_return = Unit
single_surface_mapload_scope = 1
no_new_route_authority = 1
```

The caller-orientation authority validates that the supplied row identity
resolves to the generated typed MapLoad caller contract and matches the
existing MapLoad policy/oracle metadata. It does not choose a route, emit MIR,
allocate a ValueId, dispatch runtime code, lower backend code, mutate state, or
execute publication.

The exhaustive pilot set is exactly one row:

```text
MapLoadScalarI64Routes / MapLoadScalarI64 / map_load_scalar_i64_routes
```

Read eight-row and non-Delete Write three-row assertion closeouts remain
background evidence only. String, Collection, Write, Delete, ScalarKnown-wide,
and Source Selfhost are excluded from the authority scope.
