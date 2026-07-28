Status: Done
Date: 2026-06-18
Scope: gate JoinIR VM bridge execution while keeping conversion modules available
Related:
  - docs/development/current/main/phases/phase-296x/296x-1134-BUILD-VM-DIRECT-CALLER-GATE-SELECTION-003.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-VM-JOINIR-BRIDGE-REFERENCE-GATE-001

## Result

```text
output_contract=build-vm-joinir-bridge-reference-gate-v0

selected_family=join_ir_vm_bridge_reference_gate
run_joinir_via_vm_cfg_split=1
bridge_conversion_modules_gated=0
join_ir_bridge_vm_import_outside_cfg=0
default_behavior_changed=0

cargo_check_default_green=1
no_default_features_check_green=0
no_default_features_vm_error_count_after=2
no_default_features_non_vm_plugin_stub_errors_visible=1

selected_next_task=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-004
summary=ok
```

`run_joinir_via_vm` now keeps the existing VM execution path under
`vm-reference`. Without `vm-reference`, it preserves the public signature and
returns an explicit `JoinIrVmBridgeError`. The JoinIR to MIR conversion modules
remain available without the VM engine.

## Remaining VM Direct Import Families

```text
remaining_vm_import_family=runner_common_vm_execution
remaining_vm_import_family=runner_common_vm_user_factory
```

## Stop Lines

```text
do_not_gate_bridge_conversion_modules=1
do_not_gate_keep_vm_common_helpers_in_bridge_row=1
do_not_remove_vm_reference_from_default=1
do_not_claim_no_default_features_green=1
```

