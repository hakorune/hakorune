# 3445 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-CALLER-ORIENTATION-AUTHORITY-PILOT-BASIS-001

## Decision

Select `StringScalarI64Routes` as the next caller-orientation contract
authority pilot after MapLoad. This card is basis-only; implementation remains
zero until 3446.

```text
proof axis:
  PriorScopedMapLoadCallerOrientationAuthorityContinuation
  + HomogeneousScalarI64NoPublicationReadSurface
  + ExistingStringHakoRouteDecisionAuthorityRetained
  + RustOracleCompatFailFastRetained
```

## Inventory Result

```text
selected:
  StringScalarI64Routes
  - string_indexof_scalar_i64_routes
  - string_lastindexof_scalar_i64_routes
  - string_contains_scalar_i64_routes

deferred:
  CollectionScalarI64Routes = mixed receiver-domain + AnyLength/Box boundary
  NonDeleteWrite = mutation + Any-write boundary
  DeleteSurfacePolicy = retired special case requiring separate revival
  ScalarKnownWide = no authority-bearing multi-surface proof yet
```

String is selected by semantic shape, not by row count, owner name, source
path, route membership, or manual preference. The same MapLoad-to-String
ordering is already established by the route-decision authority rollout.

## Authorized Next Task

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-CALLER-ORIENTATION-AUTHORITY-PILOT-001
```

The pilot is limited to the exact three generated String rows,
`PolicyRowIdOnly` input, `Unit` return, generated caller-contract versus
generated String policy comparison, and the existing Rust oracle fail-fast
veto. It must not create route, runtime, backend, mutation, or publication
authority.

## Claims

```text
string_caller_orientation_authority_pilot_basis = 1
prior_scoped_mapload_caller_orientation_authority_continuation = 1
homogeneous_scalar_i64_no_publication_read_surface = 1
string_exact_three_row_scope = 1
string_hako_route_decision_authority_retained = 1
string_rust_oracle_compat_checker_retained = 1
string_mismatch_fail_fast_required = 1
basis_only = 1
no_new_route_authority = 1
```

## Non-Claims

```text
string_caller_orientation_authority_pilot = 0
collection_caller_orientation_authority = 0
non_delete_write_caller_orientation_authority = 0
delete_hako_route_decision_authority_pilot = 0
scalar_known_wide_authority = 0
caller_orientation_runtime_path = 0
hako_runtime_route_authority = 0
route_selection_authority_switch = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
runtime_fallback = 0
source_selfhost_claim = 0
```

## Guard

```text
python3 tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_string_caller_orientation_authority_pilot_basis.py --check
bash tools/checks/rust_lifecycle_source_selfhost_family_guard.sh
```
