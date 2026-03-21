# phase29x / observability

Route observability and strict/default priority gates for phase29x.

## Layout

- `phase29x_vm_route_observability_vm.sh`: route trace and lane selection observability
- `phase29x_vm_route_strict_dev_priority_vm.sh`: strict/dev priority and compat fallback evidence
- `phase29x_vm_route_compat_bypass_guard_vm.sh`: compat bypass guard
- `phase29x_vm_route_pin_guard_vm.sh`: route pin allowlist guard

## Contract

- The family keeps the remaining VM route observability pins together after `derust`.
- The scripts are evidence pins, not daily discovery entries.
- Keep the family separate from `vm_hako/` and from the remaining `phase29x` residual buckets.
