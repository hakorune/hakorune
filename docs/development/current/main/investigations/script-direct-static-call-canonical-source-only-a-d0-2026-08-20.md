---
Status: Active design stop
Date: 2026-08-20
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-FACTS-PREFIX-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-parser-source-handoff-d0-2026-08-20.md
ProductionCaller: none; design only
ReplacementCell: Builder-free parser/source/Facts prefix for canonical Script
Classification: BoxCount
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-FACTS-PREFIX-D0

## Six-line brief

Decision: Narrow the broad canonical A proposal to one Builder-free
source/Facts prefix. Retain the complete parser-backed source product,
declaration Facts/catalog, resolver package, and Script window before any
Builder, physical Call, result publication, or canonical consumer is opened.

Source authority + canonical issuer: parser
`ParsedProgramWithCallableParameterSourceV1` already co-seals the single
postpass with the complete callable-parameter catalog; the postpass's initial
source owns constructor coverage. A future move-only prefix carrier must
retain that product with the digest/profile and may borrow existing
declaration/Facts owners, but it may not reissue their meaning.

Non-authority: `CompletedParserPostpassV1` by itself, AST or digest alone,
`RawScriptBodyRecipeV1`, `ModuleBuilderInvocationSessionV1`,
`builder.comp_ctx`, work-plan ordinals, pointer/name/path scans, `ValueId`,
MIR, and the current Builder-bound lifecycle output are not a source/Facts
issuer.

Fail-fast boundary: missing parser parameter-source or constructor coverage,
non-ParserBacked input, Compatibility/Deferred status, missing/foreign/
duplicate/stale Facts/window/owner, or a product that can only be paired by
AST/name/path inference keeps the row at `NoSafeSlice` before Builder effects.
There is no reparse, second resolver, Raw fallback, or guessed empty product.

Smallest next slice: specify the exact Builder-free producer input/output and
the non-Clone handoff identity shared by selected-normal and future canonical
consumers. Do not edit `normal_default_root_catalog_lifecycle.rs` or
`canonical_core_dispatch.rs` until that contract is accepted.

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

## Required prefix contract

The design must name one producer input and one move-only output. The output
must retain or linearly move, without cloning or semantic re-issuance:

```text
ParsedProgramWithCallableParameterSourceV1
  + retained source digest/profile
  + declaration Facts/catalog loan (existing issuer)
  + resolver callable package and Script window loan (existing issuer)
```

The prefix may expose borrowed views while issuing, but its published carrier
must be AST-free where the existing owner already provides an AST-free product.
It must not contain a Builder, mutable registry, physical block, `ValueId`,
signature, or MIR instruction. Target inventory, result bundle, Recipe, Join,
and physical-input products are not silently copied into this prefix: each must
either be shown to have a source-only issuer/transport or remain a later row.

The producer must be shared. Selected-normal may not continue issuing a second
source/Facts set once the eventual I0 consumes this carrier, and canonical may
not reconstruct one from the raw recipe. Until that I0 exists, the current
Builder lifecycle remains unchanged and is not evidence that the prefix is
already implemented.

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

The D0 is complete only when the card can answer all of these without a code
guess or a second issuer:

- how `ParsedProgramWithCallableParameterSourceV1` is retained or projected
  without dropping its catalog, and how the constructor catalog remains bound
  to the same parser brand;
- which existing source/Facts/resolver owners are lent or moved, and which
  products are explicitly deferred because their current issuer is Builder-
  bound;
- how one source identity is consumed by selected-normal and canonical without
  duplicate issuance or pointer/name pairing;
- how complete Script window/owner/site/cardinality coverage is checked before
  any Builder effect;
- how Compatibility, Deferred, RawLegacy, and AST-only fixtures are rejected
  rather than represented as empty candidates;
- how a future I0 can prove Builder effect zero in the producer and preserve
  the old lifecycle until the shared consumer is ready.

Positive design evidence is a source/Facts ownership table and one bounded
input/output contract. A worker report or local green test alone does not open
I0.

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
changes, production switch, raw retirement, or performance measurement until
this prefix is accepted and its source authority is explicit.
