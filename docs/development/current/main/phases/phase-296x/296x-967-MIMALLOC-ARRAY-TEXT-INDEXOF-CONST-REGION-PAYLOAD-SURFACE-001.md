# 296x-967 MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-REGION-PAYLOAD-SURFACE-001

Status: Landed
Date: 2026-06-16

## Purpose

Add the MIR metadata and MIR JSON export surface for the
`ArrayTextIndexOfConstRegionPlan` selected by 296x-966.

This row does not add C ABI readers, backend lowering, runtime helpers, or
product ArrayBox/StringBox storage changes.

## Implementation

Code surfaces added:

```text
src/mir/array_text_loop_session_plan/indexof_const_region.rs
  ArrayTextIndexOfConstRegionPlan
  ArrayTextIndexOfConstRegionPayload
  derive_indexof_const_region_payload()

src/mir/array_text_loop_session_plan.rs
  refresh_function_array_text_indexof_const_region_plans()

src/mir/function/metadata.rs
  FunctionMetadata::array_text_indexof_const_region_plans

src/runner/mir_json_emit/array_metadata.rs
  metadata.array_text_indexof_const_region_plans JSON export
```

The producer consumes existing MIR-owned facts:

```text
array_text_observer_routes
range_index_facts
counting-loop/header/body shape
```

Accepted v0 shape:

```text
observer_kind=indexof
observer_arg0_repr=const_utf8
consumer_shape=found_predicate
publication_boundary=none
single_body_loop=1
loop_index_initial_const=0
loop_index_step=1
loop_bound_const known
row_modulus_const known
accumulator_initial_const=0
accumulator_next=select(indexof_result >= 0, accumulator + 1, accumulator)
```

Unsupported shapes produce no plan. There is no fallback/partial plan.

## Refresh Order

`array_text_indexof_const_region_plans` depends on
`array_text_observer_routes`, so semantic refresh now runs the indexOf const
region producer after `refresh_function_array_text_observer_routes()`.

## Target Front Proof

Command:

```bash
cargo run --quiet --bin hakorune -- \
  --emit-mir-json /tmp/kilo_leaf_array_string_indexof_const.region_surface.mir.json \
  benchmarks/bench_kilo_leaf_array_string_indexof_const.hako
```

Observed for `main`:

```text
array_text_observer_route_count=1
array_text_indexof_const_region_plan_count=1
array_text_loop_session_plan_count=0

loop_header=25
loop_body=26
loop_exit=28
array_value=5
index_value=78
get_instruction_index=7
observer_instruction_index=9
needle_const_text=line
needle_byte_len=4
consumer_shape=found_predicate
selected_helper_symbol=hako.array_text.indexof_const_found_count_region

loop_bound_const=400000
row_modulus_const=64
get_result_value=60
indexof_result_value=62
predicate_value=92
accumulator_phi_value=52
accumulator_next_value=91
exit_accumulator_value=54
```

`selected_helper_symbol` remains report/debug metadata. The producer does not
use helper symbol spelling, benchmark name, or source variable names as proof.

## Result

```text
output_contract=hako-mimalloc-array-text-indexof-const-region-payload-surface-v0
target_front=kilo_leaf_array_string_indexof_const

array_text_indexof_const_region_plan_surface_enabled=1
json_field=array_text_indexof_const_region_plans
target_region_plan_count=1
array_text_loop_session_plan_count=0

needle_const_required=1
consumer_shape_required=found_predicate
publication_boundary_required=none

backend_reader_enabled=0
backend_lowering_enabled=0
runtime_helper_added=0
product_default_changed=0
helper_symbol_inference_allowed=0
benchmark_name_branch_allowed=0
raw_backend_window_scan_allowed=0

selected_next=MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-BACKEND-READER-SURFACE-001
summary=ok
```

## Proof Bundle

```bash
cargo test --lib build_mir_json_root_emits_array_text_indexof_const_region_plans -- --nocapture
cargo test --lib array_text_loop_session_plan -- --nocapture
cargo check --bin hakorune
cargo fmt --check
cargo run --quiet --bin hakorune -- \
  --emit-mir-json /tmp/kilo_leaf_array_string_indexof_const.region_surface.mir.json \
  benchmarks/bench_kilo_leaf_array_string_indexof_const.hako
jq '.functions[] | select(.name=="main") |
    {observer_count:(.metadata.array_text_observer_routes|length),
     region_count:(.metadata.array_text_indexof_const_region_plans|length),
     region:.metadata.array_text_indexof_const_region_plans[0]}' \
  /tmp/kilo_leaf_array_string_indexof_const.region_surface.mir.json
```

## Stop Line

```text
do not add backend reader/lowering in this row
do not add runtime helper in this row
do not let backend read array_text_observer_routes directly
do not infer legality from helper symbol spelling
do not change product ArrayBox/StringBox storage
```
