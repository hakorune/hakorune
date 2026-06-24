# 296x-1578 MIRBUILDER-TYPED-HARNESS-REWRITE-FIRST-SLICE-SELECTION-001

Status: landed
Date: 2026-06-22

## Purpose

Select the first typed harness rewrite slice from the remaining MirBuilder
coverage hygiene inventory.

## Scope

```text
BoxCount: one consultation selection
owner: MirBuilder typed harness rewrite first slice
input: converter coverage hygiene inventory
output: durable first-slice selection
```

## Observed State

```text
binding_context_main_lines=1
variable_context_simple_map_main_lines=1
box_compilation_context_main_lines=1
variable_context_snapshot_restore_main_lines=1
carrier_snapshot_main_lines=2
return_source_sites=1
route_selection_opened=0
nightly_rustc_adapter_opened=0
runtime_fallback_opened=0
```

## Required Checks

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Acceptance

```text
the first typed harness rewrite slice is explicitly selected
the selected slice is the BindingContext and VariableContext simple-map harness family
the typed converter core remains untouched
route selection remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
```

## Stop Line

```text
do_not_select_route=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_start_typed_harness_implementation=1
```
