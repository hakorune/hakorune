# Generic Loop A2 Candidate Authority — Design Consultation

Status: Active evidence-incomplete design stop; implementation is stashed.
Date: 2026-07-13
Decision required: first approve a bounded observation-capture slice, then
choose the candidate discriminator and its neutral authority owner.
Classification: BoxShape only.

## Why this consultation is required

The accepted A2 direction correctly established:

```text
assignment observed
  != induction step proven
```

The WIP now has pure write/use observation, provisional
`CanonicalInduction | BodyManagedCursor` classification, and aggregate
order-independent selection. `BodyManagedCursor` is only a WIP spelling for a
candidate with body-managed updates. It does not establish cursor identity or
progression ownership. Focused tests were green before stashing and all Rust
files stayed below 800 lines. The full ProgramV0 contract-pin advances
substantially, but does not close.

Successive clean-corpus blockers are:

```text
1. StringHelpers.read_digits/2
   candidates: out, pos

2. ParserControlBox.parse_block/3
   candidates: j, body

3. ParserRecordDeclarationBox.parse/3
   observed terminal outcome:
     AmbiguousLoopVarCandidates
   candidate inventory and per-candidate observation tuples:
     NOT CAPTURED
```

The first two can be separated structurally:

```text
read_digits:
  pos has a closed canonical recurrence and non-step uses
  out is a rebased/local accumulator

parse_block:
  j is used across many body statements
  body is updated inside one local conditional region
```

The third outcome proves neither which source values competed nor why they
tied. Source inspection must not reconstruct that missing runtime inventory.
Adding another evidence rank for each newly exposed corpus shape
would turn A2 into heuristic score tuning. That violates the intent of the
BoxShape decision even if no source names are inspected.

## Current evidence

Stash:

```text
a8f50bbb / wip/generic-loop-progression-role-a2
  (focused green, contract-pin advances but remains red)
```

Evidence status:

```text
the test results below are pre-stash observations
the stash is WIP, not a mergeable proof artifact
classification.rs and selection.rs are untracked payloads in its third parent
focused green does not prove semantic uniqueness or corpus exhaustiveness
the terminal ProgramV0 failure preserves location/reason, not candidate tuples
```

Focused evidence before stashing:

```text
cargo test -q progression_role --lib
  19 passed

cargo test -q generic_loop_v0 --lib
  3 passed

cargo test -q generic_loop_v1 --lib
  18 passed

largest generic-loop Rust source:
  648 lines
```

Full gate:

```text
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh
  RED at ParserRecordDeclarationBox.parse/3
  reason = multiple loop_var candidates matched (ambiguous)
```

No parser/source-carrier file was changed. No environment toggle, source
rewrite, function/callee/variable-name branch, ShapeId, or fallback was added.

## Mandatory evidence slice before authority selection

Before selecting a semantic discriminator, capture one deterministic
candidate table at the existing Ambiguous boundary. This is diagnostic
evidence only and changes no acceptance.

Required fields:

```text
candidate diagnostic label
candidate discovery source
condition_anchored
existing_true_loop_increment_derived
writes and stable structural sites
canonical_step_sites
uses_outside_canonical_step
post_update_uses
conditional_writes
provisional update-shape classification
provisional evidence rank
final comparison/tie result
```

Candidate labels may appear in diagnostics, but may not be classifier input.
No new rank, Recipe item, CFG wiring, parser change, or acceptance widening is
allowed in this capture slice.

## Authority question after evidence capture

For a `loop(true)` with no condition-anchored variable, what is the smallest
non-heuristic authority that may choose one progression candidate?

The final decision has three independent axes. Do not present them as one
exclusive A/B/C/D choice.

```text
Axis 1 — semantic discriminator
  E: closed evidence algebra
  R: candidate-complete verified Recipe
  N: no discriminator currently proven

Axis 2 — authority location
  P: existing neutral declarative policy owner
  X: new neutral owner required
  U: Unsupported boundary only

Axis 3 — outcome
  exactly one proven candidate -> Accepted
  zero proven candidates       -> Unsupported
  multiple proven candidates   -> Ambiguous
```

## Semantic discriminator candidates

### E — closed evidence algebra

Define a finite, schema-owned evidence vocabulary such as:

```text
CanonicalRecurrenceObserved
BodyStateAcrossStatements
LocalUpdateOnly
```

and a total ordering or explicit dominance relation.

Required proof:

```text
the vocabulary is exhaustive for the accepted subset
ties remain Ambiguous
new corpus functions do not add ad-hoc ranks
the selected role preserves every BodyManaged update in Recipe
```

Risk: an apparently generic rank can still encode corpus-specific preference
and may not be semantic authority.

### R — Recipe-first candidate competition

For every observed candidate:

```text
observation
  -> role classification
  -> candidate-specific complete Recipe
  -> verifier outcome
```

Select only when exactly one candidate yields a complete verified execution
recipe. Zero candidates is Unsupported; multiple verified candidates is
Ambiguous.

Required proof:

```text
candidate evaluation is side-effect-free
Recipe construction does not mutate shared planner state
selection is order-independent
Lower consumes only the selected VerifiedRecipe + StepMode
```

Risk: if multiple candidate-specific Recipes preserve the same body, Recipe
verification alone may not identify progression ownership. It may also expose
a need for a new Recipe item or CFG wiring, which is outside current A2.

Recipe-first is admissible only if a `CandidateRecipeDraft` can be built
without a preselected progression owner or `StepMode`, and its verifier uses
candidate-independent invariants. Otherwise the design is circular:

```text
select candidate using VerifiedRecipe
  -> Recipe requires progression role / StepMode
  -> progression role requires selected candidate
```

If every candidate preserves the same complete body, Recipe completeness is
not a discriminator and the result remains Ambiguous. Needing a new Recipe
item or CFG wiring is a stop condition, not an implementation detail.

## Authority-location constraint

### P/X — neutral policy owner

Reuse an existing canonical loop/route observation only if it already owns a
declarative state-carrier role independent of ShapeId and source names.

Required proof:

```text
no route-name or detector-list priority becomes semantic authority
Facts does not depend on Lower/backend routing
the owner is shared declarative policy rather than a layer-crossing re-export
```

Risk: current route recognizers are largely diagnostic/coverage shapes and
the previous consultation explicitly rejected ShapeId as acceptance
authority.

## Mandatory outcome boundary

### U — keep unproven true-loop families Unsupported/Ambiguous

Do not generically select body-derived candidates when more than one remains.
Defer the typed-parser P1 reconnection or split a separately proven loop
family.

This is the smallest sound boundary, but clean-HEAD ProgramV0 remains red and
selfhost progress pauses.

## A2 and P1 dependency direction

```text
A2 has no source/build dependency on the P1 WIP
P1 restoration depends operationally on clean-HEAD A2 closeout
P1 evidence may not become A2 candidate-selection authority
A2 and P1 remain separate commits and separate gate runs
```

Stable stash identities:

```text
a8f50bbb  generic-loop A2 WIP — KEEP, decision pending
ce69e049  source-carrier P1 WIP — KEEP/PARK after A2
fded1abc  old S3 typed-carrier prototype — reference-only/archive candidate
47907362  old S3 recursive reader — direct restore forbidden/archive candidate
3af1b029  old 3226 snapshot AOT — superseded/archive candidate
```

Never restore current pointer documents from a code stash. Restore active
code WIP on a clean temporary branch, inspect its untracked third-parent
payload, and rerun current gates. Archive/drop of historical stashes is a
separate lifecycle task and requires preserved patch, metadata, base/stash
hashes, untracked payload, and user approval before drop.

## Required answer

Please return:

```text
Evidence decision:
  approve bounded capture | request different capture

After capture:
  semantic discriminator = E | R | N | hybrid
  authority location      = P | X | U
  outcome boundary        = Unique/Zero/Multiple rule

Canonical authority
Non-authority
Accepted boundary
Rejected boundary
Candidate-selection algorithm
Recipe/Verifier/Lower responsibility split
Whether captured candidate recipes expose missing Recipe/CFG vocabulary
  (answer Unknown until the inventory exists)
Smallest implementation slice
Required fixtures/gates
Implementation may claim
Implementation must not claim
Retirement path
Stop conditions
```

The answer must explicitly address the first two structural witnesses and the
third evidence gap without using source names as classifier inputs:

```text
read_digits-like accumulator + cursor
parse-block-like multi-write cursor + local body accumulator
record-declaration terminal Ambiguous with candidate inventory not captured
```

## Frozen constraints

```text
parser/source-carrier changes = 0
ProgramV0 schema widening = 0
source rewrite = 0
function/Box/callee/variable-name policy = 0
new blocker-specific ShapeId = 0
environment toggle = 0
silent fallback = 0
Facts Some without VerifiedRecipe = 0
Lower role rediscovery = 0
BodyManaged assignment filtering = 0
synthetic increment = 0
all source files < 800 lines
new evidence rank before capture = 0
new Recipe item / CFG wiring during capture = 0
```

Do not restore A2 until the bounded capture is approved. Then restore it only
on a clean temporary branch for acceptance-neutral evidence collection. Do
not restore P1 until the final A2 authority is recorded, A2 lands
independently, and the clean-HEAD ProgramV0 contract-pin is green.
