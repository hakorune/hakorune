---
Status: queued__2026-09-22__AwaitingStaticResultTargetOnlyTerminalI0
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-CUTOVER-I3
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-physical-i2-2026-09-22.md
Implementation permission: false; resolve MIR-CALL-PARSER-STATIC-RESULT-TARGET-ONLY-TERMINAL-I0 before I3 task 4
NextCard: mir-call-parser-static-result-target-only-terminal-i0-2026-09-22.md
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
fallback or a VM repair. The predecessor existing-owner pre-front
structured-source I0 is now closed at rows 1–4 and has handed this card the
named publication frontier. Once task 4 and task 5 close, the successor
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

## Recheck correction — 2026-09-22

The named no-selected-handoff frontier is earlier than the selected parser
tuple in the full merged package. A read-only owner audit and a temporary
test-only observation (removed after the run) found an armed
`StringHelpers.skip_ws/2 -> StringHelpers.is_space/1` row at
`[Body(4), LoopBody(0), IfCondition]`. The publication owner classifies this
callee as target-only because the current ExactI64 result contract cannot
prove it. The selected `ParserProgramBox.parse/2 -> starts_with/3` rows are
ExactI64, but they are not the first reached row.

This does not authorize filtering the target-only row: the accepted I0
contract requires every resolver item to be `SelectedStatic` or `CoreMethod`,
and the package acceptance D0 forbids target-only lowering or partial package
coverage. I3 remains queued until the static-result authority D1 card names an
existing owner or a typed terminal for this finite family. The earlier
preflight D1 closed with `NoSafeSlice__ResultFamilyOwnerAbsent` after observing
57 target-only rows; D0 closed with the same bounded NoSafeSlice and the new D1
now owns the static-result authority design. No code,
fallback, publication, caller switch, or retirement claim is made here.

## Acceptance frontier recheck — 2026-09-22

The selected merged parser inventory contains one composite LoopBreak candidate
after the earlier callable loop sites. The pre-front structured-source I0 now
consumes those finite rows, and the lifecycle invocation reaches the existing
named publication boundary
`[freeze:contract][callable-loop/static-publication/no-selected-handoff]`.
The selected `parse/2 -> starts_with/3` row is therefore visible to I3 task 4,
but its one-shot publication handoff has not yet been accepted. The old
`callable-loop/route-not-front-selected` / `GenericLoopV1NotSelected` terminal
is no longer the merged-parser frontier.

This is a publication boundary, not permission to add a fallback. No AST
rescan, name-based inference, VM repair, generic fallback, or new publication
authority is authorized.

The ordered queue is consequently:

1. **I3 task 4 — publication acceptance:** connect the selected source row to
   the existing one-shot publication owner and record the named outcome.
2. **I3 task 5 — caller switch:** switch only that selected parser caller after
   the publication guard is green.
3. **R0 task 6 — retirement:** prove caller-zero, remove the exclusive old
   edge and temporary assets, and retain the re-entry guard.

Until item 1 is green, items 2–3 remain queued and no production cutover or
retirement claim is made. The warning cohort remains paused at I147.

## Current design evidence

The merged parser inventory reaches the selected `starts_with/3` rows, and the
pre-front consumer carries the finite preceding rows through the existing
source owner. The lifecycle invocation now stops at the named
`callable-loop/static-publication/no-selected-handoff` boundary. Task 4 must
reuse the existing publication ingress/physical bridge rather than add a
second authority or relax the terminal.

## I3 task 4 preflight evidence — 2026-09-22

The bounded handoff plumbing is present in the existing owners and their
focused guards. The selected composite LoopBreak row must still take the
existing static-result handoff, install it in the callable lowering ledger,
and let the source expression port consume it once. The normalizer and
physical bridge are already the selected `GlobalCall`/publication owners;
missing, foreign, mismatched, duplicate, and residual handoffs remain named
contract errors. Direct and generic LoopBreak routes are unchanged.

Focused evidence is green: source-route 16/16, raw child entry 11/11, raw
child-port 2/2, package 8/8, and publication bridge 2/2. `cargo check
--profile quick --lib` and `cargo fmt --all -- --check` also pass with the
existing warning baseline.

The end-to-end merged parser fixture now stops at the named
`[freeze:contract][callable-loop/static-publication/no-selected-handoff]`
terminal after the pre-front consumer. This evidence does not claim
publication acceptance or caller cutover. Task 5 remains unopened until task
4 proves the selected `parse/2 -> starts_with/3` handoff. R0 still owns
caller-zero and old-edge deletion.

## I0 handoff receipt — 2026-09-22

The predecessor I0 is closed at rows 1–4. Focused evidence is route 35/35,
package 19/19, finite parser source retention 1/1, and the merged parser
frontier guard 1/1. The source bridge reaches this I3 boundary without
changing VM/compatibility, generic fallback, or the direct LoopBreak route.
