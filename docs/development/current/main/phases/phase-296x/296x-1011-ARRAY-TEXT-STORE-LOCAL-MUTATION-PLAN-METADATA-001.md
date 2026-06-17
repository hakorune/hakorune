# 296x-1011 ARRAY-TEXT-STORE-LOCAL-MUTATION-PLAN-METADATA-001

Status: Landed
Date: 2026-06-17
Scope: metadata-only route extension

## Contract

```text
output_contract=hako-array-text-store-local-mutation-plan-metadata-v0
source_evidence=296x-1010,target/fresh-compiler-owner-selection-1008
row_kind=metadata

target_front=kilo_meso_substring_concat_array_set
selected_owner_family=array_text_slot_insert_store_boundary

array_text_edit_route_extended=1
new_proof=array_get_lenhalf_insert_mid_dest_slot_len_only
same_slot_proof_preserved=1
destination_array_value_exported=1
result_len_value_exported=1

target_route_count=1
target_route_array_value=5
target_route_destination_array_value=7
target_route_get_instruction_index=7
target_route_set_instruction_index=26
target_route_result_len_value=52

backend_consumer_changed=0
existing_same_slot_consumer_reaches_new_proof=0
product_arraybox_storage_changed=0
product_stringbox_storage_changed=0
benchmark_name_branch_count=0
source_name_branch_count=0
helper_name_inference_count=0

next_task=ARRAY-TEXT-STORE-LOCAL-MUTATION-BACKEND-CONSUMER-DESIGN-001
summary=ok
```

## Purpose

Create a MIR-owned metadata row for the selected `src.get(row) -> dst.set(row,
out)` shape before touching backend lowering.

The existing `array_text_edit_routes` surface only covered same-slot edits:

```text
array.get(i) ... array.set(i, out)
```

The selected front is cross-array:

```text
src.get(row) ... dst.set(row, out)
total += len + 2
```

This row extends the route vocabulary without changing backend behavior.

## Shape

Accepted shape:

```text
src.get(i)
len = source.length()
split = len / 2
left = source.substring(0, split)
right = source.substring(split, len)
out = left + <const text> + right
dst.set(i, out)
result_len = len + const_text_len
```

Constraints:

```text
source array and destination array must be different
index roots must match
result text must have no uncovered use except the matched store
result_len is recorded as metadata, not lowered in this row
unknown shape => no route
```

## Result

For `kilo_meso_substring_concat_array_set`, MIR JSON now contains:

```text
array_text_edit_routes=1
proof=array_get_lenhalf_insert_mid_dest_slot_len_only
array_value=5
destination_array_value=7
get_instruction_index=7
set_instruction_index=26
middle_text=xx
middle_byte_len=2
result_len_value=52
```

## Behavior

No backend lowering consumes the new proof yet.

The existing C shim `array_text_edit_lenhalf_route_valid(...)` still requires:

```text
proof=array_get_lenhalf_insert_mid_same_slot
```

so the new route is visible but unreachable by the old same-slot consumer.

## Verification

```bash
cargo test -q array_text_edit_plan --lib
cargo check -q --release --bin hakorune
cargo build -q --release --bin hakorune
HAKO_STAGE1_MODE=emit-mir HAKO_EMIT_MIR_JSON=1 STAGE1_EMIT_MIR_JSON=1 \
  target/release/hakorune --emit-mir-json \
  target/fresh-compiler-owner-selection-1008/kilo_meso_substring_concat_array_set.after_edit_route.mir.json \
  benchmarks/bench_kilo_meso_substring_concat_array_set.hako
```

## Stop Line

```text
do not reuse same-slot backend consumer for cross-array routes
do not skip dst.set until destination contents observation is proven
do not branch by benchmark/source/helper name
do not change product ArrayBox or StringBox storage
```

## Next

```text
ARRAY-TEXT-STORE-LOCAL-MUTATION-BACKEND-CONSUMER-DESIGN-001
```

Decide whether to consume the route as:

```text
cross-array store_len helper
```

or:

```text
dead local store / length-only route
```
