# 3449 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-AUTHORITY-PILOT-BASIS-001

## Decision

Resolve 3448 with option B and select the exact four-row
`CollectionScalarI64Routes` caller-orientation authority pilot.

This is a mechanical continuation, not a new mixed-domain design. Cards 3396
through 3398 already established four-row Collection route-decision authority,
including explicit receiver-domain checks and `AnyLength -> Box` as row
metadata rather than a wildcard. Cards 3424 and 3429 already restrict the
caller boundary to policy-row-ID-only Unit assertions.

## Exact Scope

```text
collection_map_entry_count_scalar_i64_routes -> MapBox
collection_array_slot_len_scalar_i64_routes  -> ArrayBox
collection_string_len_scalar_i64_routes      -> StringBox
collection_any_length_scalar_i64_routes      -> Box

consumer_input = PolicyRowIdOnly
consumer_return = Unit
receiver_domain_input = Forbidden
authority_scope = policy_row_id_contract_only
```

`AnyLength -> Box` remains one explicit generated policy row. It does not grant
global Box authority, receiver-domain widening, wildcard selection, or runtime
fallback.

## Task Packet

```text
3449 basis and inventory decision
3450 four-row Collection authority pilot implementation
3451 freshness rerun
3452 post-Collection design consultation before Write mutation
```

## Claims

```text
collection_caller_orientation_authority_pilot_basis = 1
collection_exact_four_row_scope = 1
collection_mixed_receiver_domain_boundary_retained = 1
collection_anylength_box_explicit_row_retained = 1
collection_hako_route_decision_authority_retained = 1
collection_rust_oracle_compat_checker_retained = 1
collection_mismatch_fail_fast_required = 1
basis_only = 1
no_new_route_authority = 1
```

## Non-Claims

```text
collection_caller_orientation_authority_pilot = 0
receiver_domain_authority_switch = 0
receiver_domain_widening_authority = 0
any_length_wildcard_selector = 0
runtime_box_domain_fallback = 0
non_delete_write_caller_orientation_authority = 0
delete_hako_route_decision_authority_pilot = 0
scalar_known_wide_authority = 0
caller_orientation_runtime_path = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
runtime_fallback = 0
source_selfhost_claim = 0
```

## Guard

```text
python3 tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_collection_caller_orientation_authority_pilot_basis.py --check
bash tools/checks/rust_lifecycle_source_selfhost_family_guard.sh
```
