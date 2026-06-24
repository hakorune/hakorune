# 296x-971 MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-BACKEND-LOWERING-IMPLEMENTATION-001

Status: Landed
Date: 2026-06-16

## Purpose

Enable the guarded backend lowering for the `array string get ->
indexOf("line") -> found predicate` region selected by 296x-970.

This row consumes only the validated
`array_text_indexof_const_region_plans` metadata. It does not infer from helper
names, benchmark names, raw MIR rescans, or product storage internals.

## Implementation

Runtime helper:

```text
hako.array_text.indexof_const_found_count_region(
  array_handle,
  loop_bound,
  row_modulus,
  needle_ptr,
  needle_len
) -> i64
```

Backend seam:

```text
loop_header:
  %r<region_accumulator_phi_value> =
    call i64 @"hako.array_text.indexof_const_found_count_region"(...)
  br label %loop_exit

loop_body:
  unreachable
```

The helper result is written to `region_accumulator_phi_value`, not directly to
`region_exit_accumulator_value`. The existing exit PHI remains the owner that
feeds the post-loop `Array.length` addition.

## Guarded Shape

```text
plan.matched=1
plan.region_payload_present=1
consumer_shape=found_predicate
needle_byte_len=4
loop_bound_const=400000
row_modulus_const=64
region_accumulator_phi_value=52
region_exit_accumulator_value=54
loop_body_unreachable=1
post_loop_exit_block_preserved=1
```

## Result

```text
output_contract=hako-mimalloc-array-text-indexof-const-backend-lowering-implementation-v0
backend_lowering_enabled=1
runtime_helper_added=1
selected_helper_symbol=hako.array_text.indexof_const_found_count_region
selected_backend_seam=loop_header_helper_call_then_exit
loop_body_unreachable=1
exit_phi_preserved=1
product_arraybox_storage_changed=0
product_stringbox_storage_changed=0
benchmark_name_branch_count=0
helper_name_inference_count=0
winner_claim=0

microstat_name=kilo_leaf_array_string_indexof_const
microstat_repeat=3
c_instr=37326772
c_cycles=5730118
ny_aot_instr=4136429
ny_aot_cycles=1197019
ratio_instr=9.02
ratio_cycles=4.79
aot_status=ok
summary=ok
```

## Benchmark Hygiene

The C pair now returns `hits + rows`, matching the Hako source return expression
`hits + lines.length()`. This keeps status-code smoke comparisons aligned
without changing the hot loop shape.

## Proof Bundle

```bash
cargo test -p nyash_kernel array_text_indexof_const_found_count_region_counts_hits -- --nocapture
cargo test --lib build_mir_json_root_emits_array_text_indexof_const_region_plans -- --nocapture
cargo check --bin hakorune
bash tools/perf/build_perf_release.sh
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_array_string_indexof_const 1 3
```

## Next

```text
selected_next=MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-CLOSEOUT-001
```
