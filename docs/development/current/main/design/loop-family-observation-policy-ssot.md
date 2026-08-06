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

## NestedPredicate S1 design-stop

`LOOP-FAMILY-NESTED-OBSERVATION-S1` is now a closed design boundary. The row
is a caller-zero source-to-disposition witness, not a Nested route admission
or a production plan.

### Decision

The sole source authority is
`compiler/nested_predicate_projection.rs::issue_nested_predicate_source_projection_v1`.
It observes the natural source once and returns the existing AST-free,
move-only `VerifiedNestedLoopSourceProjectionV1`. The S1 compiler adapter may
translate that projector's typed rejection into neutral source-attempt
outcomes, but the policy module must not import the compiler rejection enum,
read AST, or call any later producer.

The word `Candidate` at this row means **exact bounded NestedPredicate source
projection** only. It does not claim that root initializer values, physical
recurrence effects, Recipe relations, or MIR lowering are production-admitted.
Those later constraints remain owned by the future Nested demand/Recipe
producer. S1 must not copy the producer's `validate_shape` checks into the
policy; in particular, producer-only initializer-value and recurrence-role
checks are explicit later non-claims, not hidden S1 facts.

The five-row admission window remains the only cross-family authority. A
known G0-shaped or otherwise non-Nested source declines naturally when the
exact Nested projection does not match. The common assembler owns duplicate
family-tag/input-row rejection; the later selector owns semantic candidate
overlap rejection. The Nested observer never invents precedence or suppresses
another family.

### Authority map

| Product | Sole authority | Non-authority / forbidden use |
| --- | --- | --- |
| owner, function origin, source kind, root site, root frame | resolver/function input and resolved loop forest | route ID, fixture name, or policy-created identity |
| exact Nested source shape | `issue_nested_predicate_source_projection_v1` | `nested_predicate_producer.rs`, profile probe, topology, or physical cutover |
| portable forest binding | `bind_resolved_loop_source_forest_v1` | synthetic path reconstruction or Recipe key issuance |
| compiler-error to neutral reason | `cfg(test)` Nested source adapter | compiler error enum in route policy |
| mode and source-window coverage | sealed observation context | environment reads, fallback, or schedule inference |
| Nested family disposition | `loop_route_policy/nested_predicate_observation.rs` | `family_selection.rs`, legacy `policy.rs`, Builder/MIR, or route IDs |
| cross-family selection and overlap | common five-row admission window | per-family priority or retry |

### Sealed input and identity

The policy receives one AST-free move-only source attempt and one separately
sealed context. Both must agree on:

```text
owner + function origin + source kind + root site + root frame
Release | Strict | StrictPlannerRequired
Complete | Incomplete loop-window coverage
```

The candidate envelope carries the projection, its verified forest binding,
shape, and root frame. The adapter must validate candidate identity against the
context before publishing the attempt; the policy performs only the final
co-seal check and disposition. No source kind, frame, or owner is reconstructed
from a route label.

### Disposition matrix

| Source/context condition | S1 result |
| --- | --- |
| complete, sealed mode, same owner/origin/kind/site/frame, exact two-member root→child forest and the projector's exact body/condition/update/literal/binding shape | `Candidate(VerifiedNestedLoopSourceProjectionV1)` |
| complete, successfully looked-up source with known non-Nested forest cardinality/parent shape, root/child predicate shape, root/child body schedule, or known literal/update shape | `Declined(NotNestedPredicateShape)` |
| incomplete coverage, unsealed mode, source navigation failure, missing source lookup/forest root, or missing/opaque resolver fact | `Unresolved` |
| foreign owner, source-kind/forest-owner mismatch, root-frame/site mismatch, upvar, non-binding target, binding/initializer-binding mismatch, lexical-scope mismatch, duplicate/orphan/skipped/unsupported forest invariant, or context mismatch | `Rejected` |

`ForestShape` is a decline only when the resolver lookup succeeded and the
forest is known to be a different bounded shape. The current projector
collapses some lookup failures into `ForestShape`; the implementation slice
must split/preserve that information before the neutral adapter maps it. A
missing root is `Unresolved`; a malformed or conflicting resolved forest is
`Rejected`. S1 must never turn a lost source distinction into a permissive
decline.

All three modes produce the same Candidate for the exact complete projection.
Mode is an evidence snapshot, not a route selector. Incomplete or unsealed
coverage is never `Declined`, because unseen source can change the result.

### Implementation slice after this stop

The next implementation row is finite and shallow:

1. Preserve the `Forest` lookup/invariant distinction in the existing source
   projector (or add a sealed resolver-side forest adapter) without invoking
   Recipe/JoinSig/physical code.
2. Add the neutral AST-free Nested source-attempt DTO and a `#![cfg(test)]`
   compiler adapter that maps the matrix above.
3. Add one pure `loop_route_policy` observer and focused tests for all three
   modes, known declines, incomplete/unsealed unresolved inputs, identity and
   binding rejects, and forest lookup/invariant boundaries.
4. Extend the shared recursive authority/line/caller-zero guard; do not add a
   row-specific shell guard or a production caller.
5. Update the exact reference receipt, module README, current pointer, and
   workstream in the same implementation commit.

### LoopTrue S1 implementation receipt

The finite caller-zero slice is landed. The source projection now exposes the
AST-free identity `owner + function origin + source kind + loop site + frame`;
the neutral source-attempt/identity/mode/coverage DTO lives under
`loop_structural_facts`; and the `#![cfg(test)]` compiler adapter preserves the
typed projection disposition without exporting compiler enums to policy. The
pure `loop_route_policy` observer rechecks identity, frame, mode, and coverage
before issuing only `Candidate`, `Declined`, `Unresolved`, or `Rejected`.

Nine focused policy tests and eight projection tests are green, and the shared
recursive authority/line/caller-zero guard is green. The implementation commit
updates the exact reference matrix, compiler/structural-facts/policy READMEs,
current pointer, and workstream in the same commit. Selector, Recipe/JoinSig,
Builder/MIR, physical lowering, retry/fallback, production callers, and legacy
retirement remain closed.

### Stop lines

Nested S1 must not:

```text
call nested_predicate_profile.rs or probe_nested_predicate_source_unit_v1
call nested_predicate_producer.rs or issue a Recipe/JoinSig/BindingKey
call family_selection.rs or legacy loop_route_policy::policy
issue LoopRouteId, route schedule, cursor, retry, or fallback
enter Builder/MIR/ValueId/PHI/physicalization
trust producer-only synthetic role/visibility labels as independent facts
add a production caller or delete the old Nested route
```

The design stop is complete; implementation may begin only from the bounded
slice above. Its completion still requires a focused guard, reference update,
and explicit caller-zero evidence; no selection or production claim is
implied by this design closeout.

## NestedPredicate S1 implementation receipt

`LOOP-FAMILY-NESTED-OBSERVATION-S1` is now landed as a caller-zero,
AST-free source observation. The existing projector preserves
`ResolvedLoopSourceForestRejectV1` provenance instead of collapsing lookup and
invariant failures into `ForestShape`; the test-only compiler adapter maps
those typed errors into the neutral disposition matrix. The pure policy
observer consumes one sealed source attempt and one identity/mode/coverage
context and issues only `Candidate`, `Declined`, `Unresolved`, or `Rejected`.

Seven policy tests and eight projection tests are green, including missing
forest-root provenance. The shared recursive authority/line/caller-zero guard
is green. This receipt does not open selection, Recipe/JoinSig/BindingKey,
Builder/MIR/physicalization, retry/fallback, a production caller, or legacy
retirement; the next design boundary is the LoopTrue observation row.

## LoopTrue S1 design stop

`LOOP-FAMILY-LOOPTRUE-OBSERVATION-S1` is a worker-reviewed, caller-zero
design boundary. The executable task is
`docs/development/current/main/investigations/loop-family-looptrue-observation-s1-design-task-2026-08-06.md`.

### Decision

The sole syntax authority is
`compiler/loop_true_break_continue_projection.rs::issue_loop_true_break_continue_source_projection_v1`.
It observes the natural `FunctionSourceViewV1`, resolver-issued loop source,
BindingRef, region, and exit products once and returns an AST-free, move-only
projection. The existing `loop_route_policy/loop_true_break_continue.rs`
schedule/cursor/winner demand is a separate migration owner; it is forbidden
as an S1 input.

S1 adds a neutral source-attempt transport under `loop_structural_facts` and a
pure observer under `loop_route_policy`. The policy receives one attempt and
one separately sealed identity/mode/coverage context and emits only
`Candidate`, `Declined`, `Unresolved`, or `Rejected`. Candidate means exact
bounded source projection, not policy admission, Recipe readiness, or a
physical route.

The candidate identity contract is:

```text
owner + function origin + source kind + loop site + execution frame
```

The source projection must expose this identity through an AST-free accessor or
product before the observer can seal a candidate. An adapter-only precheck is
not the durable identity contract.

### Disposition matrix

| Source/context condition | S1 result |
| --- | --- |
| complete coverage, sealed mode, matching identity/frame, exact `loop(true)` + one explicit-else If + direct Break/Continue + Equal(local BindingRef, integer) + resolver exit targets | `Candidate` |
| known syntactic non-shape: root condition/body/branch arity, branch/else shape, non-Equal condition, or non-integer bound | `Declined(NotLoopTrueBreakContinueShape)` |
| incomplete/unsealed mode or coverage, source navigation/lookup failure, missing binding, missing exit fact | `Unresolved` |
| foreign identity/frame, mode mismatch, upvar, exit-target mismatch, source-binding owner/structural conflict, candidate/context mismatch | `Rejected` |

`ExitResolution` is a missing resolver fact and remains `Unresolved`; an
`ExitTargetMismatch` is a structural conflict and is `Rejected`. Root source
binding errors retain their typed owner/structural provenance. No source
distinction may be collapsed into a permissive decline. Release, Strict, and
StrictPlannerRequired are snapshots only; they do not select, suppress, retry,
or fallback.

### Finite implementation slice

1. Extend the source projection with the AST-free identity accessor/product and
   preserve source/lookup distinctions at the adapter boundary.
2. Add the neutral source-attempt/identity/mode/coverage DTO and a `#![cfg(test)]`
   compiler adapter; do not export compiler rejection enums to policy.
3. Add one pure policy observer and focused positive, three-mode, decline,
   incomplete/unsealed, identity/frame/mode, missing-fact, binding, and exit
   conflict tests.
4. Extend the existing shared recursive authority/line/caller-zero guard; no
   row-specific shell guard.
5. Update the exact reference receipt, module READMEs, current pointer, and
   workstream in the same implementation commit. That commit must explicitly
   record reference-document synchronization.

### Stop lines

S1 must not import `LoopRouteId`, frozen schedules/cursors, legacy policy
demand, `family_selection.rs`, Recipe/JoinSig/BindingKey, Builder/MIR/ValueId/
PHI, physical lowering, retry/fallback, or a production caller. The old
LoopTrue policy/Recipe/Builder route remains migration-only until a later
common selector and physical cutover prove zero non-historical callers.

## Common family admission D0 worker-reviewed design stop

`LOOP-FAMILY-COMMON-ADMISSION-WINDOW-D0` reconciles the existing Generic
five-row contract with the landed DirectAccum, NestedPredicate, and LoopTrue
caller-zero observations. The required semantic tags are exactly:

```text
DirectAccum
NestedPredicate
LoopTrueBreakContinue
LoopCondBreakContinue
GenericG0
```

The common boundary has two products and two owners, preceded by one resolver
source-brand prerequisite:

```text
resolver-issued AST-free WindowIdentityLease
  + exactly five family-tagged rows
  -> VerifiedLoopFamilyAdmissionWindowV1  (completeness/co-seal only)
  -> CanonicalLoopFamilySelectionV1       (sole future winner owner)
```

The resolver lease issuer is a separate caller-zero S0 product. The only
issuer is `VerifiedResolvedFunctionV1`, which consumes one exact
`VerifiedResolvedLoopSourceV1` lookup and wraps that non-`Clone` source token
in `VerifiedLoopFamilyWindowLeaseV1`. The lease owns only the function owner
and the resolver-branded source identity/frame; mode and coverage remain
policy-row evidence. The assembler never constructs a lease from loose
coordinates, clones/relooks up a source token, or imports the test-only
AST-bearing shared-window witness. S0 is recorded in
`loop-family-window-lease-issuer-s0-task-2026-08-06.md`; family projector
fan-out remains a later explicit resolver seam.

The window lease is a non-Clone identity brand issued once at the resolver
source seam. It contains no AST, names, route IDs, cursors, schedules, Recipe
data, or physical IDs. Because family projectors consume non-Clone source
capabilities, the resolver must issue an explicit family-scoped fan-out or
equivalent branded capability; the assembler may not clone/relookup source or
use the AST-bearing shared test window witness as production authority.

`FAMILY-ROW-CONTEXT-RETENTION-R0` is landed before assembler opening. All five
observer enums now retain a typed non-Clone evidence envelope on `Candidate`,
`Declined`, `Unresolved`, and `Rejected`; it contains both expected and
observed identity/mode/coverage. The source attempt is decomposed exactly once
before validation, so a mismatch retains both sides. The later typed-envelope
to common-row projection must remain lossless, with one owner for
family-specific to common mode/coverage conversion. An assembler failure must
retain consumed rows/lease evidence rather than return a bare reason.

Each row is normalized to exactly one of `Candidate`, `Declined`,
`Unresolved`, or `Rejected`, with family tag and typed provenance retained.
`Blocked` belongs only to the legacy schedule/policy vocabulary. Generic G0
requires a source-attempt normalization layer because its current policy lacks
`Declined` and its context lacks common origin/source-kind/site/frame identity.
LoopCond and Generic row normalization, plus the R0 evidence-retention
refactor, are now landed caller-zero products; the common assembler must
consume their evidence-retaining row envelopes. A
missing row is an assembler `Unresolved(MissingFamilyObservation)`, never a
synthetic `Declined`.

The window assembler validates exact five-tag coverage, duplicate/missing rows,
owner/origin/source-kind/site/frame equality, mode equality, and coverage. It
consumes all rows and either seals an arbitrary-order exact five-row product or
returns an evidence-bearing unresolved/rejected outcome retaining the lease
and every consumed row. It never counts candidates or selects. The later
selector consumes only the assembler's `Ready(window)` product exactly once;
non-Ready assembler outcomes never reach it. It applies only this
algebra:

```text
one Candidate + four Declined -> Selected
two or more Candidates        -> Rejected(Overlap) [selector only]
five Declined                 -> Unresolved(OutOfWindow) [selector only]
NoCandidate                   -> not an S2 outcome; M8 whole-unit proof only
```

For the assembler itself, Candidate rows are opaque typed payloads: one or
more Candidates (including two or more) plus Declined rows is `Ready`; overlap
and `OutOfWindow` are selector responsibilities. A row-level Rejected or any
unsealed/incomplete row evidence produces the corresponding top-level
Rejected/Unresolved result with all rows and the lease retained; it is not
passed to the selector. The selector's failure products retain the consumed
lease and all five Ready rows, but only carry `Overlap` or `OutOfWindow`.

The observation prerequisites are landed in a shallow ordered ladder inside
the linked D0 task: LoopCond observation, Generic row normalization, row
context retention R0, and resolver window lease issuer S0. The next bounded
cell was the common assembler; selector promotion remained a separate task
until the implementation receipt below.
Shared guard changes must use reusable helpers because the existing
logical-demand guard is already near the 800-line boundary.

## LoopCond observation S1 design closeout

Worker review closed `LOOP-FAMILY-LOOPCOND-OBSERVATION-D0` as a design stop.
The legacy `LoopCondBreakContinueFacts`/`Recipe` family is broad and
environment-gated; it remains migration-only and is not a source authority for
the new row. It must not be copied into the neutral boundary.

The first caller-zero observation slice is deliberately one bounded shape:

```text
loop(non-true supported condition) {
  if (supported condition) {
    break
  } else {
    continue
  }
}
```

The source projection may claim only the resolver-sealed loop/condition/branch
sites, direct exit roles and targets, owner/function-origin/source-kind/site/
frame identity, and complete source-window coverage. It does not claim boolean
operator policy, effect admissibility, carrier/update semantics, return paths,
nested loops, program containers, or physical lowering.

The neutral mapping is fixed:

| Source/context condition | LoopCond disposition |
| --- | --- |
| complete, sealed mode, matching identity/frame, exact bounded shape and matching loop exit targets | `Candidate` |
| complete lookup with a known root-true or known non-LoopCond shape, wrong body/branch arity or unsupported bounded syntax | `Declined(NotLoopCondBreakContinueShape)` |
| incomplete/unsealed coverage or mode, missing/opaque source navigation/region/binding/exit fact, or an unclassified legacy-like variant | `Unresolved` |
| foreign owner/origin/kind/site/frame, exit-target mismatch, conflicting/duplicate resolver evidence, or context mismatch | `Rejected` |

The observer is AST-free, move-only, and caller-zero. Its test adapter may
translate compiler projection errors, but policy must not import compiler error
enums. It must not import Builder/MIR/ValueId/PHI, Recipe/JoinSig/BindingKey,
route IDs, schedules/cursors, environment gates, retry/fallback, or the
production selector. Existing LoopCond planner/VM fixtures remain historical
evidence and are not S1 acceptance gates.

The next implementation slice is therefore finite: add one resolver-backed
LoopCond projection, one neutral attempt/observer, the focused C/D/U/R tests,
and one shared caller-zero guard extension. The same implementation commit
must update the reference matrix, observation SSOT, module READMEs, current
pointer/workstream, and record the post-implementation reference-document
synchronization. Broader LoopCond variants remain a later M8 cohort.

## LoopCond S1 implementation receipt

The bounded LoopCond observer is landed caller-zero. Its compiler projector
uses `FunctionSourceViewV1` plus the resolver-issued loop source token, then
emits only resolver-owned sites and typed direct-exit origin/target evidence.
The neutral source attempt and route-policy observer recheck owner/origin/
source-kind/site/frame identity, mode, and coverage before issuing exactly
`Candidate`, `Declined`, `Unresolved`, or `Rejected`.

The acceptance evidence is nine policy tests, five projection tests, and the
shared parameterized family-observer guard. The guard replaces the temporary
LoopTrue-only helper and covers both LoopTrue and LoopCond without extending
the near-800-line logical-demand guard. The implementation commit also synced
this SSOT, the stage matrix, compiler/structural-facts/route-policy READMEs,
current mirrors, and the workstream in the same commit; that is the required
post-implementation reference-document receipt.

No selector, common assembler, Generic normalization, Recipe/JoinSig,
Builder/MIR, physical route, retry/fallback, production caller, or legacy
LoopCond deletion is implied. The next bounded cell is the common five-family
admission assembler.

## Family row context-retention R0 implementation receipt

`FAMILY-ROW-CONTEXT-RETENTION-R0` is landed as a behavior-neutral BoxShape
refactor. The five typed observer modules retain expected and observed
identity/mode/coverage evidence on every disposition, expose one outer
`evidence()` accessor, and preserve typed reason/payload ownership. The
focused observation suite reports 89 passing tests; the shared caller-zero
guard rejects bare reason-only constructors and all changed files remain below
800 lines. No selector, Recipe, Builder/MIR, production caller, retry, or
legacy deletion was opened. The common admission assembler is the next owner.

## Window lease issuer S0 implementation receipt

`LOOP-FAMILY-WINDOW-LEASE-ISSUER-S0` is landed as a separate caller-zero
resolver source-brand product. `VerifiedResolvedFunctionV1` issues one
non-`Clone`/non-`Copy` `VerifiedLoopFamilyWindowLeaseV1` only from an exact
`VerifiedResolvedLoopSourceV1` lookup. The lease retains the resolver source
token and owner brand and exposes only origin/source-kind/site/frame identity;
mode and coverage remain policy-row evidence. The focused lease suite has
three passing tests, the shared in-place guard is green, and the lease has no
AST, forest, route, Recipe, Builder, MIR, selector, retry, or production
authority. The common admission assembler is the next owner.

## Generic G0 row-normalization S1 implementation receipt

`GENERIC-G0-ROW-NORMALIZATION-S1` is landed caller-zero. The test-only
compiler adapter consumes the existing S0A/S0B/S0C products exactly once and
maps their typed errors into a neutral source-attempt algebra. The neutral
structural-facts transport carries owner, origin, source kind, root loop site,
execution frame, mode, and coverage; the route observer rechecks that identity
and maps the source attempt into exactly `Candidate`, `Declined`, `Unresolved`,
or `Rejected` before calling the existing Generic policy issuer.

The adapter requires an explicit `NumericTarget`; no target is inferred from
the source or global environment. Known non-G0 syntax is `Declined`, missing
or opaque source/type/numeric facts are `Unresolved`, and foreign/conflicting
identity or typed facts are `Rejected`. The current S0B/S0C `ForestShape` and
`BindingLookup` ambiguity is deliberately preserved as conservative
`Unresolved`; a resolver-side distinction is a later bounded row, not guessed
here. Twelve adapter tests, seven policy tests, `cargo check --lib`, and the
shared caller-zero/line guard are green. Selector, common admission assembly,
Recipe/JoinSig, Builder/MIR, physical lowering, retry/fallback, production
callers, and legacy retirement remain closed.

## Selector S2 caller-zero implementation receipt (2026-08-06)

`LOOP-FAMILY-SELECTOR-S2-IMPLEMENTATION` is closed for its bounded
caller-zero cell. `family_selector.rs` consumes only an assembler
`Ready(window)` by value and moves the five typed rows without clone,
relookup, or reconstruction. Its complete output algebra is:

```text
1 Candidate + 4 Declined -> Selected
2+ Candidates            -> Rejected(Overlap)
5 Declined               -> Unresolved(OutOfWindow)
```

Assembler `Rejected`/`Unresolved` evidence remains terminal before this
boundary, and `NoCandidate` remains an M8 whole-unit proof. The selector keeps
the source lease and common mode/coverage on success; both failure products
keep the lease and every row. Focused tests cover all five candidates,
overlap, and retained five-row `OutOfWindow` evidence. The selector guard and
the shared caller-zero guards are green, with every touched source/test file
below 800 lines.

This receipt does not open Recipe/JoinSig, Builder/MIR, physical lowering,
production selection, retry removal, or legacy deletion. D4-S4-S0 remains a
NoSafeSlice audit. The bounded
`GENERIC-SELECTION-OPEN-D0-I0-R0` candidate-envelope witness is closed.
The next shallow design boundary is
`GENERIC-SELECTION-POLICY-HANDOFF-D0`, recorded in
`docs/development/current/main/investigations/generic-selection-policy-handoff-d0-design-task-2026-08-06.md`.
Any later implementation commit synchronizes `docs/reference/**`, the
reference matrix, module README, workstream, and current mirrors together.
