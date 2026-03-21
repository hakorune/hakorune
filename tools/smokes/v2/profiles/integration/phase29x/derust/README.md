# phase29x / derust

De-rust route and lane contract gates for phase29x.

## Layout

- `phase29x_derust_route_dualrun_vm.sh`: Rust/.hako route selection parity
- `phase29x_derust_verifier_vm.sh`: verifier mismatch fail-fast
- `phase29x_derust_safety_vm.sh`: lifecycle safety fail-fast
- `phase29x_derust_strict_default_route_vm.sh`: strict/dev default route cutover
- `phase29x_derust_done_matrix_vm.sh`: replay / done-sync matrix

## Contract

- The family keeps the de-rust route skeleton and its fail-fast evidence pins together.
- The scripts are evidence pins, not daily discovery entries.
- Keep the family separate from `vm_hako/` and from the remaining `phase29x` residual buckets.
