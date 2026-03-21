# phase21_5 / perf / apps

Apps wallclock family. This bundle is the remaining `phase21_5/perf/apps` split after `chip8/`, `kilo/`, `entry_mode/`, and `mir_mode/` landed.

## Active Split

- `entry_mode/`
  - app entry-mode compare family
  - 5 smokes
- `mir_mode/`
  - app MIR input-mode compare family
  - 5 smokes
- singleton slices still in the bundle root:
  - `phase21_5_perf_apps_case_breakdown_contract_vm.sh`
  - `phase21_5_perf_apps_compile_run_split_contract_vm.sh`
  - `phase21_5_perf_apps_crosslang_bundle_contract_vm.sh`
  - `phase21_5_perf_apps_emit_mir_jsonfile_route_contract_vm.sh`
  - `phase21_5_perf_apps_startup_subtract_contract_vm.sh`

## Migration Note

- The remaining `phase21_5_perf_*` scripts still live under `tools/smokes/v2/profiles/integration/apps/`.
- Keep new `phase21_5_perf` work under this family tree; do not add more `phase21_5_perf_*` files to `apps/`.
- After `entry_mode/` and `mir_mode/`, the next live family inside this bundle is the remaining singleton app/perf slices listed above.
