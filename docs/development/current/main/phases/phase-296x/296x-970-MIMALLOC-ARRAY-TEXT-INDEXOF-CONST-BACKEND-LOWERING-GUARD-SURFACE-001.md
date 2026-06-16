# 296x-970 MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-BACKEND-LOWERING-GUARD-SURFACE-001

Status: Landed
Date: 2026-06-16

## Purpose

Fix the post-implementation guard surface for the indexOf const region backend
lowering now that the MIR payload, C ABI reader, and helper contract exist.

This row is guard/design only. It does not emit helper calls.

## Selected Seam

Use the same region-executor shape as the length loop-session row:

```text
at loop_header:
  emit helper call
  branch to loop_exit

for loop_body:
  emit unreachable / skip body emission
```

The helper call computes the found-count region:

```text
%r<region_accumulator_phi_value> =
  call i64 @"hako.array_text.indexof_const_found_count_region"(
    i64 %r<region_array_root_value>,
    i64 <region_loop_bound_const>,
    i64 <region_row_modulus_const>,
    ptr <needle_bytes>,
    i64 <needle_byte_len>
  )
```

For the current front:

```text
emit_block=loop_header=25
skip_block=loop_body=26
exit_block=28
dst=region_accumulator_phi_value=52
exit_phi_value=region_exit_accumulator_value=54
array=region_array_root_value=5
loop_bound=400000
row_modulus=64
needle_const_text=line
needle_byte_len=4
predicate_value=92
```

The post-loop `lines.length()` in exit block 28 must remain emitted and added to
the region count.

## Guard Conditions

The backend implementation row may emit only when:

```text
plan.matched=1
plan.region_payload_present=1
plan.consumer_shape=found_predicate
plan.needle_byte_len>0
plan.backend_consumer_enabled=0 before enabling row; implementation may consume despite JSON flag only in this guarded row
region_loop_index_initial_const=0
region_accumulator_initial_const=0
region_loop_bound_const>=0
region_row_modulus_const>0
region_accumulator_phi_value present
region_predicate_value present
```

The implementation must not:

```text
infer missing fields from raw MIR
branch by benchmark name
branch by helper-name evidence
reuse mutating edit-region helpers
remove the exit block post-loop Array.length addition
change product ArrayBox/StringBox storage
```

## Result

```text
output_contract=hako-mimalloc-array-text-indexof-const-backend-lowering-guard-surface-v0
selected_backend_seam=loop_header_helper_call_then_exit
selected_helper_symbol=hako.array_text.indexof_const_found_count_region
selected_emit_block=loop_header
selected_skip_blocks=loop_header,loop_body
post_loop_exit_block_preserved=1
raw_mir_window_rescan_allowed=0
benchmark_name_branch_allowed=0
helper_name_inference_allowed=0
backend_lowering_enabled=0
runtime_helper_added=0
product_default_changed=0
winner_claim=0

selected_next=MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-BACKEND-LOWERING-IMPLEMENTATION-001
summary=ok
```

## Proof Bundle

```bash
rg -n "match_array_text_indexof_const_region_plan_by_header_metadata|array_text_indexof_const_region_has_payload_plan" \
  lang/c-abi/shims/hako_llvmc_ffi_array_text_loop_session_metadata.inc
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
