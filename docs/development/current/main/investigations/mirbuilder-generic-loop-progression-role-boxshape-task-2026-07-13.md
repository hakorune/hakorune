# Generic Loop Progression Role V0 — BoxShape Cleanup Taskboard

Status: Active prerequisite; A0-A1 closed, A2 selected.
Date: 2026-07-13
Decision: `A — compiler_boxshape_cleanup`
Classification: BoxShape only.

## Purpose

Repair the generic-loop observation boundary before reconnecting the parked
typed-parser P1 work.

```text
assignment observed
  != induction step proven
```

The current generic-loop path can classify a body-managed scan cursor update
as an extractable induction step and freeze on its later ordinary use before a
complete Recipe is considered. This task separates candidate observation,
progression-role classification, deterministic selection, Recipe
construction, and lowering consumption.

Required order:

```text
A-only generic-loop BoxShape cleanup
  -> clean HEAD ProgramV0 contract-pin GREEN
  -> commit A alone
  -> restore typed-parser P1 stash
  -> P1 focused + ProgramV0 contract-pin GREEN
  -> consider parser-private lightweight seam B only if still evidenced
```

Parser/source-carrier files are outside A's write scope.

## Current evidence

Clean `HEAD` is already red under:

```text
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh
```

Observed owners include:

```text
ParserDelegateExposesBox._parse_delegate/3
  loop cursor: j
  update: j = ctx.skip_ws(...)
  post-update use: ctx.starts_with_kw(src, j, "as")
  reject: loop_var_used_after_in_body_step

FuncScannerBox._scan_methods/4
  reject observed in another closure/order:
    no_valid_loop_var_candidates
```

The labels explain fixtures only. Box names, function names, variable names,
and callee names are forbidden classification inputs.

## Authority

```text
loop candidate observation:
  generic-loop Facts condition observation

progression role:
  LoopProgressionRoleObservationV0

accepted execution structure:
  complete RecipeBlock
  -> Recipe verifier
  -> VerifiedRecipeBlock

lowering input:
  VerifiedRecipeBlock + explicit StepModeV0

typed parser construction during A:
  unchanged and stash-only
```

Condition-derived candidates are primary. If a condition identifies a
candidate, unrelated body assignments must not compete as loop variables.
Body-only candidate discovery remains outside the first accepted slice.

## Non-authority

```text
ParserDelegateExposesBox / FuncScannerBox names
_parse_delegate / _scan_methods names
skip_ws / starts_with_kw or any callee spelling
source variable names such as j, i, or pos
first freeze in module/function order
compile-closure order
GenericLoopV1ShapeId by itself
dummy ASTNode::Variable sentinel
ProgramV0 JSON
source rewrite
P1 SourceCarrierBuilder dependency closure
environment toggle
Lower-side role rediscovery
```

## Closed vocabulary

```text
LoopProgressionRoleV0 =
  CanonicalInduction {
    var,
    step_site,
    recurrence,
    placement
  }
  | BodyManagedCursor {
      var,
      update_profile
    }

BodyManagedUpdateProfileV0 =
  Rebased
  | MultipleWrites
  | PostUpdateUse
  | Mixed

StepModeV0 =
  ExtractedCanonical
  | BodyManaged
```

Candidate-local classification may additionally return:

```text
NotCandidate
Ambiguous
Unsupported(reason)
```

These are observation/selection outcomes, not new source syntax or ShapeId
acceptance authority.

## Accepted boundary

### CanonicalInduction

All must hold:

```text
condition-anchored candidate
one normal-backedge step owner
RHS belongs to the existing closed recurrence vocabulary
no competing write
no ordinary use after the extracted step
placement belongs to the existing StepPlacement contract
```

### BodyManagedCursor

At least one structural witness exists:

```text
non-affine/rebased assignment
multiple writes in one iteration
conditional write
ordinary post-update use
extracting the update would change body meaning
```

Acceptance additionally requires:

```text
one condition-anchored candidate
all writes retained in the body Recipe
complete RecipeBlock without holes
successful Recipe verification
existing Lower wiring can consume BodyManaged mode
```

Body-managed invariants:

```text
assignment filtering = 0
synthetic increment = 0
implicit backedge step = 0
```

## Rejected boundary

Exact Unsupported or Ambiguous:

```text
no condition-anchored candidate
multiple accepted candidates
write roles cannot be classified uniquely
any candidate write is absent from the Recipe
Recipe contains a hole or unsupported item
Recipe verifier rejects
Lower lacks BodyManaged wiring
new nested-loop/if CFG wiring is required
classification requires a source/Box/callee name
source rewrite is required
```

If `FuncScanner` proves to require unrestricted body-only candidate discovery,
stop and split `conditionless/body-derived state loop` into another BoxShape
decision. Do not widen A1.

## Task order

### A0 — exact baseline and minimized fixtures (closed)

- Capture clean-HEAD per-function/per-loop outcomes for both real owners.
- Add minimized structural fixtures for rebased/post-update and no-unique-step
  cursor loops.
- Record candidate list, write/use sites, selected rule, Recipe digest, and
  final behavior independently of first-freeze order.

Landed-ready structure:

```text
progression_role_baseline_tests.rs
  delegate-style rebased/post-update cursor
  scanner-style body-only cursor

test_support.rs
  one process-wide JoinIR environment lock/restore owner

generic_loop_progression_role_v0_guard.sh
  structural unit fixtures
  real-source ProgramV0 contract-pin observation
  source-size and parser-isolation assertions
```

A0 intentionally separates two observations:

```text
minimized per-loop structural fixtures:
  no accepted progression role
  no dependence on first-freeze ordering

real-source contract-pin:
  known RED at an existing cursor-loop owner/reason
```

This avoids making the first compiled function or freeze string semantic
authority. A0 changes no parser source and no generic-loop acceptance.

Executable evidence:

```text
bash tools/checks/generic_loop_progression_role_v0_guard.sh
  a0_structural_fixtures=green
  clean_head_contract_pin=known_red
  parser_source_change=0

cargo test -q generic_loop_v1 --lib
  18 passed

largest generic-loop Rust source:
  648 lines
```

### A1 — pure candidate observation (closed)

Add a physically separate observation module:

```text
CandidateObservationV0 {
  condition_anchored,
  writes[],
  uses[],
  canonical_step_sites[],
  post_update_uses[],
  conditional_writes[]
}
```

Collection is read-only and side-effect-free. It performs no selection,
Recipe construction, lowering, or source rewrite.

Closed structure:

```text
facts/progression_role/observation.rs
  CandidateObservationV0
  CandidateSiteV0
  observe_candidate_progression_v0(...)
```

The existing condition and canonical-step owners supply
`condition_anchored` and the optional canonical increment. The observer does
not rediscover either policy. It records:

```text
all current-loop writes
all current-loop uses in evaluation order
canonical step sites using the existing matcher
post-update uses
conditional writes
stable preorder and top-level statement indices
```

Nested-loop state is deliberately excluded because it belongs to another
loop observation. Assignment RHS uses are visited before the write, matching
evaluation order and preventing `i = i + 1` from becoming a false
post-update use.

Evidence:

```text
cargo test -q progression_role --lib
  6 passed

bash tools/checks/generic_loop_progression_role_v0_guard.sh
  a1_pure_candidate_observation=green
  generic_loop_acceptance_change=0
  clean_head_contract_pin=known_red

cargo test -q generic_loop_v1 --lib
  18 passed
```

The observation module contains no parser/fixture owner names, callee
spellings, environment reads, selection, Recipe, or Lower dependency.

### A2 — progression-role classification and selection (active)

- Classify each observation as CanonicalInduction, BodyManagedCursor,
  NotCandidate, Ambiguous, or Unsupported.
- Evaluate all candidates before choosing the loop outcome.
- Replace candidate-loop early Freeze with `Vec<CandidateOutcomeV0>`.
- Select exactly one accepted role; use a stable reason priority when none is
  accepted; report Ambiguous when multiple are accepted.
- Prove order independence by permuting candidate and unrelated function
  order.

### A3 — explicit StepMode through Recipe and Lower

- Add `StepModeV0` to `GenericLoopV1Facts`.
- Replace the body-managed dummy `ASTNode::Variable` sentinel with explicit
  mode plus optional canonical step.
- Retain every BodyManaged assignment in the body Recipe.
- Return Facts only after complete Recipe verification.
- Pass `VerifiedRecipeBlock + StepModeV0` to Lower.
- Lower must not inspect AST to rediscover step role.

Stop if this needs a new RecipeItem or new CFG wiring.

### A4 — A-only closeout

Run and freeze:

```text
role classifier unit fixtures
two minimized real-source fixtures
order-independence gate
Facts -> VerifiedRecipe -> Lower identity gate
exact negative fixtures
name/import isolation guard
clean HEAD ProgramV0 contract-pin
source-size and compile-shape guard
```

Only after A4 is green may A be committed and P1 stash restored.

## Required gates

1. **Role classifier fixtures**

   ```text
   final i = i + 1
     -> CanonicalInduction

   i = i + 1; use(i)
     -> not CanonicalInduction
     -> BodyManaged only with verified complete Recipe

   j = scan(j); use(j)
     -> BodyManagedCursor(Rebased/PostUpdateUse)

   j = skip(j); conditional j = parse(j); use(j)
     -> BodyManagedCursor(MultipleWrites/Mixed)

   two condition candidates
     -> Ambiguous

   no condition candidate
     -> exact Unsupported
   ```

2. **Real-source minimized fixtures**
   - ParserDelegateExposes structural witness;
   - FuncScanner structural witness;
   - assert role, StepMode, retained assignments, Recipe completeness,
     verification, lowering, and output parity.

3. **Order-independence gate**
   - compile fixtures in both orders and with unrelated functions inserted;
   - compare per-loop CandidateOutcome, StepMode, Recipe digest, plan rule, and
     result rather than the first error string.

4. **Facts/Recipe/Lower gate**
   - every accepted fixture has Facts, VerifiedRecipe, and Lower consumption
     of the same Recipe;
   - Lower shape rediscovery count is zero.

5. **Negative fixtures**
   - ambiguous candidate;
   - incomplete Recipe;
   - unsupported statement/nested wiring;
   - body-derived-only candidate;
   - unclassified write.

6. **Isolation guard**
   - role/classifier/selection modules contain no parser/fixture owner names,
     callee spellings, parser source paths, or new environment flag.

7. **Baseline closeout**

   ```text
   bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh
   # expected: GREEN on A-only clean tree
   ```

8. **Source-size/shape guard**
   - every source file remains below 800 lines;
   - observation, classification, selection, Recipe projection, and tests are
     physically separated.

## Implementation may claim

After A-only closeout:

```text
canonical induction and body-managed cursor are structurally distinct
accepted body-managed updates remain in the verified Recipe
candidate evaluation is deterministic and order-independent per loop
Facts acceptance requires a complete VerifiedRecipe
Lower consumes explicit StepMode without reclassification
the two declared cursor fixtures need no source/callee-name special case
clean HEAD ProgramV0 contract-pin is green
```

## Implementation must not claim

```text
P1 Return migration landed
SourceBodyAnalysisSnapshotV1 connected
parser construction closure reduced
lightweight seam B implemented
all parser/state-cursor loops supported
arbitrary multi-update loops supported
cursor monotonicity or loop termination proven
ShapeId is acceptance authority
new generic-loop vocabulary without Recipe proof
planner/backend/runtime authority moved
source rewrite proves semantic equivalence
first-freeze order is a semantic contract
```

## Retirement path

1. Remove the dummy body-managed increment sentinel.
2. Represent both modes as `StepModeV0 + Option<CanonicalStepV0>`.
3. Remove candidate-local early Freeze and centralize stable final rejection.
4. Apply `LoopVarUsedAfterInBodyStep` only to ExtractedCanonical mode.
5. Restore P1 stash only after the A-only gate is green and committed.
6. Open B separately only if a post-P1 dependency audit proves accidental
   publication/sealer closure remains.

## Stop conditions

Stop and return to design if:

1. parser source must be rewritten to pass;
2. any Box/function/callee/variable name enters classification;
3. a blocker-specific ShapeId or allowlist is added;
4. Facts returns `Some` without a VerifiedRecipe;
5. BodyManaged removes assignments or synthesizes an increment;
6. Lower reclassifies AST step roles;
7. candidate-local first failure still decides the final loop outcome;
8. FuncScanner requires unrestricted body-only candidate discovery;
9. a new RecipeItem or nested CFG wiring is required;
10. A is mixed with B, parser/P1 wiring, or another BoxCount;
11. clean HEAD remains red while only the P1 branch becomes green;
12. an environment toggle, silent fallback, or generic VM fallback appears;
13. a source file reaches 800 lines;
14. the gate asserts only which function freezes first rather than per-loop
    facts, Recipe, plan, and behavior.

## Handoff to P1

After A is committed and pushed:

```text
restore:
  stash label: wip/hako-source-carrier-p1

required before P1 commit:
  P0 carrier gate GREEN
  P1 Return focused gate GREEN
  ProgramV0 contract-pin GREEN
  compile-shape gate GREEN
  Language-v1 grammar gate GREEN
  planner connection = 0
```

The exact stash index must be rechecked at restore time; the descriptive stash
message is the stable identifier.
