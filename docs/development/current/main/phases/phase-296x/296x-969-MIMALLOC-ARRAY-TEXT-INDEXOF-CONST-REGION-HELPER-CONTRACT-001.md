# 296x-969 MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-REGION-HELPER-CONTRACT-001

Status: Landed
Date: 2026-06-16

## Purpose

Fix the runtime helper contract for lowering an
`ArrayTextIndexOfConstRegionPlan` into one region call.

This row is docs-only. It does not add the helper implementation or backend
lowering.

## Helper Contract

Selected helper name:

```text
hako.array_text.indexof_const_found_count_region
```

Semantic operation:

```text
count rows in a loop-local array-text region where:
  indexOf(array[row], const_needle) >= 0
```

Equivalent source shape:

```hako
local hits = 0
loop (i < loop_bound) {
  local row = i % row_modulus
  local pos = array.get(row).indexOf("needle")
  if pos >= 0 {
    hits = hits + 1
  }
}
```

The helper returns the number of found rows. Backend lowering will add the
returned count to the surrounding accumulator according to the MIR region plan.

## Proposed C ABI Shape

The concrete C ABI row may choose exact integer widths used by the current shim,
but the semantic argument order is fixed here:

```text
hako_array_text_indexof_const_found_count_region(
  array_handle,
  loop_bound,
  row_modulus,
  needle_ptr,
  needle_len
) -> i64
```

Rules:

```text
array_handle:
  product ArrayBox handle / runtime array handle

loop_bound:
  number of loop iterations proven by MIR

row_modulus:
  row index period proven by MIR

needle_ptr, needle_len:
  const UTF-8 bytes from MIR metadata
```

The helper owns runtime-private iteration over rows. It must not ask the backend
to rescan raw MIR windows.

## Semantics

For each `i` in `[0, loop_bound)`:

```text
row = i % row_modulus
cell = array[row]
pos = indexOf(cell, needle)
if pos >= 0:
  count += 1
```

Required behavior:

```text
empty needle:
  follow StringBox/indexOf semantics for the product runtime

non-string cell:
  follow the same selected route semantics as the existing array-text observer
  helper family

out-of-range row:
  fail-fast or product-compatible runtime error; do not silently ignore
```

## Ownership Boundaries

MIR owns:

```text
loop_bound
row_modulus
const needle bytes
found-predicate accumulator shape
publication_boundary=none
```

C ABI reader owns:

```text
metadata decoding and validation
```

Runtime helper owns:

```text
ArrayBox/StringBox compatible row access
indexOf execution
found count accumulation
```

Backend owns:

```text
mapping the validated region plan to the helper call
adding the helper result to the post-region accumulator
```

## Result

```text
output_contract=hako-mimalloc-array-text-indexof-const-region-helper-contract-v0

helper_contract_selected=1
helper_symbol=hako.array_text.indexof_const_found_count_region
helper_semantics=found_count_region
helper_added=0
backend_lowering_enabled=0
product_default_changed=0

mir_owns_region_legality=1
c_abi_reader_owns_metadata_decode=1
runtime_helper_owns_string_indexof_execution=1
backend_owns_helper_call_mapping=1

selected_next=MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-BACKEND-LOWERING-GUARD-SURFACE-001
summary=ok
```

## Stop Line

```text
do not implement the helper in this row
do not add backend lowering in this row
do not infer legality from helper symbol spelling
do not change product ArrayBox/StringBox storage
do not claim a performance win before measurement
```
