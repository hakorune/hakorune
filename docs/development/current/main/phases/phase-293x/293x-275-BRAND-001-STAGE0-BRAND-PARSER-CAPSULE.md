# 293x-275 BRAND-001 Stage0 Brand Parser Capsule

Status: complete
Date: 2026-05-14

## Scope

Accept `brand Name: TypeRef` as top-level syntax and transport the declaration as
read-only metadata.

## Landed changes

- Added `brand` tokenizer keyword and declaration dispatch.
- Added `ASTNode::BrandDeclaration` with `name` and `underlying_type_name`.
- Added parser support for `brand PageId: i64`.
- Added AST JSON roundtrip/JoinIR-compatible metadata shape.
- Added Program JSON v0 root `brand_decls` metadata.
- Updated EBNF to list `brand_decl` as a top-level declaration.
- Added parser tests and a dedicated guard.

## Non-goals

- No brand mismatch checker.
- No constructor or unwrap syntax.
- No implicit conversion policy.
- No verifier facts.
- No MIR/CorePlan typing semantics.

## Guard

```bash
bash tools/checks/k2_wide_brand_parser_capsule_guard.sh
```

## Next selected row

`BRAND-002 Stage1 brand constructor unwrap policy`.

`LOOP-003 Stage1 LoopRange lowering` remains open and should be handled as a
JoinIR/CorePlan route, not as source-level range-loop desugar.
