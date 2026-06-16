# 296x-931 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-PLAN-INVENTORY-001

Status: Landed
Date: 2026-06-16

## Purpose

Inventory whether `kilo_leaf_array_string_len` can produce an
`ArrayTextLoopSessionPlan` from existing MIR metadata before opening backend
loop-session lowering.

This row does not add backend consumers, MIR JSON export for
`ArrayTextLoopSessionPlan`, raw FFI, or ArrayBox/StringBox runtime changes.

## Inventory

Command:

```bash
target/release/hakorune --emit-mir-json \
  /tmp/kilo_leaf_array_string_len.inventory.mir.json \
  benchmarks/bench_kilo_leaf_array_string_len.hako
```

Observed hot loop:

```text
block=26
array_value=5
index_value=72
source_value=59
len_value=60
array_string_len_window_routes=1
array_text_residence_sessions=0
array_text_observer_routes=0
array_text_combined_regions=0
```

Existing facts before the follow-up domain fix:

```text
range_index_fact_count=2
hot_loop_index_value=52
hot_loop_row_value=72
row_is_i_mod_64=1
row_range_index_fact_present=0
```

## Decision

The front is a valid loop-session candidate, but existing facts did not prove
the `%row = %i % 64` index domain required by `ArrayTextLoopSessionPlan`.

```text
output_contract=hako-mimalloc-array-text-loop-session-plan-inventory-v0
target_front=kilo_leaf_array_string_len
candidate_array_value=5
candidate_index_value=72
candidate_block=26
candidate_len_call_count=1

same_array_handle_observable=1
read_only_region_observable=1
no_mutation_region_observable=1
no_drop_or_publication_boundary_observable=1
index_domain_guarded_observable=0

array_text_loop_session_plan_producer_enabled=0
mir_json_export_enabled=0
backend_consumer_enabled=0
backend_loop_session_lowering_enabled=0
raw_array_text_session_ffi_enabled=0
product_default_changed=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-MODULO-INDEX-DOMAIN-FACT-001
summary=ok
```

## Stop Line

```text
do not lower ArrayTextLoopSessionPlan without index-domain proof
do not infer index bounds from benchmark name or helper symbol
do not pass raw ArrayTextSession or ArrayBox pointers through FFI
do not change ArrayBox/StringBox product runtime defaults
```
