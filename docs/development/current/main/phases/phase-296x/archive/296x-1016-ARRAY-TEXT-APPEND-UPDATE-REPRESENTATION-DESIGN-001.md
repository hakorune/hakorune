# 296x-1016 ARRAY-TEXT-APPEND-UPDATE-REPRESENTATION-DESIGN-001

Status: Landed
Date: 2026-06-17
Scope: design route for indexOf-guarded const suffix append/update with length result

## Contract

```text
output_contract=hako-array-text-append-update-representation-design-v0
source_evidence=296x-1015
row_kind=design
implementation_started=0

target_front=kilo_meso_indexof_append_array_set
selected_shape=indexof_const_suffix_store_len_sum_region
selected_route_family=array_text_observer_store_region

existing_indexof_observer_route_reused=1
existing_observer_store_region_contract_extended=1
existing_store_count_helper_reused=0
new_helper_contract_required=1

selected_helper_symbol=nyash.array.string_indexof_suffix_store_len_sum_region_hiisisi
selected_helper_signature=i64(i64 handle, i64 loop_bound, i64 row_modulus, ptr needle, i64 needle_len, ptr suffix, i64 suffix_len)
helper_effect=mutates_array_text_cells
helper_returns_accumulated_length=1
helper_materializes_public_stringbox=0
helper_changes_product_arraybox_storage=0

required_contract_capabilities=compare_only,sink_store,length_result_carry
publication_boundary=none
materialization_policy=text_resident_or_stringlike_slot
backend_lowering_enabled=0
runtime_helper_enabled=0
product_default_changed=0

next_task=ARRAY-TEXT-APPEND-UPDATE-REGION-PAYLOAD-SURFACE-001
summary=ok
```

## Purpose

Select the narrow representation route for the broad
`kilo_meso_indexof_append_array_set` materialization owner.

The design goal is to remove the current per-iteration public String handle
materialization:

```text
array.get
current.indexOf("line")
current + "ln"
array.set(row, updated)
updated.length()
```

and replace the selected region with an array-text cell update route:

```text
if cell.contains("line"):
  cell.append_suffix("ln")
  acc += cell.length()
```

## Existing Pieces

Already available:

```text
array_text_observer_routes
  detects array.get(...).indexOf(const_utf8)
  selected_route=hako.array_text.session_indexof_const_utf8

array_text_observer_region_contract
  can recognize conditional same-slot const suffix store
  current capabilities=compare_only,sink_store

runtime helper:
  nyash.array.string_indexof_suffix_store_region_hisisi
  mutates matching cells, but returns store count
```

Why this is not enough:

```text
current benchmark consumes updated.length()
existing store-region helper returns stores, not accumulated length
actual MIR keeps the get/concat result live because length reads the updated handle
```

So the next route must add a length-carry capability instead of reusing the
store-count helper.

## Selected Route

```text
route_id=array_text.indexof_suffix_store_len_sum_region
proof_region=loop_backedge_single_body
execution_mode=single_region_executor
effects=observe.indexof,store.cell(const_suffix_append),scalar_accumulator(length)
consumer_capabilities=compare_only,sink_store,length_result_carry
publication_boundary=none
carrier=array_lane_text_cell
materialization_policy=text_resident_or_stringlike_slot
```

The runtime helper contract:

```text
symbol=nyash.array.string_indexof_suffix_store_len_sum_region_hiisisi
signature=i64(i64 handle, i64 loop_bound, i64 row_modulus, ptr needle, i64 needle_len, ptr suffix, i64 suffix_len)

acc = 0
for idx in 0..loop_bound:
  if text_cell[idx].contains(needle):
    text_cell[idx].append_suffix(suffix)
    acc += text_cell[idx].length()
return acc
```

For the selected front:

```text
handle=%r5
loop_bound=320000
needle="line"
suffix="ln"
dst=loop accumulator next / exit accumulator
```

## Why A New Helper

Do not reuse:

```text
nyash.array.string_indexof_suffix_store_region_hisisi
```

because its return value is the number of stores. Treating that as length would
silently change semantics.

Do not use the existing combined-region helper either:

```text
nyash.array.string_lenhalf_insert_mid_periodic_indexof_suffix_region_*
```

because that helper combines an outer len-half edit with a periodic nested
observer store. The current front is a single loop with an indexOf guard and a
same-row suffix append.

## Producer Shape

The MIR producer should extend the existing observer-store region contract only
when all are true:

```text
observer route is array_get_receiver_indexof
observer arg0 is const_utf8
consumer shape is found_predicate
then/else branch contains same-array same-index const-suffix concat
same branch stores concat result back to the same slot
the concat result has no non-covered use except length()
the length result feeds the loop accumulator
publication_boundary=none
array value and index roots are stable through copy chains
```

The current failure mode is expected:

```text
array_text_observer_executor_contract_count=0
reason=concat_result_length_use_not_covered_by compare_only/sink_store contract
```

The next row should add passive metadata only. It must not emit the helper yet.

## Backend Shape

First backend consumer row should eventually replace the region body with:

```llvm
%acc.next = call i64 @"nyash.array.string_indexof_suffix_store_len_sum_region_hiisisi"(
  i64 %array,
  i64 <loop_bound>,
  i64 <row_modulus>,
  ptr @needle,
  i64 <needle_len>,
  ptr @suffix,
  i64 <suffix_len>
)
```

The exact accumulator placement must be MIR-owned metadata. The C backend must
not infer loop bounds, accumulator values, row mapping, or result placement from
raw instruction names.

## Stop Line

```text
do not enable backend lowering in this design row
do not add runtime helper implementation in this design row
do not reuse store-count helper for length-sum semantics
do not infer region shape in C from raw MIR
do not special-case kilo_meso_indexof_append_array_set
do not change product ArrayBox or StringBox storage
```

## Next

```text
ARRAY-TEXT-APPEND-UPDATE-REGION-PAYLOAD-SURFACE-001
```

Add passive MIR metadata for the selected route and a guard surface proving that
the current front exposes exactly one candidate before any runtime/backend
implementation.
