# Generic Loop Baseline V1 — Taskboard

Status: Superseded by Resolved Region Flow V1; generic baseline remains the loop consumer.
Date: 2026-07-13
Decision: `generic_baseline_plus_optional_canonical`.
Final retirement target: mandatory single-`loop_var` family = 0 callers.

## Purpose

Superseding taskboard:

```text
docs/development/current/main/investigations/
  mirbuilder-resolved-region-flow-v1-task-2026-07-13.md
```

Make complete-body generic semantics the correctness baseline, move canonical
step extraction to an optional proven optimization, then retire the legacy
single-progression skeleton:

```text
generic correctness:
  condition
  + complete body Recipe
  + loop-carried binding set
  + ports
  + exact source coverage

optional optimization:
  exact canonical-step proof
```

One progression owner is not required to execute a generic loop.  Condition
variables are ordinary operands.  SSA still requires a set of loop-carried
bindings and verified edge merges.

## Decision lock

```text
generic acceptance requires selected loop_var          = 0
generic acceptance requires selected step              = 0
generic body statement filtering                       = 0
generic synthetic/implicit step                        = 0
generic candidate Unique/Zero/Multiple rejection       = 0

canonical specialization requires verified baseline   = 1
canonical specialization is optional                   = 1
Lower-time fallback                                    = 0
```

`GenericBaseline` and `CanonicalSpecialized` are explicit planning choices.
`NotApplicable` optimization is not a semantic failure or fallback.

## Canonical authority

```text
source loop semantics:
  canonical Loop AST condition + ordered body

condition execution:
  VerifiedConditionPlanV1

body execution:
  VerifiedCompleteBodyRecipeV1

source coverage:
  VerifiedSourceRecipeProjectionV0

loop-carried bindings:
  VerifiedLoopStateClosureV1

ports:
  VerifiedLoopPortContractV1

generic acceptance:
  VerifiedGenericLoopRecipeV1

optional canonical optimization:
  VerifiedCanonicalLoopOptimizationV0

final Lower input:
  VerifiedLoopLoweringPlanV1
```

## Generic verified types

```text
VerifiedGenericLoopRecipeV1 {
  condition: VerifiedConditionPlanV1,
  body: VerifiedCompleteBodyRecipeV1,
  source_coverage: VerifiedSourceRecipeProjectionV0,
  loop_state: VerifiedLoopStateClosureV1,
  ports: VerifiedLoopPortContractV1,
}

VerifiedLoopLoweringPlanV1 =
  Generic(VerifiedGenericLoopRecipeV1)
  | Canonical(
      VerifiedGenericLoopRecipeV1,
      VerifiedCanonicalLoopOptimizationV0,
    )
```

Neither type requires `loop_var`, `loop_increment`, candidate rank, or a
body-managed cursor role.

## Generic acceptance contract

Ready requires all of:

```text
complete condition lowering
complete body Recipe
every executable source statement accounted exactly once
all backedge loop-state merges closed
fallthrough / continue / break / return ports closed
supported generic while skeleton
```

Generic outcome:

```text
Ready(VerifiedGenericLoopRecipeV1)
Unsupported(path, reason)
InternalProjectionContractViolation
InternalRecipeContractViolation
InternalLoopStateContractViolation
InternalPortContractViolation
```

Generic outcome has no candidate ambiguity or missing-loop-variable reason.

## Condition operands

All values read by the condition are ordinary operands:

```text
body-mutated binding:
  current header PHI value

body-unmodified binding:
  loop-invariant entry value
```

Evaluation order, short-circuit behavior, call order, and once-per-header-entry
frequency follow source semantics.  `CondProfile` remains diagnostic or
optimization observation, never generic acceptance authority.

## Loop-state closure

The first conservative carrier set is:

```text
all bindings that exist before loop entry
and may be reassigned by any body path
```

Rules:

```text
condition operand + body mutation       -> carrier
body mutation + later/after-loop use     -> carrier
condition-only invariant                 -> not carrier
body-local declaration                   -> iteration-local
field/array mutation                      -> effect, not binding reassignment
path without assignment                  -> carry incoming value
```

No selected binding is removed from the common carrier mechanism.

## Neutral skeleton

```text
GenericWhileSkeletonV1 {
  preheader,
  header,
  body,
  latch,
  after,
  condition_value,
  carrier_phis[],
}
```

The latch merges state only.  It is not a mandatory step block.  Continue
targets the neutral latch and carries values visible at the continue edge.

The existing `GenericLoopSkeleton` remains legacy during migration; do not
change its meaning in place.

## Optional canonical specialization

```text
VerifiedCanonicalLoopOptimizationV0 {
  baseline_contract_id,
  exact_step_site,
  recurrence,
  placement,
  specialized_body_projection,
  motion_proof,
}
```

The baseline is verified first.  Specialization may move exactly one source
site only when purity, execution-path equivalence, continue/break/return
behavior, evaluation order, and source coverage are proven.

```text
CanonicalOptimizationOutcomeV0 =
  Applied(VerifiedCanonicalLoopOptimizationV0)
  | NotApplicable(reason)
  | InternalOptimizationContractViolation
```

`NotApplicable` selects no route.  The planner explicitly chooses the already
verified generic baseline.  If a selected specialization fails in Lower, that
is an internal contract violation; Lower never switches routes.

## Source identity and coverage

The accepted source-site decision remains active, with a changed purpose:

```text
SourceStmtSiteV0:
  canonical structural identity

identity-preserving typed projection:
  complete body coverage and nested provenance

sealed paired positional provenance:
  checked source/Recipe witness

SourceRecipeBijectionVerifierV0:
  omission/duplicate/nested-child verification
```

Baseline dispositions:

```text
executable statement -> RecipeStatement
transparent ScopeBox -> TransparentContainer
CanonicalStep         -> zero entries
```

Optional specialization alone may account one exact site as CanonicalStep.
Candidate preorder, flattened index, AST equality, pointer, and span are not
identity.

## Existing A2 disposition

```text
CandidateObservationV0:
  move to optional canonical analysis / diagnostics

LoopProgressionCandidatePolicyV0:
  retire from generic; shrink to optional canonical proof policy if needed

LoopProgressionProofV0:
  move/rename to CanonicalStepProofV0

CandidateRecipeDraftV0:
  retire from generic; optional specialization draft only

LoopProgressionSelectionV0:
  retire from generic

BodyManagedCursor:
  retire

StepModeV0:
  replace with LoopLoweringChoiceV1

loop_increment:
  retire from generic Facts

matches_loop_increment:
  retire as filtering/identity authority

StepPlacement and post-update-use checks:
  optional step-motion proof only

Unique / Zero / Multiple:
  retire from generic acceptance
```

## Parked WIP

The interrupted pre-decision source-projection implementation is parked as:

```text
05fb9b0577  wip/c2-i0-source-projection before generic-baseline supersession
```

It contains useful path/model/builder/sealer work but also known review gaps.
Do not apply wholesale.  Restore selected files/hunks only after G0 ownership
and tests are fixed on a clean tree.

Parser/source-carrier P1 remains separately parked and is never generic-loop
authority.

## Task order

### G0 — identity and complete-body coverage (active)

1. Reframe `SourceStmtSiteV0` around complete body coverage.
2. Add typed identity-preserving projection for the closed If/Loop/Scope
   vocabulary.
3. Certify ScopeBox as a transparent container with ordered positional ranges.
4. Preserve exact If/Loop child-body identity and absent versus present-empty.
5. Seal only complete, reachable, single-owner projected bodies.
6. Add source/Recipe omission and duplicate evidence without connecting
   Recipe product acceptance yet.
7. Preserve old flatten AST-sequence parity over the declared accepted corpus.
8. Keep product, planner, Lower, ProgramV0, and P1 connections zero.

Known review requirements for restored WIP:

```text
node kind <-> typed child role exact match
embedded AST child sequence <-> typed child body exact match
root-reachable body graph
single owner for every child body
complete ordered Scope expansion, not set-only evidence
table/corpus parity, not one synthetic tree
```

### G1 — disconnected generic baseline witness

Add `VerifiedGenericLoopRecipeV1` construction without candidate discovery.
The body remains complete and unfiltered.  Build four mandatory witnesses:

```text
multiple-condition variables
state-machine loop
multiple identical updates
parser cursor
```

Planner and Lower connections remain zero.

### G2 — loop-state and port closure

Add the conservative carrier-set owner, per-edge state observations, and
verified fallthrough/continue/break/return contracts.  No special carrier
exception is permitted.

### G3 — neutral skeleton reference route

Add `GenericWhileSkeletonV1` as a test/reference route.  First bounded subset:

```text
no nested loop
one or two carriers
simple break
continue only after its edge-state contract is green
```

No product route cutover yet.

### G4 — differential corpus

Run explicit legacy and generic-baseline routes over the same corpus.  Compare:

```text
return value
final bindings
observable effects
condition evaluation order/count
break / continue / return behavior
```

No runtime fallback is allowed between routes.

### G5 — generic acceptance cutover

Make `VerifiedGenericLoopRecipeV1` the generic acceptance authority.  Remove
candidate Zero/Multiple from generic rejection.  Product Lower consumes only
`VerifiedLoopLoweringPlanV1`.

### O0 — optional canonical lane

Move useful A2 observations into a nonfatal canonical optimization family.
Use exact source site and motion proof; never shape-based filtering.

### R0 — legacy single-loop-var retirement

After G5 and O0 gates are green, prove product caller zero and delete:

```text
GenericLoopV1Facts.loop_var
GenericLoopV1Facts.loop_increment
legacy loop_var_init/current/next skeleton fields
selected-loop-var carrier exclusion
mandatory step handoff / step block
BodyManagedCursor and dummy step sentinel
shape-based increment filtering
candidate Unique/Zero/Multiple generic freezes
LoopVarUsedAfterInBodyStep generic acceptance check
legacy GenericLoopSkeleton when caller count reaches zero
```

R0 is part of this Epic's completion definition.  New and legacy families may
coexist temporarily, but coexistence without a caller-zero retirement gate is
forbidden.

## Mandatory fixtures

### Multiple condition/state variables

```text
loop left < right && count < limit {
  left = scan_left(left)
  right = scan_right(right)
  count = count + 1
}

baseline Ready
carrier set = {left, right, count}
selected progression owner = none
```

### State machine

```text
loop true {
  state = transition(state)
  if state.done() { break }
  output = consume(output, state)
}

baseline Ready
canonical optimization NotApplicable
```

### Identical updates

```text
loop condition {
  i = i + 1
  if retry { i = i + 1 }
}

both statements remain in baseline
no omission
optional canonical result = NotApplicable(MultipleExactSteps)
```

### Parser cursor

```text
loop true {
  cursor = skip_ws(cursor)
  if at_end(cursor) { break }
  cursor = parse_item(cursor)
}

baseline Ready
both writes preserved
BodyManagedCursor absent
```

## Required gates

```text
source projection identity and deterministic path
complete-body source/Recipe bijection
omission / duplicate / child-swap rejection
Scope expansion order/completeness
condition order and header-frequency
carrier then/else passthrough
continue-edge state
break/after final-state merge
body-local exclusion from carrier set
legacy/baseline differential corpus
explicit route selection; no fallback
product caller-zero before R0 deletion
all source files < 800 lines
```

## Dependency and isolation guards

Generic baseline construction must not import or call:

```text
progression candidate selection
canonical step filtering
ShapeId detector priority
name/callee/Box policy
ProgramV0
parser/source-carrier P1
backend/runtime fallback
```

Lower must not read raw AST or rediscover candidate, step, carrier, or route.

## May claim

After G1:

```text
generic-loop semantic representation needs no selected progression owner
condition variables are ordinary operands
complete body is preserved without step filtering
loop-carried state is represented as a set
candidate ambiguity is not a generic semantic rejection
```

After G5/R0 and their full gates:

```text
generic baseline is product acceptance authority
canonical extraction is optional proven optimization
legacy mandatory single-loop-var family is retired
```

## Must not claim

```text
loop-state tracking is unnecessary
all loops or nested CFG shapes are supported
termination is proven
existing GenericLoopSkeleton already satisfies V1
generic Recipe verifier alone proves source coverage
canonical parity is universal
legacy candidate structures can be deleted before caller-zero
Hako planner/Lower authority has moved
parser/source-carrier P1 is connected
```

## Stop conditions

1. Generic baseline requires `loop_var` or `loop_increment`.
2. Condition operands become progression candidates for generic acceptance.
3. Body construction filters any step-shaped statement.
4. Baseline adds a synthetic or implicit step.
5. One selected variable is removed from the carrier set.
6. Condition reads stale preheader values after body mutation.
7. Continue/break edges lose their visible state.
8. Optimization NotApplicable becomes generic Unsupported.
9. Selected specialization falls back inside Lower.
10. Canonical route accepts a loop without a verified baseline.
11. Coverage is approximated by item count or AST equality.
12. ShapeId, rank, name, or detector order returns to generic authority.
13. Lower rediscovers source roles.
14. New RecipeItem or CFG wiring is added without a separate BoxShape card.
15. ProgramV0 or parser P1 enters the lane.
16. Unsupported backend falls back to VM.
17. Legacy loop-var callers remain uncounted at cutover.
18. Any source file reaches 800 lines.
