# 296x-1018 ARRAY-TEXT-APPEND-UPDATE-PRODUCER-SHAPE-DIAGNOSTIC-001

Status: Landed
Date: 2026-06-17
Scope: diagnostic-only producer shape report

## Contract

```text
output_contract=hako-array-text-append-update-producer-shape-diagnostic-v0
source_evidence=296x-1017,target/array-text-append-update-producer-shape-1018
row_kind=diagnostic
backend_lowering_enabled=0
runtime_helper_enabled=0
product_default_changed=0

target_front=kilo_meso_indexof_append_array_set
array_text_observer_route_count=1
array_text_observer_executor_contract_count=0

producer_shape_const_suffix_concat_seen=1
producer_shape_same_slot_set_seen=1
producer_shape_concat_length_use_seen=1
producer_shape_concat_length_use_count=1
producer_shape_non_length_concat_use_count=0
producer_shape_row_index_mod_const_seen=1
producer_shape_row_modulus_const=128
producer_shape_length_result_feeds_accumulator_add=1

producer_shape_failure_reason=store_only_contract_rejects_length_carry
next_task=ARRAY-TEXT-APPEND-UPDATE-REGION-PAYLOAD-SURFACE-001
summary=ok
```

## Purpose

Add a narrow diagnostic report before extending the observer executor contract.
The previous row proved that broad passive vocabulary can compile while still
producing no target contract. This row answers which producer predicate fails
without adding backend lowering or runtime helper semantics.

## Implementation

The route now carries a diagnostic-only `producer_shape_diagnostic` object in
MIR JSON. It is not an executor contract and must not be consumed by backend
lowering.

The diagnostic observes:

```text
const suffix concat from source
same-array same-row set
concat length use
non-length concat uses
row index as loop_index % const
length result feeding accumulator add
```

The row also split `array_text_observer_region_contract` into facade, model,
matcher, and types modules before adding this diagnostic so future contract
growth stays contained.

## Target Observation

Command:

```bash
mkdir -p target/array-text-append-update-producer-shape-1018

HAKO_STAGE1_MODE=emit-mir HAKO_EMIT_MIR_JSON=1 STAGE1_EMIT_MIR_JSON=1 \
  target/release/hakorune --emit-mir-json \
  target/array-text-append-update-producer-shape-1018/indexof_append_array_set.mir.json \
  benchmarks/bench_kilo_meso_indexof_append_array_set.hako

python3 tools/hako_check/state_explain.py \
  --mir-json target/array-text-append-update-producer-shape-1018/indexof_append_array_set.mir.json \
  --topn 3 \
  | tee target/array-text-append-update-producer-shape-1018/state_explain.log
```

Observed:

```text
array_text_observer_route_0_producer_shape_failure_reason=store_only_contract_rejects_length_carry
array_text_observer_route_0_producer_shape_const_suffix_concat_seen=1
array_text_observer_route_0_producer_shape_same_slot_set_seen=1
array_text_observer_route_0_producer_shape_concat_length_use_seen=1
array_text_observer_route_0_producer_shape_concat_length_use_count=1
array_text_observer_route_0_producer_shape_non_length_concat_use_count=0
array_text_observer_route_0_producer_shape_row_index_mod_const_seen=1
array_text_observer_route_0_producer_shape_row_modulus_const=128
array_text_observer_route_0_producer_shape_length_result_feeds_accumulator_add=1
```

## Reading

The producer does see the intended shape:

```text
row = loop_index % 128
updated = current + "ln"
lines.set(row, updated)
total += updated.length()
```

The reason no executor contract is produced is not missing row-modulus,
same-slot set, or accumulator observation. The blocker is exactly the old
store-only contract:

```text
consumer_capabilities=compare_only,sink_store
```

The next row may add passive metadata for:

```text
consumer_capabilities=compare_only,sink_store,length_result_carry
```

## Stop Line

```text
do not let backend consume producer_shape_diagnostic
do not enable runtime helper lowering
do not reuse store-count helper for length result semantics
do not infer row modulus in C backend
do not change product ArrayBox / StringBox storage
```

## Validation

```text
cargo test -q array_text_observer --lib
python3 -m unittest tools.hako_check.tests.test_fastmem_report_key_consistency.FastMemReportKeyConsistencyTest.test_state_explain_emits_array_text_observer_route_rows
cargo check -q --release --bin hakorune
target MIR/state_explain command above
```
