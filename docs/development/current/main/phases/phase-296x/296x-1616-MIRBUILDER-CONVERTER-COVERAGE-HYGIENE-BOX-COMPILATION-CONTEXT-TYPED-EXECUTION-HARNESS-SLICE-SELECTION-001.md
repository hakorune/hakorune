# 296x-1616 MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-BOX-COMPILATION-CONTEXT-TYPED-EXECUTION-HARNESS-SLICE-SELECTION-001

Status: landed
Date: 2026-06-22

## Purpose

Record the review consensus from `glm` and `pro` and select the next
implementation slice without opening any broader crate bundle or converter
framework work.

## Scope

```text
BoxCount: one consultation selection contract
owner: MirBuilder converter coverage hygiene BoxCompilationContext slice
input: glm/pro review notes
output: next implementation slice selection
```

## Review Summary

```text
glm review:
  converter authority is split between apps/ and tools/
  per-family generators are a hardcode hiding place

pro review:
  do not open partial crate bundle or crate linker work yet
  keep new family selection, route selection, and guard files parked
  narrow the next slice to BoxCompilationContext only
```

## Selection

The next implementation slice is:

```text
Implement typed BoxCompilationContext execution harness
```

The slice is intentionally narrow:

```text
main_lines -> typed operations
BoxCompilationContext only
Main only
```

The slice does not open:

```text
partial crate bundle
crate linker
new family selection
new route selection
new guard files
BindingContext integration
VariableContext integration
CarrierInfo integration
CoreContext integration
TypeContext integration
MetadataContext integration
```

## Deferred Work

Keep these parked until a later review says otherwise:

```text
FamilyArtifactSpec expansion beyond Main payload selection
crate surface facts
crate linker / symbol merge policy
partial crate bundle
cross-family integration
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
the next slice is explicit and BoxCompilationContext-only
partial crate bundle remains parked
crate linker remains parked
no new route, guard, or family selection is opened
the review consensus is recorded in the inventory note
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
