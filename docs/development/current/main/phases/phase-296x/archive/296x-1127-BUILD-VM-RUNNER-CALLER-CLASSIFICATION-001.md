Status: Done
Date: 2026-06-18
Scope: classify live runner VM callers before removing vm-reference from default
Related:
  - docs/development/current/main/phases/phase-296x/296x-1126-BUILD-VM-REFERENCE-FEATURE-SCAFFOLD-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-VM-RUNNER-CALLER-CLASSIFICATION-001

## Result

```text
output_contract=build-vm-runner-caller-classification-v0

classification_only=1
code_behavior_changed=0
vm_reference_default_changed=0

terminal_vm_execution_owner=NyashRunner::execute_mir_module_quiet_exit
terminal_vm_execution_owner_fan_in=high
immediate_default_off_selected=0

selected_next_task=BUILD-VM-TERMINAL-EXECUTION-ROUTE-DESIGN-001
summary=ok
```

The active blocker is not a single import. The core issue is that many product
or bridge routes still converge on `NyashRunner::execute_mir_module_quiet_exit`
or `NyashRunner::execute_mir_module`, both of which instantiate
`MirInterpreter`.

## Caller Classification

### VM Reference Routes

These remain `vm-reference` routes until intentionally retired or migrated:

```text
route=explicit_backend_vm
owner=src/runner/dispatch.rs backend=vm
classification=vm_reference_route

route=explicit_backend_vm_hako
owner=src/runner/dispatch.rs backend=vm-hako
classification=vm_reference_route

route=runner_keep_vm
owner=src/runner/keep/vm.rs
classification=vm_reference_route

route=runner_keep_vm_fallback
owner=src/runner/keep/vm_fallback.rs
classification=vm_reference_route

route=repl_eval
owner=src/runner/repl/repl_runner.rs
classification=vm_reference_route

route=joinir_runner
owner=src/mir/join_ir_runner/*
classification=vm_reference_route

route=joinir_vm_bridge
owner=src/mir/join_ir_vm_bridge/*
classification=vm_reference_route
```

### Product / Bridge Routes Still Terminating In VM

These routes are not semantically "VM product targets" anymore, but their
current terminal executor is still the VM. They need a separate terminal
execution design before `vm-reference` can leave default.

```text
route=backend_mir_plain_execution
owner=src/runner/modes/mir.rs
current_terminal=execute_mir_module_quiet_exit
target_classification=exe_aot_product_route_or_fail_fast_reference

route=mir_json_file_direct_execution
owner=src/runner/dispatch.rs
current_terminal=execute_mir_module_quiet_exit
target_classification=exe_aot_product_route_or_core_direct

route=core_executor_loaded_mir_module
owner=src/runner/core_executor.rs
current_terminal=execute_mir_module_quiet_exit
target_classification=exe_aot_product_route_or_core_direct

route=selfhost_accept_stage_a_mir_module
owner=src/runner/selfhost.rs
current_terminal=execute_mir_module
target_classification=exe_aot_product_route

route=stage1_binary_only_direct_run
owner=src/runner/stage1_bridge/direct_route/mod.rs
current_terminal=execute_mir_module_quiet_exit
target_classification=vm_reference_route_until_keep_route_retire
```

### Compile/Emit Routes Not Requiring VM

These are already safe to keep without the interpreter engine:

```text
route=emit_mir_json
classification=does_not_require_vm_reference

route=emit_exe
classification=does_not_require_vm_reference

route=dump_ast
classification=does_not_require_vm_reference

route=dump_mir_verify_mir
classification=does_not_require_vm_reference_for_emit_or_verify_only
```

## Decision

```text
selected_next_task=BUILD-VM-TERMINAL-EXECUTION-ROUTE-DESIGN-001
reason=product_and_bridge_routes_share_execute_mir_module_quiet_exit_VM_terminal

vm_reference_remove_from_default_allowed=0
runner_cfg_patch_allowed=0
terminal_route_design_required=1
```

The next row should define how already-compiled `MirModule` payloads execute
when `vm-reference` is disabled. It must not silently fall back between VM and
AOT. It should decide whether the terminal route is:

```text
terminal_route=exe_aot_child
terminal_route=core_direct_inproc
terminal_route=fail_fast_requires_vm_reference
```

per caller family.

## Stop Lines

```text
do_not_remove_vm_reference_from_default=1
do_not_cfg_out_execute_mir_module_quiet_exit_before_terminal_design=1
do_not_silently_route_vm_to_aot=1
do_not_silently_route_aot_to_vm=1
do_not_delete_repl_or_joinir_vm_bridge_in_classification_row=1
```

