# phase29x / derust

De-rust route and lane contract gates for phase29x.

## Layout

- `phase29x_derust_route_dualrun_vm.sh`: Rust/.hako route selection parity
- `phase29x_derust_verifier_vm.sh`: verifier mismatch fail-fast
- `phase29x_derust_safety_vm.sh`: lifecycle safety fail-fast
- `phase29x_derust_strict_default_route_vm.sh`: strict/dev default route cutover
- `phase29x_derust_done_matrix_vm.sh`: replay / done-sync matrix
- `archive/phase29x/derust/phase29x_backend_owner_hako_ll_compare_min.sh`: explicit `.hako ll emitter` compare-owner canary, archived out of the active suite
- `phase29x_backend_owner_daily_ret_const_min.sh`: daily owner flip pin for `ret_const_min_v1`
- `phase29x_backend_owner_daily_bool_phi_branch_min.sh`: daily owner flip pin for `bool_phi_branch_min_v1`
- `phase29x_backend_owner_daily_print_min.sh`: daily owner flip pin for `hello_simple_llvm_native_probe_v1`
- `phase29x_backend_owner_daily_string_length_min.sh`: daily owner flip pin for `string_length_ascii_min_v1`
- `phase29x_backend_owner_daily_string_indexof_min.sh`: daily owner flip pin for `string_indexof_ascii_min_v1`
- `phase29x_backend_owner_daily_indexof_line_min.sh`: daily owner flip pin for `indexof_line_pure_min_v1`
- `phase29x_backend_owner_daily_substring_concat_loop_min.sh`: daily owner flip pin for `substring_concat_loop_pure_min_v1`
- `phase29x_backend_owner_daily_concat3_extern_min.sh`: daily owner flip pin for `string_concat3_extern_min_v1`
- `phase29x_backend_owner_daily_runtime_data_length_min.sh`: daily owner flip pin for `runtime_data_string_length_ascii_min_v1`
- `phase29x_backend_owner_daily_runtime_data_array_length_min.sh`: daily owner flip pin for `runtime_data_array_length_min_v1`
- `phase29x_backend_owner_daily_runtime_data_map_size_min.sh`: daily owner flip pin for `runtime_data_map_size_min_v1`
- `phase29x_backend_owner_daily_runtime_data_array_has_min.sh`: daily owner flip pin for `runtime_data_array_has_missing_min_v1`
- `phase29x_backend_owner_daily_runtime_data_array_get_min.sh`: daily owner flip pin for `runtime_data_array_get_missing_min_v1`
- `phase29x_backend_owner_daily_runtime_data_array_push_min.sh`: daily owner flip pin for `runtime_data_array_push_min_v1`
- `phase29x_backend_owner_daily_runtime_data_map_has_min.sh`: daily owner flip pin for `runtime_data_map_has_missing_min_v1`
- `phase29x_backend_owner_daily_runtime_data_map_get_min.sh`: daily owner flip pin for `runtime_data_map_get_missing_min_v1`

## Contract

- The family keeps the de-rust route skeleton and its fail-fast evidence pins together.
- The explicit backend-owner compare lane is archive-suite only (`phase29x-derust-archive.txt`); it is a temporary owner-cutover bridge, not an app benchmark, and its archive-home script lives under `archive/phase29x/derust/`.
- Narrow daily owner flips for boundary-only shapes live here so daily evidence stays in the active suite while compare proof is archived.
- The legacy boundary locks for those flipped shapes now live in `phase29ck-boundary-legacy.txt`, not in the default `phase29ck-boundary` suite.
- The scripts are evidence pins, not daily discovery entries.
- Keep the family separate from `vm_hako/` and from the remaining `phase29x` residual buckets.
