# 293x-371 METADATA-PROMOTE-002 Typed Static Verify

Status: landed
Date: 2026-05-15

## Decision

`METADATA-PROMOTE-002` adds verifier hardening for the active
`typed_object_plans` and `static_data_plans` metadata rows.

This row promotes existing module-level metadata invariants into verifier-owned
contracts. It does not change MIR JSON shape, typed-object layout shape,
static-data row shape, backend lowering behavior, or runtime behavior.

## Responsibility

New verifier owner:

```text
src/mir/verification/module_metadata.rs
```

The owner checks module-level metadata invariants before backend/CorePlan
consumers may rely on the rows.

## Verified Contracts

`typed_object_plans`:

- each plan has a nonempty unique `box_name`;
- each plan has a nonzero unique `type_id`;
- `layout_kind` is `runtime_slot_object_v0`;
- `field_count` matches the field list;
- field names and slots are unique;
- slots are contiguous from `0..field_count`;
- weak fields remain rejected for backend-active typed-object layout.

`static_data_plans`:

- `source_name` and `symbol` are nonempty and unique;
- element names are limited to supported static-data integer lanes;
- `align` matches the element lane;
- values fit the declared element lane;
- `StaticDataLoad` instructions have matching plan source/symbol, element,
  align, and length;
- current load lowering remains the narrow `u16` route.

## Stop Lines

- Do not make type ids contiguous in this row; existing tests and hand-written
  route fixtures use arbitrary stable ids.
- Do not add new typed-object storage lanes.
- Do not add static-data element load routes beyond the current `u16` load
  path.
- Do not move hako_alloc-specific metadata checks into this generic module
  verifier.

## Evidence

```text
cargo test -q --lib module_metadata
cargo test -q --lib static_data_plan
cargo test -q --lib typed_object_plan
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
