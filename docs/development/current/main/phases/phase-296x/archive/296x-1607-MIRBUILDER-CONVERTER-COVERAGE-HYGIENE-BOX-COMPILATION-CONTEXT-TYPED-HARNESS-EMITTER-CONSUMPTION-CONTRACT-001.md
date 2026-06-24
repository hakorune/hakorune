# 296x-1607 MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-TYPED-HARNESS-EMITTER-CONSUMPTION-CONTRACT-001

Status: landed
Date: 2026-06-22

## Purpose

Define the emitter consumption contract for the selected BoxCompilationContext
typed harness payload schema. The contract stays consultation-only and names
the final shared-emitter boundary that a later implementation would need,
without opening code changes.

## Scope

```text
BoxCount: one consultation emitter-consumption contract
owner: MirBuilder converter coverage hygiene BoxCompilationContext harness
input: builder rendering contract
output: emitter consumption contract
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
typed_harness_builder_rendering_contract=present
box_compilation_context_main_lines=1
route_selection_opened=0
nightly_rustc_adapter_opened=0
runtime_fallback_opened=0
implementation_opened=0
```

## Emitter Contract

The emitter consumption contract must stay minimal and explicit:

- the emitter stays BoxCompilationContext only
- the emitter consumes the builder-rendered output for the selected slice
- the emitter does not widen route selection
- the emitter does not open the nightly rustc adapter path
- the emitter does not open runtime fallback
- the emitter does not add new family selection

The emitter contract must not encode:

- route selection
- nightly rustc facts
- runtime fallback
- any new family selection

## Deferred Work

Keep these work items deferred until a later implementation slice:

- emitted output formatting details
- any change to `mirbuilder_family_artifacts.py`
- any change to `shared_mirbuilder_emitter.py`

## Required Checks

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Acceptance

```text
the emitter consumption contract is explicit
the contract remains consultation-only
route selection remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
no code change is started by this emitter contract
```

## Stop Line

```text
do_not_open_route_selection=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_start_implementation_changes=1
do_not_add_new_family_selection=1
```
