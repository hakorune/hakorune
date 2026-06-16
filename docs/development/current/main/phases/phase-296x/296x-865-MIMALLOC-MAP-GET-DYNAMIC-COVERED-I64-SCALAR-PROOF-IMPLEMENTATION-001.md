# 296x-865 MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-IMPLEMENTATION-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-get-dynamic-covered-i64-scalar-proof-implementation-v0
source_evidence=296x-862,296x-863,296x-864
row_kind=implementation
target_front=kilo_leaf_map_get_dynamic_covered_i64
target_source=benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako

implementation_owner=src/mir/generic_method_route_plan/map_set_scalar_proof.rs
route_policy_owner=src/mir/generic_method_route_plan/collection_read_routes.rs
backend_boundary_owner=lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
backend_need_owner=lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_metadata_rules.inc

new_route_proof=map_set_scalar_i64_covered_dynamic_i64_key_no_escape
accepted_key_shape=nonnegative_loop_index_mod_const_positive
accepted_modulus_current=3
accepted_preseed_shape=const_i64_key_to_const_i64_value
requires_all_residues_preseeded=1
requires_no_same_receiver_unknown_mutation_or_escape_before_get=1

post_loop_map_get_route_kind=map_load_scalar_i64
post_loop_map_get_helper=nyash.map.scalar_load_hi
post_loop_runtime_data_get_hh_count=0
post_scalar_route_count_min=1
post_object_imports_scalar_load_hi=1
post_object_imports_runtime_data_get_hh=0
post_final_const_key_route_may_remain_map_load_any=1
post_slot_load_hh_allowed_for_final_const_get=1

map_storage_representation_changed=0
mirbuilder_object_management_enabled=0
helper_symbol_inference_count=0
benchmark_name_branch_count=0
c_pair_measurement_used=0
winner_claim=0
selected_next=MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-MEASUREMENT-001
summary=ok
```

## Notes

- The selected benchmark remains Hako-only. This row proves route-to-AOT shape,
  not C parity.
- The proof is intentionally narrow: the current accepted dynamic key is
  `i % 3`, with all three residues preseeded to scalar i64 values.
- The runtime MapBox storage is unchanged. The loop read avoids the
  `RuntimeDataBox.get` host-handle facade only because the MIR route proof says
  the covered dynamic key can use the existing scalar Map helper.

## Stop Lines

- do not generalize arbitrary dynamic keys without a new proof row
- do not special-case `kilo_leaf_map_get_dynamic_covered_i64`
- do not infer scalar legality from helper names
- do not change MapBox storage representation
- do not move object management into MIRBuilder
- do not claim Hako-vs-C parity from this row

