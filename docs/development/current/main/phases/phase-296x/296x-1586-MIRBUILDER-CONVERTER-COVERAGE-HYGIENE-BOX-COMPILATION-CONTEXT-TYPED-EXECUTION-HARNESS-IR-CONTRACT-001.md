# 296x-1586 MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-TYPED-EXECUTION-HARNESS-IR-CONTRACT-001

Status: landed
Date: 2026-06-22

## Purpose

Define the typed execution harness IR contract for the selected
BoxCompilationContext harness slice. The raw harness text remains in place for
now, but the next consultation step is to make the intended harness shape
explicit before any implementation attempt.

## Scope

```text
BoxCount: one consultation contract
owner: MirBuilder converter coverage hygiene BoxCompilationContext harness
input: selected BoxCompilationContext raw-string harness slice
output: typed execution harness IR contract
```

## Observed State

```text
selected_slice=BoxCompilationContext_harness
selected_candidate=BoxCompilationContext
box_compilation_context_main_lines=1
box_compilation_context_crate_smoke_docs=present
route_selection_opened=0
nightly_rustc_adapter_opened=0
runtime_fallback_opened=0
implementation_opened=0
```

## Contract

The BoxCompilationContext harness contract must be expressed as typed
execution intent rather than raw `main_lines` text.

This consultation slice must preserve the following boundaries:

- the selected harness slice stays BoxCompilationContext only
- the typed contract does not widen route selection
- the typed contract does not open the nightly rustc adapter path
- the typed contract does not open runtime fallback
- the typed contract does not start implementation changes
- the typed contract does not add any new family selection

## Deferred Work

Keep these work items deferred until a later implementation slice:

- typed execution harness IR shape
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
the BoxCompilationContext harness contract is explicit
the contract remains consultation-only
route selection remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
no code change is started by this contract
```

## Stop Line

```text
do_not_open_route_selection=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_start_implementation_changes=1
do_not_add_new_family_selection=1
```
