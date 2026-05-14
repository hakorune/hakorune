# 293x-280 REC-002 Stage1 Record Construction/Read Lowering

Status: complete
Date: 2026-05-14

## Scope

Validate explicit record literal construction against record declarations and
lower record field reads through a record-owned Program JSON v0 shape.

## Landed changes

- Added a Stage1 record declaration index to Program JSON v0 lowering.
- Rejected unknown/missing/extra fields for `RecordName { field: expr }`.
- Added declared field index and declared type metadata to lowered record
  literal fields.
- Tracked locals initialized from record literals within a lowering body.
- Lowered tracked `meta.field` reads to `RecordField` with record metadata.
- Added tests for valid construction, missing field rejection, extra field
  rejection, and field-read lowering.
- Added a dedicated guard.

## Non-goals

- No record layout planning.
- No scalar replacement or packed ArrayBox lowering.
- No shorthand field syntax.
- No `with` update semantics.
- No record field write-through.
- No record methods/delegate/interface implementation.

## Guard

```bash
bash tools/checks/k2_wide_record_construction_read_lowering_guard.sh
```

## Next selected row

`REC-003 record with-update lowering`.

`LOOP-003 Stage1 LoopRange lowering` remains open and should be handled as a
JoinIR/CorePlan route, not as source-level range-loop desugar.
