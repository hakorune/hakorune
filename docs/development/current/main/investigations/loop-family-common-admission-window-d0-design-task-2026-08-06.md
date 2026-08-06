---
Status: Closed worker-reviewed design / common assembler S1 landed / selector next
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

The common window is incomplete, not complete, while a required family row is
unavailable. The assembler must retain
`Unresolved(MissingFamilyObservation)`; it may not synthesize a `Declined` row
or promote a partial pilot to the canonical window. Generic normalization,
`FAMILY-ROW-CONTEXT-RETENTION-R0`, and the resolver
`LOOP-FAMILY-WINDOW-LEASE-ISSUER-S0` are landed caller-zero products. R0
preserves expected/observed metadata on every family disposition and S0 now
supplies the non-`Clone` source brand, so the common assembler may open.

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

1. The resolver-owned lease issuer is a separate caller-zero prerequisite;
   the assembler may consume only its non-`Clone` lease, never loose identity
   coordinates.
2. Exactly one `Candidate` plus four exact `Declined` rows is selector-ready
   and yields `Selected(family candidate)` only in the later selector.
3. Two or more candidates yield `Rejected(Overlap)` only in that selector;
   family priority, route order, cursor, or retry is forbidden everywhere.
4. Any `Rejected` yields top-level `Rejected` while retaining every row,
   including unresolved provenance. With no rejection, any `Unresolved` yields
   `Unresolved`; five `Declined` rows remain `Unresolved(OutOfWindow)` only in
   the selector rather than whole-unit `NoCandidate`.
5. Missing/duplicate family tags, foreign identity/frame, mode mismatch, and
   contradictory row provenance reject; incomplete/unsealed coverage remains
   unresolved. `Blocked` is legacy schedule vocabulary, not a common-row
   disposition.
6. The common products remain caller-zero and import no AST, Builder/MIR,
   Recipe/JoinSig/BindingKey, route ID, schedule/cursor, retry, or fallback.

## Ordered finite ladder

The next work is intentionally a shallow four-cell ladder inside this task;
the cells are separate acceptance shapes and commits:

```text
A  LOOP-FAMILY-LOOPCOND-OBSERVATION-D0/S1
   worker-reviewed source authority and bounded neutral row design, then one
   caller-zero observer/fixture; landed with 9 policy + 5 projection tests
B  GENERIC-G0-ROW-NORMALIZATION-S1
   one source-attempt adapter supplying common identity and explicit Declined
C  FAMILY-ROW-CONTEXT-RETENTION-R0
   all five observer dispositions retain expected/observed identity, mode,
   coverage, and typed reason/payload without clone or relookup; landed
S0 LOOP-FAMILY-WINDOW-LEASE-ISSUER-S0
   resolver-only exact source lookup -> non-Clone window identity lease;
   landed as a separate caller-zero source-brand product
D  LOOP-FAMILY-COMMON-ADMISSION-ASSEMBLER-S1
   consume the S0 lease plus arbitrary-order five typed rows; co-seal only,
   retain all evidence on unresolved/rejected outcomes; selector remains closed
```

### Cell A — LoopCond observation design and S1 (landed)

Worker review fixed the sole source authority, exact bounded shape, neutral
disposition mapping, identity/frame contract, and caller-zero acceptance
boundary. The implementation slice is now taskized; no production selection is
opened by this closeout.

The first accepted source shape is exactly:

```text
loop(non-true supported condition) {
  if (supported condition) {
    break
  } else {
    continue
  }
}
```

The AST-free projection carries only resolver-sealed loop/condition/branch
sites, direct exit roles/targets, owner/function-origin/source-kind/site/frame,
and source-window coverage. It does not import or preserve legacy
`LoopCondBreakAcceptKind`, `LoopCondBreakContinueFacts`, recipes, route IDs, or
planner/environment policy.

The C/D/U/R matrix is fixed: exact complete matching shape is `Candidate`;
known root-true/non-shape/body/branch mismatch is `Declined`; missing or
unsealed source/region/binding/exit evidence, incomplete coverage, and
unclassified legacy-like variants are `Unresolved`; foreign identity/frame,
exit-target conflict, duplicate/conflicting resolver evidence, and context
mismatch are `Rejected`. A missing broader variant must never be synthesized as
`Declined` merely to complete the common five-row window.

The resolver-backed projection, neutral attempt/observer, focused C/D/U/R tests,
and reusable shared guard extension are landed. Nine policy tests and five
projection tests are green. The implementation commit updated the exact
reference documents, current mirrors, and workstream; post-implementation
reference synchronization is recorded in that same commit.

Cell A is now closed as a worker-reviewed design and implementation slice. Cell
B and Cell C are also closed. R0 retains the expected/observed evidence on all
four dispositions and its shared guard rejects bare reason-only constructors.
Cell D is now the next bounded implementation row; selector promotion is a
separate S2 task. Shared guards must be extended through
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

## Cell C — row-context retention prerequisite

`FAMILY-ROW-CONTEXT-RETENTION-R0` was a BoxShape refactor, not a new accepted
source shape. Each of the five `FamilyObservationV1` enums must retain a typed
non-Clone evidence envelope on `Candidate`, `Declined`, `Unresolved`, and
`Rejected`. The envelope keeps expected and observed identity/mode/coverage;
the source attempt is decomposed exactly once before early-return validation so
foreign, mode-mismatch, incomplete, and candidate-policy errors retain both
sides of the evidence. The later typed-envelope-to-common-row projection must
be lossless and use one owner for family-specific to common mode/coverage
conversion. A shared guard rejects bare reason-only constructors, and focused
tests cover non-Candidate mismatch retention. The implementation landed with
89 focused observation tests, the shared guard, and synchronized reference
documents/current mirrors. The next open cell is the common assembler.

## Cell S0 — resolver window lease issuer

`LOOP-FAMILY-WINDOW-LEASE-ISSUER-S0` is landed as a separate caller-zero
source-brand product. `VerifiedResolvedFunctionV1` consumes one exact
`VerifiedResolvedLoopSourceV1` lookup and issues a non-`Clone`/
non-`Copy` `VerifiedLoopFamilyWindowLeaseV1` retaining only owner and the
resolver source token. Mode and coverage remain policy-row evidence. Focused
issuer tests and the shared in-place guard are green; no family fan-out,
assembler, selector, Recipe, Builder/MIR, or production caller was opened.
The next bounded row is the common assembler.

## Cell D — common admission assembler S1 (landed)

`LOOP-FAMILY-COMMON-ADMISSION-ASSEMBLER-S1` is now landed as a caller-zero
route-policy product in `src/mir/loop_route_policy/family_admission.rs`.
`assemble_loop_family_admission_window_v1` consumes one resolver-issued
`VerifiedLoopFamilyWindowLeaseV1` and an arbitrary-order move-only
`Box<[LoopFamilyObservationRowV1]>`. It validates exact five-tag coverage,
duplicate/missing rows, lease-vs-expected/observed identity and frame, common
mode, complete coverage, and row disposition without reissuing or relooking
up source evidence.

Success canonicalizes into fixed DirectAccum, NestedPredicate,
LoopTrueBreakContinue, LoopCondBreakContinue, and GenericG0 fields. Failure
outcomes retain the consumed lease, every input row, and typed issues; any
rejection dominates unresolved evidence, while unsealed/incomplete evidence
stays unresolved. Candidate payloads remain opaque and are not counted, so
semantic overlap and `OutOfWindow` remain selector-only.

Six focused assembler tests cover exact canonicalization, one-candidate
acceptance, missing rows, duplicate tags, foreign identity, and unsealed mode.
The shared guard now
protects the assembler/test boundary and extends the lease/observer caller
allowlists. Reference READMEs, the stage matrix, current mirrors, and this
task receipt were updated in the same implementation commit. Selector
promotion, Recipe/Builder/MIR production, physical cutover, and legacy
retirement remain closed; the next bounded cell is selector design/consumer.

## Stop lines

Do not implement or edit the legacy 19-route evaluator,
Recipe/JoinSig/BindingKey, Builder/MIR, physical route IDs, retry/fallback, or
production callers from this task. A worker-reviewed decision, a source-to-
neutral disposition matrix, the landed row-context receipt, and the S0 lease
issuer receipt are recorded in the design SSOT. The common assembler is the
only next implementation scope.
