Status: Done
Date: 2026-06-18
Scope: gate the isolated REPL VM direct import
Related:
  - docs/development/current/main/phases/phase-296x/296x-1130-BUILD-VM-DIRECT-CALLER-GATE-SELECTION-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-VM-REPL-REFERENCE-GATE-001

## Result

```text
output_contract=build-vm-repl-reference-gate-v0

selected_family=runner_repl_vm_reference_gate
repl_eval_line_cfg_split=1
repl_vm_import_outside_cfg=0
default_behavior_changed=0

cargo_check_default_green=1
no_default_features_check_green=0
no_default_features_vm_error_count_after=5
no_default_features_non_vm_plugin_stub_errors_visible=1

selected_next_task=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-002
summary=ok
```

`ReplRunnerBox::eval_line` now keeps the existing VM execution path under
`vm-reference`. Without `vm-reference`, REPL evaluation returns an explicit
error instead of importing `MirInterpreter`.

## Remaining VM Direct Import Families

```text
remaining_vm_import_family=join_ir_runner
remaining_vm_import_family=join_ir_vm_bridge
remaining_vm_import_family=runner_common_vm_execution
remaining_vm_import_family=runner_common_vm_user_factory
```

## Stop Lines

```text
do_not_remove_vm_reference_from_default=1
do_not_gate_joinir_in_repl_row=1
do_not_gate_keep_vm_common_helpers_in_repl_row=1
do_not_fix_plugin_stub_errors_in_vm_row=1
```

