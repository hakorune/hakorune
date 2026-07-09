# 3454 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-I64-CALLER-ORIENTATION-AUTHORITY-PILOT-001

## Decision

Pro selects option A: the first non-Delete Write caller-orientation authority
pilot is the one generated MapStoreI64 row. This card combines the approved
basis and the ready implementation contract; no separate inventory card is
created.

## Exact Scope

```text
surface = SetSurfacePolicy
route_kind = MapStoreI64
policy_row_id = map_store_i64_set_surface
consumer_input = PolicyRowIdOnly
consumer_return = Unit
authority_scope = policy_row_id_contract_only
value_boundary = ScalarI64
mutation_boundary = declared metadata only
```

The caller validates that this is a mutation-bearing row. It does not gain
mutation, route, runtime, backend, publication, MIR, or ValueId authority.

## Required Delta

1. Replace the MapStoreI64 assertion-only consumer with a policy-row-ID-only
   authority validator returning `Unit`.
2. Compare its generated caller contract with the generated typed MapStoreI64
   policy metadata, including ScalarI64 value boundary, mutation class, and
   `NonePublication` boundary.
3. Retain the existing `.hako` route decision and Rust oracle fail-fast veto.
4. Test valid row, unknown/wrong row, contract/policy metadata drift, mutation
   metadata leakage, non-Unit boundary, and no runtime/backend/MIR/value-ID/
   mutation/publication path.
5. Record an implementation fixture. Rerun before entering 3455.

## Completion Claims

```text
mapstore_i64_caller_orientation_authority_pilot = 1
mapstore_i64_caller_orientation_authority_scope = policy_row_id_contract_only
mapstore_i64_caller_orientation_consumer_input = PolicyRowIdOnly
mapstore_i64_caller_orientation_consumer_return = Unit
set_surface_policy_mapstore_i64_single_row_scope = 1
mapstore_i64_hako_route_decision_authority_retained = 1
mapstore_i64_rust_oracle_compat_checker_retained = 1
mapstore_i64_mismatch_fail_fast = 1
mutation_boundary_declared_but_not_authorized = 1
any_value_boundary_not_opened = 1
no_new_route_authority = 1
```

## Non-Claims

```text
caller_selected_route_authority = 0
caller_runtime_dispatch_authority = 0
runtime_mutation_authority = 0
caller_orientation_runtime_path = 0
backend_lowering_authority = 0
publication_execution = 0
array_append_any_caller_authority = 0
mapstore_any_caller_authority = 0
delete_hako_route_decision_authority_pilot = 0
scalar_known_wide_authority = 0
runtime_fallback = 0
source_selfhost_claim = 0
```

## Next

After a green fixture-backed rerun, enter:

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-MAPSTORE-I64-CALLER-ORIENTATION-PILOT-DESIGN-STOP-001
```
