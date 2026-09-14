---
Status: open__fast__2026-09-15
Task: MIR-CALL-RESOLVER-IF-SOURCE-RESULT-PRODUCT-ISSUER-D7
Date: 2026-09-15
Priority: implement the source-result product selected by D6
Parent: mir-call-resolver-if-source-result-product-d6-2026-09-15.md
NextCard: MIR-CALL-RESOLVER-IF-SOURCE-EXPR-RELATION-D8
Implementation permission: true for this source-result owner only
---

# Expression If source-result product issuer

## Six-line brief

```text
Decision: implement one resolved_value_profile source-result transaction for the finite D6 inventory; do not admit expression If or lower MIR in this slice.
Source authority + canonical issuer: the new source-result issuer consumes exact resolved source/call/core rows and issues VerifiedSourceResultProductV1 plus SourceBoolFactV1.
Non-authority: callable_result_representation's i64/nominal catalog, names, receiver class, helpers tables, MIR ValueId/type inference, VM, defaults, and compatibility fallback.
Fail-fast boundary: foreign owner/site, missing target/header/manifest, Loop-only contract at non-Loop site, unknown/mixed/recursive result, nullable operand drift, duplicate operation, and consumer-site drift.
Smallest next slice: implement class/fact rows, finite body-result worklist, recursive same-class closure, and source-site guards for int_to_str, length/substring, and sval != null.
Non-claims: AST If resolver admission, JoinSig, physical CFG/PHI emission, publication, VM, fallback, caller cutover, or legacy retirement.
```

`Census boundary: selected source-backed MIR package -> the five deferred
callable terminals; includes their String/i64 tails, condition calls, and
direct source consumers; excludes statement-If, unrelated callables, VM keep,
and compatibility lanes.`

## Ordered implementation tasks

1. Add the source-result class/fact vocabulary and owner-branded product under
   `resolved_value_profile`; keep all rows keyed by exact source sites.
2. Consume the sealed declaration row directly for the first source-result
   transaction. Do not add name lookup or an alternate target resolver.
3. Implement the finite direct-value body walk: locals, string concatenation,
   and empty-prelude expression-If tails. Unknown calls and unsupported body
   shapes must stop before Builder effects.
4. Add the nullable `String/null` Bool fact only after the local source
   binding, Null site, operator, and owner checks pass.
5. In the same card, prepare the next handoff points for branded static-call
   result rows, the non-Loop core-method sibling, and same-class recursive
   closure; do not silently approximate those rows here.
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

The card remains open. Static-call result rows, the sibling non-Loop core
relation, and same-class recursive body closure are still required before this
source-result owner can close. Those rows must be consumed from existing
branded source issuers; they cannot be filled from method names or MIR types.

The card closes only when the complete finite D6 inventory is consumed by a
focused test harness, all negative states fail before Builder effects,
source-site/owner brands are checked, and the module README records the
authority and non-claims. A local green result does not claim expression-If
admission or production completion.
