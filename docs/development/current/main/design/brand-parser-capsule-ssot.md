# BRAND-001 Stage0 Brand Parser Capsule SSOT

Status: accepted
Date: 2026-05-14
Lane: phase-293x language minimal surface lane

## Decision

Accept brand declarations as syntax and metadata transport only.

```hako
brand PageId: i64
brand BlockId: i64
brand PtrId: i64
```

A brand declaration gives a scalar storage type a distinct language-level name,
but this Stage0 capsule does not enforce distinctness.

## Stage0 owns

- Parse `brand Name: TypeRef` at top level.
- Store `name` and `underlying_type` in AST metadata.
- Preserve the same metadata in AST JSON and Program JSON v0.
- Document that brand semantics are not owned by Stage0.

## Stage0 does not own

- Brand constructor syntax.
- Unwrap syntax.
- Implicit or explicit conversion policy.
- Checking that `PageId` and `BlockId` are not mixed.
- Verifier facts or CorePlan lowering decisions.

## Stage1 owns later

- Constructor/unwrap policy.
- Brand mismatch diagnostics.
- Verifier facts for branded scalar values.
- Any MIR or CorePlan typing semantics.

## Metadata shape

AST JSON:

```json
{
  "kind": "BrandDeclaration",
  "name": "PageId",
  "underlying_type": "i64"
}
```

Program JSON v0 root metadata:

```json
{
  "brand_decls": [
    { "name": "PageId", "underlying_type": "i64" }
  ]
}
```

## Stop lines

```text
no Stage0 brand checker
no implicit brand conversion
no constructor/unwrap semantics in this row
no MIR type policy expansion in this row
```

## Retire condition

Retire this Rust Stage0 capsule once the selfhost parser emits the same
`BrandDeclaration` / `brand_decls` metadata shape before Stage1 consumes it.
