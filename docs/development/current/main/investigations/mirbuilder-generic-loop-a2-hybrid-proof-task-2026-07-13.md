# Generic Loop A2 — Capture, Proof, Recipe Taskboard

Status: Active prerequisite; A2-C0 closed, A2-C1 neutral proof selected.
Date: 2026-07-13
Decision: acceptance-neutral capture first, then asymmetric `E + R` hybrid.
Classification: BoxShape only.

## Purpose

Close generic-loop progression ownership without corpus-specific ranks:

```text
CandidateObservation
  -> neutral closed evidence policy (E)
  -> LoopProgressionProofV0
  -> candidate-local Recipe draft (R)
  -> candidate-independent verifier
  -> Unique / Zero / Multiple
  -> SelectedLoopProgressionV0
  -> Lower
```

The two proofs are asymmetric:

```text
E proves candidate identity and progression role
R proves that the proven role has a complete executable Recipe

E selects; R gates execution
R never ranks or prefers candidates
```

Until A2-C0 captures the missing inventory:

```text
semantic discriminator = N
authority location      = U
acceptance widening     = 0
```

## Current WIP and restore order

Stable stash identities:

```text
a8f50bbb  wip/generic-loop-progression-role-a2
ce69e049  wip/hako-source-carrier-p1
```

Rules:

```text
A2 has no source/build dependency on P1
P1 is operationally blocked on clean-HEAD A2 closeout
A2 and P1 remain separate commits and gate runs
P1 evidence is never A2 candidate authority
```

For C0, restore only A2 on a clean temporary branch/worktree. Include the
untracked third-parent payload (`classification.rs`, `selection.rs`) but do
not restore stashed `CURRENT_STATE.toml` or other stale current pointers.
P1 remains parked.

## Authority

### Candidate discovery

```text
owner:
  generic-loop Facts observation

input:
  canonical AST condition
  flattened current-loop body
  structural write/use sites

output:
  ordered CandidateObservationV0[]
```

Discovery enumerates candidates only. It does not select them.

### Candidate semantic role

```text
owner:
  LoopProgressionCandidatePolicyV0

input:
  normalized CandidateObservationV0

output:
  Proven(LoopProgressionProofV0)
  Excluded(reason)
  Unproven(reason)
```

This is a new neutral declarative owner. It has no parser, Recipe, Lower,
backend, runtime, environment, ShapeId, or source-name dependency.

### Execution proof

```text
owner:
  candidate Recipe builder
  candidate-independent Recipe verifier

output:
  VerifiedCandidateLoopRecipeV0
```

### Final selection

```text
owner:
  LoopProgressionSelectionV0

input:
  candidate proof + candidate verified recipe

outcome:
  exactly one -> Unique / Accepted
  zero        -> Unsupported
  multiple    -> Ambiguous
```

### Lower

```text
input only:
  SelectedLoopProgressionV0 {
    candidate_id,
    progression_proof,
    step_mode,
    verified_recipe,
  }
```

Lower performs no candidate, step, or role rediscovery.

## Non-authority

```text
variable / Box / function / callee names
parser source paths
candidate discovery order
numeric or provisional evidence rank
BodyStateAcrossStatements alone
write/post-update/conditional counts alone
GenericLoopV1ShapeId
route recognizer priority
first observed freeze
ProgramV0
P1 source-carrier evidence
Recipe success without independent E proof
Lower/backend success
```

`BodyManagedCursor` is a provisional WIP spelling only. It currently means
"candidate with body-managed updates", not proven cursor identity.

## Closed proof algebra

```text
LoopControlAnchorV0 =
  HeaderCondition
  | CurrentLoopExitGuard
  | CurrentLoopBackedgeGuard

UpdateOwnershipV0 =
  CanonicalClosedRecurrence
  | BodyManagedAllWritesRetained
```

```text
LoopProgressionProofV0 =
  ConditionAnchoredCanonical { anchor_site, step_site }
  | ConditionAnchoredBodyManaged { anchor_site, write_sites }
  | ControlAnchoredCanonical { control_site, step_site }
  | ControlAnchoredBodyManaged { control_site, write_sites }
```

`CurrentLoopExitGuard` requires candidate use in a condition whose branch
contains a break targeting the current loop depth or a function return.
Nested-loop break does not count. `CurrentLoopBackedgeGuard` directly guards
a continue targeting the current loop depth.

### CanonicalClosedRecurrence first subset

```text
exactly one write
candidate = candidate + invariant_step
  or candidate = candidate - invariant_step
competing write = 0
step belongs to the current closed recurrence vocabulary
exact step site is accounted
```

First accepted invariant steps:

```text
integer literal
known local/parameter not written in this body
```

No type inference or method semantics are introduced.

### BodyManagedAllWritesRetained

The control/header anchor is proven, but updates remain body semantics:

```text
all candidate writes stay in Recipe body
assignment filtering = 0
synthetic increment = 0
implicit step = 0
```

## Candidate identity

Names are diagnostic-only. Product policy uses structural identity:

```text
CandidateIdV0 {
  discovery_source,
  first_write_path,
  discovery_ordinal_within_path,
}
```

All candidates are evaluated before final outcome. Candidate-local failure
never returns early from the loop-wide selection.

## Task order

### A2-C0 — acceptance-neutral bounded capture (closed)

Add physically separate diagnostic/report types:

```text
CandidateObservationRowV0
CandidateDecisionRowV0
CandidateSelectionReportV0
```

Each row records:

```text
diagnostic label
structural CandidateIdV0
discovery source
condition anchoring
true-loop increment derivation
stable write sites
canonical step sites
non-step uses
post-update uses
conditional writes
provisional update-shape classification
provisional evidence rank (report-only)
comparison/tie result
```

Requirements:

1. Capture all rows at the existing Ambiguous boundary.
2. Normalize rows by structural identity, never candidate name.
3. Freeze the real record-declaration terminal inventory as a fixture.
4. Preserve the exact pre-C0 planner/freeze outcome.
5. Do not alter candidate selection, rank, Recipe, CFG, parser, or Lower.
6. Merge C0 independently before semantic policy work.

C0 may claim deterministic/replayable candidate evidence only. It may not
claim a progression owner or new accepted loop.

Closed evidence:

```text
cargo test -q progression_role --lib
  8 passed

bash tools/checks/generic_loop_progression_role_v0_guard.sh
  c0_candidate_capture=green
  c0_acceptance_widening=0
  c0_product_path_connection=0
  c0_preexisting_terminal=preserved
  c0_record_inventory=exact_fixture
  parser_source_change=0
```

The isolated probe captures three diagnostic rows:

```text
j
  provisional BodyManaged/Mixed
  no canonical step
  broad non-step and post-update uses

fields
  provisional BodyManaged/Mixed
  one canonical site plus conditional/extra writes

field_count
  provisional CanonicalInduction
  one canonical site
```

These spellings and provisional classes are evidence only. They prove no
progression owner. The fixture is exact and the clean product route remains
at its pre-C0 rejection.

### A2-C0 checkpoint

Compare the captured three witness families:

```text
read-digits-like accumulator + cursor
parse-block-like multi-write state + local accumulator
record-declaration captured inventory
```

If the record-declaration rows do not fit the closed proof algebra, retain
`N + U`, keep the family Unsupported/Ambiguous, and stop for a new decision.
Do not add another rank or constructor to pass the corpus.

Checkpoint result:

```text
inventory completeness:
  sufficient to begin the declared closed-anchor proof implementation

semantic discriminator:
  still N

authority location:
  still U

accepted progression owner:
  none
```

C1 may implement only the already accepted header/exit/backedge anchor
constructors. If those constructors produce Zero or Multiple on the captured
family, retain Unsupported/Ambiguous and stop; do not extend the algebra from
source inspection.

### A2-C1 — neutral progression proof (active)

Only after C0 evidence supports the declared algebra:

1. Add `LoopProgressionCandidatePolicyV0` in a neutral module.
2. Add closed anchor/update/proof enums.
3. Remove provisional rank from selection input.
4. Prove every candidate independently.
5. Preserve Zero/Multiple as Unsupported/Ambiguous.

No Recipe or Lower changes in C1.

### A2-C2 — candidate-local Recipe draft and verifier

```text
Canonical hypothesis:
  separate the exact proven step site as CandidateStepWitnessV0

BodyManaged hypothesis:
  retain every source statement and write in body Recipe
```

Verifier invariants are candidate-independent:

```text
all source statement sites accounted exactly once
no Hole or duplicate statement
canonical witness maps to one exact source site
body-managed mode filters no write
ports, exits, and carriers are closed
```

`CandidateRecipeDraftV0` must not require a final selected candidate,
`StepMode`, Lower result, or backend route. If it does, stop for circularity.

### A2-C3 — Unique / Zero / Multiple selection

Evaluate all candidates:

```text
observation
  -> E proof
  -> candidate-local R draft
  -> candidate-independent verification
  -> viable candidate set
```

Only exactly one proven+verified candidate produces `Facts::Some`.

### A2-C4 — explicit StepMode and sentinel retirement

Replace dummy `loop_increment = Variable(loop_var)` with:

```text
StepModeV0 =
  CanonicalExternalStep
  | BodyManaged

StepModeV0 + Option<CandidateStepWitnessV0>
```

Rename the provisional `BodyManagedCursor` WIP vocabulary to the corresponding
proof constructors. Apply the old post-step-use validation only to canonical
external steps.

### A2-C5 — A2-only closeout

Required:

```text
focused observation/proof/Recipe/selection tests GREEN
real three-family fixtures GREEN under declared outcomes
order-independence GREEN
Facts -> proof -> verified Recipe -> Lower identity GREEN
clean-HEAD ProgramV0 contract-pin GREEN
parser/source-carrier changes = 0
all source files < 800 lines
```

Commit and push A2 independently. Only after C5 may P1 be restored and
revalidated in its own commit series.

## Required gates

### C0 capture gate

- all required fields are present;
- candidate names are diagnostic-only;
- normalized equality ignores discovery/function order;
- before/after planner outcome and freeze category are exact;
- record-declaration inventory is captured rather than source-inferred.

### Order-independence gate

Permute candidate discovery, function order, unrelated functions, and module
closure order. Normalized report equality must remain exact.

### Proof-policy gate

After C1:

```text
closed constructor inventory is exhaustive
unknown constructor is impossible/wildcard-free
counts/lifetime/rank alone produce no proof
control anchors respect current loop depth
```

### Unique / Zero / Multiple gate

After C3:

```text
one proof + one verified recipe          -> Unique
zero proof                               -> Unsupported
two proofs                               -> Ambiguous
one proof + recipe rejection             -> Unsupported
two proofs, one recipe rejection         -> Unique
two proofs, two verified recipes         -> Ambiguous
```

### Circularity/dependency gate

Policy owner must not import parser, Recipe, Lower, backend, runtime,
environment, ShapeId detector lists, or source path strings. Recipe draft must
not require final selection or final `StepMode`.

### Facts contract

```text
Facts::Some implies exactly one:
  LoopProgressionProofV0
  VerifiedCandidateLoopRecipeV0
  SelectedLoopProgressionV0
```

### Source-size gate

Keep observation, report model, policy, Recipe draft, selection, and tests in
separate files. Every source file remains below 800 lines.

## Accepted boundary

```text
complete observation
one closed proof constructor
candidate-local complete Recipe draft
candidate-independent verifier success
all source statements accounted exactly once
exactly one proven+verified candidate
```

## Rejected boundary

```text
missing inventory
no header/control anchor
unproven candidate
unclassified write
Recipe Hole/duplicate/missing statement
Recipe verifier rejection
Recipe draft requiring final StepMode
multiple proven+verified candidates
new Recipe item or CFG wiring required
Lower role rediscovery required
```

Zero is Unsupported. Multiple is Ambiguous. Neither becomes fallback.

## Implementation may claim

After C0 only:

```text
candidate inventory is deterministic and replayable
captured candidate-local observations are complete for the fixture
comparison/tie evidence is fixed
acceptance behavior is unchanged
names are diagnostic-only
```

After C5:

```text
the declared subset uses a closed progression-proof algebra
every accepted candidate also has a verified execution Recipe
candidate evaluation is side-effect-free and order-independent
exactly one proven+verified candidate is required
all BodyManaged updates remain in Recipe
Lower consumes the selected verified result without rediscovery
```

## Implementation must not claim

```text
arbitrary loop(true) support
all parser cursors supported
termination or monotonicity proof
Recipe completeness proves candidate identity
evidence rank is semantic authority
record-declaration support before captured Unique outcome
clean ProgramV0 gate before it is green
P1 reconnection during A2
new Recipe/CFG support
```

## Stop conditions

1. C0 changes acceptance or final failure outcome.
2. Record-declaration candidates are inferred from source inspection.
3. A numeric rank/dominance weight is added.
4. Broad statement use or closed recurrence alone selects a candidate.
5. Names, source paths, ShapeId, or detector order enter policy.
6. CandidateRecipeDraft requires final selection or StepMode.
7. Recipe verifier contains candidate-specific semantic preference.
8. Equal complete body Recipes are tie-broken.
9. Multiple becomes Accepted.
10. BodyManaged writes are filtered or a synthetic increment is created.
11. A new Recipe item, carrier port, or CFG wiring is required.
12. Candidate evaluation mutates shared planner state.
13. Lower rediscovers role from AST.
14. P1/parser/source-carrier is changed before A2 closeout.
15. A source file reaches 800 lines.

## Retirement path

1. Remove provisional rank from all selection inputs after C0 evidence.
2. Replace `BodyManagedCursor` with proof-bearing constructors.
3. Remove dummy loop-increment sentinel.
4. Carry explicit `StepModeV0` and optional exact step witness.
5. Remove candidate-local early Freeze in favor of aggregate outcome.
6. Keep only stable rejection diagnostics or test-only report formatting.
7. Land A2, prove clean ProgramV0 GREEN, then restore P1 separately.
