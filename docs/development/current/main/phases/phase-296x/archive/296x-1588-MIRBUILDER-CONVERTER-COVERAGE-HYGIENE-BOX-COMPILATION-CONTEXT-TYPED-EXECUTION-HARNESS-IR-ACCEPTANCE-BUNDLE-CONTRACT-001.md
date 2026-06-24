# 296x-1588 MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-TYPED-EXECUTION-HARNESS-IR-ACCEPTANCE-BUNDLE-CONTRACT-001

Status: landed
Date: 2026-06-22

## Purpose

Define the acceptance bundle contract for the selected
BoxCompilationContext typed execution harness IR shape. The contract stays
consultation-only and names the bundle that a later implementation would
need to satisfy, without opening execution or route changes.

## Scope

```text
BoxCount: one consultation bundle contract
owner: MirBuilder converter coverage hygiene BoxCompilationContext harness
input: typed execution harness IR shape contract
output: acceptance bundle contract
```

## Observed State

```text
selected_slice=BoxCompilationContext_harness
selected_candidate=BoxCompilationContext
typed_harness_ir_contract=present
typed_harness_ir_shape_contract=present
box_compilation_context_main_lines=1
route_selection_opened=0
nightly_rustc_adapter_opened=0
runtime_fallback_opened=0
implementation_opened=0
```

## Bundle

The acceptance bundle must remain a consultation-only descriptor of:

- the raw harness carrier that remains in place until a later slice
- the typed harness payload the later implementation would carry
- the validation boundary for the consultation harness
- the summary artifact that records the consultation result

The bundle must not encode:

- builder behavior
- emitter behavior
- route selection
- nightly rustc facts
- runtime fallback
- any new family selection

## Deferred Work

Keep these work items deferred until a later implementation slice:

- acceptance bundle execution wiring
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
the BoxCompilationContext acceptance bundle is explicit
the contract remains consultation-only
route selection remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
no code change is started by this bundle contract
```

## Stop Line

```text
do_not_open_route_selection=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_start_implementation_changes=1
do_not_add_new_family_selection=1
```
