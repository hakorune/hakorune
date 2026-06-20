# 296x-1533 MIRBUILDER-NEXT-FAMILY-LIFECYCLE-FACTS-PILOT-SELECTION-001

Status: open
Date: 2026-06-21

## Purpose

Select exactly one non-VariableContext family for bounded facts extraction,
or keep VariableContext as the only active behavioral family if readiness is
absent.

This is a design consultation row. No route selection or artifact generation
belongs here.

## Selected By

```text
296x-1532-MIRBUILDER-NEXT-FAMILY-LIFECYCLE-READINESS-INVENTORY-001
```

## Observed Readiness

```text
context: skeleton transport only; no lifecycle facts / plan / recipe /
oracle / artifact manifest / route entry found
core_context: skeleton transport only; no lifecycle facts / plan / recipe /
oracle / artifact manifest / route entry found
type_context: skeleton transport only; no lifecycle facts / plan / recipe /
oracle / artifact manifest / route entry found
metadata_context: skeleton transport only; no lifecycle facts / plan / recipe /
oracle / artifact manifest / route entry found
```

## Candidate Decision

```text
keep_variable_context_only
or
select_one_non_variable_context_family
```

## Stop Line

```text
do_not_select_route_in_same_row=1
do_not_generate_new_behavior_in_this_row=1
do_not_open_nightly_rustc_adapter=1
do_not_expand_to_mirbuilder_wide_claim=1
do_not_add_runtime_fallback=1
```
