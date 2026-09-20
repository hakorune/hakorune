---
Status: fast__2026-09-21__ResolvedQualifiedReceiverIdentityCarrier
Task: MIR-CALL-D1B-RESOLVED-QUALIFIED-RECEIVER-IDENTITY-I0
Date: 2026-09-21
Parent: mir-call-d1b-resolved-qualified-receiver-identity-coseal-d0-2026-09-21.md
Implementation permission: true; receiver identity carrier only
NextCard: MIR-CALL-D1B-RESOLVED-QUALIFIED-RECEIVER-CATALOG-COISSUE-D0
---

# Resolver qualified receiver identity carrier I0

## Six-line brief

```text
Decision: retain the exact source spelling of a qualified receiver during the
  existing resolver traversal and co-seal it with the existing method-call row.
Source authority + canonical issuer: ShadowResolverV0 -> body-shape seal ->
  VerifiedResolvedMethodCallSourceV1; no second AST traversal is allowed.
Non-authority: import aliases, canonical owners, name/arity lookup, targets,
  ABI, Recipe keys, MIR symbols, Script inventory, and compatibility routes.
Fail-fast boundary: missing/duplicate identity, lexical-bound receiver,
  dynamic/me/wrong-kind receiver, foreign owner, or nested-owner crossing.
Smallest next slice: add the private identity carrier, thread it through the
  existing resolver product, and add focused positive/negative source-shape tests.
Non-claims: no declaration/import co-seal, target/loan, package publication,
  dispatcher, fallback, backend, production caller switch, or old-edge deletion.
```

## Finite implementation scope

Only the existing qualified `MethodCall` receiver path is in scope:

- `src/mir/resolved_semantics/body_shape_resolver.rs`
- `src/mir/resolved_semantics/body_shape_seal.rs`
- `src/mir/resolved_semantics/body_shape.rs`
- the existing resolver/body-shape focused tests

The source name is retained only when the same receiver site is proven
`QualifiedUnbound`. Lexical variables keep their existing binding-only row;
`me`, dynamic receivers, fields, nested lambdas, and non-qualified expressions
remain outside the carrier.

## Required checks

1. A direct qualified receiver preserves its exact source spelling and site.
2. A qualified alias spelling preserves the alias; no canonical owner is inferred.
3. A lexical variable at the same syntactic shape does not receive the carrier.
4. `me`, dynamic, wrong-kind, and nested-owner shapes remain typed non-carrier
   rows.
5. Missing, duplicate, or mismatched receiver identity rejects at the existing
   body-shape/method-call source boundary.

The implementation must preserve the current `QualifiedUnbound` disposition
for existing consumers. The carrier is passive source identity only and must
not alter resolver acceptance or introduce a target lookup.

## Exit and next design dependency

Close this I0 only with focused positive/negative evidence and the existing
source-shape guard green. Then move to
`MIR-CALL-D1B-RESOLVED-QUALIFIED-RECEIVER-CATALOG-COISSUE-D0`, which decides how
the carrier joins the existing declaration catalog and invocation import view
once, with explicit brand/alias precedence. That next D0 must not reuse the
Script AST-backed inventory or infer a target from the carrier name.
