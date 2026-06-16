# 296x-927 LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-MEASUREMENT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-entry-table-materialization-measurement-v0
source_evidence=296x-926
row_kind=measurement
target_front=kilo_leaf_map_get_dynamic_covered_i64

active_backend_consumer=C_ABI_ny_llvmc_generic_method_get_policy
producer_metadata_direct_storage_plan_count=1
producer_metadata_entry_value_tracking_count=3
entry_table_dispatch_reached=1
ny_main_hot_loop_map_helper_call_count=0
post_loop_slot_load_hh_call_count=1
scalar_helper_symbol_present=0
result_value=4000001

cycles_after=6465372
previous_cycles_before_approx=480000000
winner_claim=0
hako_vs_c_claim=0

new_runtime_helper_enabled=0
product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
helper_name_inference=0
benchmark_name_branch=0

process_policy_update=lean_guard_policy_applied
dedicated_measurement_guard_added=0
next_task=MAP-HASH-OWNER-REFRESH-AFTER-ENTRY-TABLE-MATERIALIZATION-001
summary=ok
```

## Measurement Commands

```bash
bash tools/perf/build_perf_release.sh

target/release/hakorune --emit-mir-json \
  /tmp/kilo_leaf_map_get_dynamic_covered_i64.entrytable.mir.json \
  benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako

target/release/ny-llvmc \
  --in /tmp/kilo_leaf_map_get_dynamic_covered_i64.entrytable.mir.json \
  --out /tmp/kilo_leaf_map_get_dynamic_covered_i64.entrytable.exe \
  --emit exe \
  --nyrt target/release

nm -C /tmp/kilo_leaf_map_get_dynamic_covered_i64.entrytable.exe | \
  rg 'ny_main|nyash\.map\.(slot_load_hh|scalar_load_hi|local_i64_get_hi)'

objdump -d --demangle \
  /tmp/kilo_leaf_map_get_dynamic_covered_i64.entrytable.exe | \
  rg -n 'ny_main|nyash\.map\.slot_load_hh|nyash\.map\.scalar_load_hi|nyash\.map\.local_i64_get_hi'

/tmp/kilo_leaf_map_get_dynamic_covered_i64.entrytable.exe

KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh \
  kilo_leaf_map_get_dynamic_covered_i64 ny_main 1
```

## Result

The active exact-AOT route is the C ABI `ny-llvmc` backend, not the Python LLVM
backend. The entry-table materialization consumer therefore landed in
`lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc`.

The target executable shows the intended backend-local result:

```text
ny_main hot loop map helper calls: 0
post-loop fallback call: nyash.map.slot_load_hh
Result: 4000001
cycles after: ~6.46M
```

This is a strong target-front reachability result for entry-table materialized
local i64 map reads. It is not a Hako-vs-C winner claim because this row does
not re-baseline a matched C map workload.

## Process Note

This row intentionally has no dedicated shell guard. It is measurement evidence,
not a stable regression gate. The stable executable behavior is already covered
by the implementation/validation guard family plus the current-state pointer
guard. Future docs-only, shadow, and measurement rows should follow this lean
process unless they become public reusable gates.

## Stop Lines

- no Hako-vs-C winner claim
- no product `MapBox` storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no new runtime helper
- no helper-name or benchmark-name inference
