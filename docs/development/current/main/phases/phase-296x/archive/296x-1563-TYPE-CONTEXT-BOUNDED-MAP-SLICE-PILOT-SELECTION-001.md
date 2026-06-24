# 296x-1563 TYPE-CONTEXT-BOUNDED-MAP-SLICE-PILOT-SELECTION-001

Status: landed
Date: 2026-06-22

## Purpose

Select the next easy-tier family pilot after the CoreContext consultation
rows.

TypeContext is selected only as a bounded map-slice pilot. This row does not
open route selection, behavior generation, or the nightly rustc adapter path.

## Scope

```text
BoxCount: one consultation selection
owner: TypeContext bounded map-slice pilot selection
input: TypeContext readiness inventory and current task-order SSOT
output: one durable pilot selection fixture and guard
```

## Decision

```text
select TypeContext bounded map slice as the next easy-tier family pilot after CoreContext consultation rows
keep the pilot bounded to the map-slice inventory only
do not select route or nightly rustc adapter
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_type_context_bounded_map_slice_pilot_selection.py --check-reference
bash tools/checks/rust_mirbuilder_type_context_bounded_map_slice_pilot_selection_guard.sh
```

## Acceptance

```text
selected_next_pilot=TypeContext
pilot_scope=TypeContext_bounded_map_slice_only
route_selection=0
nightly_rustc_adapter=0
summary=ok
```

## Stop Line

```text
do_not_select_route=1
do_not_open_nightly_rustc_adapter=1
do_not_claim_mirbuilder_wide_conversion=1
do_not_add_runtime_fallback=1
```
