---
Status: design_stop__2026-09-22__ResultFamilyAuthority
Task: MIR-CALL-PARSER-PUBLICATION-RESULT-FAMILY-D0
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-publication-preflight-d1-2026-09-22.md
Implementation permission: false; select or explicitly park the result-family owner
NextCard: mir-call-parser-loopbreak-composite-source-cutover-i3-2026-09-22.md
---

# Parser publication result-family authority D0

## Six-line brief

```text
Decision: define how the complete merged parser package classifies static-call
  results beyond the existing ExactI64 publication family.
Source authority + canonical issuer: same-invocation source target/result
  products and one owner-branded result-family profile selected by this D0.
Non-authority: MIR ValueId types, AST/name inference, target-only filtering,
  partial package installation, GenericLoop retry, VM, or compatibility paths.
Fail-fast boundary: every static target row receives one typed result-family
  disposition before publication acceptance; missing/foreign/duplicate rows
  remain named rejects.
Smallest next slice: classify the finite 57-row target-only census and choose
  one existing sibling owner or a named NoSafeSlice; no ABI/code changes here.
Non-claims: publication, caller switch, old-edge deletion, and warning work.
```

## Input inventory

The preceding D1 census covers the merged parser import closure:

| boundary | observed inventory |
| --- | ---: |
| target-only rows | 57 |
| caller keys | 18 |
| caller-to-target pairs | 31 |
| rows under `LoopBody` | 10 |
| callers with a `LoopBody` target-only row | 5 |

The selected `ParserProgramBox.parse/2 -> ParserStringUtilsBox.starts_with/3`
rows are `ExactI64` and remain the later acceptance tuple. The same caller has
target-only calls such as `RuneContractBox.invalid_placement_tag/1` and
`ParserDeclarationBox.parse_or_null/3`; these cannot be dropped while claiming
whole-package coverage.

## Existing authority audit

`VerifiedCallableResultRepresentationV1` currently exposes only `ExactI64`,
and `VerifiedStaticCallResultPublicationOwnerV1` emits either a selected
ExactI64 handoff or `TargetOnly`. The `CoreMethod` result-kind table is a
separate bound-receiver authority and cannot classify same-module static
targets. The existing source-result design for String/conditional values may
be a candidate sibling, but it is not yet a callable publication owner.

The decision must therefore distinguish these options without silently
changing the ExactI64 contract:

1. reuse an existing source-result owner if it can carry exact caller/site,
   target, result class, effect, and physical consumer relations;
2. select a sibling result-family authority and define its co-seal boundary;
3. record `NoSafeSlice` with a finite reopen trigger if neither exists.

Widening the ExactI64 enum in place, inferring from MIR, or treating
target-only as outside-family is not an accepted option in this D0.

## Required design work

1. Partition all 57 rows by source result class and physical need, preserving
   caller/site/target identity and package brand.
2. Compare each partition with existing source-result, callable-result, and
   CoreMethod owners; identify the first missing relation rather than adding a
   default or empty disposition.
3. Choose one issuer and one consumer for any admitted non-ExactI64 family,
   including duplicate, foreign, missing, result-site, and residual guards.
4. If no owner can cover the finite inventory without a new semantic ABI,
   close with `NoSafeSlice` and record the exact reopening owner. Do not code.
5. If an owner is selected, return the pointer to I3 task 4 with an explicit
   implementation card; task 5 caller switch and R0 task 6 retirement stay
   queued.

## Queue handoff

The warning cohort is intentionally closed at I147: `unused_imports=17` is
the measured mechanical result, while `dead_code` remains owner debt. Do not
spend the semantic lane's design-stop time on another warning sweep.

After this D0 has an accepted result-family owner (or a named `NoSafeSlice`),
the queue is fixed:

1. I3 task 4 — publish the selected composite LoopBreak source row through
   the existing one-shot owner.
2. I3 task 5 — switch the selected parser caller after the publication guard
   is green.
3. R0 task 6 — prove caller-zero, delete the exclusive old edge, and retain a
   re-entry guard.
4. Resume the `dead_code` owner sweep only after the semantic sequence closes,
   unless a newly selected owner-specific warning blocks that sequence.

## Evidence and non-claims

The merged parser lifecycle test is 1/1 at the named
`static-publication/no-selected-handoff` frontier. The target-only census was
collected through a temporary test-only diagnostic and the diagnostic was
removed immediately afterward; the worktree has no production code change.
This card claims only an authority design decision, not publication or
source-to-MIR success. The warning cohort remains paused at I147.
