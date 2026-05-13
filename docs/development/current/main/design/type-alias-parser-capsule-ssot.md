# TYPE-001 Stage0 Type Alias Parser Capsule SSOT

Status: accepted
Date: 2026-05-14
Lane: phase-293x language minimal surface lane

## Decision

Accept type aliases as syntax and metadata transport only.

```hako
type Bytes = usize
type PageList = Array<PageId>
```

A type alias is not a brand. It does not create a distinct type and does not
change runtime representation.

## Stage0 owns

- Parse `type Name = TypeRef` at top level.
- Store `name` and `target_type` in AST metadata.
- Preserve the same metadata in AST JSON and Program JSON v0.
- Keep alias semantics out of Stage0.

## Stage0 does not own

- Alias expansion.
- Alias-cycle detection.
- Type checking.
- Brand distinction.
- Verifier facts.
- MIR/CorePlan type policy.

## Stage1 owns later

- Alias diagnostics.
- Alias expansion facts for error messages.
- Exact verifier-readable metadata if needed.

## Metadata shape

AST JSON:

```json
{
  "kind": "TypeAliasDeclaration",
  "name": "Bytes",
  "target_type": "usize"
}
```

Program JSON v0 root metadata:

```json
{
  "type_alias_decls": [
    { "name": "Bytes", "target_type": "usize" }
  ]
}
```

## Stop lines

```text
no Stage0 alias checker
no alias-as-brand behavior
no runtime behavior change
no MIR type policy expansion
```

## Retire condition

Retire this Rust Stage0 capsule once the selfhost parser emits the same
`TypeAliasDeclaration` / `type_alias_decls` metadata shape before Stage1 consumes it.
