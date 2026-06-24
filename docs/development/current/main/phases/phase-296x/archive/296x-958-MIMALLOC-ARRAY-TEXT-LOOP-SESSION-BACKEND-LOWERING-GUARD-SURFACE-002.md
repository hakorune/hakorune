# 296x-958 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BACKEND-LOWERING-GUARD-SURFACE-002

Status: Landed
Date: 2026-06-16

## Purpose

Fix the post-implementation guard surface for the read-only array text
loop-session backend lowering now that the MIR payload, C ABI reader, and
runtime helper surface exist.

This row is guard/design only. It does not emit helper calls.

## Selected Seam

Use the existing region-executor lowering shape:

```text
at loop_header:
  emit helper call
  branch to loop_exit

for loop body:
  emit unreachable
```

The helper call is:

```text
%r<region_exit_accumulator_value> =
  call i64 @"nyash.array.string_len_sum_region_hiii"(
    i64 %r<region_array_root_value>,
    i64 <region_loop_bound_const>,
    i64 <region_row_modulus_const>,
    i64 %r<region_accumulator_initial_value> or i64 <region_accumulator_initial_const>
  )
```

For the current front:

```text
emit_block=loop_header=25
skip_block=loop_body=26
exit_block=28
dst=region_exit_accumulator_value=56
array=region_array_root_value=5
loop_bound=600000
row_modulus=64
initial_accumulator=0
```

The post-loop `lines.length()` in exit block 28 must remain emitted.

## Guard Conditions

The backend implementation row may emit only when:

```text
plan.matched=1
plan.region_payload_present=1
plan.backend_session_lowering_allowed=1
plan.backend_consumer_enabled=0 before enabling row; implementation may consume despite JSON flag only in this guarded row
region_loop_index_initial_const=0
region_accumulator_initial_const=0
region_loop_bound_const>=0
region_row_modulus_const>0
region_exit_accumulator_value==region_accumulator_phi_value
```

The implementation must not:

```text
infer missing fields from raw MIR
branch by benchmark name
branch by helper-name evidence
reuse mutating edit-region helpers
remove the exit block post-loop Array.length addition
```

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-backend-lowering-guard-surface-v2
selected_backend_seam=loop_header_helper_call_then_exit
selected_helper_symbol=nyash.array.string_len_sum_region_hiii
selected_emit_block=loop_header
selected_skip_blocks=loop_header,loop_body
post_loop_exit_block_preserved=1
raw_mir_window_rescan_allowed=0
benchmark_name_branch_allowed=0
helper_name_inference_allowed=0
backend_lowering_enabled=0
product_default_changed=0
winner_claim=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BACKEND-LOWERING-IMPLEMENTATION-001
summary=ok
```

## Proof Bundle

```bash
rg -n "match_array_text_loop_session_plan_by_header_metadata|active_region_executor|active_combined_region" \
  lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc \
  lang/c-abi/shims/hako_llvmc_ffi_array_text_loop_session_metadata.inc
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
