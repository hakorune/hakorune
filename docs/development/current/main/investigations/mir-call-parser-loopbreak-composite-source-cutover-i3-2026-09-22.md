---
Status: design_stop__2026-09-22__AwaitingPhysicalI2
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-CUTOVER-I3
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-physical-i2-2026-09-22.md
Implementation permission: false; wait for I2 physical acceptance
NextCard: closeout__composite-loopbreak-old-edge-retirement
---

# Parser composite LoopBreak production cutover I3

## Six-line brief

```text
Decision: pending I2 physical acceptance; switch only the selected parser
  caller to the source-backed composite LoopBreak route.
Source authority + canonical issuer: the accepted I1 package and I2 physical
  owner, with the existing semantic package as the caller boundary.
Non-authority: VM/compatibility lanes, generic fallback, names, AST rescans,
  and acceptance smoke results from an unselected backend.
Fail-fast boundary: selected caller, source-to-MIR terminal, publication
  relation, caller-zero proof for the old edge, and a stable retirement guard.
Smallest next slice: one parser caller switch plus one source-to-MIR acceptance
  invocation; delete the old edge only in the same bounded closeout.
Non-claims: no whole-repository migration, backend promotion, warning cleanup,
  or unrelated legacy retirement.
```

This row remains planning-only while I1 and I2 are open. A failed or deferred
source terminal reopens the owning semantic row; it does not authorize a
fallback or a VM repair.

