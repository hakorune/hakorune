# 296x-1602 MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-VARIABLE-CONTEXT-IMMUTABLE-BORROW-RETURNSOURCE-CONTRACT-DECISION-001

Status: landed
Date: 2026-06-22

## Purpose

Decide the contract for the selected VariableContext immutable-borrow
surface. The contract stays consultation-only and names the decision point
that a later implementation would need, without opening code changes.

## Scope

```text
BoxCount: one consultation decision contract
owner: MirBuilder converter coverage hygiene VariableContext immutable-borrow
input: remaining raw-string debt inventory
output: ReturnSource contract decision
```

## Observed State

```text
selected_slice=VariableContext_immutable_borrow
selected_candidate=VariableContext
typed_harness_ir_contract=present
typed_harness_ir_shape_contract=present
typed_harness_ir_acceptance_bundle_contract=present
typed_harness_ir_acceptance_owner_summary_contract=present
typed_harness_ir_validation_boundary_contract=present
typed_harness_ir_validation_execution_bundle_contract=present
typed_harness_ir_validation_summary_artifact_contract=present
typed_harness_ir_later_implementation_boundary_contract=present
typed_harness_ir_validation_run_summary_contract=present
typed_harness_ir_implementation_start_boundary_contract=present
typed_harness_ir_implementation_entry_contract=present
typed_harness_ir_implementation_touch_set_contract=present
box_compilation_context_main_lines=1
route_selection_opened=0
nightly_rustc_adapter_opened=0
runtime_fallback_opened=0
implementation_opened=0
```

## Decision

The immutable-borrow surface must not keep the raw `ReturnSource` contract
as-is. The contract decision is:

- replace the raw returned borrow surface with an owned snapshot contract
- keep the simple-map and snapshot/restore surfaces as the neighboring
  references for the rewrite
- do not add a read-view layer in this decision slice
- do not open route selection, nightly rustc adapter, or runtime fallback

## Deferred Work

Keep these work items deferred until a later implementation slice:

- implementation behavior changes
- builder rendering of typed harness payload
- emitter consumption of typed harness payload
- any change to `mirbuilder_family_artifacts.py`

## Required Checks

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Acceptance

```text
the ReturnSource contract decision is explicit
the raw immutable-borrow alias contract is not preserved
route selection remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
no code change is started by this decision contract
```

## Stop Line

```text
do_not_open_route_selection=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_start_implementation_changes=1
do_not_add_new_family_selection=1
```
