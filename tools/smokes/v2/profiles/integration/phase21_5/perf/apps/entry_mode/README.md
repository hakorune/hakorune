# phase21_5 / perf / apps / entry_mode

App entry-mode comparison family.

## Contains

- `phase21_5_perf_apps_entry_mode_contract_vm.sh`
- `phase21_5_perf_apps_entry_mode_delta_contract_vm.sh`
- `phase21_5_perf_apps_entry_mode_significance_contract_vm.sh`
- `phase21_5_perf_apps_entry_mode_spread_contract_vm.sh`
- `phase21_5_perf_apps_entry_mode_case_hotspot_contract_vm.sh`

## Contract

- The smoke set keeps the app entry-mode compare path pinned.
- The benchmark driver remains `tools/perf/bench_apps_entry_mode_compare.sh`.

## Shared Helpers

- `../../../../../../lib/test_runner.sh`
- `../../../../../../lib/perf_apps_contract.sh`
