# 293x-294 ASTCLEAN-001 legacy AST enum removal

Status: implemented, complete

## Decision

Decision: accepted.

The old split AST enums (`StructureNode`, `ExpressionNode`, `StatementNode`)
and `ASTNodeType` classification enum are legacy residue. The compiler uses
the unified `ASTNode` enum. Keep only the direct expression predicate required
by block expression tail validation.

## Scope

- Remove `ASTNodeType`.
- Remove `StructureNode`, `ExpressionNode`, and `StatementNode`.
- Remove `ASTNode::classify`, `is_structure`, and `is_statement`.
- Keep `ASTNode::is_expression()` as direct pattern matching.

## Non-goals

- Do not alter parser behavior.
- Do not alter `ASTNode` variants.
- Do not remove `Outbox`, Stage-3 compatibility tokens, or deprecated `this`.

## Acceptance

```bash
bash tools/checks/k2_wide_astclean_legacy_enum_guard.sh
```

## Next

Continue AST cleanup with `ASTCLEAN-002 normalize logical ops helper`, or return
to `LOCALTYPE-001` if cleanup is paused.

## Local guard

- `bash tools/checks/k2_wide_astclean_legacy_enum_guard.sh` passed locally.
