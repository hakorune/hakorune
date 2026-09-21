---
Status: fast__2026-09-22__ParserPublicationAcceptance
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-CUTOVER-I3
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-physical-i2-2026-09-22.md
Implementation permission: true; accepted bounded task 4 publication acceptance only, followed by task 5 caller switch after its guard is green
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
The design stop is accepted for this bounded slice. A failed or deferred
source terminal reopens the owning semantic row; it does not authorize a
fallback or a VM repair. Once task 4 and task 5 close, the successor
retirement card owns caller-zero and the exclusive delete set.

## Accepted design decision — 2026-09-22

The read-only publication audit is accepted as the implementation boundary for
this slice. The existing
`VerifiedStaticCallResultPublicationOwnerV1::take_for_source` remains the
canonical issuer and one-shot owner. The existing callable lowering ledger is
the transport, and the existing selected static-result physical bridge is the
only emitter. The composite LoopBreak source site, exact target, `ExactI64`
representation, required argument ordinals, and the handoff are co-sealed at
the selected parser tuple before physical emission.

The bounded implementation must fail fast on missing owner/catalog, foreign or
mismatched site/target, `TargetOnly`/`NoExactStaticTarget`, duplicate take, and
residual handoff. It may reuse the existing plan normalizer and physical
`GlobalCall` bridge, but it must not add a second publication authority, infer
from AST or names, revive VM/compatibility fallback, or alter generic/direct
LoopBreak routes. The acceptance invocation is only
`ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3`; caller switch
and old-edge deletion remain task 5 and R0 respectively.

## Queue reconciliation — 2026-09-22

The warning cohort is deliberately paused at I147. `unused_imports=17` is the
remaining mechanical tail; `dead_code` remains owner debt and is not a reason
to keep the semantic lane waiting. No warning cleanup row is selected while
this parser publication boundary is open.

The next bounded order is:

1. **I3 task 4 — publication acceptance:** connect the selected
   `ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3` source row
   to the existing one-shot publication owner and record the named terminal.
2. **I3 task 5 — caller switch:** switch that selected parser caller to the
   source-backed composite LoopBreak route after the publication guard is
   green. Do not change VM/compatibility or generic fallback routes.
3. **R0 task 6 — retirement:** after the switch, prove caller-zero, delete the
   exclusive old edge and temporary assets, and retain a guard against
   re-entry.

The warning cohort may resume only after this I3/R0 sequence closes, or when
an owner-specific warning becomes a newly selected blocker. This ordering is
the task queue; it does not claim publication, cutover, or retirement yet.

## Current design evidence

The merged parser inventory reaches the selected `starts_with/3` rows, while
the lifecycle invocation still stops at the named publication boundary. The
existing composite envelope validates and stores source-target relations, but
does not consume the publication owner's one-shot handoff. Therefore the next
slice must reuse the existing publication ingress/physical bridge rather than
add a second authority or relax the terminal.
