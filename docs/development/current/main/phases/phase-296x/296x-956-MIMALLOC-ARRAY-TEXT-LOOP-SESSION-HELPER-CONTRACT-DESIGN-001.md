# 296x-956 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-HELPER-CONTRACT-DESIGN-001

Status: Landed
Date: 2026-06-16

## Purpose

Select the runtime helper contract for the read-only
`kilo_leaf_array_string_len` loop-session lowering now that MIR owns the region
payload and the C ABI backend can read it.

This row is design-only. It does not add the helper implementation, emit LLVM
calls, enable backend lowering, or change product ArrayBox/StringBox defaults.

## Decision

Add a read-only length-sum region helper, separate from the existing
store/edit/observer region helpers.

Selected helper:

```text
symbol=nyash.array.string_len_sum_region_hiii
signature=i64 (i64 handle, i64 loop_bound, i64 row_modulus, i64 initial_accumulator)
```

Semantics:

```text
acc = initial_accumulator
for step in 0..loop_bound:
  idx = step % row_modulus
  acc += text_length(array[idx])
return acc
```

The helper is read-only. It must not mutate array cells, promote public ArrayBox
semantics, or materialize public StringBox values. It may use the same private
text-slot read substrate as `nyash.array.string_len_hi`.

## Why Not Reuse Existing Helpers

Existing region helpers are not equivalent:

```text
nyash.array.string_insert_mid_subrange_len_store_region_hiisi
  mutates the slot and returns update sum

nyash.array.string_indexof_suffix_store_region_hisisi
  observes and conditionally stores suffixes

nyash.array.string_lenhalf_insert_mid_periodic_indexof_suffix_region_*
  combines edit + observer effects
```

The active front is read-only:

```text
body:
  v = lines[i % 64]
  sum = sum + v.length()
```

So the helper must stay read-only and should be named as a length-sum helper,
not as an edit-region helper.

## Backend Contract

The backend may emit this helper only when all are true:

```text
ArrayTextLoopSessionPlanMetadata.matched=1
region_payload_present=1
backend_session_lowering_allowed=1
backend_consumer_enabled=0 in MIR JSON until the enabling row flips it
region_loop_index_initial_const=0
region_accumulator_initial_const=0 or explicitly passed as helper arg
region_loop_bound_const >= 0
region_row_modulus_const > 0
```

The first implementation row should pass:

```text
handle=%r<region_array_root_value>
loop_bound=region_loop_bound_const
row_modulus=region_row_modulus_const
initial_accumulator=%r<region_accumulator_initial_value> or literal 0
dst=%r<region_exit_accumulator_value>
```

For the selected front, this means:

```text
handle=%r5
loop_bound=600000
row_modulus=64
initial_accumulator=0
dst=%r56
exit_block=28
```

The post-loop `lines.length()` remains outside this helper and stays in the
exit block.

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-helper-contract-design-v0
target_front=kilo_leaf_array_string_len
row_kind=design

selected_helper_symbol=nyash.array.string_len_sum_region_hiii
selected_helper_signature=i64(i64,i64,i64,i64)
helper_effect=readonly
helper_returns_accumulated_length=1
helper_mutates_array=0
helper_materializes_public_stringbox=0
backend_lowering_enabled=0
runtime_helper_enabled=0
product_default_changed=0
winner_claim=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-HELPER-SURFACE-001
summary=ok
```

## Stop Line

```text
do not reuse mutating edit-region helpers for read-only length sum
do not emit helper calls in this design row
do not skip the post-loop Array.length addition
do not infer region shape from raw MIR in C
do not change product ArrayBox/StringBox behavior
do not claim performance before measurement
```

## Proof Bundle

```bash
rg -n "string_len_sum_region|string_insert_mid_subrange_len_store_region|array.string_len_hi" \
  crates/nyash_kernel/src lang/c-abi/shims src/boxes/array/ops/text.rs
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
