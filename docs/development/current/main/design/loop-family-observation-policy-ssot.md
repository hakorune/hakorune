---
Status: SSOT
Date: 2026-08-06
Scope: caller-zero Loop family observation rows before the common admission window.
Related:
  - docs/development/current/main/design/generic-loop-source-to-portable-recipe-ssot.md
  - docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - docs/reference/mir/generic-loop-stage-matrix.md
  - src/mir/loop_route_policy/README.md
---

# Loop family observation policy

## Decision

`LOOP-FAMILY-DIRECT-OBSERVATION-S1` is accepted as a caller-zero observation
row. It adapts the existing resolver-branded DirectAccum structural product
into one typed family disposition. It does not select a winner, issue a
Recipe/key, or enter Builder/MIR.

The S1 boundary has two products:

```text
resolver/source adapter
  -> AST-free VerifiedDirectAccumSourceAttemptV1
policy observer
  -> DirectAccumFamilyObservationV1
```

The source adapter is the only place that may translate compiler projection
errors into neutral source-attempt reasons. The policy observer never imports
`DirectAccumProjectionRejectV1`, reads AST, or reconstructs source facts.

## Authority map

| Product | Sole authority | Must not own |
| --- | --- | --- |
| source identity/site/owner/frame | resolver products | route IDs or AST-local identity |
| AST -> DirectAccum facts | `compiler/direct_accum_projection.rs` | policy, Recipe, Builder/MIR |
| structural/disjointness proof | `loop_structural_facts` | mode policy or route selection |
| source-attempt reason mapping | test-only compiler adapter | legacy error enums in policy |
| family disposition | `loop_route_policy/direct_accum_observation.rs` | schedule, winner, Recipe, physical IDs |
| later family selector | admission-window S2 | profile-specific shape rechecking |

The legacy `loop_route_policy/policy.rs` evaluator, frozen schedule, raw
cursor, `probe_direct_accum_source_unit_v1`, and live DirectAccum plan remain
migration/live owners. They are not inputs to this observation row.

## Sealed input

`VerifiedDirectAccumSourceAttemptV1` is AST-free and move-only. It contains one
of the following neutral source outcomes plus the resolver-issued identity
(`owner`, `frame`, source kind/site), exact mode snapshot, and loop-window
coverage. The policy receives it together with a separately sealed context;
both must agree before shape disposition is considered.

```text
Candidate(VerifiedDirectAccumSingletonObservationV1)
Declined(DirectAccumSourceDeclineV1)
Unresolved(DirectAccumSourceUnresolvedV1)
Rejected(DirectAccumSourceRejectV1)
```

The candidate reuses the existing non-Clone
`VerifiedDirectAccumSingletonObservationV1`. Non-candidate source outcomes do
not carry a fake DirectAccum payload.

The context records exactly:

```text
owner + source/frame identity
Release | Strict | StrictPlannerRequired
Complete | Incomplete loop-window coverage
```

Loose owner/site/frame constructors, route labels, fixture names, and
environment reads inside the policy are forbidden. An unsealed mode or
incomplete coverage is not a decline: the unseen source may change the
profile result.

## Disposition matrix

| Input condition | S1 result |
| --- | --- |
| complete, same identity/mode, exact Less + two Add assignments + verified BindingRef/disjointness | `Candidate` |
| complete, known non-Direct shape (nested forest/body arity/condition/update/step/known literal shape) | `Declined(NotDirectAccumShape)` |
| incomplete coverage, missing source lookup/navigation, missing/opaque fact, unsealed mode | `Unresolved` |
| foreign owner/source/frame, source-kind mismatch, upvar/non-binding target, BindingRef mismatch/collision, duplicate/stale receipt, context mismatch | `Rejected` |

`StrictPlannerRequired` is retained as an exact mode snapshot. For the
complete canonical DirectAccum source it produces the same `Candidate` as
Release and Strict; it is never converted into Generic suppression,
fallback, or a legacy route decision.

`Declined` means “this profile is not present”; it is not `NoCandidate` for the
whole unit and it does not authorize NestedPredicate or another family.

## Implementation slice

1. Add a neutral source-attempt product and a test-only source adapter that
   maps the existing DirectAccum projector output without leaking compiler
   reject enums.
2. Add `loop_route_policy/direct_accum_observation.rs` with one sealed context,
   one move-only candidate envelope, and typed Candidate/Declined/Unresolved/
   Rejected outcomes.
3. Add focused positive, known-decline, incomplete/unsealed, foreign/frame,
   and source-reason tests. Keep the issuer caller-zero.
4. Extend the shared recursive guard for AST/Builder/MIR/legacy-schedule
   absence, one policy module, focused test count, and caller-zero references.
5. Update README, reference matrix, current pointer, and workstream in the
   same commit. The implementation commit must update the exact reference
   receipt; no later documentation debt is allowed.

## Stop lines

S1 must not:

```text
call family_selection.rs
call issue_direct_accum_route_admission_v1
read LoopRouteId, schedule, cursor, or legacy policy evidence
issue Recipe/JoinSig/BindingKey demand
enter Builder/MIR/ValueId/PHI/physicalization
add retry, fallback, or production caller
delete the old DirectAccum route
```

## S1 implementation receipt

The DirectAccum S1 slice is landed in the source/structural/policy modules:
the compiler adapter is `#![cfg(test)]`, the policy observer has no legacy or
physical imports, and seven focused tests cover the positive three-mode,
known-shape, incomplete/unsealed, foreign-context, and typed source-reject
boundaries. The shared recursive authority/line guard and the reference/current
mirrors are updated in the same change. The next design boundary is
`LOOP-FAMILY-NESTED-OBSERVATION-S1`; no selector or production activation is
implied by this receipt.

The next row is the common five-family admission window. It may consume this
typed disposition but owns selection and overlap handling itself.
