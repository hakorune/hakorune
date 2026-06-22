# 296x-1617 MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-TYPED-EXECUTION-HARNESS-MAIN-OPERATIONS-DESIGN-001

Status: landed
Date: 2026-06-22

## Purpose

Define the smallest follow-on design slice for the selected
`BoxCompilationContext` execution harness: replace the remaining raw `Main`
payload with typed operations, without opening crate bundling or cross-family
integration.

## Scope

```text
BoxCount: one consultation design contract
owner: MirBuilder converter coverage hygiene BoxCompilationContext Main payload
input: typed BoxCompilationContext execution harness slice selection
output: typed Main operation design
```

## Observed State

```text
selected_slice=BoxCompilationContext_typed_execution_harness
box_compilation_context_main_lines=1
box_compilation_context_main_operations=0
typed_main_op_vocabulary=unselected
partial_crate_bundle_opened=0
crate_linker_opened=0
new_family_selection_opened=0
new_route_selection_opened=0
new_guard_files_opened=0
```

## Design Decision

The remaining BoxCompilationContext gap is the `Main` payload only.

The design must therefore move from:

```text
main_lines
```

to:

```text
main_operations
```

for BoxCompilationContext only.

The typed `Main` operation vocabulary is intentionally minimal:

```text
NewBox
StaticCall
AssertEq
Print
ReturnI64
```

The design must not add specialized one-off operations such as:

```text
AssertStaticCallEq
AssertTypedMainOnly
```

because those would reintroduce family-specific special casing into the
emitter and make the converter harder to keep shared.

## Shape Contract

The BoxCompilationContext `Main` payload should express this flow:

```text
create BoxCompilationContext
call is_empty
assert the result is 1
print the existing success line
return 0
```

The `Main` payload may use `StaticCall` plus `AssertEq` to express the check.
It must not fall back to raw Hako string bodies for this family.

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
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Acceptance

```text
BoxCompilationContext Main is described as typed operations only
main_lines remains the current raw surface until implementation
no crate bundle / linker / new route selection is opened
the typed Main vocabulary is minimal and shared
the next implementation step can be narrowed cleanly
```

## Stop Line

```text
do_not_open_partial_crate_bundle=1
do_not_open_crate_linker=1
do_not_open_new_family_selection=1
do_not_open_new_route_selection=1
do_not_add_new_guard_files=1
do_not_start_implementation_changes=1
```
