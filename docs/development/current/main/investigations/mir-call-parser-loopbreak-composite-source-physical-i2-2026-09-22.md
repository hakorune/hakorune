---
Status: design_stop__2026-09-22__AwaitingCompositePackageI1
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PHYSICAL-I2
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-package-i1-2026-09-22.md
Implementation permission: false; wait for I1 package/Recipe acceptance
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

