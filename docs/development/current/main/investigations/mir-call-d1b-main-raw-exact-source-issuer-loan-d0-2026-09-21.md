---
Status: design_stop__2026-09-21__MainRawExactSourceIssuerLoan
Task: MIR-CALL-D1B-MAIN-RAW-EXACT-SOURCE-ISSUER-LOAN-D0
Date: 2026-09-21
Parent: mir-call-d1b-direct-call-source-owner-lineage-coseal-d1-2026-08-26.toml
Implementation permission: false; source-authority census and bounded decision only
NextCard: none
---

# Main raw exact source issuer and loan D0

## Six-line brief

```text
Decision: keep target publication and the affine raw loan closed until one
  existing source authority binds the exact site to one callable declaration;
  do not turn raw lineage or name lookup into a target issuer.
Source authority: resolver-owned SourceExprSiteV1 and
  VerifiedResolvedMethodCallSourceV1; canonical target issuer is only the
  existing FreeStatic index in this boundary. Qualified MethodCall remains
  facts-only until an existing owner is selected.
Non-authority: RawInvocationRootLineageV1, name/arity or symbol lookup,
  static-candidate uniqueness, Main spelling, AST re-scan, compatibility
  resolver, and the package loan itself.
Fail-fast boundary: missing, foreign, duplicate, mixed-brand, wrong-family, or
  site/owner mismatch must stop before argument descent, effects, collector
  mutation, target publication, or MirInstruction::call.
Smallest next slice: census each source call family and decide whether an
  existing owner can issue an exact site relation; otherwise seal a typed
  reject/compatibility park with the missing relation named.
Non-claims: no Rust/Hako implementation, target/Callee, package-plus-loan,
  raw dispatcher, fallback removal, backend, JSON, or Call-schema change.
```

## Finite boundary

The selected boundary is the installed source-backed App Main root and its
source-call observations before raw argument descent. It includes four finite
forms: (1) bare `FunctionCall` with the existing FreeStatic index, (2) qualified
`MethodCall` with resolver-owned receiver/site relations, (3) source-backed
Cataloged Main/root provenance, and (4) compatibility/raw forms that must remain
separate. It excludes VM/JSON, nested-owner inheritance, InstanceConstructor
publication, and broad legacy recovery.

## Current evidence

`ResolvedDirectCallObservationV1` records only the observed name/arity while its
`SourceExprSiteV1` is the map key. The existing FreeStatic index can issue an
exact `ResolvedDirectCallTargetV1` for its own source family. In contrast,
`VerifiedResolvedMethodCallSourceV1` records owner, site, receiver relation,
arguments, selector, and arity but deliberately carries no target or ABI.
`VerifiedSourceBoundCoreMethodCallV1` is a selected Loop/CoreMethod product and
is not a general StaticBoxMethod issuer.

The Main identity/catalog companion and live Cataloged root-scope witness are
already landed. They prove provenance and raw owner/site/body-kind equality;
they do not bind a bare call site to a StaticBoxMethod declaration. The current
language contract therefore keeps bare StaticBoxMethod recovery at typed
pre-effect reject or named compatibility park. A target or loan cannot be
opened by wrapping `name + arity`, candidate uniqueness, or raw lineage.

The current-head consumer census confirms the gap. The resolver's
`ResolvedDirectCallObservationV1` is consumed by the exact FreeStatic index and
the package loan, while `VerifiedResolvedMethodCallSourceV1` is consumed by
Map/CoreMethod owners and deliberately has no target field. The complete
`VerifiedWholeSourceStaticCallTargetInventoryV1` is consumed by the separate
normal Script/static-result lane and its tests; it has no normal Main package
consumer. Reusing that AST-backed inventory as a new Main issuer would create a
second source authority unless one shared source traversal is explicitly
accepted by a later design.

## Decision — design stop

No single existing issuer currently covers all four forms. FreeStatic is an
exact source family and qualified method relations have a separate resolver
owner, while the Cataloged Main root product is provenance-only. Reusing the
old AST-backed whole-source inventory as a new canonical issuer would create a
second source authority; using the raw scope or package loan would reverse the
authority chain. The exact source issuer/loan row therefore remains
`design_stop` until one existing owner is selected for a finite source family.

The next design decision must choose one family only, name its existing issuer,
and define the exact site-to-declaration relation before any target/Callee or
affine loan transport is implemented. Missing or foreign relations remain
typed rejects; no fallback or retry may reopen the legacy path.

## Reopen trigger

Reopen only when a finite source family has one resolver/catalog issuer, one
owner/site bijection, same-session brand evidence, and a pre-effect consumer
that can take the relation exactly once. A local green test, a raw lineage
shape probe, a name/arity match, or a backend result is not sufficient.
