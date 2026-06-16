# 296x-932 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-MODULO-INDEX-DOMAIN-FACT-001

Status: Landed
Date: 2026-06-16

## Purpose

Add a conservative MIR fact producer for modulo-derived loop indices so
`ArrayTextLoopSessionPlan` can prove `row = i % rows` without reading source
spelling, helper names, or benchmark names.

This is a compiler fact row. It does not create `ArrayTextLoopSessionPlan`
metadata, export that plan to MIR JSON, or lower backend code.

## Implementation

```text
src/mir/function/facts.rs:
  RangeIndexFactOriginKind::ModuloOfRangeIndex

src/mir/range_index_fact.rs:
  append_modulo_range_index_facts()
```

Producer rule:

```text
source fact:
  index in [0, upper)
  lower is const 0
  body is read-only
  no loop-carried writes

MIR instruction:
  dst = source_index % const_positive_modulus

derived fact:
  index_value=dst
  lower_value=source.lower_value
  upper_exclusive_value=modulus_value
  origin_kind=modulo_of_range_index
  step=0
```

`step=0` is intentional: modulo-derived indices are bounded but not monotonic,
so existing consumers that require `step=1` do not accidentally consume this
fact.

## Evidence

After rebuilding `target/release/hakorune`, the selected front emits:

```text
range_index_count=4

fact_id=1
origin_kind=counting_loop
body_bb=26
index_value=52
upper_exclusive_value=66
step=1

fact_id=3
origin_kind=modulo_of_range_index
body_bb=26
index_value=72
upper_exclusive_value=75
step=0
```

This gives the hot-loop `Array.get(row)` index a MIR-owned bounded-domain fact.

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-modulo-index-domain-fact-v0
target_front=kilo_leaf_array_string_len
modulo_range_index_fact_enabled=1
modulo_range_index_fact_origin=modulo_of_range_index
hot_loop_row_index_value=72
hot_loop_row_range_index_fact_present=1
hot_loop_row_range_index_step=0

array_text_loop_session_plan_producer_enabled=0
mir_json_export_enabled=0
backend_consumer_enabled=0
backend_loop_session_lowering_enabled=0
product_default_changed=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-PLAN-PRODUCER-001
summary=ok
```

## Proof Bundle

```bash
cargo fmt --check
cargo test --lib range_index_fact -- --nocapture
cargo check --bin hakorune
cargo build --release --bin hakorune
target/release/hakorune --emit-mir-json \
  /tmp/kilo_leaf_array_string_len.inventory2.mir.json \
  benchmarks/bench_kilo_leaf_array_string_len.hako
```

## Stop Line

```text
do not treat modulo-derived facts as monotonic
do not let existing step=1 consumers consume modulo facts accidentally
do not add backend loop-session lowering in this row
do not infer bounds from source variable names
do not change product ArrayBox/StringBox behavior
```
