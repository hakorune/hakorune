# 296x-1618 MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-TYPED-EXECUTION-HARNESS-IMPLEMENTATION-001

Status: landed
Date: 2026-06-22

## Purpose

Record the implementation of the selected BoxCompilationContext typed Main
slice. The family now emits typed operations for its execution harness instead
of raw `main_lines`, while every broader integration step stays parked.

## Scope

```text
BoxCount: one implementation contract
owner: MirBuilder converter coverage hygiene BoxCompilationContext typed Main
input: typed Main operations design
output: implemented BoxCompilationContext typed execution harness
```

## Implementation Summary

```text
BoxCompilationContext main_lines -> main_operations
typed Main vocabulary: NewBox, StaticCall, AssertEq, Print, ReturnI64
other families remain on main_lines
```

The typed `Main` payload now executes this minimal flow:

```text
create BoxCompilationContext
call BoxCompilationContextApi.is_empty
assert the result is 1
print the success line
return 0
```

## Observed State

```text
selected_slice=BoxCompilationContext_typed_execution_harness
main_operations=present
main_lines_carrier=retained_for_other_families
partial_crate_bundle_opened=0
crate_linker_opened=0
new_family_selection_opened=0
new_route_selection_opened=0
new_guard_files_opened=0
runtime_fallback_opened=0
nightly_rustc_adapter_opened=0
```

## Deferred Work

Keep these parked until a later review says otherwise:

```text
partial crate bundle
crate linker
crate surface facts
family-wide typed Main migration
BindingContext integration
VariableContext integration
CarrierInfo integration
CoreContext integration
TypeContext integration
MetadataContext integration
BoxCompilationContext::size_info
ValueId-key ordered map generalization
runtime fallback
nightly rustc adapter
native authority promotion
```

## Required Checks

```text
python3 tools/rust_lifecycle/generate_box_compilation_context_artifact.py --check
bash tools/checks/rust_lifecycle_box_compilation_context_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_box_compilation_context_derived_route_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Acceptance

```text
BoxCompilationContext generated Hako uses typed Main operations
generated Hako parse/MIR/EXE are green
main_lines is not used by BoxCompilationContext
no new route, guard, or family selection is opened
partial crate bundle remains parked
crate linker remains parked
```

## Stop Line

```text
do_not_open_partial_crate_bundle=1
do_not_open_crate_linker=1
do_not_open_new_family_selection=1
do_not_open_new_route_selection=1
do_not_add_new_guard_files=1
do_not_start_new_implementation_changes=1
```
