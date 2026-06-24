# 296x-961 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BORROWED-TEXT-ACCESS-OWNER-INVENTORY-001

Status: Landed
Date: 2026-06-16

## Purpose

Inventory the hot owner selected by 296x-960 before changing text storage or
borrowed-handle representation.

The measured remaining owner is:

```text
BorrowedHandleBox::as_str_fast
```

## Evidence

296x-960 direct micro-ASM:

```text
event_count_approx=1211190
top_symbol=BorrowedHandleBox::as_str_fast
top_symbol_percent=95.24
```

Relevant helper path:

```text
nyash.array.string_len_sum_region_hiii
  -> array_string_len_sum_region()
  -> ArrayTextSession::slot_text_len_sum_region_raw()
  -> ArrayBox::slot_text_len_sum_region_raw()
```

Current source owner:

```text
src/boxes/array/ops/text.rs:
  ArrayStorage::Text(values) =>
    values.get(idx).map(|value| value.len() as i64)

  ArrayStorage::Boxed(items) =>
    items.get(idx).and_then(|item| item.as_str_fast().map(|value| value.len() as i64))
```

The target front builds its array through `lines.push("...")`, which lowers to:

```text
nyash.array.push_hh
  -> array_slot_append_any()
  -> slot_append_box_raw()
  -> ArrayStorage::Boxed
```

So the region helper currently executes the Boxed branch for every loop
iteration. That means the current helper calls `item.as_str_fast()` roughly:

```text
loop_bound=600000
row_modulus=64
as_str_fast_call_count ~= 600000
```

Objdump for the hot `BorrowedHandleBox::as_str_fast` symbol shows this is a
thin projection through the stable object:

```text
load text_keep/source backing
jump through stable_box.as_str_fast vtable slot
```

The cost is therefore not a new semantic loop owner. It is repeated text-length
projection over a small resident row domain.

## Candidate Seams

### A. Change Array push to text lane

Rejected for this row.

```text
reason=public_array_storage_semantics
product_default_risk=medium
```

`push_hh` currently stores borrowed aliases in `ArrayStorage::Boxed`. Moving it
to `ArrayStorage::Text` would change representation and possibly visible
objectization/publication behavior. This needs a separate Map/Array storage
policy row, not a helper-internal owner fix.

### B. Special-case BorrowedHandleBox in ArrayBox

Rejected for this row.

```text
reason=representation_specific_downcast_in_array_hot_path
product_default_risk=medium
```

This would couple ArrayBox to a kernel plugin value-codec carrier and would
create a product-route dependency from the array box to a private runtime alias.

### C. Precompute row-domain lengths inside the region helper

Selected.

```text
selected_seam=region_helper_local_length_table
product_storage_changed=0
borrowed_handle_representation_changed=0
array_push_route_changed=0
```

The helper already has the exact region contract:

```text
loop_bound
row_modulus
initial_accumulator
```

For this read-only repeated-row region, it can first read the `row_modulus`
lengths once into a local length table, then run the loop over integers:

```text
for idx in 0..row_modulus:
  lens[idx] = slot_text_len_raw(idx)?

for step in 0..loop_bound:
  total += lens[step % row_modulus]
```

For the target front, this reduces `as_str_fast` projections from 600000 to 64
without changing public ArrayBox storage or any product default route.

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-borrowed-text-access-owner-inventory-v0
target_front=kilo_leaf_array_string_len

remaining_hot_owner=borrowed_handle_text_access
remaining_top_symbol=BorrowedHandleBox::as_str_fast
remaining_top_symbol_percent=95.24

array_storage_observed=Boxed
array_text_lane_required=0
array_push_route_changed=0
product_storage_changed=0
borrowed_handle_representation_changed=0

loop_bound=600000
row_modulus=64
current_as_str_fast_projection_count_estimate=600000
selected_as_str_fast_projection_count_estimate=64

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-LENGTH-TABLE-IMPLEMENTATION-001
summary=ok
```

## Stop Line

```text
do not change ArrayBox public storage policy
do not change nyash.array.push_hh
do not downcast BorrowedHandleBox inside ArrayBox
do not alter StringBox / BorrowedHandleBox representation
do not claim a product default semantic change
```

## Proof Bundle

```bash
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_array_string_len 1 3
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_array_string_len ny_main 3
objdump -d --demangle --start-address=0x4139c0 --stop-address=0x413a60 \
  target/perf_ny_kilo_leaf_array_string_len.microasm.1483023.exe
```
