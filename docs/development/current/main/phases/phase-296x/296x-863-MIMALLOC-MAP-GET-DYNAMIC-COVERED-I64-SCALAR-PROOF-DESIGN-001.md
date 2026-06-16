# 296x-863 MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-DESIGN-001

Status: Landed
Date: 2026-06-16

## Purpose

Fix the proof boundary for the selected repeated map-get front before code
changes. The selected front is `kilo_leaf_map_get_dynamic_covered_i64`, where
`map.get(i % 3)` remains a real loop body lookup today.

This row is design-only. It does not implement the proof and does not change
MapBox storage.

## Decision

```text
output_contract=hako-mimalloc-map-get-dynamic-covered-i64-scalar-proof-design-v0
source_evidence=296x-862
row_kind=design

target_front=kilo_leaf_map_get_dynamic_covered_i64
target_current_loop_helper=nyash.runtime_data.get_hh
target_desired_loop_helper=nyash.map.scalar_load_hi

selected_owner=generic_method_route_plan_map_key_coverage_proof
selected_owner_scope=route_proof_only
map_storage_representation_changed=0
runtime_helper_semantics_changed=0
product_mapbox_semantics_changed=0

proof_shape=covered_dynamic_i64_key_scalar_values
accepted_key_shape=nonnegative_loop_index_mod_const_positive
accepted_key_modulus=3
accepted_preseed_shape=const_i64_key_to_const_i64_value
requires_all_residues_preseeded=1
requires_no_same_receiver_mutation_between_preseed_and_get=1
requires_no_publication_or_unknown_escape_before_get=1

route_kind_on_success=map_load_scalar_i64
helper_on_success=nyash.map.scalar_load_hi
value_demand_on_success=scalar_i64
publication_policy_on_success=no_publication

fallback_on_missing_proof=existing_runtime_data_get_or_map_load_any
fallback_silent=0
benchmark_name_branch_count=0
helper_symbol_inference_count=0
implementation_started=0
selected_next=MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-GUARD-SURFACE-001
summary=ok
```

## Layer Boundary

`generic_method_route_plan` owns the proof because it already decides whether a
`MapGet` site can use a scalar helper. The proof must not move into:

- `MIRBuilder`, because MIRBuilder should emit source meaning, not route policy.
- Map runtime storage, because this row does not change the String-key / generic
  storage representation.
- LLVM shims, because backend must consume route metadata rather than infer from
  helper names or benchmark shapes.

## Required Proof Inputs

The next implementation row may accept only a narrow version:

```text
map.set(0, const_i64)
map.set(1, const_i64)
map.set(2, const_i64)
...
local k = i % 3
local v = map.get(k)
```

The `i % 3` key is accepted only when the index is proven to be the standard
nonnegative loop index produced by the current loop recipe. If that proof is
unavailable, the route must stay on the existing fallback.

## Stop Lines

- do not infer coverage from benchmark name
- do not infer scalar legality from helper symbol names
- do not accept arbitrary `% n` keys without nonnegative index proof
- do not accept partial residue coverage
- do not accept non-const stored scalar values in this first row
- do not change MapBox storage representation
- do not make LLVM backend reinterpret generic map layout
- do not claim C parity or winner status from this proof row
