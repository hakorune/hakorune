---
Status: accepted_design__2026-09-21__ResolvedMethodStaticTargetCoissueI0
Task: MIR-CALL-D1B-RESOLVED-METHOD-STATIC-TARGET-COISSUE-I0
Date: 2026-09-21
Parent: mir-call-d1b-resolved-method-static-target-coissue-d0-2026-09-21.md
Implementation permission: true; owned canonical-key relation and one-shot
  pre-effect consumption only
NextCard: MIR-CALL-D1B-MAIN-RAW-EXACT-SOURCE-ISSUER-LOAN-D0
---

# Resolver MethodCall static-target co-issue I0

## Six-line brief

```text
Decision: retain the exact declaration key already verified by the Main
  resolver/catalog co-issuer and consume that relation once before effects.
Source authority + canonical issuer: the resolver-owned MethodCall row, the
  lifecycle-owned import view, and the same declaration catalog through the
  existing VerifiedNormalCallableSemanticPackageV1 issuer.
Non-authority: AST-backed Script inventory, route-facts replay, receiver/name
  lookup, raw lineage, physical symbols, and any target reconstruction later.
Fail-fast boundary: Main slot/owner, exact site, QualifiedUnbound identity,
  import/catalog brand, alias/direct precedence, static declaration key,
  selector/arity, and duplicate or second consumption.
Smallest next slice: add the canonical declaration key to the owned relation,
  transport it through install, and expose one package-port take terminal.
Non-claims: no Callee emission, affine loan, argument lowering, publication,
  production caller switch, compatibility retry, backend, or legacy deletion.
```

## Finite implementation boundary

This I0 covers only the qualified direct-variable `MethodCall` rows of the
source-backed App Main batch. It may touch the existing relation model, package
install transport, package-port accessor, and focused package tests. The
canonical key must be copied from the declaration returned by the existing
catalog lookup; it must not be reconstructed from `receiver`, `selector`, or
`arity` after the issuer returns.

The one-shot terminal may borrow or take the installed owned relation through
the existing package port, but it must reject a foreign Main site, a missing
relation, and a second take before argument descent. Empty source-backed Main
relations are valid and must be distinguishable from an unavailable Main
relation. The terminal is a transport proof only; it does not lower a call.

Keep `VerifiedSourceMethodCallSiteV1`,
`VerifiedQualifiedCallRouteFactsV1`, and
`VerifiedWholeSourceStaticCallTargetInventoryV1` out of this I0. They are
AST-backed and Script/compatibility-owned. No AST walk, alias re-resolution,
or new semantic issuer may be added.

## Required focused evidence

1. Direct canonical receiver stores and returns the exact declaration key.
2. Imported alias stores the same canonical key while preserving alias admission.
3. Foreign import/catalog, wrong namespace, selector/arity drift, missing or
   duplicate receiver identity, and duplicate relation remain named rejects.
4. The installed package-port terminal takes one exact Main relation and
   rejects a second take or an unavailable relation before any argument effect.
5. `cargo fmt --check`, `git diff --check`, touched-file line counts below
   760/800, and the existing warning baseline classification are recorded.

## Non-claims and reopen

This I0 does not create a public target catalog, `Callee`, affine source loan,
result publication, raw production switch, fallback removal, backend parity,
or old-edge deletion. The next D0 decides how this exact relation enters the
source-to-raw loan and physical target consumer. If the package port cannot
prove one-shot ownership without a second relation or an empty-as-available
fallback, close this I0 as `NoSafeSlice` and retain the typed terminal.
