# 3420 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-NARROW-CALLER-ORIENTATION-BASIS-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-NARROW-CALLER-ORIENTATION-BASIS-001
```

## Purpose

Define a MapLoad-only caller-orientation contract while retaining the existing
scoped `.hako` route-decision authority and Rust oracle/compat-checker boundary.
This card is basis-only and metadata-only.

## Scope

```text
surface: MapLoadScalarI64Routes
route_kind: MapLoadScalarI64
caller_orientation: expected contract metadata only
route_selector: unchanged
runtime_path: forbidden
backend_lowering: forbidden
mutation/publication: forbidden
```

The caller does not select a route, invoke a `.hako` runtime path, allocate a
ValueId, emit MIR, or change backend dispatch. Existing `.hako` policy result
versus Rust oracle comparison remains the route-correctness proof.

## Authority Boundary

```text
generated typed .hako policy
  -> existing MapLoad route-decision authority
  -> Rust oracle / compat checker
  -> mismatch fail-fast

caller-orientation basis
  -> expected MapLoad contract metadata only
  -> no runtime consumer
```

Fail-fast is required when the MapLoad contract metadata is not the expected
single-surface contract, or when a caller-orientation value is routed toward
runtime dispatch, backend lowering, mutation, or publication.

## Claims

```text
mapload_caller_orientation_basis = 1
mapload_hako_route_decision_authority_retained = 1
mapload_rust_oracle_compat_checker_retained = 1
mapload_mismatch_fail_fast = 1
basis_only = 1
mapload_single_surface_scope = 1
caller_orientation_implementation_deferred = 1
caller_orientation_contract_metadata_only = 1
no_new_route_authority = 1
prior_scoped_mapload_hako_route_decision_authority = 1
single_surface_mapload_caller_orientation_scope = 1
rust_oracle_compat_fail_fast_retained = 1
no_runtime_path_no_backend_lowering_no_mutation_no_publication = 1
```

## Non-Claims

```text
caller_orientation_runtime_path = 0
hako_runtime_route_authority = 0
scalar_known_hako_runtime_route_authority = 0
rust_fastpath_rewired = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
source_selfhost_claim = 0
delete_hako_route_decision_authority_pilot = 0
caller_selected_route_authority = 0
caller_runtime_dispatch_authority = 0
caller_orientation_result_consumed_by_runtime = 0
caller_orientation_result_consumed_by_backend = 0
route_selection_authority_switch = 0
mapload_to_scalar_known_wide_authority = 0
read_surface_to_runtime_authority = 0
write_surface_authority_closeout = 0
write_wide_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
route_count_as_proof = 0
row_count_as_proof = 0
coverage_percentage_as_proof = 0
source_path_as_authority = 0
owner_name_as_proof = 0
route_membership_alone_as_proof = 0
manual_surface_selection = 0
```

## ScalarKnown-Wide Gate

This basis does not promote caller orientation to ScalarKnown-wide authority.
Wide promotion requires a closed enumerated surface set, the same caller
orientation schema for every surface, per-surface generated artifact/live
consumer/oracle/fail-fast evidence, and an exhaustiveness guard. Parked Delete
remains excluded until a separate design decision changes that status.

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_scalar_known_fastpath_mapload_narrow_caller_orientation_basis_guard.sh
```
