---
Status: accepted design; I0 implementation not landed
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-owner-binding-i0-implementation-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-BODY-FACTS-QUERY-D0

## Decision

The neutral resolver body-shape issuer is now landed, so this row may open a
bounded private Query projection. The first implementation must consume only
the already-selected Query owner link:

```text
VerifiedInstanceMethodBodyOwnerCatalogV1
  -> private QueryBodyFactsObserver
  -> atomic VerifiedCallableQueryBodyFactsCatalogV1
```

The observer reads `owner_row.carrier().body_shape()` and does not receive the
declared contract catalog, raw Query syntax, the parser envelope, or a second
function array. Query/Home/signature/ABI meaning is already owned upstream and
is never re-issued here. The current owner catalog is Query-selected in its
type and implementation; general non-Query owner reuse is a future D0, not a
claim of this row.

## Bounded cohort

The first facts cohort is deliberately the lexical receiver `Me` shape:

```text
one body statement:
  Return(value)

one returned expression:
  Me(receiver BindingRef)

one exact ReturnValue relation:
  Return statement -> returned Me expression

no other statement, expression, or effect row
```

This makes `return me` the positive facts fixture. The body-facts product does
not validate that `me` has the declared result type; semantic I64/Unit belongs
to the declaration catalog and type/conformance remains later. `return 0`, an
empty body, a local read, multiple returns, a field access, and a method call
are fully observed but outside this first lexical-Me cohort.

`Me.receiver` is accepted only when the BindingRef belongs to the exact root
function owner and its resolver binding record is the receiver binding. The
observer never tests the diagnostic name `"me"`.

## Product and authority

The public semantic boundary is one catalog-level, non-`Clone` aggregate. Row
DTOs remain private to the issuer and cannot be independently constructed or
recombined:

```text
VerifiedCallableQueryBodyFactsCatalogV1
  rows: VerifiedCallableQueryBodyFactsRowV1[]

row (borrowed from one owner row)
  - exact owner/declaration identity by borrow
  - ReceiverReadFact { expression site, receiver BindingRef }
  - OrdinaryReturnFact { statement site, value site }
```

The row borrows the owner-link/carrier source authority rather than copying
`FunctionOwnerId`, Query, Home, signature, ABI, or MIR facts. The catalog
atomically requires exactly one facts row for every selected Query owner row,
with no missing, duplicate, foreign, or extra row. Sparse source order is
inherited from the owner catalog; no name, ordinal, vector position, or owner
number is used to pair rows.

The canonical source authority remains:

```text
resolver parser-private syntax lease
  -> VerifiedResolvedBodyShapeInventoryV1
  -> carrier/body-owner link
  -> private Query facts projection
```

The facts issuer is a projection only. It does not issue a new semantic axis,
classify a Query, or infer a public contract from the body.

## Effects and conformance boundary

This I0 does not issue a complete effect-absence receipt. The neutral shape
inventory currently records an effect vocabulary, but its absence is not yet
the complete proof required for Query conformance. Therefore the facts row
does not claim `Pure`, `Query`, `EffectMask`, Home safety, or conformance.

`CALLABLE-CONTRACT-CONFORMANCE-D0` remains closed until a later decision fixes
the complete effect/control coverage owner. Conformance must consume one
canonical facts/effect authority; it may not rescan AST, infer from MIR, or
silently treat a missing effect row as proof of purity.

## Disposition and fail-fast

`NoSafeSlice` is a development state, not a source outcome:

```text
shape issuer/evidence unavailable:
  NoSafeSlice

complete shape, outside lexical-Me cohort:
  Declined

required evidence opaque or incomplete:
  Unresolved

foreign owner/root/body-root, foreign receiver BindingRef,
duplicate/missing Return or ReturnValue relation, mixed catalog:
  Rejected

exact lexical-Me cohort for every selected Query row:
  Candidate
```

The priority remains:

```text
Rejected > Unresolved > Declined > Candidate
```

An empty body is valid source coverage upstream, but it is not a Candidate for
this facts cohort. Non-Query body rows remain valid unselected rows upstream;
they are not converted into a Query disposition and receive no default facts.

## Required negative matrix

```text
return 0 / return local / empty body
multiple returns or extra statements
return me.field
return me.method()
return me plus an extra effect/expression
foreign owner catalog, carrier, root, or body_root
Me with a non-receiver/local/foreign BindingRef
missing or duplicate Return / ReturnValue relation
selected Query row missing or duplicated in the facts catalog
nested callable body leaking into the parent shape
AST re-scan, raw rune/name/ordinal pairing, Query re-selection
```

Field/state authority, calls, allocation, writes, suspension, qmark, throw,
panic, and non-local control remain outside this I0. If a future cohort needs
an effect/control fact that the neutral issuer cannot prove, stop at
`NoSafeSlice` and add a separate issuer D0; do not add a fallback or an empty
verified fact.

## Implementation order after this D0

```text
1. private QueryBodyFactsObserver over owner rows only
2. exact lexical-Me/ordinary-return positive fixture (`return me`)
3. catalog-level atomic negative matrix
4. same-slice module README/reference/task closeout
5. stop before CALLABLE-CONTRACT-CONFORMANCE-D0
```

The implementation row must not touch the parser transaction, owner issuer,
declared Query/Home aggregate, target, source-bound call, Recipe/CallSlot,
Builder/MIR/CFG/PHI, runtime, provider, fallback, or production selection.

## Non-claims

```text
no AST-bearing product
no Query/Home/signature/ABI re-issuance
no type inference or body->contract inference
no complete effect-absence/conformance proof
no FunctionOwner issuer or owner-number comparison
no field/state authority
no target/source-bound call/Recipe/CallSlot
no Builder/MIR/physicalization
no fallback/retry/provider/runtime dispatch
```
