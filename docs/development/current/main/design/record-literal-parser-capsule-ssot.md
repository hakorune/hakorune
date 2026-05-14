# REC-001 Stage0 Record Literal Parser Capsule SSOT

Status: accepted
Date: 2026-05-14
Lane: phase-293x language minimal surface lane

## Decision

Accept explicit named-field record literals as syntax and metadata transport only.

```hako
record Meta {
  ptr: i64
  size: usize
}

local meta = Meta { ptr: 1, size: 2 }
```

The literal is a shape capsule. Stage0 does not validate that the record exists
or that the field set matches the record declaration.

## Stage0 owns

- Parse `RecordName { field: expr, ... }` in expression position.
- Require explicit `field: expr` pairs.
- Reject duplicate field names within the literal shape.
- Store `record_type` and ordered field expressions in AST metadata.
- Preserve the same shape in AST JSON and Program JSON v0.

## Stage0 does not own

- Missing-field validation.
- Extra-field validation.
- Record layout or scalarization.
- Field-read lowering.
- PackedArray eligibility.
- Shorthand literal `RecordName { field }`.
- `with` update semantics.

## Stage1 owns later

- Resolve the record declaration.
- Validate missing/extra fields.
- Lower construction/read into the chosen semantic representation.
- Provide scalarization / packed metadata facts to CorePlan.

## Metadata shape

AST JSON:

```json
{
  "kind": "RecordLiteral",
  "record_type": "Meta",
  "fields": [
    { "name": "ptr", "value": { "kind": "Literal" } }
  ]
}
```

Program JSON v0 expression:

```json
{
  "type": "RecordLiteral",
  "record": "Meta",
  "fields": [
    { "name": "ptr", "value": { "type": "Int", "value": 1 } }
  ]
}
```

## Stop lines

```text
no record-as-box lowering
no field shape validation in Stage0
no shorthand fields in this row
no with-update semantics
no packed lowering
```

## Retire condition

Retire this Rust Stage0 capsule once the selfhost parser emits the same
`RecordLiteral` metadata shape before Stage1 consumes it.
