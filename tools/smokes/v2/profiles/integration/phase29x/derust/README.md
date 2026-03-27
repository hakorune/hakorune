# phase29x / derust

De-rust route and lane contract gates for phase29x.

## Layout

- `phase29x_derust_route_dualrun_vm.sh`: Rust/.hako route selection parity
- `phase29x_derust_verifier_vm.sh`: verifier mismatch fail-fast
- `phase29x_derust_safety_vm.sh`: lifecycle safety fail-fast
- `phase29x_derust_strict_default_route_vm.sh`: strict/dev default route cutover
- `phase29x_derust_done_matrix_vm.sh`: replay / done-sync matrix
- `phase29x_backend_owner_hako_ll_compare_min.sh`: explicit `.hako ll emitter` compare-owner canary
- `phase29x_backend_owner_daily_ret_const_min.sh`: daily owner flip pin for `ret_const_min_v1`
- `phase29x_backend_owner_daily_bool_phi_branch_min.sh`: daily owner flip pin for `bool_phi_branch_min_v1`
- `phase29x_backend_owner_daily_print_min.sh`: daily owner flip pin for `hello_simple_llvm_native_probe_v1`
- `phase29x_backend_owner_daily_string_length_min.sh`: daily owner flip pin for `string_length_ascii_min_v1`
- `phase29x_backend_owner_daily_string_indexof_min.sh`: daily owner flip pin for `string_indexof_ascii_min_v1`
- `phase29x_backend_owner_daily_concat3_extern_min.sh`: daily owner flip pin for `string_concat3_extern_min_v1`

## Contract

- The family keeps the de-rust route skeleton and its fail-fast evidence pins together.
- The explicit backend-owner compare lane also lives here because it is a temporary owner-cutover bridge, not an app benchmark.
- Narrow daily owner flips for boundary-only shapes also live here so compare and daily evidence stay side-by-side.
- The legacy boundary locks for those six flipped shapes now live in `phase29ck-boundary-legacy.txt`, not in the default `phase29ck-boundary` suite.
- The scripts are evidence pins, not daily discovery entries.
- Keep the family separate from `vm_hako/` and from the remaining `phase29x` residual buckets.
