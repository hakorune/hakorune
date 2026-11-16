# REC-002 Stage1 Record Construction/Read Lowering SSOT

> **注意(2026-11-17、メンテナ決定):** この文書には、[MirBuilder の設計図](mirbuilder-blueprint.md) が上書きした所がある。特に「loop の家族・route・Recipe が、source を受け付けるかを決める」という記述と、段の順(`Facts -> RoutePolicy -> Recipe -> ...`)は、もう設計ではない。食い違う時は、設計図が勝つ。

Status: accepted
Date: 2026-05-14
Lane: phase-293x language minimal surface lane

## Decision

Stage1 owns the first semantic slice for explicit record literals and record
field reads.

```hako
record Meta {
  ptr: i64
  size: usize
}

local meta = Meta { ptr: 1, size: 2 }
return meta.ptr
```

`record` remains an identity-free aggregate. Stage1 validates the literal field
shape against the declared record and lowers reads through a record-owned node.
This is not ordinary box construction.

## Stage1 owns

- Build a record declaration index from parsed record declarations.
- Reject unknown record constructors.
- Reject missing fields in `RecordName { field: expr }`.
- Reject extra fields in `RecordName { field: expr }`.
- Preserve declared field index and declared field type metadata on lowered
  record literal fields.
- Track local variables initialized from record literals within the current
  Program JSON v0 lowering body.
- Lower tracked `recordLocal.field` reads to `RecordField` nodes.
- Reject reads of fields not declared on the tracked record type.

## Stage1 does not own in this row

- Record layout choice.
- Scalar replacement.
- PackedArray eligibility or packed backend lowering.
- `with` update semantics.
- Shorthand literal fields.
- Record methods, delegate, or interface implementation.
- Cross-function record type inference.
- Branch-sensitive data-flow joins.

## Lowered shape

Record construction:

```json
{
  "type": "RecordLiteral",
  "record": "Meta",
  "fields": [
    {
      "name": "ptr",
      "field_index": 0,
      "declared_type": "i64",
      "value": { "type": "Int", "value": 1 }
    }
  ]
}
```

Record read:

```json
{
  "type": "RecordField",
  "record": "Meta",
  "recv": { "type": "Var", "name": "meta" },
  "field": "ptr",
  "field_index": 0,
  "declared_type": "i64"
}
```

## Stop lines

```text
no record-as-box construction
no silent fallback for missing/extra fields
no implicit record field creation
no packed lowering in this row
no with-update lowering in this row
no field write-through semantics
```

## Retire condition

Retire this Rust Stage1 slice once the selfhost Stage1 owner can validate the
same record literal field set and emit the same construction/read semantic
shape before backend planning consumes it.
