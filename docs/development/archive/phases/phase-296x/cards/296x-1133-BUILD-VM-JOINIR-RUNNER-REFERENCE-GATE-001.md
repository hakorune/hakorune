Status: Done
Date: 2026-06-18
Scope: gate the structure-only JoinIR runner VM direct imports
Related:
  - docs/development/current/main/phases/phase-296x/296x-1132-BUILD-VM-DIRECT-CALLER-GATE-SELECTION-002.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-VM-JOINIR-RUNNER-REFERENCE-GATE-001

## Result

```text
output_contract=build-vm-joinir-runner-reference-gate-v0

selected_family=join_ir_runner_vm_reference_gate
join_ir_runner_api_cfg_split=1
join_ir_runner_exec_cfg_split=1
join_ir_runner_vm_import_outside_cfg=0
default_behavior_changed=0

cargo_check_default_green=1
no_default_features_check_green=0
no_default_features_vm_error_count_after=3
no_default_features_non_vm_plugin_stub_errors_visible=1

selected_next_task=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-003
summary=ok
```

The structure-only `join_ir_runner` API and executor are now compiled only with
`vm-reference`. This does not touch `join_ir_vm_bridge`, which is a separate
semantic Route B family.

## Remaining VM Direct Import Families

```text
remaining_vm_import_family=join_ir_vm_bridge
remaining_vm_import_family=runner_common_vm_execution
remaining_vm_import_family=runner_common_vm_user_factory
```

## Stop Lines

```text
do_not_gate_join_ir_vm_bridge_in_join_ir_runner_row=1
do_not_gate_keep_vm_common_helpers_in_join_ir_runner_row=1
do_not_remove_vm_reference_from_default=1
do_not_claim_no_default_features_green=1
```

