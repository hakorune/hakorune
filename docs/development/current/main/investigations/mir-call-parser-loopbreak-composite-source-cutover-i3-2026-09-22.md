---
Status: design_stop__2026-09-22__AwaitingParserPublicationAcceptance
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-CUTOVER-I3
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-physical-i2-2026-09-22.md
Implementation permission: false; I2 physical envelope is focused-green; task 4 publication acceptance and task 5 caller switch are bounded here
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-RETIREMENT-R0
---

# Parser composite LoopBreak production cutover I3

## Six-line brief

```text
Decision: prove publication for the selected parser tuple, then switch only
  that parser caller to the source-backed composite LoopBreak route.
Source authority + canonical issuer: the accepted I1 package and I2 physical
  owner, with the existing semantic package as the caller boundary.
Non-authority: VM/compatibility lanes, generic fallback, names, AST rescans,
  and acceptance smoke results from an unselected backend.
Fail-fast boundary: selected caller, source-to-MIR terminal, publication
  relation, and a stable handoff guard; caller-zero and physical deletion are
  owned by the successor retirement card.
Smallest next slice: one publication acceptance invocation followed by one
  parser caller switch; do not delete the old edge in this card.
Non-claims: no whole-repository migration, backend promotion, warning cleanup,
  or unrelated legacy retirement.
```

I1 and I2 are closed at their package/Recipe and focused physical boundaries.
This row remains a design stop until the selected source-to-MIR invocation is
named and its publication relation is accepted. A failed or deferred source
terminal reopens the owning semantic row; it does not authorize a fallback or
a VM repair. Once task 4 and task 5 close, the successor retirement card owns
caller-zero and the exclusive delete set.
