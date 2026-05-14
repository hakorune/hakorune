# 293x-281 REC-003 Record With-Update Lowering

Status: complete
Date: 2026-05-14

## Scope

Parse and lower `expr with { field: value }` as identity-free record replacement,
not mutation.

## Landed changes

- Added `ASTNode::RecordUpdate` with base expression and ordered updates.
- Parsed `with` as a contextual expression-postfix token.
- Kept record shorthand `RecordName { field }` rejected without breaking enum
  record-pattern shorthand.
- Added AST traversal/classification/JSON transport for `RecordUpdate`.
- Lowered tracked record updates to Program JSON v0 `RecordUpdate` nodes.
- Rejected update fields that do not exist on the base record type.
- Preserved record type tracking so reads after update lower as `RecordField`.
- Added parser and Program JSON tests plus a dedicated guard.

## Non-goals

- No mutation semantics.
- No array element field write-through.
- No packed ArrayBox set integration.
- No scalar replacement or layout planning.
- No shorthand update fields.

## Guard

```bash
bash tools/checks/k2_wide_record_with_update_lowering_guard.sh
```

## Next selected row

`CONTRACT-002 contract syntax metadata capsule`.

`LOOP-003 Stage1 LoopRange lowering` remains open and should be handled as a
JoinIR/CorePlan route, not as source-level range-loop desugar.
