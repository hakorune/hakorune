# 296x-1015 INDEXOF-APPEND-MATERIALIZATION-BOUNDARY-INVENTORY-001

Status: Landed
Date: 2026-06-17
Scope: inventory materialization / allocator boundary for `kilo_meso_indexof_append_array_set`

## Contract

```text
output_contract=hako-indexof-append-materialization-boundary-inventory-v0
source_evidence=296x-1014,target/indexof-append-materialization-inventory-1015
row_kind=inventory
implementation_started=0

target_front=kilo_meso_indexof_append_array_set
selected_owner_family=text_materialization_allocator_boundary

array_text_observer_route_count=1
array_text_observer_selected_route_count=1
array_text_observer_selected_bridge_symbol_count=1
array_text_observer_publication_none_count=1
array_text_residence_session_count=0
array_text_combined_region_count=0
array_text_edit_route_count=0
array_text_loop_session_plan_count=0
array_text_indexof_const_region_plan_count=0

generic_method_route_count=6
generic_array_get_route_count=1
generic_array_set_route_count=1
generic_string_len_route_count=1
generic_array_len_route_count=1
concat_binop_materialization_count=1
local_fastpath_fact_count=0

indexof_direct_observer_already_selected=1
remaining_boundary_is_append_update_materialization=1
allocator_boundary_visible=1
compiler_route_seam_selected=0

next_task=ARRAY-TEXT-APPEND-UPDATE-REPRESENTATION-DESIGN-001
summary=ok
```

## Purpose

Classify the broad `indexof + append + array.set + length` front selected by
296x-1014 before opening implementation.

The key question is whether the large Hako-vs-C gap is a missing narrow
compiler-route consumer or a representation/materialization boundary.

## Source Shape

Hako body:

```hako
local current = lines.get(row)
if current.indexOf("line") >= 0 {
  local updated = current + "ln"
  lines.set(row, updated)
  total = total + lines.get(row).length()
}
```

C body:

```c
if (strstr(lines[row], "line") != NULL) {
  lines[row][lens[row]] = 'l';
  lines[row][lens[row] + 1] = 'n';
  lens[row] += 2;
  lines[row][lens[row]] = '\0';
  total += (int64_t)lens[row];
}
```

The C pair mutates the row buffer in place. The Hako route currently creates a
new String handle for `current + "ln"` and then stores it back into the array.

## MIR / Metadata Inventory

Commands:

```bash
mkdir -p target/indexof-append-materialization-inventory-1015

HAKO_STAGE1_MODE=emit-mir HAKO_EMIT_MIR_JSON=1 STAGE1_EMIT_MIR_JSON=1 \
  target/release/hakorune --emit-mir-json \
  target/indexof-append-materialization-inventory-1015/indexof_append_array_set.mir.json \
  benchmarks/bench_kilo_meso_indexof_append_array_set.hako

python3 tools/hako_check/state_explain.py \
  --mir-json target/indexof-append-materialization-inventory-1015/indexof_append_array_set.mir.json \
  --topn 8 \
  | tee target/indexof-append-materialization-inventory-1015/state_explain.log
```

Observed:

```text
array_text_observer_route_count=1
array_text_observer_indexof_count=1
array_text_observer_selected_route_count=1
array_text_observer_selected_bridge_symbol_count=1
array_text_observer_found_predicate_count=1
array_text_observer_publication_none_count=1
array_text_session_count=0
array_text_state_residence_route_count=0
array_text_selected_route_count=0
array_text_publication_in_selected_region_count=0
```

The selected observer route is already direct:

```text
selected_route=hako.array_text.session_indexof_const_utf8
fallback_route=nyash.array.string_indexof_hisi
proof_region=array_get_receiver_indexof
publication_boundary=none
consumer_shape=found_predicate
```

Generic method route inventory:

```text
generic_method_routes_count=6
array.get route_kind=array_slot_load_any key_route=unknown_any
indexOf route_kind=string_indexof selected observer route exists
array.length route_kind=array_slot_len
array.set route_kind=array_store_any key_route=unknown_any
string.length route_kind=string_len
push route_kind=array_append_any
```

Block 26 contains the materializing append:

```text
08: %81 = copy %38
09: %82 = const "ln"
10: %80 = %81 + %82
11: lines.set(row, %80)
12: %42 = copy %80
13: %41 = %42.length()
```

## Perf Inventory

Command:

```bash
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh \
  kilo_meso_indexof_append_array_set ny_main 1 \
  | tee target/fresh-compiler-owner-selection-1014/indexof_append_array_set_microasm.log
```

Top sampled owners:

```text
malloc_consolidate=34.50%
ArrayBox::boxed_from_text=19.55%
_int_malloc=16.32%
__memmove_avx512_unaligned_erms=13.62%
unlink_chunk=4.92%
malloc=1.81%
```

`ny_main` still calls the mixed route:

```asm
call hako.array_text.session_indexof_const_utf8
test %rax,%rax
js ...
call nyash.array.get_hi
call nyash.string.concat_hh
call nyash.array.set_his
call nyash.string.len_fast_h
```

## Reading

This is not a fresh narrow compiler fastpath seam like the 296x-1012
len-only store consumer.

The already-selected direct observer proves that the `indexOf("line")`
predicate is not the main missing route. The remaining gap is caused by the
update path:

```text
array.get returns a String handle
current + "ln" publishes/materializes a new String handle
array.set stores the new handle
length reads that new handle
ArrayBox text storage can be forced through boxed_from_text
allocator / memmove dominate the sampled cycles
```

The next design row should therefore decide whether this front can use a
local array-text append/update representation route:

```text
precondition: indexof observer predicate already direct
update shape: current + const_suffix
store shape: same array / same row
result demand: updated.length()
candidate route: append const suffix to resident array text cell and return len
publication boundary: none inside selected region
```

## Not Selected

```text
compiler_route_seam_selected=0
reason=no single missing generic metadata consumer; direct indexOf observer
already exists and hot samples point to text materialization / allocator.

LocalSSA / Copy:
  closed by 296x-1013 for this lane.

Array get/set generic route:
  visible but not enough alone; the semantic gap is update representation
  versus C in-place append.
```

## Stop Line

```text
do not implement from this inventory row
do not special-case benchmark/source/helper names
do not infer a route from nyash.string.concat_hh alone
do not change product ArrayBox / StringBox storage
do not claim C parity until materialization is removed from the active route
```

## Next

```text
ARRAY-TEXT-APPEND-UPDATE-REPRESENTATION-DESIGN-001
```

Design the narrow representation route for the observed shape, or reject it if
the same-array/same-row publication and length-demand proof cannot be made
without hardcoding.
