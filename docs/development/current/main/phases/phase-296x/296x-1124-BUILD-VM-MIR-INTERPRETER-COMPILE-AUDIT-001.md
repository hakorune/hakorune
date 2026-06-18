Status: Done
Date: 2026-06-18
Scope: audit the default-compiled MIR interpreter surface before any VM gate or deletion
Related:
  - docs/development/current/main/phases/phase-296x/296x-1123-BUILD-BACKEND-AOT-PASSIVE-CLOSEOUT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-VM-MIR-INTERPRETER-COMPILE-AUDIT-001

## Result

```text
output_contract=build-vm-mir-interpreter-compile-audit-v0

mir_interpreter_default_compiled=1
mir_interpreter_file_count=66
mir_interpreter_lines=12944

vm_product_route_retired=1
vm_semantic_reference_subset_alive=1
vm_types_live_outside_interpreter=1
vmvalue_vmerror_delete_allowed=0
immediate_mir_interpreter_delete_selected=0
immediate_mir_interpreter_feature_gate_selected=0
behavior_changed=0

summary=ok
```

`src/backend/mir_interpreter` is a real default-compiled backend surface, but it
is not safe to delete or feature-gate in the same row as the audit. The VM
product route is retired for app/selfhost validation, while the Rust VM still
has a semantic-reference subset and many tests still instantiate `backend::VM`.

## Dependency Inventory

```text
runner_dispatch_mirinterpreter_dependency=1
runner_bench_vm_dependency=1
runner_repl_mirinterpreter_dependency=1
join_ir_runner_mirinterpreter_dependency=1
join_ir_vm_bridge_mirinterpreter_dependency=1
json_v0_bridge_vm_tests_dependency=1
src_tests_vm_dependency=1
external_tests_vm_dependency=1
```

The observed default callers include:

```text
src/runner/dispatch.rs
src/runner/modes/bench.rs
src/runner/modes/common_util/vm_execution.rs
src/runner/modes/mir_interpreter.rs
src/runner/repl/repl_runner.rs
src/mir/join_ir_runner/*
src/mir/join_ir_vm_bridge/*
src/tests/*
tests/*
```

## VM Types Boundary

`VMValue` and `VMError` are already separate from the interpreter implementation
in `src/backend/vm_types.rs` and must remain available even if the interpreter
is later gated.

```text
vm_types_owner=src/backend/vm_types.rs
host_api_vmvalue_dependency=1
join_ir_ops_vmvalue_dependency=1
join_ir_vm_bridge_vmerror_dependency=1
runtime_type_tag_vmvalue_dependency=1
runtime_type_spec_vmvalue_dependency=1
abi_util_vmvalue_dependency=1
gc_helpers_vmvalue_dependency=1
```

Representative live non-interpreter consumers:

```text
src/runtime/host_api/common.rs
src/runtime/host_api/host_array_ops.rs
src/runtime/host_api/host_box_ops.rs
src/runtime/host_api/host_string_ops.rs
src/mir/join_ir_ops.rs
src/mir/join_ir_vm_bridge/mod.rs
src/backend/abi_util.rs
src/backend/gc_helpers.rs
src/backend/runtime_type_spec.rs
src/backend/runtime_type_tag.rs
```

## Decision

```text
selected_next_task=BUILD-VM-MIR-INTERPRETER-FEATURE-GATE-DESIGN-001
reason=mir_interpreter_is_default_compiled_but_has_live_reference_callers

vm_types_always_available=1
mir_interpreter_gate_requires_runner_classification=1
mir_interpreter_gate_requires_test_classification=1
backend_vm_alias_compat_requires_design=1
```

The next row should design the feature boundary before editing code. It must
preserve `VMValue` / `VMError` as always-available types and classify runner,
REPL, JoinIR, JSON-v0, and test callers into either EXE/AOT route, semantic
reference VM route, or archive/retire route.

## Stop Lines

```text
do_not_delete_vm_types=1
do_not_gate_vm_types_with_mir_interpreter=1
do_not_remove_backend_vm_alias_without_compat_plan=1
do_not_delete_mir_interpreter_in_audit_row=1
do_not_treat_vm_product_route_retirement_as_vm_type_retirement=1
do_not_hide_aot_gap_with_vm_fallback=1
```
