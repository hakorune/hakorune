# 296x-1587 MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-TYPED-EXECUTION-HARNESS-IR-SHAPE-CONTRACT-001

Status: landed
Date: 2026-06-22

## Purpose

Define the typed execution harness IR shape for the selected
BoxCompilationContext harness slice. The contract stays consultation-only:
it names the data shape that a later implementation would carry, but it does
not open builder rendering, emitter consumption, or any route changes.

## Scope

```text
BoxCount: one consultation shape contract
owner: MirBuilder converter coverage hygiene BoxCompilationContext harness
input: typed execution harness IR contract
output: typed execution harness IR shape contract
```

## Observed State

```text
selected_slice=BoxCompilationContext_harness
selected_candidate=BoxCompilationContext
typed_harness_ir_contract=present
box_compilation_context_main_lines=1
route_selection_opened=0
nightly_rustc_adapter_opened=0
runtime_fallback_opened=0
implementation_opened=0
```

## Shape

The typed execution harness IR shape must stay minimal and explicit:

- a family identifier for BoxCompilationContext
- a typed harness payload for the selected slice
- a contract for the raw harness carrier that remains present until a later slice
- an acceptance bundle descriptor for the consultation harness

The shape must not encode:

- builder behavior
- emitter behavior
- route selection
- nightly rustc facts
- runtime fallback
- any new family selection

## Deferred Work

Keep these work items deferred until a later implementation slice:

- typed harness payload schema details
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
the BoxCompilationContext typed harness IR shape is explicit
the contract remains consultation-only
route selection remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
no code change is started by this shape contract
```

## Stop Line

```text
do_not_open_route_selection=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_start_implementation_changes=1
do_not_add_new_family_selection=1
```
