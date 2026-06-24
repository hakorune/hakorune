# 296x-965 MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-OWNER-INVENTORY-001

Status: Landed
Date: 2026-06-16

## Purpose

Inventory the selected `kilo_leaf_array_string_indexof_const` front before
opening any implementation row.

This row follows 296x-964 and must not patch helpers, backend lowering, MIR
producer code, ArrayBox storage, or benchmark source.

## Front

Source shape:

```hako
loop (i < ops) {
  local row = i % rows
  local cur = lines.get(row)
  local pos = cur.indexOf("line")
  if (pos >= 0) {
    hits = hits + 1
  }
  i = i + 1
}
```

`ny_main` currently lowers the hot loop to:

```text
row = i & 63
call hako.array_text.session_indexof_const_utf8(handle, row, "line", 4)
hits += (pos >= 0)
```

## Measurement Evidence

Paired microstat from 296x-964:

```text
c_instr=37326776
c_cycles=5773338
c_ms=4
ny_aot_instr=109926040
ny_aot_cycles=33004811
ny_aot_ms=10
ratio_instr=0.34
ratio_cycles=0.17
ratio_ms=0.40
aot_status=ok
```

Direct micro-ASM:

```text
event_count_approx=36050108
top_symbol=memchr::arch::x86_64::memchr::memchr_raw::find_avx2
top_symbol_percent=65.53
secondary_symbol=std::thread::local::LocalKey<T>::with
secondary_symbol_percent=32.52
```

## MIR Metadata Evidence

Command:

```bash
cargo run --quiet --bin hakorune -- --emit-mir-json \
  /tmp/kilo_leaf_array_string_indexof_const.current.mir.json \
  benchmarks/bench_kilo_leaf_array_string_indexof_const.hako
```

The metadata has no loop-session plan yet:

```text
array_text_loop_session_plan_count=0
```

It does have a precise observer route:

```text
array_text_observer_route_count=1
observer_kind=indexof
consumer_shape=found_predicate
selected_route=hako.array_text.session_indexof_const_utf8
array_value=5
block=26
get_instruction_index=7
observer_instruction_index=9
index_value=78
source_value=60
result_value=62
observer_arg0_repr=const_utf8
observer_arg0_text=line
observer_arg0_byte_len=4
publication_boundary=none
result_repr=scalar_i64
```

Generic route metadata confirms the same seam:

```text
generic_method.get:
  route_kind=array_slot_load_any
  result_origin_box=StringBox
  receiver_value=5
  key_value=78

generic_method.indexOf:
  route_kind=string_indexof
  publication_policy=no_publication
  receiver_value=60
  result_value=62
  return_shape=scalar_i64

generic_method.length:
  route_kind=array_slot_len
  receiver_value=5
```

## Owner Analysis

The hot owner is not a missing scalar route for the single indexOf call. That
route already exists and is visible in `ny_main`.

The remaining Hako-slower work is:

```text
per_iteration_session_indexof_call=1
loop_bound=400000
row_modulus=64
needle_const_utf8=line
consumer_shape=found_predicate
```

This is structurally analogous to the closed length front:

```text
old length front:
  per-iteration array.get -> length
  fixed rows
  read-only
  reducible to region helper

current indexOf front:
  per-iteration array.get -> indexOf("line") -> pos >= 0
  fixed rows
  read-only
  reducible to region helper
```

But the necessary MIR-owned region plan is missing. Therefore the next owner is
not backend lowering yet; it is the plan producer / payload surface for an
indexOf const region.

## Selected Next

```text
selected_next=MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-REGION-PAYLOAD-DESIGN-001
```

Required shape:

```text
ArrayTextIndexOfConstRegionPlan:
  array_root_value
  loop_header
  loop_body
  loop_exit
  loop_index_phi_value
  loop_index_initial_const
  loop_index_next_value
  loop_bound_value
  loop_bound_const
  row_index_value
  row_modulus_value
  row_modulus_const
  accumulator_phi_value
  accumulator_initial_const
  accumulator_next_value
  exit_accumulator_value
  observer_result_value
  needle_ptr/const_text metadata
  needle_byte_len
  found_predicate_shape
```

The exact field list should be finalized in the design row. The implementation
must reuse existing observer-route evidence and must not infer from helper
symbol spelling.

## Result

```text
output_contract=hako-mimalloc-array-text-indexof-const-owner-inventory-v0
target_front=kilo_leaf_array_string_indexof_const

selected_owner_family=array_text_indexof_const_region_plan_producer
array_text_observer_route_count=1
array_text_loop_session_plan_count=0
backend_lowering_ready=0

per_iteration_session_indexof_call=1
loop_bound=400000
row_modulus=64
needle_const_utf8=line
needle_byte_len=4
consumer_shape=found_predicate
publication_boundary=none

top_symbol=memchr_find_avx2
top_symbol_percent=65.53
secondary_symbol=thread_local_key_with
secondary_symbol_percent=32.52

implementation_started=0
backend_changed=0
runtime_helper_changed=0
product_default_changed=0
benchmark_source_changed=0

selected_next=MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-REGION-PAYLOAD-DESIGN-001
summary=ok
```

## Stop Line

```text
do not patch hako.array_text.session_indexof_const_utf8 before region metadata
do not add backend lowering without a MIR-owned region plan
do not infer from helper symbol spelling
do not change ArrayBox or StringBox storage policy
do not select the meso mutating indexOf front before this leaf is closed
```

## Proof Bundle

```bash
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_array_string_indexof_const ny_main 3
cargo run --quiet --bin hakorune -- --emit-mir-json \
  /tmp/kilo_leaf_array_string_indexof_const.current.mir.json \
  benchmarks/bench_kilo_leaf_array_string_indexof_const.hako
jq '.functions[] | select(.name=="main") | .metadata.array_text_observer_routes' \
  /tmp/kilo_leaf_array_string_indexof_const.current.mir.json
jq '.functions[] | select(.name=="main") | .metadata.array_text_loop_session_plans' \
  /tmp/kilo_leaf_array_string_indexof_const.current.mir.json
```
