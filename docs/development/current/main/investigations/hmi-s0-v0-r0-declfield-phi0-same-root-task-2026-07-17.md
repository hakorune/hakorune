---
Status: S0 closed; P0 next
Date: 2026-07-17
Decision: Candidate A′ accepted
Baseline: e741e1bbca
Evidence-Baseline: 584e1f8829
Parent: hmi-s0-v0-r0-declfield-phi0-consultation-question-2026-07-16.md
Scope: bounded use-site current-receiver Copy/Phi proof
---

# R0-DECLFIELD-PHI0 same-root receiver task

## Decision lock

The first durable authority is one immutable proof created at the declared
field lookup use site.

```text
accepted value grammar:
  exact current implicit receiver parameter 0
  ordinary Copy(accepted value)
  ordinary Phi(all incoming values accepted)

accepted graph:
  finite
  acyclic value definitions
  acyclic CFG edges for every accepted PHI

terminal root set:
  exactly { current receiver parameter 0 }

persistent receiver provenance:
  0
```

The proof is not PHI metadata, a reusable receiver-equivalence table, or an
origin update.

## Exact order

```text
R0-DECLFIELD-PHI0-S0
  -> R0-DECLFIELD-PHI0-P0
  -> R0-DECLFIELD-PHI0-I0
  -> existing R0-DECLFIELD0-G0
  -> clean HMI-S0-V0-R0-I0 rewrite
```

The next code-facing row is `R0-DECLFIELD-PHI0-S0`.

## Authority split

| Concern | Authority |
| --- | --- |
| current receiver identity | verified method parameter publication |
| value definitions | current in-progress `MirFunction` |
| predecessor inventory | current function CFG successors |
| reachability/dominance | ephemeral verification utilities |
| field declaration/type | existing `user_box_field_decls` |
| accepted receiver equivalence | `VerifiedSameRootReceiverValueV1` |
| FieldGet type publication | existing field lowering |
| final whole-function validity | final `MirVerifier` |

Non-authorities:

```text
variable_map["me"]
current_static_box
function-name parsing
MIR symbol parsing
method or field name heuristics
runtime class tags
TypedObjectPlan
generic method route result
HMI source path
stash evidence
```

## Products

```rust
pub(crate) struct VerifiedCurrentReceiverIdentityV1 {
    receiver_parameter: ValueId,
    owner_box: String,
    _seal: CurrentReceiverIdentitySealV1,
}

pub(crate) struct VerifiedSameRootReceiverValueV1 {
    value: ValueId,
    receiver: VerifiedCurrentReceiverIdentityV1,
    _seal: SameRootReceiverValueSealV1,
}
```

The same-root proof owns only:

```text
verified seed value
verified receiver parameter identity
verified owner box
```

It does not retain:

```text
definition index
predecessor index
reachable blocks
dominator tree
worklist
visiting/proven memo
normalized test fingerprint
```

Those are ephemeral construction state.

## Physical structure

Add one responsibility-separated module:

```text
src/mir/builder/field_receiver_provenance/
  README.md
  mod.rs
  analysis.rs
  cfg.rs
  definitions.rs
  tests.rs
```

Responsibilities:

```text
README.md:
  authority/non-authority/consumer/retirement boundary

mod.rs:
  sealed products
  typed error vocabulary
  public(crate) verification facade

analysis.rs:
  explicit value worklist
  visiting/proven memo
  Copy/Phi grammar

cfg.rs:
  predecessor/reachability/dominance views
  PHI edge availability
  CFG cycle/backedge rejection

definitions.rs:
  exact parameter/instruction definition index
  instruction-position lookup without cloned instruction truth
  traversal budget

tests.rs:
  synthetic functions
  normalized fingerprints
  accepted/rejected matrix
```

Target budgets:

```text
README.md <= 120 lines
mod.rs <= 280 lines
analysis.rs <= 420 lines
cfg.rs <= 320 lines
definitions.rs <= 220 lines
tests.rs <= 700 lines
field_facts.rs after I0 <= 160 lines
every source/check file < 800 lines
```

Do not place the entire proof and fixture matrix in `field_facts.rs`.

## Current receiver identity verification

`VerifiedCurrentReceiverIdentityV1::verify` co-validates:

```text
current function exists

declared_param_decls[0]:
  name = me
  implicit_receiver = true

function.params[0]:
  exists

MirValueKind:
  Parameter(0)

function.signature.params[0]:
  Box(owner)

type_ctx.value_types[param0]:
  Box(same owner)

type_ctx.value_origin_newbox[param0]:
  same owner

user_box_field_decls:
  contains same owner
```

No individual row is sufficient by itself.

The proof must not use:

```text
variable_map["me"]
function_param_names
current_static_box
function symbol prefix
```

## Value-level admission

Every traversed value, including the seed, Copy source/destination, PHI
destination, and incoming value, must satisfy:

```text
value_types[value] = Box(current owner)

value_origin_newbox[value]:
  absent
  OR same current owner
```

Reject:

```text
missing value type
different value type
foreign origin
```

A′ proves identity preservation. It does not infer a missing generic value
type.

## Exact definition index

Existing `find_value_def` returns the first matching definition and is
insufficient for `MultipleDefinition`.

Existing `compute_def_blocks` stores one block per ValueId and can overwrite a
duplicate.

S0 therefore builds one ephemeral exact index by scanning:

```text
function parameters
all block instructions
all terminators
```

For each ValueId it records:

```text
definition count
definition block
definition instruction kind
```

The index rejects:

```text
missing definition
multiple definitions
parameter/instruction identity collision
```

It is never stored in `MirBuilder`, `MirFunction`, metadata, or a registry.

## Accepted definition grammar

```text
Parameter(0):
  must equal the verified current receiver

Copy(src):
  src must be available at the Copy definition site
  recursively prove src

Phi(inputs):
  validate exact PHI/CFG law
  recursively prove every input
```

Reject every other definition:

```text
foreign parameter
CopyOwned
Select
Call
FieldGet
NewBox
LocalContractWrite
VariantMake
Load
MemOp
any other instruction
```

## PHI/CFG law

Each accepted PHI must satisfy:

```text
PHI block reachable = true
input count >= 2
duplicate input predecessor count = 0
actual reachable predecessor count >= 2
input predecessor set = actual reachable predecessor set
phantom predecessor count = 0
missing predecessor count = 0
unreachable attached predecessor count = 0
```

Incoming availability:

```text
definition block == attached predecessor
OR
definition block dominates attached predecessor
```

The proof must create this authority before final whole-function verification.
It may reuse:

```text
verification::utils::compute_predecessors
verification::utils::compute_reachable_blocks
verification::utils::compute_dominators
```

It must not claim that a future final verifier success is sufficient current
authority.

## Loop/backedge rejection

For every incoming edge:

```text
predecessor -> PHI block
```

reject when:

```text
PHI block == predecessor
OR
PHI block can reach predecessor
```

The reachability query is an explicit bounded worklist over current CFG
successors.

This rejects:

```text
self-loop
natural loop header PHI
irreducible CFG cycle PHI
```

even when every terminal value root is the receiver.

No persistent CFG SCC product is introduced.

## Host-stack law

Do not implement value proof or CFG reachability with unbounded recursive host
calls.

Use:

```text
explicit frame/work stack
VisitStateV1::Visiting
VisitStateV1::Proven
definition-count traversal budget
block-count CFG budget
```

Revisiting `Visiting` is `ValueDefinitionCycle`.

Shared nested PHI subgraphs reuse `Proven` memo state.

## Normalized test view

Copy is transparent:

```text
receiver:
  R

Copy(receiver):
  R
```

PHI retains child multiplicity but sorts child fingerprints:

```text
Phi(receiver, receiver):
  P[R,R]

Phi(Copy(receiver), Phi(receiver, receiver)):
  P[P[R,R],R]
```

Normalization excludes:

```text
ValueId
BasicBlockId
PHI row order
declaration order
DFS/worklist order
```

The normalized view is test/report evidence only and is not stored in the
production proof.

## Typed failures

```rust
pub(crate) enum SameRootReceiverProofErrorV1 {
    NoCurrentFunction,
    NotInstanceMethod,
    MissingImplicitReceiverMetadata,
    MissingReceiverParameter,
    ReceiverKindMismatch,
    ReceiverOwnerMismatch,
    ReceiverRegistryMissing,

    MissingUseSite,
    SeedUnavailable,
    SeedTypeMissing,
    SeedTypeMismatch,
    ForeignOrigin,

    CfgSuccessorCacheMismatch,
    MissingCfgBlock,
    MissingDefinition,
    MultipleDefinition,
    UnsupportedDefinitionKind,
    ForeignParameter,
    CopySourceUnavailable,

    PhiUnreachable,
    PhiTooFewInputs,
    DuplicatePhiPredecessor,
    PhantomPhiPredecessor,
    MissingPhiPredecessor,
    UnreachablePhiPredecessor,
    PhiIncomingUnavailable,

    ValueDefinitionCycle,
    CfgCycleOrBackedge,
    TraversalBudgetExceeded,
}
```

Production I0 may collapse proof failure to no recovery:

```rust
verify(...).ok()
```

Tests must retain exact error reasons.

## R0-DECLFIELD-PHI0-S0

Production behavior delta:

```text
0
```

Add:

```text
sealed receiver identity
sealed same-root receiver value proof
ephemeral exact definition/CFG analysis
iterative Copy/Phi proof
typed errors
normalized test view
focused synthetic tests
```

Production consumers:

```text
0
```

The only call sites are focused tests.

S0 required tests:

```text
direct receiver
Copy* receiver
one fallthrough PHI
two-level nested acyclic PHI
shared PHI subgraph
foreign leaf reject
value cycle reject
self-loop reject
natural-loop reject
irreducible CFG-cycle reject
duplicate/missing/phantom predecessor reject
incoming dominance reject
receiver co-validation failures
```

S0 may claim:

```text
one disconnected bounded proof implementation
finite acyclic Copy/Phi closure
typed rejection vocabulary
production caller count zero
```

S0 must not claim:

```text
declared field recovery
FieldGet type delta
Known method route delta
HMI execution
```

### S0 closeout

`R0-DECLFIELD-PHI0-S0` is closed.

Implemented structure:

```text
README.md       34 lines
mod.rs         159 lines
analysis.rs    291 lines
cfg.rs         115 lines
definitions.rs 147 lines
tests.rs       570 lines
```

The disconnected implementation adds:

```text
one sealed current-receiver identity
one sealed same-root receiver value proof
one exact definition-position index
one validated ephemeral normal-CFG view
one explicit Copy/Phi worklist
one typed failure vocabulary
one normalized test-only fingerprint
```

Three mechanical safety refinements were required without changing the
accepted semantic grammar:

```text
definition truth:
  store instruction positions and read the current MirFunction
  never clone instructions into a second definition authority

use-site availability:
  the selected seed must already be available at the current instruction site

CFG substrate:
  cached successors must exactly match terminator-derived successors before
  predecessor/reachability/dominance utilities are used
```

Validation:

```text
cargo test -q field_receiver_provenance
  11 passed

cargo check -q
  pass (existing warnings only)

python3 tools/checks/lib/current_receiver_declared_field_proof.py .
  PHI-ROOT-DESIGN-REQUIRED
  selected base = Copy(Phi(current_receiver))
  selected declared/result type = Unknown
  ownership operation counts = 0

bash tools/checks/current_state_pointer_guard.sh
  pass
```

Production consumers remain zero. `declared_field_type_for_value`,
`FieldGet`, PHI metadata publication, method routing, runtime behavior, HMI
source, ownership operations, fallback, and retry are unchanged.

The next row is `R0-DECLFIELD-PHI0-P0`.

## R0-DECLFIELD-PHI0-P0

Production behavior delta:

```text
0
```

Complete the proof matrix:

```text
P1 direct receiver
P2 Copy* receiver
P3 one fallthrough PHI
P4 Copy* around one PHI
P5 two-level nested PHI
P6 three-level nested PHI
P7 three-predecessor PHI
P8 shared nested subgraph
P9 source/branch reorder normalized parity

R1 foreign parameter leaf
R2 NewBox leaf
R3 Call leaf
R4 FieldGet leaf
R5 Select leaf
R6 CopyOwned leaf
R7 nested foreign leaf
R8 mixed owner
R9 same-type explicit parameter
R10 static parameter zero
R11 receiver metadata absent
R12 seed type missing
R13 seed type mismatch
R14 duplicate predecessor
R15 phantom predecessor
R16 missing predecessor
R17 unreachable predecessor
R18 unavailable incoming definition
R19 value cycle
R20 self-loop
R21 natural loop
R22 irreducible CFG cycle
R23 missing field
R24 untyped field
```

P0 also runs the existing DECLFIELD0 fixture through a disconnected test
adapter and records:

```text
A1 = R
A2 = P[R,R]
A3 = R
A4 = R
```

Final `MirVerifier` parity is required for every accepted synthetic function.

## R0-DECLFIELD-PHI0-I0

Production behavior delta:

```text
one declared-field lookup shape
```

Modify only:

```text
field_facts.rs::declared_field_type_for_value
```

Order:

```text
1. existing value_origin_newbox route
2. VerifiedSameRootReceiverValueV1::verify
3. existing declared_field_type_name
4. existing parse_type_name_to_mir
```

Production consumer count:

```text
1
```

Keep unchanged:

```text
declared_field_contract_identity
resolve_property_getter_name
publish_field_result_origin
propagate_phi_meta
generic method planner
FieldGet schema
runtime
```

Post-I0 required evidence:

```text
selected A2 FieldGet declared_type = ArrayBox
selected A2 destination type = ArrayBox
selected A2 push = ArrayBox / Known
selected A2 length = ArrayBox / Known
selected RuntimeDataBox / Union count = 0
MapBox regression remains Known
N1/N2 remain unrecovered
new MIR instructions = 0
```

## Existing R0-DECLFIELD0-G0

Do not add a second public proof ID.

Register only:

```text
MAPFIELD-R0-DECLFIELD0
```

Artifacts:

```text
apps/current-receiver-declared-field-proof/test.sh
tools/checks/manifests/proof_apps/compiler_map_field_owner.toml
docs/tools/check-scripts-index.md
```

The existing normalized checker becomes the public proof entry. Its expected
selection changes only after I0 from:

```text
PHI-ROOT-DESIGN-REQUIRED
```

to the closed success token fixed during I0.

After G0 only, clean HMI R0-I0 may be rewritten. No stash apply/pop/restore or
wholesale copy is allowed.

## Counters and guards

```text
same-root receiver proof definitions = 1
current receiver identity definitions = 1
production consumers:
  S0/P0 = 0
  I0/G0 = 1

accepted terminal roots = receiver param0 only
accepted definition kinds = Parameter0 | Copy | Phi
accepted loop/backedge PHIs = 0
accepted value cycles = 0

persistent receiver-equivalence maps = 0
new ValueId -> owner maps = 0
new ValueId -> type maps = 0
value_origin_newbox proof writes = 0
PHI metadata behavior delta = 0
FieldGet schema delta = 0

property getter consumers = 0
field contract consumers = 0
method planner direct consumers = 0

function-name/current_static_box inference = 0
variable_map["me"] identity reads = 0
method/field/HMI name special cases = 0
runtime tag reads = 0
TypedObjectPlan backfeed = 0

CopyOwned = 0
DestroyOwned = 0
selected ReleaseStrong = 0
fallback/retry/legacy probing = 0
HMI source delta before G0 = 0
new proof manifest families = 0
source/check files >= 800 lines = 0
```

## Required validation

S0:

```bash
cargo test -q field_receiver_provenance
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

P0:

```bash
cargo test -q field_receiver_provenance
tools/checks/lib/current_receiver_declared_field_proof.py .
bash tools/checks/run_proof_app.sh --only MAPFIELD-R0-TYPE0
```

I0:

```bash
cargo test -q field_receiver_provenance
tools/checks/lib/current_receiver_declared_field_proof.py .
bash tools/checks/run_proof_app.sh --only MAPFIELD-R0-STOP0
bash tools/checks/run_proof_app.sh --only MAPFIELD-R0-DELTA0
bash tools/checks/run_proof_app.sh --only MAPFIELD-R0-TYPE0
cargo check -q
```

G0 adds manifest health, neighboring field mutation, HMI/JSON isolation, file
size checks, and quick gate.

## Implementation may claim after I0

```text
current receiver identity survives finite acyclic non-loop Copy/Phi graphs
every terminal root is independently verified as receiver param0
nested fallthrough PHIs recover existing declared field types
ArrayBox/MapBox reuse existing Known routes
unsupported provenance retains prior behavior
no persistent equivalence/origin authority is added
```

## Implementation must not claim

```text
general PHI type/origin propagation
loop/backedge receiver PHI support
Select or CopyOwned equivalence
object identity across calls
property getter or field-contract recovery
general declared-field propagation
borrow/noescape ABI
ownership grammar
HMI completion
backend widening
runtime class inference
```

## Stop conditions

Stop the row if it requires:

1. `value_origin_newbox` proof-result backfill;
2. a second mutable ValueId-to-owner/type map;
3. a second production consumer;
4. function names, `current_static_box`, or `variable_map["me"]`;
5. field/method/HMI-name special cases;
6. runtime tags or downstream-plan backfeed;
7. Select, CopyOwned, Call, or FieldGet leaves;
8. loop/backedge PHI acceptance;
9. final verifier success as the only current authority;
10. predecessor-set validation without edge availability;
11. unbounded recursive host traversal;
12. source restructuring or typed-helper production detour;
13. fallback, retry, or legacy probing;
14. HMI stash restoration;
15. any source/check file reaching 800 lines.

## Final lock

> Candidate A′ is selected. One bounded use-site proof admits the complete
> finite acyclic non-loop closure of ordinary Copy and Phi values whose every
> terminal root is the exact verified current receiver parameter zero.
> Construction uses ephemeral exact definition and CFG analysis, explicit
> worklists, typed failures, and deterministic test normalization. It writes no
> origin or PHI metadata and creates no persistent equivalence map. S0 and P0
> remain disconnected; I0 adds exactly one consumer in
> `declared_field_type_for_value`; the existing DECLFIELD0 G0 publishes the
> result. Loop/backedge PHIs, other carrier kinds, global metadata widening,
> runtime/name inference, ownership widening, fallback, and stash restoration
> remain forbidden.
