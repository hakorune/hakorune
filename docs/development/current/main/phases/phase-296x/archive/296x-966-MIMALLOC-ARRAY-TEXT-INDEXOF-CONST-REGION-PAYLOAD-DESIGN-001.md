# 296x-966 MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-REGION-PAYLOAD-DESIGN-001

Status: Landed
Date: 2026-06-16

## Purpose

Design the MIR-owned region payload needed to optimize
`kilo_leaf_array_string_indexof_const`.

This row is docs-only. It does not add producer code, backend lowering, runtime
helpers, or benchmark changes.

## Decision

Add a new indexOf-specific region plan instead of overloading the length-only
`ArrayTextLoopSessionPlan`.

Name:

```text
ArrayTextIndexOfConstRegionPlan
```

Reason:

```text
length region:
  accumulator += len(array[row])

indexOf const region:
  pos = indexOf(array[row], const_needle)
  accumulator += (pos >= 0)
```

The two regions share loop/row metadata but have different observer semantics.
Keeping a separate plan avoids turning the length plan into a union with
ambiguous fields.

## Producer Owner

```text
src/mir/array_text_loop_session_plan.rs
```

The producer should reuse existing metadata facts:

```text
counting_loop_facts
array_text_observer_routes
generic_method_routes
```

It must not infer from:

```text
helper symbol spelling
benchmark name
source variable names
raw backend instruction windows
```

## Required Region Shape

The first accepted shape is intentionally narrow:

```text
loop:
  i starts at 0
  i increments by 1
  loop bound is a known const

row:
  row = i % rows
  rows is a known const

observer:
  cur = array.get(row)
  pos = cur.indexOf("const")
  if pos >= 0:
    hits = hits + 1

publication:
  array text observer route has publication_boundary=none
```

Unsupported in v0:

```text
dynamic needle handle
non-const needle
non-found-predicate indexOf consumer
mutating body
array publication inside the loop
multi-body loop
non-zero loop start
non-unit loop step
```

Unsupported shapes must fail to produce the plan; they must not fall back to a
partial region plan.

## Payload Fields

The initial MIR JSON payload should be explicit and backend-owned:

```text
array_text_indexof_const_region_plans[]:
  loop_header
  loop_body
  loop_exit

  array_root_value

  loop_index_phi_value
  loop_index_initial_value
  loop_index_initial_const
  loop_index_next_value

  loop_bound_value
  loop_bound_const

  row_index_value
  row_modulus_value
  row_modulus_const

  accumulator_phi_value
  accumulator_initial_value
  accumulator_initial_const
  accumulator_next_value
  exit_accumulator_value

  get_result_value
  indexof_result_value
  predicate_value

  needle_const_text
  needle_byte_len

  consumer_shape
  selected_helper_symbol
```

For the target front, expected values are:

```text
loop_bound_const=400000
row_modulus_const=64
needle_const_text=line
needle_byte_len=4
consumer_shape=found_predicate
selected_helper_symbol=hako.array_text.session_indexof_const_utf8
```

`selected_helper_symbol` is report/debug metadata only. It must not be used as
the producer proof.

## Backend Contract

The backend must only consume the region plan after a later reader/lowering row.

Allowed later lowering shape:

```text
call hako.array_text.indexof_const_found_count_region(...)
add result to post-loop array length
```

Not allowed in this design row:

```text
backend reads array_text_observer_routes directly
backend scans raw MIR windows
backend matches helper symbol text to infer legality
backend changes ArrayBox/StringBox storage
```

## Result

```text
output_contract=hako-mimalloc-array-text-indexof-const-region-payload-design-v0
target_front=kilo_leaf_array_string_indexof_const

new_plan_name=ArrayTextIndexOfConstRegionPlan
producer_owner=src/mir/array_text_loop_session_plan.rs
json_field=array_text_indexof_const_region_plans

reuse_array_text_observer_routes=1
reuse_counting_loop_facts=1
reuse_generic_method_routes=1
helper_symbol_inference_allowed=0
benchmark_name_branch_allowed=0
raw_backend_window_scan_allowed=0

needle_const_required=1
consumer_shape_required=found_predicate
publication_boundary_required=none
multi_body_loop_supported=0
mutating_body_supported=0

backend_reader_enabled=0
backend_lowering_enabled=0
runtime_helper_added=0
product_default_changed=0

selected_next=MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-REGION-PAYLOAD-SURFACE-001
summary=ok
```

## Stop Line

```text
do not implement backend lowering before JSON export and C reader rows
do not overload ArrayTextLoopSessionPlan with indexOf observer semantics
do not create a plan from helper symbol spelling
do not accept dynamic needle in v0
do not change product ArrayBox/StringBox storage
```

## Proof Bundle

```bash
cargo run --quiet --bin hakorune -- --emit-mir-json \
  /tmp/kilo_leaf_array_string_indexof_const.current.mir.json \
  benchmarks/bench_kilo_leaf_array_string_indexof_const.hako
jq '.functions[] | select(.name=="main") | .metadata.array_text_observer_routes' \
  /tmp/kilo_leaf_array_string_indexof_const.current.mir.json
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
