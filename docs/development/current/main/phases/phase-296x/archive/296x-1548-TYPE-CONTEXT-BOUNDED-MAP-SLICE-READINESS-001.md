# 296x-1548 TYPE-CONTEXT-BOUNDED-MAP-SLICE-READINESS-001

Status: landed
Date: 2026-06-22

## Purpose

Record the bounded TypeContext map slice as a consultation-only inventory.

TypeContext is broader than the scalar-counter CoreContext slice because it
mixes deterministic maps, a HashMap kind registry, snapshot transfer, and
non-String key/value shapes. This row keeps the current source shape explicit
without opening route selection or Hako lifecycle planning.

## Scope

```text
BoxCount: one consultation inventory
owner: MirBuilder TypeContext bounded map slice
input: current TypeContext source shape
output: one durable readiness inventory and guard
```

## Observed Slice

```text
value_types
value_kinds
value_origin_newbox
string_literals
map_value_types
map_literal_value_types

get_type
set_type
get_kind
set_kind
get_origin_box
set_origin_box
clear_origin_boxes
take_snapshot
restore_snapshot
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_type_context_bounded_map_slice_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_type_context_bounded_map_slice_guard.sh
```

## Acceptance

```text
bounded map slice source shape is fixed in a machine-readable fixture
route selection remains unopened
nightly rustc adapter remains unopened
```

## Stop Line

```text
do_not_select_route=1
do_not_open_nightly_rustc_adapter=1
do_not_claim_mirbuilder_wide_conversion=1
do_not_add_runtime_fallback=1
```
