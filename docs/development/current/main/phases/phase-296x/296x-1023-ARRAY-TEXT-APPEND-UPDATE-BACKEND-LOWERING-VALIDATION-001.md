Status: Done
Date: 2026-06-17
Scope: target-front reachability validation for append/update observer len-sum backend lowering.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-1022-ARRAY-TEXT-APPEND-UPDATE-BACKEND-LOWERING-IMPLEMENTATION-001.md
  - target/array-text-append-update-backend-validation-1023/state_explain.log
  - target/array-text-append-update-backend-validation-1023/microasm.log
  - target/array-text-append-update-backend-validation-1023/ny_main.objdump.txt

# ARRAY-TEXT-APPEND-UPDATE-BACKEND-LOWERING-VALIDATION-001

## Purpose

Validate that the implementation row is reachable on the active target front
before any measurement or winner claim.

This row is validation-only. It does not add another lowering rule, change the
runtime helper, or claim Hako-vs-C speedup.

## Commands

```bash
mkdir -p target/array-text-append-update-backend-validation-1023
cargo build -q --release --bin hakorune
HAKO_STAGE1_MODE=emit-mir HAKO_EMIT_MIR_JSON=1 STAGE1_EMIT_MIR_JSON=1 \
  target/release/hakorune --emit-mir-json \
  target/array-text-append-update-backend-validation-1023/indexof_append_array_set.mir.json \
  benchmarks/bench_kilo_meso_indexof_append_array_set.hako
python3 tools/hako_check/state_explain.py \
  --mir-json target/array-text-append-update-backend-validation-1023/indexof_append_array_set.mir.json \
  --topn 3 \
  | tee target/array-text-append-update-backend-validation-1023/state_explain.log

bash tools/perf/build_perf_release.sh
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh \
  kilo_meso_indexof_append_array_set ny_main 1 \
  | tee target/array-text-append-update-backend-validation-1023/microasm.log
```

## Evidence

MIR metadata still exposes exactly one len-sum executor contract:

```text
array_text_observer_executor_contract_count=1
array_text_observer_route_0_executor_contract_consumer_capabilities=compare_only,sink_store_len_sum
array_text_observer_route_0_region_mapping_row_modulus_const=128
array_text_observer_route_0_region_mapping_accumulator_phi_value=33
array_text_observer_route_0_region_mapping_accumulator_next_value=43
summary=ok
```

The direct micro-ASM runner reaches the new helper in `ny_main`:

```text
call nyash.array.string_indexof_suffix_store_len_sum_region_hiisisi
call nyash.array.len_h
```

The old per-iteration materialization route is not present in the `ny_main`
objdump snippet:

```text
nyash.array.string_indexof_suffix_store_region_hisisi=0
nyash.array.string_indexof_hisi=0
nyash.array.get_hh=0
nyash.array.set_his=0
nyash.string.len=0
nyash.string.concat=0
```

The executable returned:

```text
Result: 805440128
```

## Result

```text
output_contract=hako-array-text-append-update-backend-lowering-validation-v0
target_front=kilo_meso_indexof_append_array_set
mir_executor_contract_count=1
backend_new_helper_reachable=1
store_count_helper_reused=0
per_iteration_materialization_route_removed_from_ny_main=1
product_default_changed=0
winner_claim=0
summary=ok
```

## Notes

`cargo fmt --check` was not used as a blocker for this row because it reports
pre-existing formatting drift in files outside this implementation slice.

## Next

```text
ARRAY-TEXT-APPEND-UPDATE-BACKEND-MEASUREMENT-001
```

Measure the target front after reachability is proven. Winner claims remain
closed until measurement says so.
