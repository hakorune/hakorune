# 296x-959 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BACKEND-LOWERING-IMPLEMENTATION-001

Status: Landed
Date: 2026-06-16

## Purpose

Enable the guarded backend lowering for the read-only array text loop-session
front by consuming MIR-owned `array_text_loop_session_plans` metadata and the
runtime helper from 296x-957.

## Implementation

Code changes:

```text
src/mir/array_text_loop_session_plan.rs
src/runner/mir_json_emit/array_metadata.rs
src/runner/mir_json_emit/tests/array_routes.rs
lang/c-abi/shims/hako_llvmc_ffi_array_text_loop_session_metadata.inc
lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc
lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc
```

The implementation adds `loop_body` to the MIR-owned plan/export/reader so the
backend does not infer skipped blocks from raw MIR. The backend then emits:

```text
%r<region_exit_accumulator_value> =
  call i64 @"nyash.array.string_len_sum_region_hiii"(
    i64 %r<region_array_root_value>,
    i64 <region_loop_bound_const>,
    i64 <region_row_modulus_const>,
    i64 <region_accumulator_initial_const>
  )
br label %bb<loop_exit>
```

The loop body is emitted as `unreachable`. The exit block remains live and still
adds the post-loop `lines.length()` result.

## Evidence

For `kilo_leaf_array_string_len`, generated `ny_main` now contains:

```text
call nyash.array.string_len_sum_region_hiii
call nyash.array.len_h
add helper_result, array_length
ret
```

The direct executable returned:

```text
Result: 5400064
exit=0
```

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-backend-lowering-implementation-v0
target_front=kilo_leaf_array_string_len

backend_lowering_enabled=1
selected_helper_symbol=nyash.array.string_len_sum_region_hiii
loop_body_metadata_export_enabled=1
raw_mir_window_rescan_allowed=0
benchmark_name_branch_added=0
helper_name_inference_added=0
post_loop_exit_block_preserved=1
product_default_changed=0
winner_claim=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BACKEND-LOWERING-MEASUREMENT-001
summary=ok
```

## Stop Line

```text
do not claim keeper until measurement
do not use this row as a product default semantic change
do not generalize to mutating edit/observer regions
do not remove the post-loop Array.length path
```

## Proof Bundle

```bash
cargo test --lib array_text_loop_session_plan -- --nocapture
cargo test --lib build_mir_json_root_emits_array_text_loop_session_plans -- --nocapture
bash tools/build_hako_llvmc_ffi.sh
bash tools/perf/build_perf_release.sh
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_array_string_len ny_main 1
/home/tomoaki/git/hakorune-selfhost/target/perf_ny_kilo_leaf_array_string_len.microasm.1479766.exe
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
