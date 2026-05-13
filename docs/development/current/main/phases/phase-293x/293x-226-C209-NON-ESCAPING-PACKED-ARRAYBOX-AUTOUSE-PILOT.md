# 293x-226 C209 Non-Escaping Packed ArrayBox Auto-Use Pilot

Status: Complete

## Purpose

C209 is the first packed `ArrayBox` auto-use pilot row. It consumes C207
eligibility and C208 materialization-boundary metadata, but opens only the
non-escaping integer-lane direct-read shape.

## Decision

Decision: accepted.

Add metadata-only `array_record_packed_autouse_pilot_plans` and a private
runtime seam for compiler-selected inline-record integer columns.

Allowed now:

- C209 plan rows for C207-eligible + C208 non-escaping boundary rows
- private runtime inline-record i64 column construction
- private runtime direct i64 column read by `layout_id`, row, and column

Still closed:

- public `ArrayBox.get(i)` record materialization
- returned record elements
- host/backend record escape
- hako_alloc live migration
- backend lowering / `.inc` consumption
- handle / weak / reflection columns
- boxed fallback when materialization is missing

## Row Contract

C209 emits:

```text
pilot_kind = integer_lane_direct_reads_v0
direct_indexed_field_reads_enabled = true
private_runtime_storage_enabled = true
public_array_get_materialization_enabled = false
hako_alloc_migration_enabled = false
backend_lowering_enabled = false
```

The runtime seam is intentionally crate-private and direct:

```text
ArrayBox::new_with_inline_record_i64_columns_for_compiler_autouse(...)
ArrayBox::inline_record_load_i64_column_raw(layout_id, row, column)
```

These methods are not a public `ArrayBox` record API. `ArrayBox.get(i)` still
returns the existing unmaterialized diagnostic for inline-record storage.

## Stop Lines

- Do not make `ArrayInlineRecordProbe` production/public.
- Do not allow record object materialization.
- Do not route hako_alloc metadata stores to packed storage yet.
- Do not add backend lowering or silent boxed fallback.
- Do not rediscover legality in `.inc`, Python backend, or hako_alloc by string
  matching.

## Acceptance

- MIR metadata includes `array_record_packed_autouse_pilot_plans`.
- MIR JSON exposes the new rows.
- The C209 planner consumes only eligible C207 + non-materializing C208 rows.
- Runtime direct i64 column reads work for matching layout/row/column and reject
  mismatched layout, OOB row, OOB column, and ragged construction.
- Public `ArrayBox.get(i)` remains unmaterialized.
- The C209 guard stays local-run / index-listed and is not added to quick/dev
  gates.

## Verification

```bash
bash tools/checks/k2_wide_arraybox_inline_record_autouse_pilot_guard.sh
cargo test -q mir::array_record_packed_autouse_pilot
cargo test -q boxes::array::tests::inline_record_autouse_pilot_reads_i64_columns_without_materializing
```
