---
Status: closed implementation
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

- [x] one atomic ordered ordinary parameter-list product exists;
- [x] neutral projection is one-way and not a second authority;
- [x] missing/duplicate/foreign/closed-state cases fail fast;
- [x] parser branches remain disconnected;
- [x] focused guard and README are updated;
- [x] all touched Hako files remain below 800 lines;
- [x] next pointer advances only to H2-S2 after closeout.

## Implementation receipt

The landed product is split by responsibility:

```text
parameter_syntax_records_v1.hako
  immutable syntax rows, source site, neutral borrowed projection, product

parameter_list_builder_v1.hako
  unpublished ordered mutation; automatic ordinal assignment

parameter_list_sealer_v1.hako
  full validation and the only product issuance
```

The first cohort constructs `Ordinary` rows only. Product identity is exact
method source site plus parameter ordinal; names and type tokens are payload.
The neutral view borrows a sealed row and cannot carry or reconstruct transfer
syntax. Empty lists are exact zero coverage, while duplicate/empty/foreign/
ordinal/lifecycle failures publish no product.

Focused evidence:

```text
bash tools/checks/hako_parser_parameter_list_h2_s1_guard.sh
  -> parser_branch_connection=0
  -> take_syntax_construction=0
  -> resolver_home_semantics=0
  -> parameter_ordinals_source_ordered=1
  -> neutral_projection_one_way=1
  -> partial_product_publication=0
  -> source_files_below_800=1
```

No contextual token, parser branch, method draft, body product, H3 seal,
resolver relation, or Home meaning landed.
