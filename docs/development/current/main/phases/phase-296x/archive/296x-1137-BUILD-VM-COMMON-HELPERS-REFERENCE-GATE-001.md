Status: Done
Date: 2026-06-18
Scope: gate keep/vm common helpers while preserving emit routes
Related:
  - docs/development/current/main/phases/phase-296x/296x-1136-BUILD-VM-DIRECT-CALLER-GATE-SELECTION-004.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-VM-COMMON-HELPERS-REFERENCE-GATE-001

## Result

```text
output_contract=build-vm-common-helpers-reference-gate-v0

selected_family=runner_common_vm_helpers_reference_gate
vm_user_factory_mirinterpreter_import_cfg_split=1
vm_execution_mirinterpreter_import_cfg_split=1
vm_execution_no_feature_failfast=1
emit_mir_json_early_exit_preserved=1
emit_exe_early_exit_preserved=1
hidden_aot_fallback_added=0

cargo_check_default_green=1
no_default_features_vm_error_count_after=0
no_default_features_check_green=0
no_default_features_remaining_owner=plugins_disabled_stub_surface

selected_next_task=BUILD-VM-REFERENCE-GATE-CLOSEOUT-001
summary=ok
```

The final VM direct import family is now gated. `VmUserFactoryState` remains
available without `vm-reference`, while the method that registers declarations
into `MirInterpreter` is gated. `run_vm_compiled_module` preserves MIR JSON and
EXE emit early exits, then fail-fasts if execution reaches the VM terminal
without `vm-reference`.

## Remaining no-default Failure

```text
remaining_no_default_failure_is_vm=0
remaining_no_default_failure=plugins_disabled_stub_surface
```

`cargo check --no-default-features` still fails because disabling default
features also disables `plugins`, exposing unrelated plugin-loader stub API
gaps. That is not part of the VM retirement row.

## Stop Lines

```text
do_not_remove_vm_reference_from_default=1
do_not_fix_plugin_stub_errors_in_vm_row=1
do_not_add_hidden_aot_fallback=1
```

