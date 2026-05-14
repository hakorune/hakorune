# 293x-279 REC-001 Stage0 Record Literal Parser Capsule

Status: complete
Date: 2026-05-14

## Scope

Accept `RecordName { field: expr }` as expression syntax and transport its field
shape as metadata.

## Landed changes

- Added `ASTNode::RecordLiteral` with `record_type_name` and ordered field values.
- Added parser support for explicit named-field record literals.
- Rejected shorthand fields and duplicate literal field names.
- Added AST JSON roundtrip/JoinIR-compatible shape.
- Added Program JSON v0 `RecordLiteral` expression shape.
- Updated EBNF to list record literal expression syntax.
- Added parser and Program JSON tests plus a dedicated guard.

## Non-goals

- No record declaration resolution.
- No missing/extra field validation.
- No construction/read lowering.
- No scalarization or packed lowering.
- No shorthand literal fields.
- No `with` update semantics.

## Guard

```bash
bash tools/checks/k2_wide_record_literal_parser_capsule_guard.sh
```

## Next selected row

`REC-002 Stage1 record construction/read lowering`.

`LOOP-003 Stage1 LoopRange lowering` remains open and should be handled as a
JoinIR/CorePlan route, not as source-level range-loop desugar.
