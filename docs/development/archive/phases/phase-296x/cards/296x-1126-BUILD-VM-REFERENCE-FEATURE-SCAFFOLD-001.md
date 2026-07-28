Status: Done
Date: 2026-06-18
Scope: add the default-on vm-reference feature seam without changing default behavior
Related:
  - docs/development/current/main/phases/phase-296x/296x-1125-BUILD-VM-MIR-INTERPRETER-FEATURE-GATE-DESIGN-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-VM-REFERENCE-FEATURE-SCAFFOLD-001

## Result

```text
output_contract=build-vm-reference-feature-scaffold-v0

feature_name=vm-reference
feature_added=1
feature_in_default=1
default_behavior_changed=0

vm_types_feature_gated=0
mir_interpreter_module_feature_gated=1
backend_mirinterpreter_export_feature_gated=1
backend_vm_alias_feature_gated=1
backend_vm_value_error_always_available=1

default_off_claim=0
no_default_features_check_green=0
no_default_features_vm_caller_errors_visible=1
no_default_features_non_vm_plugin_stub_errors_visible=1
selected_next_task=BUILD-VM-RUNNER-CALLER-CLASSIFICATION-001
summary=ok
```

This row creates the structural feature seam only. It intentionally keeps
`vm-reference` in the default feature set so existing runner and test callers
continue to compile until they are classified.

## Code Boundary

```text
always_available:
  backend::VMValue
  backend::VMError
  backend::vm::VMValue
  backend::vm::VMError

cfg_feature_vm_reference:
  backend::mir_interpreter
  backend::MirInterpreter
  backend::NyashVm
  backend::VM
  backend::vm::VM
```

## Next

```text
selected_next_task=BUILD-VM-RUNNER-CALLER-CLASSIFICATION-001
reason=feature_exists_but_runner_modes_still_select_VM_paths_by_default
```

The next row should not remove `vm-reference` from default. It should classify
runner, REPL, JoinIR bridge, and JSON-v0 bridge callers into:

```text
exe_aot_product_route
vm_reference_route
archive_or_retire
```

## Verification

```text
cargo_check_default_green=1
current_state_pointer_guard_green=1
cargo_check_no_default_features_green=0
```

`cargo check --no-default-features` is intentionally not a success gate for
this row. It proves the next classification work is still needed. The first VM
errors are direct imports from JoinIR runner, JoinIR VM bridge, runner dispatch,
common VM execution helpers, and REPL. The same command also exposes unrelated
non-VM plugin-stub errors when default plugins are disabled, so it is not a
pure VM gate yet.

## Stop Lines

```text
do_not_remove_vm_reference_from_default=1
do_not_gate_vm_types=1
do_not_migrate_runner_routes_in_scaffold_row=1
do_not_delete_vm_tests_in_scaffold_row=1
```
