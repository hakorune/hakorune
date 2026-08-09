---
Status: ready; implementation 0
Date: 2026-08-09
Decision: BoxShape only; add one atomic parser-private ordinary parameter-list product
Parent: `HAKO-PARSER-TAKE-PARAMETER-CARRIAGE-H2-D0`
Predecessor: `HAKO-PARSER-BOX-SOURCE-SESSION-H2-S0` closed
---

# HAKO-PARSER-PARAMETER-LIST-PRODUCT-H2-S1

## Goal

Add one disconnected parser-private parameter-list product for exact ordinary
parameters. It is the future source truth for method header parameters and the
only origin of the neutral compatibility projection.

```text
exact method source site
  -> unpublished parameter-list builder
  -> ordered ordinary syntax rows
  -> one atomic finish
  -> ParserParameterListProductV1
       exact source rows
       read-only neutral projections
```

This is a behavior-neutral substrate row. It does not connect the ordinary
Box parser and it does not recognize `take`.

## Product boundary

Place the implementation in `parser/source_carrier_v1/`, preferably split as:

```text
parameter_syntax_records_v1.hako
parameter_list_builder_v1.hako
```

The product owns rows in source order:

```text
ParserParameterSyntaxRowV1
  exact method source site
  parameter ordinal
  parameter name
  exact supported declared type syntax token
  transfer syntax = Ordinary
```

Identity is exact method source site plus parameter ordinal. Name and type are
payload, not identity. Diagnostic spans/offsets may be added later but never
become semantic identity.

The neutral projection exposes only:

```text
name
optional declared type spelling
```

It must be derived from or borrow the sealed row; callers cannot create a
second independently mutable parameter truth. Do not widen Rust's Clone
`ParamDecl` or infer rows from it in this Hako-only substrate slice.

## Issuance and atomicity

The builder is opened with one exact method source site. It automatically
assigns ordinals `0..N-1`; callers do not provide ordinals. `finish()` is the
only product issuer and validates every row before returning a product.

```text
duplicate parameter name -> reject before append
empty name/type token     -> reject before append
foreign method site       -> reject
finish after poison       -> no product
double finish             -> reject
mutation after finish     -> reject
empty list                -> exact zero-row product
```

No public `Take` row constructor is added in H2-S1. A private transfer
vocabulary may reserve `Ordinary`, but `Take` becomes constructible only in the
later language implementation row.

## Focused tests

```text
empty exact list
ordinary rows preserve declaration order
automatic ordinals are 0,1,2
neutral projection agrees row by row
duplicate name leaves prior cardinality unchanged and poisons transaction
empty name/type rejects
foreign method site rejects
double finish and post-finish mutation reject
no Clone/copy/reconstruction API
no parser branch imports the product
all touched source files < 800 lines
```

Use or extend the H1 fixture/guard only if its responsibility remains clear;
otherwise add one focused H2-S1 fixture/guard and register it in
`docs/tools/check-scripts-index.md` in the same commit.

## Nonclaims

```text
Take syntax or contextual-token recognition
complex/generic type_ref grammar
ordinary Box parser connection
rich body product
method draft/H3 seal integration
Rust/Hako parity
resolver signature/source relation
Home demand/capability/Flow
Recipe/Builder/MIR/runtime
new accepted language shape
```

## Done

- [ ] one atomic ordered ordinary parameter-list product exists;
- [ ] neutral projection is one-way and not a second authority;
- [ ] missing/duplicate/foreign/closed-state cases fail fast;
- [ ] parser branches remain disconnected;
- [ ] focused guard and README are updated;
- [ ] all touched Hako files remain below 800 lines;
- [ ] next pointer advances only to H2-S2 after closeout.
