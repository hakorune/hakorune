# phase21_5 / perf / kilo

Kilo crosslang and route-hotspot family.

## Contains

- `phase21_5_perf_kilo_aot_safepoint_toggle_contract_vm.sh`
- `phase21_5_perf_kilo_kernel_crosslang_contract_vm.sh`
- `phase21_5_perf_kilo_micro_machine_lane_contract_vm.sh`
- `phase21_5_perf_kilo_parity_lock_contract_vm.sh`
- `phase21_5_perf_kilo_result_parity_contract_vm.sh`
- `phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh`
- `phase21_5_perf_kilo_text_concat_contract_vm.sh`

## Contract

- The smoke set keeps the kilo kernel compare path and the string/runtime-data route pins together.
- The route-lock pins for `bench_kilo_kernel_small.hako` use the `direct` emit route as the canonical MIR source owner.
- `hako-helper` / `hako-mainline` Stage1 emit stays outside this family while it remains a reduced/bootstrap route for this benchmark.
- The benchmark driver remains `tools/perf/bench_compare_c_py_vs_hako.sh`.
- The curated daily suite excludes `phase21_5_perf_kilo_aot_safepoint_toggle_contract_vm.sh` and `phase21_5_perf_kilo_parity_lock_contract_vm.sh`; keep them as optional stop-line probes / perf gate pins.

## Shared Helpers

- `../../../../../lib/test_runner.sh`
- `../../../../../lib/perf_crosslang_contract.sh`
