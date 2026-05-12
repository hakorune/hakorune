# 293x-208: C202 Record Surface Semantics

Status: Complete

## Purpose

Add `record` as the explicit source-level aggregate/value declaration surface.

`record` is not an ordinary `box` fast path. It is the language contract that
allows later rows to perform local scalar replacement and packed `ArrayBox`
residence without weakening ordinary object identity rules.

## MVP Surface

```hako
record HakoAllocAlignedSmallMeta {
    ptr: i64
    alignment: i64
    requested_size: i64
    usable_size: i64
}
```

Accepted:

- named record declarations
- generic type parameters in the declaration header
- fixed typed fields

Rejected in C202:

- weak fields
- untyped fields
- field initializers
- methods / constructors / `birth`
- `fini`
- inheritance / implements clauses

## Stop Line

C202 does not add:

- local scalar replacement
- packed/columnar `ArrayBox` storage
- ordinary `box` identity erasure
- reflection semantics
- allocator-specific syntax
- native/provider/hook behavior

## Acceptance

- `docs/reference/language/EBNF.md` records `Decision: accepted`.
- Parser regression proves accepted typed record declarations.
- Parser regression rejects weak/untyped/method-bearing records.
- Runtime user-box factory and MIR declaration indexing do not treat records as
  ordinary user boxes.
