---
Status: closeout__2026-09-22__NoSafeSliceResultFamilyOwner
Task: MIR-CALL-PARSER-LOOPBREAK-PUBLICATION-PREFLIGHT-D1
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-cutover-i3-2026-09-22.md
Implementation permission: false; resolve the target-only source family before I3 publication
NextCard: mir-call-parser-publication-result-family-d0-2026-09-22.md
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

## Target-only census closeout — 2026-09-22

The same merged-parser lifecycle invocation emitted a finite target-only
census of **57 rows**, covering **18 callers** and **31 caller-to-target
pairs**. Ten rows under a `LoopBody` site cover five callers; the first armed
row is the `StringHelpers.skip_ws/2 -> is_space/1` row above. The selected
`ParserProgramBox.parse/2` caller also contains three target-only rows
(`RuneContractBox.invalid_placement_tag/1` and
`ParserDeclarationBox.parse_or_null/3`), so reaching `starts_with/3` cannot
make whole-package acceptance complete.

The existing `VerifiedCallableResultRepresentationV1` and publication bridge
issue only `ExactI64`. `CoreMethod` rows cover bound receiver methods and do
not cover these static targets. The source/result owners therefore have no
existing consumer that can classify the complete target-only family without a
new result-family authority. The package acceptance D0 forbids filtering,
partial installation, and target-only lowering.

**Decision:** close D1 as `NoSafeSlice__ResultFamilyOwnerAbsent`. The next
bounded design is the result-family authority census; I3 publication remains
queued until that design either selects an existing sibling owner or records a
named, observable reopening condition for a future result-family slice.

## Bounded design work

1. The finite target-only census is complete and recorded above.
2. The existing ExactI64 result owner cannot cover the family without a new
   result-family authority; filtering and partial package acceptance are
   rejected by the existing contract.
3. A typed outside-family terminal has no existing package owner that can
   preserve complete selected-call coverage, so it is not invented here.
4. The result-family authority census is delegated to
   `MIR-CALL-PARSER-PUBLICATION-RESULT-FAMILY-D0`; only after that design is
   accepted may the pointer return to I3 task 4. I3 task 5 and R0 task 6
   remain queued.

## Evidence and non-claims

The merged parser lifecycle test is 1/1 at the named no-selected-handoff
frontier. The temporary diagnostic code used for the row classification was
removed; no production source change is claimed by this card. The warning
cohort remains paused at I147 (`unused_imports=17`); `dead_code` remains owner
debt.
