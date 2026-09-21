---
Status: design_stop__2026-09-22__AwaitingI3PublicationAndCallerSwitch
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-RETIREMENT-R0
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-cutover-i3-2026-09-22.md
Implementation permission: false; task 6 opens only after I3 publication and caller-switch receipts
NextCard: none__retirement_closeout_pending
---

# Parser composite LoopBreak selected old-edge retirement R0

## Six-line brief

```text
Decision: retire only the selected parser compatibility edge after the
  composite source publication and caller switch are proven.
Source authority + canonical issuer: the accepted I3 source/publication
  receipt, the existing semantic package owner, and the selected caller census.
Non-authority: caller counts alone, dead-code warnings, VM/compatibility
  behavior, AST/name scans, fallback success, or an unselected backend smoke.
Fail-fast boundary: source caller, physical edge, guard/test/docs callers, and
  successor route must be exact before any deletion.
Smallest next slice: after I3, census the selected edge and prove caller-zero
  while retaining `route_loop_break_recipe` for non-source callers.
Non-claims: no repository-wide LoopBreak retirement, backend retirement,
  warning cleanup, or deletion before the finite delete tuple is recorded.
```

## Task 6 finish line

This card is deliberately design-only while I3 is open. It owns the final
bounded retirement sequence:

| order | task | completion condition |
| --- | --- | --- |
| 1 | Import I3 publication and caller-switch receipts | the selected parser source site reaches one named publication outcome and the production caller consumes the composite route before the generic terminal |
| 2 | Census the selected old edge | every production, test, guard, manifest, and documentation caller is named; `route_loop_break_recipe` non-source callers remain explicitly retained |
| 3 | Prove caller-zero and successor coverage | the selected parser edge has zero live callers, while the source-backed route and its stable guard cover the same accepted behavior |
| 4 | Delete the finite tuple atomically | remove only the selected physical edge and its owned obsolete guard/test/doc rows; no shared compatibility owner is deleted |
| 5 | Re-run acceptance and close out | source-to-MIR/publication/acceptance receipts, pointer, registry, and this card agree; no unknown red is introduced |

The delete tuple is not inferred from `rg` counts. I3 must name the exact
caller and terminal first; this card then records the owner, successor,
observable guard, and caller-zero evidence before permitting a physical delete.

## Guardrails

`LegacyCallV0`, VM routes, generic compatibility, `route_loop_break_recipe`
for non-source callers, and unrelated LoopCond/LoopTrue edges remain outside
this retirement. A failed publication or deferred source terminal reopens I3;
it does not authorize deletion or a fallback. The warning cohort remains
paused at I147, with remaining `dead_code` rows returning to their semantic
owners after this finite LoopBreak lane closes.
