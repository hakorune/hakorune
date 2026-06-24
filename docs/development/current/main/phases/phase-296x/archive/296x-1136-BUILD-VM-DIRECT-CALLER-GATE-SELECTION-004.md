Status: Done
Date: 2026-06-18
Scope: select the remaining keep/vm common helper gate family
Related:
  - docs/development/current/main/phases/phase-296x/296x-1135-BUILD-VM-JOINIR-BRIDGE-REFERENCE-GATE-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-VM-DIRECT-CALLER-GATE-SELECTION-004

## Decision

```text
output_contract=build-vm-direct-caller-gate-selection-004-v0

selection_only=1
selected_family=runner_common_vm_helpers_reference_gate
selected_next_task=BUILD-VM-COMMON-HELPERS-REFERENCE-GATE-001

reason=last_remaining_vm_direct_import_family_and_owned_by_keep_vm_routes
default_off_claim=0
summary=ok
```

## Scope

```text
target=src/runner/modes/common_util/vm_execution.rs
target=src/runner/modes/common_util/vm_user_factory.rs

allowed=cfg_split_run_vm_compiled_module
allowed=keep_VmUserFactoryState_available_without_MirInterpreter
allowed=preserve_emit_mir_json_and_emit_exe_early_exits

forbidden=remove_vm_reference_from_default
forbidden=change_keep_vm_route_selection
forbidden=fix_non_vm_plugin_stub_errors
```

## Stop Lines

```text
do_not_remove_vm_reference_from_default=1
do_not_delete_keep_vm_routes=1
do_not_add_hidden_aot_fallback=1
do_not_fix_plugin_stub_errors_in_vm_row=1
```

