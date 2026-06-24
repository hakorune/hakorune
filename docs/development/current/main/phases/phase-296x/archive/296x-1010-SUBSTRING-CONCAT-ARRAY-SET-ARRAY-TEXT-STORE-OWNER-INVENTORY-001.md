# 296x-1010 SUBSTRING-CONCAT-ARRAY-SET-ARRAY-TEXT-STORE-OWNER-INVENTORY-001

Status: Landed
Date: 2026-06-17
Scope: owner inventory / array-text store boundary

## Contract

```text
output_contract=hako-substring-concat-array-set-array-text-store-owner-inventory-v0
source_evidence=296x-1008,296x-1009,target/fresh-compiler-owner-selection-1008
row_kind=owner_inventory
implementation_started=0

target_front=kilo_meso_substring_concat_array_set
selected_owner_family=array_text_slot_insert_store_boundary
selected_front_valid_c_pair=1

c_loop_shape=fixed_buffer_substring_insert_store_total_len
hako_loop_shape=array_text_slot_len_insert_store_total_len
hako_runtime_helper_boundary=0
hako_generic_box_get_set_boundary=0
hako_selected_array_text_helpers=1

hot_symbol_0=nyash.array.string_len_hi
hot_symbol_0_pct=54.95
hot_symbol_1=nyash.array.kernel_slot_store_hi
hot_symbol_1_pct=26.01
hot_symbol_2=__memmove_avx512_unaligned_erms
hot_symbol_2_pct=6.27
hot_symbol_3=array_kernel_slot_insert_hisi
hot_symbol_3_pct=5.71
hot_symbol_4=with_array_text_session_cached
hot_symbol_4_pct=5.70

ny_main_contains_array_text_slot_len=1
ny_main_contains_kernel_slot_insert_hisi=1
ny_main_contains_kernel_slot_store_hi=1
ny_main_contains_stringbox_concat_hh=0
ny_main_contains_runtime_array_get_set_hh=0

string_dead_text_region_candidate_count=0
local_fastpath_fact_count=0
fastpath_reachability_selected_route=none
new_fastpath_consumer_selected=0

selected_next=ARRAY-TEXT-STORE-LOCAL-MUTATION-PLAN-DESIGN-001
summary=ok
```

## Purpose

Classify the next valid Hako-slower front after substring concat exact-seed
closeout and C benchmark repair.

## Front

Source:

```hako
local line = src.get(row)
local len = line.length()
local split = len / 2
local left = line.substring(0, split)
local right = line.substring(split, len)
local out = left + "xx" + right
dst.set(row, out)
total = total + out.length()
```

C pair:

```c
memcpy(dst[row], src[row], split);
dst[row][split] = 'x';
dst[row][split + 1] = 'x';
memcpy(dst[row] + split + 2, src[row] + split, len - split);
dst[row][len + 2] = '\0';
total += len + 2;
```

## Measurement

Lane measurement:

```text
c_kernel_instr=901308
c_kernel_cycles=182694
ny_kernel_instr=4554638
ny_kernel_cycles=1951681
ratio_kernel_instr=0.20
ratio_kernel_cycles=0.09
```

Micro-ASM top report:

```text
54.95% nyash.array.string_len_hi
26.01% nyash.array.kernel_slot_store_hi
 6.27% __memmove_avx512_unaligned_erms
 5.71% array_kernel_slot_insert_hisi
 5.70% with_array_text_session_cached
```

Selected `ny_main` loop:

```asm
call hako.array_text.slot_len
...
call nyash.array.kernel_slot_insert_hisi
call nyash.array.kernel_slot_store_hi
add  %r12,%r14
add  $0x2,%r14
```

## Reading

This front is already past the generic `StringBox` concat and generic
`ArrayBox` get/set boundary in the hot loop. The remaining owner is the
array-text representation boundary:

```text
slot_len + insert temp + slot_store + session cache
```

The C pair performs fixed-buffer writes and accumulates a known length. Hako
materializes a temporary stack text slot and stores it into a local `dst`
array. The `dst` array's contents are not read later; only `dst.length()` is
read after the loop.

## Decision

The next design row should decide whether this shape is handled as:

```text
local array text mutation plan
```

or:

```text
dead local array text store / total length closed-form
```

It must not add a benchmark-specific branch. The proof must come from
array-text metadata, local publication state, and use/observation of the `dst`
array.

## Stop Line

```text
do not branch by benchmark name
do not branch by source path
do not infer from helper symbol names alone
do not skip dst.set unless dst contents are proven unobserved
do not change product ArrayBox or StringBox storage
do not add a new runtime helper for this row
```

## Next

```text
ARRAY-TEXT-STORE-LOCAL-MUTATION-PLAN-DESIGN-001
```
