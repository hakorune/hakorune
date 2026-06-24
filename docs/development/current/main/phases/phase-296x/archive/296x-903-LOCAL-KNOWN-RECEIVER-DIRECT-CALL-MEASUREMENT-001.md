# 296x-903 LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-known-receiver-direct-call-measurement-v1
source_evidence=296x-900,296x-901,296x-902
target_front=kilo_leaf_map_get_dynamic_covered_i64
target_source=benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako
measurement_command=KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_map_get_dynamic_covered_i64 ny_main 1

mir_json_local_fastpath_fact_count=1
mir_json_fact_block=19
mir_json_fact_instruction_index=5
mir_json_fact_route_plan=map_repr.generic_hash_runtime
mir_json_fact_fallback_reason=null

ny_main_loop_uses_local_fastpath_helper=1
ny_main_loop_helper=nyash.map.local_i64_get_hi
ny_main_loop_slot_load_hh_count=0
post_loop_slot_load_hh_allowed=1

top_symbol=nyash_rust::boxes::map_box::MapBox::get_scalar_i64_key_domain
top_symbol_percent=70.20
second_symbol=core::hash::BuildHasher::hash_one
second_symbol_percent=22.41
winner_claim=0
reachability_success=1
selected_next=MAP-HASH-OWNER-REFRESH-AFTER-LOCAL-FASTPATH-001
summary=ok
```

## Decision

The `LocalFastPathFact` consumer reaches the measured exact-AOT front.  The
`ny_main` hot loop calls:

```text
nyash.map.local_i64_get_hi
```

The remaining `nyash.map.slot_load_hh` call is outside the hot loop and belongs
to the final public fallback get.  This matches the intended rule:

```text
Fact present:
  local fast path is allowed

Fact absent:
  product-compatible fallback remains
```

This row is a reachability success, not a performance winner.  The hot owner
has moved into the scalar helper internals:

```text
MapBox::get_scalar_i64_key_domain
BuildHasher::hash_one
```

## Stop Lines

- no Hako-vs-C winner claim from this row
- no product MapBox storage change
- no product hasher swap
- no sidecar storage
- no HostHandle bypass
- no direct storage enablement
- no MIRBuilder object/storage ownership
- no helper-name or benchmark-name inference

## Validation

```bash
cargo fmt --check
cargo test --lib mir::map_repr_plan::tests::refresh_function_map_repr_plans_emits_local_fastpath_facts_for_scalar_no_publication_get
cargo test --lib runner::mir_json_emit::tests::map_repr_plans::build_mir_json_root_emits_local_fastpath_facts
PYTHONPATH=.:src/llvm_py python3 -m unittest \
  src.llvm_py.tests.test_collection_method_call.TestCollectionMethodCall.test_mapbox_local_fastpath_fact_get_uses_known_receiver_direct_call_helper \
  src.llvm_py.tests.test_collection_method_call.TestCollectionMethodCall.test_mapbox_local_fastpath_fact_get_ignores_fallback_reason \
  src.llvm_py.tests.test_fastmem_metadata_loader.TestFastMemMetadataLoader.test_local_fastpath_fact_loader_indexes_sites
bash tools/checks/k2_wide_phase296x_local_fastpath_fact_producer_selection_guard.sh
bash tools/checks/k2_wide_phase296x_local_fastpath_fact_metadata_surface_guard.sh
bash tools/checks/k2_wide_phase296x_local_known_receiver_direct_call_pilot_guard.sh
bash tools/checks/k2_wide_phase296x_local_known_receiver_direct_call_measurement_903_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
bash tools/perf/build_perf_release.sh
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_map_get_dynamic_covered_i64 ny_main 1
```
