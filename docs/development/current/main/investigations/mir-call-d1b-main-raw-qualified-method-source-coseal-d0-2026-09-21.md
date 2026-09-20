---
Status: accepted__2026-09-21__MainQualifiedMethodSourceHandoff
Task: MIR-CALL-D1B-MAIN-RAW-QUALIFIED-METHOD-SOURCE-COSEAL-D0
Date: 2026-09-21
Parent: mir-call-d1b-main-raw-exact-source-issuer-loan-d0-2026-09-21.md
Implementation permission: false; design decision only
NextCard: MIR-CALL-D1B-MAIN-RAW-QUALIFIED-METHOD-HANDOFF-I0
---

# Main raw qualified MethodCall source co-seal D0

## Six-line brief

```text
Decision: promote the existing Main qualified-receiver relation to one exact,
  one-shot source handoff for the finite qualified StaticBoxMethod family; do
  not broaden bare FunctionCall or invent a target from selector/arity.
Source authority + canonical issuer: resolver-issued
  VerifiedResolvedMethodCallSourceV1 joined once with the lifecycle-owned
  import view and the existing same-module declaration catalog by
  VerifiedNormalCallableSemanticPackageV1::issue_app_main_qualified_receiver_catalog_relation.
Non-authority: RawInvocationRootLineageV1, Script whole-source inventory,
  names/arity/symbol lookup, CoreMethod rows, DirectCallDispositionLoansV1,
  and any separate result-publication issuer.
Fail-fast boundary: exact Main caller/site, receiver spelling and alias brand,
  declaration key/selector/arity, ordered argument-site set, and the
  Cataloged raw scope must be co-sealed before argument descent; a relation
  row is consumable exactly once and residual rows are a named closeout error.
Smallest next slice: retain resolver argument sites, take one scoped handoff in
  the installed Main adapter, validate each argument site, and reuse the
  existing target-only physical terminal. No second AST walk or source issuer.
Non-claims: no result ABI/publication, other callers, instance/dynamic calls,
  VM, fallback restoration, backend parity, or legacy-edge deletion.
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

The current result is an accepted bounded source handoff, not a new target
authority. The relation already carries the canonical declaration key from the
same resolver/import/catalog session. The next I0 may move that relation once
into the Main raw adapter and call the existing target-only physical terminal;
it must not issue a second target/result product or rescan the AST.

## Read-only owner audit

The bounded owner audit confirms that the Main relation is the selected source
owner for this handoff. It already owns the resolver ledger's caller/site,
receiver/owner/selector/arity relation and the same-brand declaration key. The
only missing product is ordered argument-site retention plus an affine,
one-shot consumer in the Main adapter. The Script whole-source inventory and
`VerifiedSourceStaticCallTargetCatalogV1` remain outside this route; they are
not reused as a second issuer or hidden retry.

## Reopen trigger

Reopen implementation only after the existing source owner can be consumed
once from the same source session and its relation reaches a pre-effect Main
consumer without AST rescan, name/arity inference, target duplication, or
compatibility retry.
