# 296x-1585 MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-NEXT-RAW-STRING-SLICE-SELECTION-BOX-COMPILATION-CONTEXT-HARNESS-001

Status: landed
Date: 2026-06-22

## Purpose

Select the next remaining raw-string coverage slice after the inventory-first
review. BoxCompilationContext harness remains the next slice to consult
about, while implementation stays unopened.

## Scope

```text
BoxCount: one consultation selection
owner: MirBuilder converter coverage hygiene next raw-string slice
input: remaining converter coverage hygiene inventory
output: next raw-string slice selection
```

## Observed State

```text
remaining_raw_harness_family_spec_slices=5
raw_ReturnSource_contract_slices=1
first_typed_harness_rewrite_slice=BindingContext_and_VariableContext_simple_map
box_compilation_context_main_lines=1
box_compilation_context_crate_smoke_docs=present
snapshot_restore_main_lines=1
carrier_snapshot_main_lines=2
immutable_borrow_return_source_present=1
route_selection_opened=0
nightly_rustc_adapter_opened=0
runtime_fallback_opened=0
```

## Selection

The next remaining raw-string coverage slice is:

```text
BoxCompilationContext harness
```

This is a consultation-only selection. It does not open route selection, the
nightly rustc adapter path, or any implementation work.

## Deferred Slices

Keep these remaining slices deferred behind this selection:

- VariableContext snapshot/restore harness
- CarrierInfo snapshot harnesses
- VariableContext immutable-borrow ReturnSource contract decision
- BindingContext and VariableContext simple-map is already the first typed
  rewrite slice and stays separate from this remaining raw-string selection

## Boundaries

- inventory-first ordering stays intact
- route selection stays unopened
- nightly rustc adapter stays unopened
- runtime fallback stays unopened
- implementation changes stay unopened
- no family selection is added
- no probe execution is started

## Required Checks

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Acceptance

```text
the next raw-string slice is explicit
BoxCompilationContext harness remains the selected slice
other raw-string slices stay deferred
route selection remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
no code change is started by this selection
```

## Stop Line

```text
do_not_open_route_selection=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_start_implementation_changes=1
do_not_add_new_family_selection=1
```
