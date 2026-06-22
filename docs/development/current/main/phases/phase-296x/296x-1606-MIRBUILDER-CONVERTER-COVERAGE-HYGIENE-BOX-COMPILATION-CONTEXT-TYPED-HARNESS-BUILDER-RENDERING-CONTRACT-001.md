# 296x-1606 MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-TYPED-HARNESS-BUILDER-RENDERING-CONTRACT-001

Status: landed
Date: 2026-06-22

## Purpose

Define the builder rendering contract for the selected BoxCompilationContext
typed harness payload schema. The contract stays consultation-only and names
the rendering boundary that a later implementation would need, without
opening code changes.

## Scope

```text
BoxCount: one consultation builder-rendering contract
owner: MirBuilder converter coverage hygiene BoxCompilationContext harness
input: typed harness payload schema contract
output: builder rendering contract
```

## Observed State

```text
selected_slice=BoxCompilationContext_harness
selected_candidate=BoxCompilationContext
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
typed_harness_ir_implementation_wiring_contract=present
typed_harness_ir_implementation_patch_sequence_contract=present
typed_harness_ir_first_representative_probe_contract=present
typed_harness_payload_schema_contract=present
box_compilation_context_main_lines=1
route_selection_opened=0
nightly_rustc_adapter_opened=0
runtime_fallback_opened=0
implementation_opened=0
```

## Rendering Contract

The builder rendering contract must stay minimal and explicit:

- the builder stays BoxCompilationContext only
- the builder consumes the typed harness payload schema for the selected
  slice
- the builder does not widen route selection
- the builder does not open the nightly rustc adapter path
- the builder does not open runtime fallback
- the builder does not add new family selection

The builder contract must not encode:

- emitter behavior
- route selection
- nightly rustc facts
- runtime fallback
- any new family selection

## Deferred Work

Keep these work items deferred until a later implementation slice:

- rendering field semantics
- emitter consumption of rendered payload
- any change to `mirbuilder_family_artifacts.py`

## Required Checks

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Acceptance

```text
the builder rendering contract is explicit
the contract remains consultation-only
route selection remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
no code change is started by this builder contract
```

## Stop Line

```text
do_not_open_route_selection=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_start_implementation_changes=1
do_not_add_new_family_selection=1
```
