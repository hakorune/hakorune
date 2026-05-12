# 293x-212: C204a Array Record Storage Descriptors

Status: Complete

## Purpose

Derive metadata-only ArrayBox packed-record storage descriptors from record
layout plans.

This is the first C204 slice. It maps `record_layout_plans` to
`array_record_storage_plans`, describing the future columnar shape without
mutating `ArrayBox` runtime storage.

## Descriptor Lane

`array_record_storage_plans` carries:

- record name
- record layout id
- storage kind `inline_record_columns_v0`
- field count
- column index per field
- storage class per column

## Stop Line

C204a does not add:

- `ArrayStorage::InlineRecord`
- public `ArrayBox` behavior changes
- runtime promotion/materialization logic
- compiler auto-use of packed record arrays
- record construction/read lowering
- hako_alloc metadata migration
- backend or `.inc` matchers

## Acceptance

- MIR derives `array_record_storage_plans` from `record_layout_plans`.
- JSON v0 bridge and semantic refresh populate the descriptor lane.
- MIR JSON emits `array_record_storage_plans`.
- Runtime `src/boxes/array/**` is not changed in this row.
- No descriptor matcher leaks into `.inc` or backend lowering.
