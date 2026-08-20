---
Status: Design accepted; transport-only I0 selected
Date: 2026-08-20
Decision: SCRIPT-DIRECT-STATIC-CALL-PARSER-CALLABLE-SOURCE-HANDOFF-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-parser-source-handoff-d0-2026-08-20.md
ProductionCaller: none; design only
ReplacementCell: Builder-free parser/source/Facts prefix for canonical Script
Classification: BoxCount
Execution row: SCRIPT-DIRECT-STATIC-CALL-PARSER-CALLABLE-SOURCE-HANDOFF-I0
---

# SCRIPT-DIRECT-STATIC-CALL-PARSER-CALLABLE-SOURCE-HANDOFF-D0

## Six-line brief

Decision: Close the source/Facts design at one transport-only handoff. The
next I0 may wrap and move the parser's existing atomic callable product, but
may not issue a second parser/Facts authority or open a Builder consumer.

Source authority + canonical issuer: parser
`string_postpass_entry::parse_with_callable_parameter_source` issues one
`ParsedProgramWithCallableParameterSourceV1` containing the postpass and the
complete parameter catalog; the postpass's initial source owns constructor
coverage. The I0 carrier is one non-Clone
`NormalParserCallableSourceHandoffV1` that co-seals this product with the
already-issued source digest/profile/read-parse receipt. Selected-normal and
canonical are alternative consumers of this same carrier contract; neither
reparses or reissues source/Facts meaning.

Non-authority: `CompletedParserPostpassV1` by itself, AST or digest alone,
`ParsedNormalCallableProgramV1`'s AST projection, `RawScriptBodyRecipeV1`,
`ModuleBuilderInvocationSessionV1`,
`builder.comp_ctx`, work-plan ordinals, pointer/name/path scans, `ValueId`,
MIR, and the current Builder-bound lifecycle output are not a source/Facts
issuer.

Fail-fast boundary: missing parser parameter-source or constructor coverage,
non-ParserBacked input, Compatibility/Deferred status, missing/foreign/
duplicate/stale Facts/window/owner, or a product that can only be paired by
AST/name/path inference keeps the row at `NoSafeSlice` before Builder effects.
There is no reparse, second resolver, Raw fallback, or guessed empty product.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-PARSER-CALLABLE-SOURCE-
HANDOFF-I0`: add the handoff in parser/source-plan sibling modules, make the
canonical frontdoor retain it, and make the selected-normal transform carry
only its non-Clone lineage projection. Keep Builder lifecycle and canonical
dispatch unchanged.

Non-claims: no A/C/B handoff, three-state disposition, canonical physical
Call/publication/Return, source admission expansion, production switch, raw or
compat retirement, ABI/backend/performance change, or Builder cleanup.

## Audit result: broad A is not open

The parser handoff I0 is closed, but the canonical frontdoor currently carries
only one completed `CompletedParserPostpassV1`. The parser already has the
correct atomic product: `ParsedProgramWithCallableParameterSourceV1` is issued
by one `NyashParser` invocation, and its `completed` postpass is paired with
the complete parameter catalog. The postpass's initial callable source also
retains the parser-issued constructor catalog. The missing seam is therefore
transport, not a new parser authority: the canonical frontdoor must retain
this existing product instead of dropping the catalog or re-deriving it later.

The current complete Script product set is still issued only inside
`normal_default_root_catalog_lifecycle.rs`. That lifecycle performs Builder
preparation/catalog installation and then issues or attaches declaration,
resolver, target, result-bundle, Recipe, and Join products before lowering.
The canonical frontdoor currently consumes `RawScriptBodyRecipeV1` and has no
consumer for a source-only A handoff. Calling the lifecycle from canonical
would create a second physical pipeline; re-running resolver or rebuilding
products from AST would create a second authority.

Therefore the previous broad A design is retained as an audit boundary, not an
implementation row. The next bounded task is still the source/Facts prefix,
but its parser input is now fixed to the existing atomic product rather than a
new postpass/parameter co-seal.

## Existing parser issuer and exact seam

The parser-side authority is already source-backed and one-shot:

```text
one NyashParser
  -> parse_postpass_s0()
  -> finish_callable_parameter_source_catalog()
  -> ParsedProgramWithCallableParameterSourceV1
       { CompletedParserPostpassV1, ParserCallableParameterSourceCatalogV1 }
```

The normal callable path separately uses the same parser session through
`finish_callable_parameter_source_for_normal()` and
`CompletedParserPostpassV1::into_normal_callable_program(...)`. The
constructor catalog is issued by the parser source-seal finalizer and is
attached to `VerifiedInitialCallableProgramSourceV1`; missing constructor
coverage is already a typed reject. These are the canonical parser issuers,
not AST/digest pairing or Builder ordinals.

The prefix design must choose one of these existing parser products as its
input and prove how selected-normal and canonical consumers borrow or consume
the same source identity. It must not create a third parser product or call
both parser entrypoints for one source.

## Accepted handoff contract

The I0 has one producer input and one move-only output. The output retains or
linearly moves, without cloning or semantic re-issuance:

```text
ParsedProgramWithCallableParameterSourceV1
  + source digest/profile/read-parse receipt
  + one source-owner identity
```

The parser product remains opaque behind the handoff. Its parameter catalog
and constructor coverage are borrowed only through bounded loans. Declaration
Facts, resolver packages, Script windows, target inventory, result bundle,
Recipe, Join, and physical input stay later products with their existing
issuers; the handoff cannot contain Builder state or a physical identifier.

The producer is shared by contract. The canonical frontdoor consumes the
handoff directly. Selected-normal consumes it before its existing callable
transform and carries only a sealed, non-Clone lineage projection inside
`VerifiedFinalCallableProgramSourceV1`; it does not retain a second postpass or
catalog. Compatibility/Deferred/RawLegacy/AST-only paths never fabricate an
empty handoff. Until the I0 lands, the current Builder lifecycle remains
unchanged and is not evidence that the handoff is already implemented.

## Owner and seam inventory

```text
parser postpass + parameter source  -> final parser-backed source owner
declaration Facts/catalog            -> existing Facts issuer
callable semantic package            -> existing resolver/package issuer
Script window/semantic source        -> existing Script source issuer
target/result/Recipe/Join             -> source-only transport design still open
physical input/Call/publication       -> later C/B/bridge rows
```

`normal_default_root_catalog_lifecycle.rs` is the evidence boundary, not the
new owner. It is near the 760/800 line policy and must not absorb the producer
or new semantic validation. New source-only models, if later authorized, must
be sibling modules with one thin lifecycle call site.

## Design acceptance

The D0 is accepted with the following bounded answers; the I0 must prove them
with focused transport tests before any physical consumer opens:

- how `ParsedProgramWithCallableParameterSourceV1` is retained or projected
  without dropping its catalog, and how the constructor catalog remains bound
  to the same parser brand;
- which existing source/Facts/resolver owners are lent or moved, and which
  products are explicitly deferred because their current issuer is Builder-
  bound;
- how one `NormalParserCallableSourceHandoffV1` contract is consumed by
  selected-normal and canonical without duplicate issuance or pointer/name
  pairing;
- how complete Script window/owner/site/cardinality coverage is checked before
  any Builder effect;
- how Compatibility, Deferred, RawLegacy, and AST-only fixtures are rejected
  rather than represented as empty candidates;
- how a future I0 can prove Builder effect zero in the producer and preserve
  the old lifecycle until the shared consumer is ready.

Positive design evidence is the source/Facts ownership table, the bounded
input/output contract above, and the worker audits confirming that the only
missing boundary is transport. The I0 is transport-only; it does not claim a
production consumer.

## Negative matrix and stop line

Stop at this D0 if any of the following remains true:

- the existing atomic parser product is split into independent postpass and
  parameter/constructor products;
- the source-only producer must call `prepare_normal_default_module`, install
  `builder.comp_ctx`, or read a mutable Builder catalog;
- target/result/Recipe/Join are reissued instead of transported from a named
  source owner;
- canonical must parse/re-resolve/re-pair by AST, name, path, pointer, or digest;
- Script window or owner/site coverage is partial, duplicate, foreign, or stale;
- Compatibility/Deferred/RawLegacy is converted to `NonCandidate` or Raw;
- a future selected-normal/canonical consumer would receive different source
  products or one route would silently retain a second issuer;
- the design requires growth of a source file to 760+ for convenience.

Do not open disposition C, carrier B, physical bridge, canonical request
changes beyond retaining the handoff, production switch, raw retirement, or
performance measurement until the transport I0 is green.
