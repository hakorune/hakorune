---
Status: fast__2026-09-21__ResolvedQualifiedReceiverCatalogCoissue
Task: MIR-CALL-D1B-RESOLVED-QUALIFIED-RECEIVER-CATALOG-COISSUE-I0
Date: 2026-09-21
Parent: mir-call-d1b-resolved-qualified-receiver-catalog-coissue-d0-2026-09-21.md
Implementation permission: true; relation-only Main co-issuer
NextCard: none
---

# Resolver qualified receiver catalog co-issue I0

## Six-line brief

```text
Decision: implement one AST-free Main co-issuer for a qualified direct-variable
  MethodCall, joining resolver identity, source ledger, import view, and the
  declaration catalog exactly once.
Source authority + canonical issuer: the resolver-issued Main batch/source
  ledger plus invocation-owned VerifiedStaticImportAliasViewV1 and the existing
  source-backed declaration catalog; the new helper only seals their relation.
Non-authority: Script whole-source inventory, AST route facts, name/arity
  lookup, MIR, physical symbols, compatibility routes, and target reconstruction.
Fail-fast boundary: exact Main batch slot/site/brand, one non-empty receiver
  identity, alias/direct-owner agreement, static namespace, selector, and arity.
Smallest next slice: add the borrowed seam, relation product, and focused
  positive/negative guards at the existing package handoff.
Non-claims: no target/loan/source-site publication, Recipe/ABI/MIR lowering,
  dispatcher, fallback, backend parity, production switch, or legacy deletion.
```

## Finite implementation boundary

The I0 covers only App Main's exact resolver batch slot and direct-variable
`QualifiedUnbound` method calls. The implementation may touch the existing
normal callable semantic package/batch handoff and its private tests. It may
retain the source spelling carried by `ResolvedQualifiedReceiverIdentityV1`,
borrow the invocation's sealed import view, and compare the resolver row's
selector/arity with the existing static-box-method declaration catalog.

It must not reuse `VerifiedQualifiedCallRouteFactsV1`,
`whole_source_inventory.rs`, or Script lookup; each performs an AST-backed
walk outside the Main authority. It must not infer a target from name/arity,
create a second import authority, or attach a target/loan before this relation
is sealed.

## Required guards and closeout

Add focused guards for one valid direct receiver and for missing/duplicate
identity, empty spelling, Main slot/site mismatch, foreign brand/import view,
alias conflict, lexical binding, wrong namespace, selector, and arity. Keep
instance/`me`, dynamic, nested, reserved, Script, and raw routes rejected.

Closeout requires the exact focused command and result, touched-file line
counts below 760/800, `git diff --check`, the current-state pointer update,
and a receipt that explicitly leaves target/loan/publication and production
cutover open. Existing warning output is classified against the known warning
baseline; it is not a new semantic claim.
