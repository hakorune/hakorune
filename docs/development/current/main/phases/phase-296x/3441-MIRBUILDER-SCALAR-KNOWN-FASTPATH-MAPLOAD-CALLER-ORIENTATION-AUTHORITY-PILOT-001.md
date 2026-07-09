# 3441 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-AUTHORITY-PILOT-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-AUTHORITY-PILOT-001
```

## Purpose

Promote only the generated typed MapLoad caller-orientation contract from an
assertion record to the authoritative contract validator for the exact
`map_load_scalar_i64_routes` row.

This is contract authority, not route or runtime authority.

## Exact Scope

```text
surface = MapLoadScalarI64Routes
route = MapLoadScalarI64
policy_row_id = map_load_scalar_i64_routes
consumer_input = PolicyRowIdOnly
consumer_return = Unit
authority_scope = policy_row_id_contract_only
```

Excluded: String, Collection, all Write surfaces, DeleteSurfacePolicy,
ScalarKnown-wide, backend/runtime paths, and Source Selfhost route selection.

## Required Delta

1. Make the MapLoad caller-orientation validator resolve the supplied
   `policy_row_id` against the generated typed caller contract.
2. Compare the resolved caller contract to the existing generated typed
   MapLoad route-decision policy metadata.
3. Retain the existing Rust oracle/compat checker as a fail-fast veto; it must
   never supply a fallback route.
4. Keep the consumer input as `policy_row_id` only and its result as `Unit`.
5. Add exact single-row, wrong-row, missing-row, extra-row, metadata-drift,
   policy-drift, and no-authority-leak guards/tests.
6. Record a checked-in pilot fixture and deterministic guard.

## Required Comparisons

The authority validator must prove consistency for the fields represented by
the current generated artifacts, including row identity, surface, route kind,
receiver domain, return shape, value demand, publication policy, effect class,
lowering tier, authority source, and Unit/no-runtime consumer boundary.

Do not duplicate route semantics in a new hand-maintained source. If a field
is not present in the generated typed contracts, stop and narrow the design
instead of inferring it from source paths, names, counts, or membership.

## Claims

```text
mapload_caller_orientation_authority_pilot = 1
mapload_caller_orientation_authority_scope = policy_row_id_contract_only
mapload_caller_orientation_consumer_unit_only = 1
mapload_hako_route_decision_authority_retained = 1
mapload_rust_oracle_compat_checker_retained = 1
mapload_mismatch_fail_fast = 1
read_caller_orientation_assertion_closeout_retained = 1
non_delete_write_caller_orientation_assertion_closeout_retained = 1
single_surface_mapload_scope = 1
no_new_route_authority = 1
```

## Non-Claims

```text
caller_orientation_runtime_path = 0
hako_runtime_route_authority = 0
scalar_known_hako_runtime_route_authority = 0
rust_fastpath_rewired = 0
route_selection_authority_switch = 0
caller_selected_route_authority = 0
caller_runtime_dispatch_authority = 0
caller_orientation_result_consumed_by_runtime = 0
caller_orientation_result_consumed_by_backend = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
delete_hako_route_decision_authority_pilot = 0
mapdeleteany_authority = 0
write_wide_authority = 0
write_surface_authority_closeout = 0
scalar_known_wide_authority = 0
new_backend_route = 0
new_abi = 0
runtime_fallback = 0
source_selfhost_claim = 0
route_count_as_proof = 0
row_count_as_proof = 0
coverage_percentage_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
manual_surface_selection = 0
```

## Fail-Fast Boundary

Fail on unknown, missing, extra, non-MapLoad, or metadata-drifted rows; caller
contract versus MapLoad policy mismatch; MapLoad policy versus Rust oracle
mismatch; non-Unit output; MIR/ValueId/registry mutation; runtime/backend
consumption; warn-only mismatch; or any fallback attempt.

## Stop Boundary

After this pilot, do not claim read-wide, non-Delete-wide, ScalarKnown-wide,
Delete, runtime/backend authority, or Source Selfhost. A green pilot must first
receive a dedicated rerun/next-proof-axis decision card.
