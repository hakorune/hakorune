---
Status: design_stop__2026-09-21__MainRawQualifiedMethodSourceCoSeal
Task: MIR-CALL-D1B-MAIN-RAW-QUALIFIED-METHOD-SOURCE-COSEAL-D0
Date: 2026-09-21
Parent: mir-call-d1b-main-raw-exact-source-issuer-loan-d0-2026-09-21.md
Implementation permission: false; reuse decision and finite relation census only
NextCard: MIR-CALL-D1B-RESOLVED-METHOD-STATIC-TARGET-COISSUE-D0
---

# Main raw qualified MethodCall source co-seal D0

## Six-line brief

```text
Decision: evaluate the existing qualified MethodCall source-target owner for
  one Main-root family; do not broaden bare FunctionCall or invent a target
  from selector/arity.
Source authority + canonical issuer: the existing declaration-catalog,
  import-alias, lexical-receiver, and QualifiedCallRouteFacts chain that can
  issue VerifiedSourceStaticCallTargetCatalogV1; resolver MethodCall facts are
  the route-neutral input, not a target by themselves.
Non-authority: RawInvocationRootLineageV1, ResolvedDirectCallObservationV1,
  static-candidate cardinality, names/arity/symbols, CoreMethod rows, the
  package loan, and the separate Script result lane as an unexamined retry.
Fail-fast boundary: one same-catalog caller/site, receiver lexical disposition,
  alias relation, declaration target, argument-site set, brand, and callable
  owner must be co-sealed before effects or target publication.
Smallest next slice: prove whether the existing qualified source product can be
  consumed from the same source loan/package without a second AST traversal or
  second issuer; otherwise record NoSafeSlice and keep the Main family parked.
Non-claims: no Rust/Hako code, target/Callee emission, affine loan, raw
  dispatcher, Script/Main route switch, fallback, backend, JSON, or Call schema.
```

## Finite boundary

The census covers only qualified `MethodCall` rows in the installed source-backed
App Main root whose receiver is a direct canonical owner or a verified import
alias. It includes caller/site, receiver-site, method selector, argument sites,
lexical disposition, import relation, declaration row, and catalog brand. It
excludes bare `FunctionCall`/FreeStatic, `me.method` CoreMethod, instance
methods, nested-owner inheritance, ScriptRoot/RawCompatibility, and result
publication.

## Existing owner evidence

`VerifiedSourceMethodCallSiteV1` and `VerifiedQualifiedCallRouteFactsV1` already
co-seal the declaration catalog, exact source site, receiver lexical status,
import alias, and canonical owner. `VerifiedSourceStaticCallTargetCatalogV1`
then verifies the StaticBoxMethod declaration and stores the exact callable key.
The normal Script lane consumes this chain through
`ScriptDirectStaticCallLookupIssuerV1`; the normal Main package does not
currently consume it. `VerifiedResolvedMethodCallSourceV1` is the resolver-owned
AST-free relation used by other owners, but intentionally contains no target.

The Script issuer is evidence of a usable target owner, not a Main solution by
itself. It performs whole-source MethodCall inventory and Script-window
observation inside the Script lane. Sending that product to Main would either
add a second AST observation beside the resolver relation or make the Script
lane a hidden retry; neither is allowed in this D0.

## Decision boundary

The only viable successor is an owner-preserving handoff from the existing
qualified source product into the Main package. It may reuse the same parser
loan, declaration catalog, import view, and exact site; it may not rerun a
MethodCall AST walk after package installation or use the Script lane as a
fallback. If the package cannot receive this product from the original source
loan, no canonical Main issuer exists in this row and the outcome is
`NoSafeSlice` with a typed pre-effect terminal.

Acceptance for this D0 is a source-only decision: one exact qualified row maps
to one declaration key with same-brand ownership, and all missing/foreign,
lexically bound, alias-conflict, duplicate, nested, and wrong-kind rows have a
named reject. No target/Callee, package field, loan, raw dispatcher, or MIR Call
may be added here.

The current result is `NoSafeSlice` for direct Main reuse. The next bounded
design row must decide whether the existing resolver MethodCall relation and the
declaration/import catalog can co-issue an AST-free qualified static target in
the same source session. If that requires a second traversal or issuer, the
Main family remains typed-reject/parked.

## Reopen trigger

Reopen implementation only after the existing source owner can be consumed
once from the same source session and its relation reaches a pre-effect Main
consumer without AST rescan, name/arity inference, target duplication, or
compatibility retry.
