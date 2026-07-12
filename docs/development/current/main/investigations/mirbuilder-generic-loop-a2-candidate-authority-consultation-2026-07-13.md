# Generic Loop A2 Candidate Authority — Design Consultation

Status: Active design stop; implementation is stashed.
Date: 2026-07-13
Decision required: choose the final candidate-selection authority for
`loop(true)` body-derived progression candidates.
Classification: BoxShape only.

## Why this consultation is required

The accepted A2 direction correctly established:

```text
assignment observed
  != induction step proven
```

The WIP now has pure write/use observation, explicit
`CanonicalInduction | BodyManagedCursor` classification, and aggregate
order-independent selection. Focused tests are green and all Rust files stay
below 800 lines. The full ProgramV0 contract-pin advances substantially, but
does not close.

Successive clean-corpus blockers are:

```text
1. StringHelpers.read_digits/2
   candidates: out, pos

2. ParserControlBox.parse_block/3
   candidates: j, body

3. ParserRecordDeclarationBox.parse/3
   multiple candidates remain tied
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

However, adding another evidence rank for each newly exposed corpus shape
would turn A2 into heuristic score tuning. That violates the intent of the
BoxShape decision even if no source names are inspected.

## Current evidence

Stash:

```text
wip/generic-loop-progression-role-a2
  (focused green, contract-pin advances but remains red)
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

## Authority question

For a `loop(true)` with no condition-anchored variable, what is the smallest
non-heuristic authority that may choose one progression candidate?

The decision must explain whether body-derived candidate selection is owned
by:

```text
1. a closed structural evidence algebra before Recipe construction;
2. candidate-specific complete Recipe construction and verification;
3. an existing canonical loop/route observation owner;
4. no generic owner, making this source family explicit Unsupported.
```

## Candidate decisions

### A — closed evidence algebra

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

### B — Recipe-first candidate competition

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

### C — existing route observation owns selection

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

### D — keep ambiguous true-loop families Unsupported

Do not generically select body-derived candidates when more than one remains.
Defer the typed-parser P1 reconnection or split a separately proven loop
family.

This is the smallest sound boundary, but clean-HEAD ProgramV0 remains red and
selfhost progress pauses.

## Required answer

Please return:

```text
Decision: A | B | C | D | hybrid

Canonical authority
Non-authority
Accepted boundary
Rejected boundary
Candidate-selection algorithm
Recipe/Verifier/Lower responsibility split
Whether ParserRecordDeclaration requires a new Recipe/CFG vocabulary
Smallest implementation slice
Required fixtures/gates
Implementation may claim
Implementation must not claim
Retirement path
Stop conditions
```

The answer must explicitly address these three corpus witnesses without using
their names as classifier inputs:

```text
read_digits-like accumulator + cursor
parse-block-like multi-write cursor + local body accumulator
record-declaration-like tied body-derived candidates
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
```

Do not restore the A2 or P1 stash until this authority decision is recorded.
