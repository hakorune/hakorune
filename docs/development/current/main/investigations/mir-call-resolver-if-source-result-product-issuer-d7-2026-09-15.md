---
Status: closed__bounded_direct_value__2026-09-15
Task: MIR-CALL-RESOLVER-IF-SOURCE-RESULT-PRODUCT-ISSUER-D7
Date: 2026-09-15
Priority: implement the source-result product selected by D6
Parent: mir-call-resolver-if-source-result-product-d6-2026-09-15.md
NextCard: MIR-CALL-RESOLVER-IF-SOURCE-EXPR-RELATION-D8.md
Implementation permission: true for this source-result owner only
---

# Expression If source-result product issuer

## Six-line brief

```text
Decision: implement and close one direct-value source-result scaffold for the finite D6 inventory; do not claim the resolver relation or admit expression If in this slice.
Source authority + canonical issuer: the scaffold consumes one sealed declaration-catalog row and issues VerifiedSourceResultProductV1 plus SourceBoolFactV1; D8 must add the resolver-owned exact relation before call rows are consumed.
Non-authority: callable_result_representation's i64/nominal catalog, names, receiver class, helpers tables, MIR ValueId/type inference, VM, defaults, and compatibility fallback.
Fail-fast boundary: foreign owner/site, missing target/header/manifest, Loop-only contract at non-Loop site, unknown/mixed/recursive result, nullable operand drift, duplicate operation, and consumer-site drift.
Smallest next slice: issue the resolver-owned expression-If relation, then return to the static/core/recursive worklist with exact nested call rows.
Non-claims: AST If resolver admission, JoinSig, physical CFG/PHI emission, publication, VM, fallback, caller cutover, or legacy retirement.
```

`Census boundary: selected source-backed MIR package -> the five deferred
callable terminals; includes their String/i64 tails, condition calls, and
direct source consumers; excludes statement-If, unrelated callables, VM keep,
and compatibility lanes.`

## Ordered implementation tasks

1. Add the source-result class/fact vocabulary and catalog-branded scaffold under
   `resolved_value_profile`; keep all rows keyed by exact source sites.
2. Consume the sealed declaration row directly for the first bounded
   transaction. Do not add name lookup, an alternate target resolver, or a raw
   AST rescan for missing resolver rows.
3. Implement the finite direct-value body walk: locals, string concatenation,
   and empty-prelude expression-If tails. Unknown calls and unsupported body
   shapes must stop before Builder effects.
4. Add the nullable `String/null` Bool fact only after the local source
   binding, Null site, operator, and owner checks pass.
5. Stop before branded static-call result rows, the non-Loop core-method
   sibling, and same-class recursive closure. D8 owns the missing resolver
   relation required for those rows.
6. Add focused positive/negative tests and a module README update. No
   expression-If resolver arm, physical lowering, fallback, or production
   caller switch belongs in D7.

## Acceptance

### Current implementation progress

`resolved_value_profile/source_result.rs` now issues the source-owned
branded `SourceResultClassV1` product for direct literals, local bindings, string
concatenation, empty-prelude expression-If tails, and the exact
`String/null` inequality fact. Four focused tests cover the String conditional
positive path, mixed-class rejection, nullable-operand rejection, and
foreign-owner rejection. The module
README records the new authority and non-claims.

This bounded direct-value transaction is closed at its source-catalog
boundary. Static-call result rows, the sibling non-Loop core relation, and
same-class recursive body closure remain unissued. The resolver currently
rejects expression `ASTNode::If`, so the exact call rows inside the five
deferred terminals do not exist yet. A raw AST rescan in this issuer would
violate the D6 authority boundary and is therefore not allowed.

The current product keeps the catalog brand identity rather than a temporary
catalog address. It remains a scaffold for direct literals, locals,
concatenation, expression-If tails, and the nullable String/null fact; it is
not evidence that expression-If admission or the full D6 inventory is closed.
The next design card must first issue the resolver relation for the outer If,
empty-prelude BlockExpr tails, consumer site, and internal call rows. D7 can
then resume the static/core/recursive worklist using those exact rows.

The complete finite D6 inventory remains a future D7 re-entry condition: it
requires the D8 resolver relation plus branded static/core/recursive rows in a
new focused harness. The bounded slice recorded here has focused positive and
negative tests, stable catalog identity, and explicit non-claims. A local green
result does not claim expression-If admission or production completion.
