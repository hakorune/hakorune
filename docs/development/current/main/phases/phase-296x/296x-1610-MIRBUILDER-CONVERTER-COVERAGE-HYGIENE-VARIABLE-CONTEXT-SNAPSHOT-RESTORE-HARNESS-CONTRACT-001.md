# 296x-1610 MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-VARIABLE-CONTEXT-SNAPSHOT-RESTORE-HARNESS-CONTRACT-001

Status: landed
Date: 2026-06-22

## Purpose

Define the typed harness contract for the selected VariableContext
snapshot/restore surface. The contract stays consultation-only and names
the harness boundary that a later implementation would need, without
opening code changes.

## Scope

```text
BoxCount: one consultation harness contract
owner: MirBuilder converter coverage hygiene VariableContext snapshot/restore
input: implementation touch set contract
output: typed harness contract
```

## Observed State

```text
selected_slice=VariableContext_snapshot_restore
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

## Harness Contract

The VariableContext snapshot/restore harness contract must stay minimal and
explicit:

- the selected harness slice stays VariableContext snapshot/restore only
- the contract does not widen route selection
- the contract does not open the nightly rustc adapter path
- the contract does not open runtime fallback
- the contract does not add new family selection

The harness contract must not encode:

- builder behavior
- emitter behavior
- route selection
- nightly rustc facts
- runtime fallback
- any new family selection

## Deferred Work

Keep these work items deferred until a later implementation slice:

- typed harness payload schema
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
the VariableContext snapshot/restore harness contract is explicit
the contract remains consultation-only
route selection remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
no code change is started by this harness contract
```

## Stop Line

```text
do_not_open_route_selection=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_start_implementation_changes=1
do_not_add_new_family_selection=1
```
