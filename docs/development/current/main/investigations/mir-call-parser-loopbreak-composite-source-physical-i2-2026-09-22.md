---
Status: design_stop__2026-09-22__CompositePackageI1AcceptedAwaitingPhysicalRelation
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PHYSICAL-I2
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-package-i1-2026-09-22.md
Implementation permission: false; I1 accepted; first name the existing physical source-port and selected-target relation, then implement one bounded adapter slice
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-CUTOVER-I3
---

# Parser composite LoopBreak physical handoff I2

## Six-line brief

```text
Decision: pending I1 package/Recipe acceptance; consume one verified
  composite source Recipe through the existing LoopBreak physical owner.
Source authority + canonical issuer: the I1 package loan, the resolver-issued
  composite projection, and the existing LoopBreak physical adapter.
Non-authority: AST rescans, LoopRouteContext reconstruction, names/line numbers,
  generic retry/fallback, VM routes, or a new physical/result authority.
Fail-fast boundary: exact owner/site/Recipe relation, source-port coverage,
  selected target/loan relation, one-shot take, and pre-effect validation.
Smallest next slice: map the accepted composite Recipe to the existing physical
  input and prove positive, missing, foreign, residual, and second-take guards.
Non-claims: no production caller switch, source-to-MIR publication claim,
  old-edge deletion, backend work, or warning cleanup.
```

## Dependency and ordered acceptance

I2 cannot start until I1 records a verified composite package row with no
residual candidate. The physical owner must consume that row once, validate the
source port before allocation, and retain the existing direct LoopBreak route
unchanged. A named source-to-MIR acceptance invocation is required before the
cutover row is opened.

## I1 handoff and I2 task order — 2026-09-22

I1 is now closed with the focused package evidence recorded in its card. I2
starts in design stop because the remaining uncertainty is an authority
relation, not a missing test: identify which existing LoopBreak physical owner
and `CallableLoopSourceExpressionPortV1` instance consume the composite
Recipe, and how its selected target/loan relation is carried without rebuilding
meaning from AST or names.

The bounded I2 rows are:

1. **Relation census** — trace the existing direct LoopBreak physical owner
   from package loan to pre-allocation validation; record the exact source port,
   selected target, loan lifetime, and cleanup terminal for the composite row.
2. **Physical adapter** — after the relation is accepted, consume one
   composite Recipe through that owner, with positive, missing, foreign,
   residual, and second-take guards. Keep direct rows unchanged.
3. **Acceptance gate** — run the selected parser source-to-MIR invocation and
   record the named terminal. Only a green source-backed handoff can open I3;
   no caller switch or old-edge deletion is part of I2.
