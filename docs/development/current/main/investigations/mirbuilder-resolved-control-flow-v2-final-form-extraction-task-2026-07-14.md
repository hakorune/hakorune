---
Status: Parked — extraction trigger not yet satisfied
Date: 2026-07-14
Decision: reserve a three-family evidence-driven final-form extraction task
Activation: only after If, Loop, and one third control-flow family are independently production-closed
Current authority: none; B0-L4-S1 remains the active blocker
Related:
  - mirbuilder-b0-l3b-a-plus-implementation-task-2026-07-13.md
  - mirbuilder-b0-l4-a-a2prime-implementation-task-2026-07-14.md
  - mirbuilder-resolved-region-flow-v1-task-2026-07-13.md
---

# Resolved Control Flow V2 Final-Form Extraction Task

## Purpose

Reserve the clean final consolidation boundary without making B0-L4 depend on
an abstraction inferred from only statement If and Loop.

This is a parked extraction card, not an active implementation card. It does
not authorize source changes, does not change `CURRENT_STATE.toml`, and must
not become `next_documented_task` until every activation gate below is green.

The target is not one universal control plan. The target is the smallest
shared verified envelope and transaction lifecycle proven by at least three
independently closed control-flow families.

```text
canonical syntax + owner-closed resolved function
                    │
                    ▼
family analyzer: If / Loop / third family
                    │
                    ▼
VerifiedLocatedControlFlowV2<Family>
  owner closure
  exact source root
  family-specific region topology
  family-specific port/state contract
  inseparable structural coverage
                    │
                    ▼
family materializer
  + shared rollback/coverage/commit kernel
                    │
                    ▼
MIR
```

## Why this remains parked

Two families do not provide enough evidence for a durable algebra.

Statement If currently proves:

```text
two independent branch working states
final PHIs with all inputs already known
implicit identity versus explicit else
join rows over outer BindingRefs
```

Loop V1 is designed to prove:

```text
header carriers
provisional PHIs patched after body lowering
shared post-condition state
condition-false exit
transaction-local repeated-region roles
```

These differences are semantic, not accidental duplication. Generalizing now
would likely introduce optional fields, boolean port state, or a large enum
whose invalid combinations are representable.

A third independently landed family, preferably Match or Try, is required to
distinguish genuinely shared vocabulary from a two-family coincidence.

## Activation gates

All gates are mandatory.

```text
G1:
  canonical fallthrough statement If is production-closed

G2:
  B0-L4 atomic I2 canonical Loop is production-closed

G3:
  one third control-flow family is independently production-closed
  preferred evidence: Match or Try

G4:
  all three families have exact located source input,
  immutable pre-Builder flow products, and zero MIR identity in analysis

G5:
  a mechanical inventory finds the same field, verifier law, or lifecycle
  operation implemented in at least three families

G6:
  the proposed extraction deletes duplicated authority or lifecycle code;
  it does not merely replace concrete names with traits/generics

G7:
  no active family blocker requires changing the proposed common vocabulary

G8:
  current pointer explicitly selects this extraction card after consultation
```

The following do not satisfy the trigger:

```text
If + Loop alone
nested If support
another Loop exit port
desire for symmetric filenames
shared words such as region, port, or flow
test helper duplication alone
an SA4 materialization map proposal
```

## Candidate final boundary

The candidate shape is provisional until F0 evidence is recorded.

```rust
pub struct VerifiedLocatedControlFlowV2<F>
where
    F: private::SealedControlFlowFamilyV2,
{
    owner: FunctionOwnerIdV1,
    root: F::SourceRoot,
    regions: F::Regions,
    contract: F::Contract,
    coverage: VerifiedPlanSourceCoverageV1,
    seal: private::ControlFlowSealV2,
}
```

Public consumers should receive family-specific aliases or wrappers, not a
freely constructible generic product.

```rust
pub type VerifiedLocatedIfFlowV2 =
    VerifiedLocatedControlFlowV2<IfFlowFamilyV2>;

pub type VerifiedLocatedLoopFlowV2 =
    VerifiedLocatedControlFlowV2<LoopFlowFamilyV2>;
```

This sketch is not permission to add a public extension trait. The family
trait, constructors, seal, and coverage field remain private to the analysis
layer. External crates cannot define a family or combine independently
obtained flow and coverage values.

If the F0 inventory shows that an enum or separate wrappers preserve stronger
invariants than the generic envelope, use those instead. The invariant is the
decision; the Rust spelling is not.

## Shared authority candidate

Only vocabulary proven common across three families may move into the shared
envelope.

Initial candidates are:

```text
owner-closed construction
exact located source root
private co-sealed structural coverage
deterministic BindingRef effect ordering
foreign-owner and duplicate rejection
exactly-once coverage consumption
pre-Builder zero-ValueId/zero-BasicBlockId law
```

Even these candidates require F0 evidence. A common name alone is not proof.

## Family-owned authority

The following remain family-specific unless three-family evidence proves an
identical law rather than a superficial resemblance:

```text
region topology
port algebra and reachable-exit set
state-source equations
join rows versus loop carrier rows
implicit branch semantics
unsupported grammar preflight
condition evaluation placement
result-value ports
```

In particular, V2 must not create a universal structure resembling:

```rust
struct ControlFlowPlan {
    has_else: bool,
    falls_through: bool,
    has_backedge: bool,
    break_port: Option<_>,
    return_port: Option<_>,
    result_value: Option<_>,
}
```

That shape makes invalid family combinations representable and recreates a
second semantic authority inside the common layer.

## Shared materialization kernel candidate

MIR materializers remain specialized. Only lifecycle operations with the same
failure and publication law may be shared.

```text
ControlMaterializationTxnKernelV2 candidate:
  value environment checkpoint
  current block checkpoint
  region stack checkpoint
  lexical scope stack checkpoint
  coverage-use ledger
  region-consumption ledger
  primary + cleanup error composition
  unpublished function-draft commit barrier
```

The kernel must not own:

```text
CFG shape
block-role allocation policy
PHI input equations
port reachability
BindingRef carrier discovery
family preflight
```

If uses final PHI definition after both ports are materialized. Loop uses
provisional header PHIs whose backedge inputs become available later. Those
lifecycle differences stay in the family materializers.

## SA4 boundary

This extraction is orthogonal to durable RegionId materialization.

```text
Resolved Control Flow V2:
  semantic identity, port/state flow, exact coverage

family materializer:
  transaction-local ValueId/BasicBlockId state

SA4:
  durable role-aware RegionId materialization authority
```

V2 must not publish `RegionId -> BasicBlockId`, and it must not absorb SA4
just to make the common envelope look complete.

## Work order after activation

### F0 — three-family mechanical inventory

Create one table from landed code, not planned names.

```text
row:
  concept
  If owner/path
  Loop owner/path
  third-family owner/path
  identical invariant?
  family-specific difference?
  deletion count if extracted
```

Acceptance:

```text
at least one authority or lifecycle row is identical across all three
no candidate depends on optional unsupported ports
no candidate moves ValueId/BasicBlockId into RegionFlow
```

If the inventory finds no three-family shared owner, close this card without
code changes and keep the separate products.

### F1 — shared vocabulary only

Move only mechanically identical carrier/verifier vocabulary.

```text
production route changes = 0
family product construction changes = 0
acceptance grammar changes = 0
```

Each move must remove one duplicated truth source and retain family-focused
fixtures.

### F2 — private sealed envelope

Introduce the generic envelope, closed enum, or shared private constructor
selected by F0.

Requirements:

```text
flow and coverage cannot be separated
foreign owner/source products cannot be paired
family extension outside the owner module is impossible
no Option-based family state
no generic Builder entry
```

Migrate one family per commit. The other families remain green and unchanged
until their own migration commit.

### F3 — materialization lifecycle kernel

Extract only failure restoration, ledger completion, and commit-barrier logic
that is identical across all three materializers.

Requirements:

```text
family CFG/PHI algorithms remain in family modules
cleanup error never overwrites primary error
partial publication remains zero
fault-injection fixtures remain family-specific
```

### F4 — retirement and naming closeout

After every family uses the selected shared boundary:

```text
remove duplicate private envelopes/verifiers
retain family aliases and focused entrypoints
update resolved_region_flow README ownership map
update one reusable authority guard
record exact deletions and remaining nonclaims
```

Do not rename the legacy materialized `CorePlan` as part of this series.

## Required fixtures

```text
foreign owner flow/coverage pairing rejected for every family
foreign source root pairing rejected
duplicate structural coverage rejected
missing structural coverage rejected
family kind/root mismatch rejected
private constructor boundary compile-fail fixture where practical
RegionFlow ValueId allocation = 0
RegionFlow BasicBlockId allocation = 0
standalone coverage publication = 0
generic Builder dispatch = 0
If final-PHI lifecycle unchanged
Loop provisional-PHI lifecycle unchanged
third-family port lifecycle unchanged
primary + cleanup error preserved for every family
partial function publication = 0
VM/reference results unchanged
```

## Implementation may claim

After F4 only:

```text
three production canonical control-flow families share one owner-closed,
coverage-inseparable verified envelope

shared materialization lifecycle has one rollback/coverage/commit owner

family port algebra, state equations, CFG, and PHI policy remain specialized
```

## Implementation must not claim

```text
one universal control-flow plan
all control-flow families supported
generic CFG or PHI materialization
Break/Continue/Return/Try semantics from shared vocabulary alone
durable RegionId materialization or SA4 cutover
legacy CorePlan retirement
default source route cutover
ProgramV0 source authority
Hako Lower parity
```

## Stop conditions

Stop extraction if it:

```text
starts before all activation gates are green
uses planned third-family names instead of landed evidence
adds Option or bool fields to represent family differences
permits external family implementations
exposes coverage construction or into_parts
combines separately acquired flow and coverage
moves region topology or state equations into a lowest-common-denominator enum
lets the common layer discover effects from AST, names, or value-map diffs
puts ValueId/BasicBlockId in the pre-Builder envelope
creates one generic Builder control-flow entry
forces If and Loop to share a PHI lifecycle
publishes scalar RegionId -> BasicBlockId state
mixes SA4, legacy CorePlan retirement, or grammar expansion into the series
changes accepted source shapes before the final migration commit
fails to delete a duplicated truth source or lifecycle owner
```

## Reopen procedure

When the third family lands:

```text
1. keep its production commit independent
2. run F0 against the three landed families
3. record whether the generic envelope, closed enum, or separate wrappers win
4. obtain an explicit current-pointer selection
5. run F1 -> F2 -> F3 -> F4 as one BoxShape-only refactor series
```

Until then:

```text
status = parked
current pointer = unchanged
implementation authority = 0
B0-L4-S1 remains next
```
