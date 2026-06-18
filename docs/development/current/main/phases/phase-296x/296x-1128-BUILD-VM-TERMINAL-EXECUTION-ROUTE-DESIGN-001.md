Status: Done
Date: 2026-06-18
Scope: design terminal execution behavior when vm-reference is unavailable
Related:
  - docs/development/current/main/phases/phase-296x/296x-1127-BUILD-VM-RUNNER-CALLER-CLASSIFICATION-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-VM-TERMINAL-EXECUTION-ROUTE-DESIGN-001

## Decision

```text
output_contract=build-vm-terminal-execution-route-design-v0

terminal_owner=NyashRunner::execute_mir_module_quiet_exit
terminal_owner_role=vm_reference_terminal
silent_vm_to_aot_fallback=0
silent_aot_to_vm_fallback=0

vm_reference_disabled_terminal_behavior=fail_fast
product_route_requires_explicit_exe_aot=1
core_direct_remains_explicit_opt_in=1

selected_next_task=BUILD-VM-TERMINAL-FAILFAST-SEAM-001
summary=ok
```

`execute_mir_module_quiet_exit` and `execute_mir_module` should remain VM
reference terminals. They should not become hidden AOT launchers. If
`vm-reference` is disabled and a caller still reaches these terminals, the
program should fail fast with a clear diagnostic that names the route requiring
`vm-reference`.

## Route Contract

```text
route=explicit_vm
terminal=vm_reference_terminal
without_vm_reference=fail_fast

route=explicit_vm_hako
terminal=vm_reference_terminal
without_vm_reference=fail_fast

route=mir_plain_execution
terminal=vm_reference_terminal_for_now
without_vm_reference=fail_fast_until_product_exe_route_selected

route=mir_json_file_direct_execution
terminal=core_direct_if_explicit_else_vm_reference_terminal
without_vm_reference=fail_fast_unless_core_direct_explicit

route=selfhost_stage_a_accept
terminal=vm_reference_terminal_for_now
without_vm_reference=fail_fast_until_exe_aot_terminal_is_selected

route=emit_exe
terminal=exe_aot_emit
requires_vm_reference=0

route=emit_mir_json
terminal=json_emit
requires_vm_reference=0
```

## Implementation Shape

```text
next_code_slice=BUILD-VM-TERMINAL-FAILFAST-SEAM-001

add_helper=runner::vm_reference_gate
helper_responsibility=emit_one_clear_error_and_exit_or_return_code

cfg_feature_vm_reference:
  execute_mir_module_quiet_exit uses MirInterpreter
  execute_mir_module uses MirInterpreter after emit-mir-json/emit-exe early exits

cfg_not_feature_vm_reference:
  execute_mir_module_quiet_exit returns failure rc with diagnostic
  execute_mir_module exits with diagnostic after emit-mir-json/emit-exe early exits
```

This first seam only handles the central terminal owner. It does not yet
rewrite REPL, JoinIR runner, JoinIR VM bridge, or keep/vm direct imports.

## Why Fail Fast, Not AOT Fallback

```text
reason_1=backend_selection_mistakes_must_be_visible
reason_2=exe_aot_requires_artifact_and_process_policy_not_owned_by_vm_terminal
reason_3=core_direct_is_explicit_experimental_route
reason_4=VM_retirement_should_not_change_execution_semantics_silently
```

## Stop Lines

```text
do_not_implement_hidden_aot_terminal=1
do_not_run_ny_llvmc_from_execute_mir_module_quiet_exit=1
do_not_change_emit_exe_or_emit_mir_json_routes=1
do_not_gate_vmvalue_vmerror=1
do_not_rewrite_repl_or_joinir_in_terminal_seam_row=1
```

