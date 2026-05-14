# REC-003 Record With-Update Lowering SSOT

Status: accepted
Date: 2026-05-14
Lane: phase-293x language minimal surface lane

## Decision

Accept record with-update as an identity-free replacement expression.

```hako
local next = meta with { size: new_size }
```

`with` is a contextual expression-postfix token, not a global reserved keyword.
The update does not mutate `meta`; it produces a new record value with selected
fields replaced.

## Stage0 owns

- Parse `expr with { field: expr, ... }` as `RecordUpdate` metadata.
- Require explicit `field: expr` update pairs.
- Reject duplicate update fields in the parsed shape.
- Preserve the base expression and ordered update expressions in AST JSON.

## Stage0 does not own

- Record declaration resolution.
- Field existence checks.
- Record materialization or layout policy.
- PackedArray set integration.
- Field write-through syntax.

## Stage1 owns

- Resolve the base expression to a tracked record type.
- Reject update fields that do not exist on that record.
- Lower updates as `RecordUpdate` replacement metadata.
- Preserve declared field index and declared field type metadata.
- Keep the result typed as the same record for subsequent field reads in the
  same lowering body.

## Lowered shape

```json
{
  "type": "RecordUpdate",
  "record": "Meta",
  "base": { "type": "Var", "name": "meta" },
  "updates": [
    {
      "name": "size",
      "field_index": 1,
      "declared_type": "usize",
      "value": { "type": "Int", "value": 3 }
    }
  ]
}
```

## Stop lines

```text
no mutation semantics
no metas[i].field write-through
no packed array set integration in this row
no shorthand update fields
no wildcard/default update
no record-as-box fallback
```

## Retire condition

Retire this Rust parser/lowering slice once selfhost Stage1 emits the same
`RecordUpdate` replacement shape and validates field existence before backend
planning consumes it.
