---
Status: Complete
Date: 2026-05-12
Scope: integer literal suffix parsing and MIR exact numeric const facts.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/reference/language/types.md
---

# 294x-11 Literal Suffix Exact Numeric Consts

## Decision

Accepted integer literal suffixes such as `0usize`, `1u8`, and `42i64` are
source metadata for exact numeric constants.

This row does not add an exact runtime numeric value representation. The
emitted MIR constant still uses the current `Integer(i64)` lane, while MIR
metadata records the declared exact numeric source name and the checked
constant value.

## Scope

This row owns:

- tokenizer recognition of decimal integer suffix text;
- AST preservation through a typed integer literal value;
- AST JSON and Stage1 Program(JSON) metadata preservation;
- range checking against the target-resolved exact numeric type;
- MIR function metadata for exact numeric literal facts;
- refresh of exact numeric value facts from those literal facts.

## Stop Line

This row does not add:

- float literal suffixes;
- negative literal folding such as treating `-1i8` as one literal;
- constants outside the current `i64` runtime lane;
- exact `VMValue` / typed-object storage;
- backend lowering to unsigned or pointer-sized native integer classes;
- hako_alloc live field migration.

The expression `-1usize` remains a unary minus applied to a checked positive
`1usize` literal until a later exact unary/conversion row exists. Allocator code
must keep sentinel-bearing fields signed.

## Runtime Boundary

`LiteralValue::TypedInteger` preserves source spelling and range-checks through
the exact numeric substrate. MIR emission still writes:

```text
ConstValue::Integer(i64)
```

and attaches side-car metadata:

```text
ExactNumericConstFact { value, declared_type_name }
```

Unsupported backend/runtime exact value behavior remains future work. This row
only makes exact numeric constants visible to subsequent verifier, VM reference,
and backend-lowering rows without silently changing the legacy integer lane.
