---
Status: closeout__2026-09-22__NoSafeSliceResultFamilyOwner
Task: MIR-CALL-PARSER-PUBLICATION-RESULT-FAMILY-D0
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-publication-preflight-d1-2026-09-22.md
Implementation permission: false; retain the named NoSafeSlice until the static-result authority D1 is designed
NextCard: mir-call-parser-static-result-authority-d1-2026-09-22.md
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

The target-only rows are not one homogeneous result family. The finite
classification is `UnknownExpression=19`,
`StaticCallTargetAuthorityUnavailable=20`, `StaticCallResultUnavailable=7`,
`KnownNonI64Return=8`, `RecursiveDependency=2`, and
`UnsupportedStatementKind=1`. This partition is part of the D0 input and must
remain keyed by exact caller/site/target identity.

## Existing authority audit

`VerifiedCallableResultRepresentationV1` exposes both `ExactI64` and
`ExactNominalBox`; the catalog also retains typed `Unavailable` reasons.
`VerifiedStaticCallResultPublicationOwnerV1` can consume a general source-call
row with either representation, but its no-general-row projection
`project_static_exact_i64_requirement_v1` admits only `ExactI64` and otherwise
returns `TargetResultUnavailable`, which becomes `TargetOnly` here. The
`CoreMethod` result-kind table is a separate bound-receiver authority and
cannot classify same-module static targets. The existing source-result design
for String/conditional values may be a candidate sibling, but it is not yet a
callable publication owner.

The decision must therefore distinguish these options without silently
changing the ExactI64 contract:

1. reuse an existing source-result owner if it can carry exact caller/site,
   target, result class, effect, and physical consumer relations;
2. select a sibling result-family authority and define its co-seal boundary;
3. record `NoSafeSlice` with a finite reopen trigger if neither exists.

Widening the ExactI64 enum in place, inferring from MIR, or treating
target-only as outside-family is not an accepted option in this D0.

## Required design work

1. Preserve the six-way disposition partition above and split each class by
   physical need, retaining caller/site/target and package brand.
2. Compare `UnknownExpression` and target/result-authority gaps with the
   existing source-result and source-target owners; do not relabel them as a
   String/nominal result merely because the target is a static method.
3. For the `KnownNonI64Return` and any nominal result rows, verify whether an
   existing general source-call row and physical nominal consumer can be
   co-sealed. A catalog enum alone is not an owner.
4. Keep recursive and unsupported rows as named typed terminals unless an
   existing source owner supplies the missing relation; no retry or default.
5. If no owner covers the finite inventory without a new semantic ABI, close
   with `NoSafeSlice` and record the exact reopening owner. Do not code.
6. If an owner is selected, return the pointer to I3 task 4 with an explicit
   implementation card; task 5 caller switch and R0 task 6 retirement stay
   queued.

## D0 closeout decision — 2026-09-22

The six-class census and existing-owner comparison are complete. No existing
owner safely covers all 57 target-only rows as a source-loop publication
family. The callable result catalog records `ExactI64`, `ExactNominalBox`,
and typed `Unavailable` reasons, but the publication owner has no consumable
source call row for these target-only sites. The source-result product records
source classes and route observations, but has no production caller/site/
target/result/effect publication consumer. `CoreMethod` remains a bound
receiver authority.

**Decision:** close D0 as
`NoSafeSlice__ResultFamilyOwnerAbsent`. The missing chain is
`callee body proof -> exact source call-site result -> representation/effect/
ABI -> physical publication or typed pre-effect terminal`. D1 now owns that
bounded static-result authority design. Do not filter target-only rows, widen
`ExactI64` in place, or advance I3 publication.

**Reopen trigger:** a bounded owner must co-seal exact caller/site/target and
package brand, preserve the six-way disposition or a justified finite subset,
and name a physical consumer or typed terminal for every row.

## Queue handoff

The warning cohort is intentionally closed at I147: `unused_imports=17` is
the measured mechanical result, while `dead_code` remains owner debt. Do not
spend the semantic lane's design-stop time on another warning sweep.

After the successor D1 has an accepted finite owner (or a named
`NoSafeSlice`), the queue is fixed:

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
