# 293x-278 TYPE-001 Stage0 Type Alias Parser Capsule

Status: complete
Date: 2026-05-14

## Scope

Accept `type Name = TypeRef` as top-level syntax and transport the declaration as
read-only metadata.

## Landed changes

- Added `type` tokenizer keyword and declaration dispatch.
- Added `ASTNode::TypeAliasDeclaration` with `name` and `target_type_name`.
- Added parser support for `type Bytes = usize`.
- Added AST JSON roundtrip/JoinIR-compatible metadata shape.
- Added Program JSON v0 root `type_alias_decls` metadata.
- Updated EBNF to list `type_alias_decl` as a top-level declaration.
- Added parser and Program JSON tests plus a dedicated guard.

## Non-goals

- No alias expansion.
- No alias-cycle detection.
- No type checker semantics.
- No brand behavior.
- No runtime behavior changes.

## Guard

```bash
bash tools/checks/k2_wide_type_alias_parser_capsule_guard.sh
```

## Next selected row

`REC-001 Stage0 explicit record literal shape capsule`.

`LOOP-003 Stage1 LoopRange lowering` remains open and should be handled as a
JoinIR/CorePlan route, not as source-level range-loop desugar.
