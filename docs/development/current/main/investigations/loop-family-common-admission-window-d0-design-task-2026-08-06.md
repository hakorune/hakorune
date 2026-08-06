---
Status: Closed worker-reviewed design / next S1 task
Date: 2026-08-06
Decision: LOOP-FAMILY-COMMON-ADMISSION-WINDOW-D0
Authority: docs/development/current/main/design/loop-family-observation-policy-ssot.md
---

# Common Loop-family admission-window design task

## Purpose

Define the single cross-family boundary that may consume the five required
caller-zero observation rows: Generic G0, DirectAccum, NestedPredicate,
LoopTrueBreakContinue, and LoopCondBreakContinue. This design consultation is
now worker-reviewed and closed. No production caller, Recipe, Builder/MIR,
physical lowering, retry/fallback, or legacy retirement is authorized by D0.

## Inputs that may be considered

Each of the five required families contributes one sealed neutral disposition:

```text
Candidate
Declined
Unresolved
Rejected
```

The common window is incomplete, not complete, while LoopCond has no neutral
observer. The assembler must retain
`Unresolved(MissingFamilyObservation::LoopCond)`; it may not synthesize a
LoopCond `Declined` row or promote a four-row pilot to the canonical G0
window. Generic also needs a source-attempt normalization layer because its
current policy outcome has no explicit `Declined` arm and its context lacks the
common origin/source-kind/site/frame identity.

The source lease is a resolver-issued, AST-free, non-Clone window identity
brand. It is distinct from each family candidate capability. The assembler
co-seals already-issued row identities against that brand; it never rereads
AST/resolver products, reconstructs sites by name, or moves one non-Clone
source token into multiple rows.

Two products are required:

```text
VerifiedLoopFamilyAdmissionWindowV1
  = one window lease + exact five family-tagged typed rows
  = completeness/identity/mode/coverage only; no selection

CanonicalLoopFamilySelectionV1
  = consumes the sealed window once; sole winner authority
```

The common boundary must consume the sealed source identity and provenance
already carried by each observation. It must not reread AST, re-resolve source,
inspect the legacy schedule/cursor/winner demand, or infer a route from a
family name. `Unresolved` and `Rejected` provenance must remain distinct.

## Closed decision

1. Exactly one `Candidate` plus four exact `Declined` rows is the only
   selector-ready input and yields `Selected(family candidate)`.
2. Two or more candidates yield `Rejected(Overlap)`; family priority, route
   order, cursor, or retry is forbidden.
3. Any `Rejected` yields top-level `Rejected` while retaining every row,
   including unresolved provenance. With no rejection, any `Unresolved` yields
   `Unresolved`; five `Declined` rows remain `Unresolved(OutOfWindow)` rather
   than whole-unit `NoCandidate`.
4. Missing/duplicate family tags, foreign identity/frame, mode mismatch, and
   contradictory row provenance reject; incomplete/unsealed coverage remains
   unresolved. `Blocked` is legacy schedule vocabulary, not a common-row
   disposition.
5. The common products remain caller-zero and import no AST, Builder/MIR,
   Recipe/JoinSig/BindingKey, route ID, schedule/cursor, retry, or fallback.

## Ordered finite ladder

The next work is intentionally a shallow three-cell ladder inside this task;
the cells are separate acceptance shapes and commits:

```text
A  LOOP-FAMILY-LOOPCOND-OBSERVATION-D0
   worker-reviewed source authority and bounded neutral row design, then one
   caller-zero observer/fixture
B  GENERIC-G0-ROW-NORMALIZATION-S1
   one source-attempt adapter supplying common identity and explicit Declined
C  LOOP-FAMILY-COMMON-ADMISSION-ASSEMBLER-S1
   resolver window brand, exact five-tag co-seal, row retention, no selection
```

### Cell A — LoopCond observation design

This is the current design blocker. Before implementation, fix the sole source
authority, exact bounded shape, neutral disposition mapping, identity/frame
contract, and one caller-zero acceptance fixture for LoopCond. Worker review is
required before the observer is taskized as an implementation slice.

Cell A is the current blocker and must receive its own worker design audit
before implementation. Cell B cannot infer Declined from a missing Generic
row. Cell C cannot claim a complete window until A and B are closed. Selector
promotion is a separate S2 task. Shared guards must be extended through
reusable helpers, not by copying another large block into the 780-line
logical-demand guard.

The resolver/source seam must issue the AST-free window brand and family-scoped
capabilities in one explicit fan-out operation. The assembler may not clone or
relookup a non-Clone `VerifiedResolvedLoopSourceV1`, and the existing
AST-bearing shared source-window test witness is not a production authority.

The smallest C witness is one natural source receipt with five typed rows:
one exact LoopTrue candidate, typed declines/unresolved rows for the other
families, and `LoopCond = Unresolved(MissingFamilyObservation)`. It proves the
window is not selector-ready without inventing a winner.

## Stop lines

Do not implement or edit the legacy 19-route evaluator, `family_selection.rs`,
Recipe/JoinSig/BindingKey, Builder/MIR, physical route IDs, retry/fallback, or
production callers from this task. A worker-reviewed decision, a source-to-
neutral disposition matrix, and one bounded acceptance fixture are now recorded
in the design SSOT. The S1 task is the only next implementation scope.
