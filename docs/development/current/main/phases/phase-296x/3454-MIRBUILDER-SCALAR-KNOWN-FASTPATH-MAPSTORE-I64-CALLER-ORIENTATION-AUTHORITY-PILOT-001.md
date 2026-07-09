# 3454 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPSTORE-I64-CALLER-ORIENTATION-AUTHORITY-PILOT-001

## Status

Selected, implementation pending. Do not implement this pilot until 3456 is
green. The original selection incorrectly treated `I64` as the stored-value
domain; Rust route matching uses it as the map-key domain.

## Exact Scope

```text
surface = SetSurfacePolicy
route_kind = MapStoreI64
policy_row_id = map_store_i64_set_surface
consumer_input = PolicyRowIdOnly
consumer_return = Unit
authority_scope = policy_row_id_contract_only
key_domain = I64
stored_value_domain = Any
mutation_boundary = declared metadata only
```

The caller validates that this is a mutation-bearing row. It does not gain
mutation, route, runtime, backend, publication, MIR, or ValueId authority.

## Authority Map

```text
route matching = Rust write_routes.rs
policy row edit source = hand-authored Hako
decision payload = Rust artifact generated from Hako
compatibility veto = Rust validator / oracle
mutation and backend = downstream Rust
caller orientation = policy-row contract acceptance or rejection only
```

Do not abbreviate this as broad "Hako route-decision authority". The owners
above remain distinct.

## Prerequisite

3456 must first replace the ambiguous value boundary with typed key and stored
value domains, centralize the duplicated policy tuple, and add independent-axis
tests. That BoxShape repair must not change route behavior or authority.

## Acceptance Targets

1. Replace the MapStoreI64 assertion-only consumer with a policy-row-ID-only
   authority validator returning `Unit`.
2. Compare its generated caller contract with typed MapStoreI64 policy
   metadata, including `key_domain=I64`, `stored_value_domain=Any`, mutation
   class, and `NonePublication` boundary.
3. Retain Rust route matching, Hako-owned decision-payload editing, and the
   independent Rust oracle fail-fast veto.
4. Test valid row, unknown/wrong row, contract/policy metadata drift, mutation
   metadata leakage, non-Unit boundary, and no runtime/backend/MIR/value-ID/
   mutation/publication path.
5. Record an implementation fixture. Rerun before entering 3455.

These become completion claims only after an implementation fixture is green:

```text
mapstore_i64_caller_orientation_authority_pilot = 1
mapstore_i64_caller_orientation_authority_scope = policy_row_id_contract_only
mapstore_i64_caller_orientation_consumer_input = PolicyRowIdOnly
mapstore_i64_caller_orientation_consumer_return = Unit
set_surface_policy_mapstore_i64_single_row_scope = 1
rust_route_match_authority_retained = 1
hako_decision_payload_edit_authority_retained = 1
rust_compatibility_veto_retained = 1
mapstore_i64_mismatch_fail_fast = 1
mutation_boundary_declared_but_not_authorized = 1
stored_value_any_boundary_declared = 1
any_key_boundary_not_opened = 1
no_new_route_authority = 1
```

Until then:

```text
mapstore_i64_caller_orientation_authority_pilot = 0
implementation_deferred = 1
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
