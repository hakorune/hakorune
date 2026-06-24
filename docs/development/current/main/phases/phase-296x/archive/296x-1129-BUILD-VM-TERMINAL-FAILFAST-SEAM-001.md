Status: Done
Date: 2026-06-18
Scope: add central fail-fast terminal behavior for vm-reference disabled builds
Related:
  - docs/development/current/main/phases/phase-296x/296x-1128-BUILD-VM-TERMINAL-EXECUTION-ROUTE-DESIGN-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-VM-TERMINAL-FAILFAST-SEAM-001

## Result

```text
output_contract=build-vm-terminal-failfast-seam-v0

central_terminal_failfast_added=1
execute_mir_module_quiet_exit_cfg_split=1
execute_mir_module_cfg_split=1
emit_mir_json_early_exit_preserved=1
emit_exe_early_exit_preserved=1
hidden_aot_fallback_added=0

cargo_check_default_green=1
current_state_pointer_guard_green=1
no_default_features_check_green=0
no_default_features_vm_error_count_after=6
no_default_features_non_vm_plugin_stub_errors_visible=1

selected_next_task=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-001
summary=ok
```

The central terminal methods now have a `cfg(not(feature = "vm-reference"))`
branch that fails fast instead of instantiating the interpreter. The default
build remains unchanged because `vm-reference` is still in the default feature
set.

## Preserved Behavior

```text
vm_reference_default_on=1
default_behavior_changed=0
VMValue_VMError_always_available=1
```

`execute_mir_module` still allows MIR JSON and EXE emission before reaching the
VM terminal. This keeps artifact emit routes independent from the interpreter
engine.

## Remaining Default-Off VM Callers

`cargo check --no-default-features` still fails, as expected. The remaining VM
direct import families are:

```text
remaining_vm_import_family=join_ir_runner
remaining_vm_import_family=join_ir_vm_bridge
remaining_vm_import_family=runner_common_vm_execution
remaining_vm_import_family=runner_common_vm_user_factory
remaining_vm_import_family=repl_runner
```

The same no-default command also exposes non-VM plugin-stub errors, so default
off cannot be used as a pure VM gate until those are separated or the check is
run with the appropriate non-VM feature set.

## Decision

```text
selected_next_task=BUILD-VM-DIRECT-CALLER-GATE-SELECTION-001
reason=central_terminal_is_gated_but_direct_import_callers_remain

default_off_claim=0
remove_vm_reference_from_default_allowed=0
```

The next row should select the next direct caller family to gate or retire. It
should not try to fix all direct imports in one patch.

## Stop Lines

```text
do_not_remove_vm_reference_from_default=1
do_not_mix_repl_joinir_keep_vm_gating_in_one_row=1
do_not_fix_plugin_stub_errors_in_vm_row=1
do_not_add_hidden_aot_fallback=1
```

