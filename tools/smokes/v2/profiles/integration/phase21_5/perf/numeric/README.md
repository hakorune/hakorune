# phase21_5 / perf / numeric

Numeric mixed-medium perf contract family.

## Contains

- `phase21_5_perf_numeric_arith_cse_contract_vm.sh`
- `phase21_5_perf_numeric_compare_chain_contract_vm.sh`
- `phase21_5_perf_numeric_hot_trace_contract_vm.sh`
- `phase21_5_perf_numeric_mixed_medium_aot_contract_vm.sh`

## Contract

- The smoke set keeps the numeric mixed-medium AOT sentinel, hot-trace schema, compare-chain IR shape, and arithmetic expr-cache reuse pins together.
- The benchmark drivers remain `tools/perf/bench_compare_c_vs_hako.sh` and `tools/ny_mir_builder.sh`.
- This family is self-contained and should stay separate from the app wallclock bundles.

## Shared Helpers

- `../../../../../lib/test_runner.sh`
- `../../../../../lib/perf_hot_trace_contract.sh`
- `../../../../../lib/emit_mir_route.sh`
