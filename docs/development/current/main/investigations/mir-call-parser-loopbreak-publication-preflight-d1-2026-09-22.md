---
Status: design_stop__2026-09-22__PrefrontTargetOnly
Task: MIR-CALL-PARSER-LOOPBREAK-PUBLICATION-PREFLIGHT-D1
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-cutover-i3-2026-09-22.md
Implementation permission: false; resolve the target-only source family before I3 publication
NextCard: mir-call-parser-loopbreak-composite-source-cutover-i3-2026-09-22.md
---

# Parser LoopBreak publication preflight D1

## Six-line brief

```text
Decision: classify every target-only static item that precedes the selected
  ParserProgramBox.parse/2 -> starts_with/3 publication row before changing I3.
Source authority + canonical issuer: the same-invocation resolver source-item
  ledger, the static result publication owner, and the installed package port.
Non-authority: target-only filtering, method/name/ordinal hardcode, AST/MIR
  rescans, partial package acceptance, GenericLoop retry, or VM fallback.
Fail-fast boundary: every resolver item in an armed source candidate must have
  an existing SelectedStatic/CoreMethod owner or a named typed terminal.
Smallest next slice: census the finite target-only rows and choose an existing
  owner or record NoSafeSlice; do not implement a new result ABI here.
Non-claims: no publication, caller switch, old-edge deletion, or warning work.
```

## Reproduced frontier — 2026-09-22

The merged parser lifecycle test still passes its named frontier, but a
temporary test-only observation (removed immediately after the run) identified
the first blocking row before the selected parser tuple:

```text
caller = StringHelpers.skip_ws/2
target = StringHelpers.is_space/1
site   = [Body(4), LoopBody(0), IfCondition]
result = target-only (callee result is unavailable to ExactI64 publication)
```

The same observation showed that the selected
`ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3` rows do have
ExactI64 publication requirements. They are not the first reached row in the
full merged package. A second target-only row is visible in the result owner
for `StringHelpers.starts_with_kw/3 -> starts_with/3`; it must be included in
the finite census even if its loop route is not selected by this card.

The existing publication owner records all exact static targets and separates
selected rows from `target_only_targets`. `source_target_for_loop` then stops
an active composite candidate at the named
`[freeze:contract][callable-loop/static-publication/no-selected-handoff]`
boundary. The source-item contract explicitly requires `SelectedStatic` or
`CoreMethod`; silently dropping this row would violate complete item coverage.

## Why I3 cannot advance yet

The current I3 card correctly forbids a target-only lowerer, method filtering,
partial package, and fallback. The existing result representation and
publication bridge only issue `ExactI64`; `is_space` is a different result
family. Therefore changing `source_target_for_loop` to ignore this row would
make the selected `starts_with` proof appear green by hiding an earlier
unconsumed product. That is not a safe slice.

## Bounded design work

1. Re-census the merged parser import closure from the resolver ledger and
   record every target-only static item with caller, exact source site, target,
   result disposition, and whether it lies under an armed LoopBreak candidate.
2. Check whether an existing result representation and physical consumer can
   cover the target-only family without a new ABI or a second publication
   authority. If not, record the family as `NoSafeSlice` with its reopen owner.
3. Check whether the existing source candidate issuer can issue a typed
   outside-family terminal before candidate consumption while preserving all
   package rows. Name the owner and all reject/consume obligations; do not add
   a filter or fallback.
4. Only after the finite inventory has one accepted owner/terminal may the
   pointer return to I3 task 4. I3 task 5 and R0 task 6 remain queued.

## Evidence and non-claims

The merged parser lifecycle test is 1/1 at the named no-selected-handoff
frontier. The temporary diagnostic code used for the row classification was
removed; no production source change is claimed by this card. The warning
cohort remains paused at I147 (`unused_imports=17`); `dead_code` remains owner
debt.
