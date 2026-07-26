---
Status: closed
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: SOURCE-ENTRY-VMREF-NEUTRAL0-L0
Scope: one source-family-neutral published-entry owner plus a separate passive VM-reference projection
ceremony_tier: T2 BoxShape inside accepted NORMAL-CANONICAL-CORE0
sunset_id: SOURCE-ENTRY-VMREF-NEUTRAL-PROOF-SUNSET-001
sunset_row: NORMAL-FILE-CANONICAL-CORE0-G0
proof_inventory_before: existing S3 Raw execution guards plus closed normal Main TX0 proofs
new_proofs: one disconnected neutral invocation fixture family
retired_or_merged_proofs: none in L0
net_proof_delta: +1 bounded T2 proof
sunset_budget: repay the disconnected fixture at canonical-core route G0
retire_when: Raw and canonical adapters consume the sole neutral owner, old direct Raw activation callers are zero, and exact target/result/status/diagnostic parity is green
budget_repayment_evidence: the canonical-core route guard absorbs durable assertions and disconnected-only constructors become zero
docs_only_closeout: forbidden
code_or_artifact_delta_required: 1
Related:
  - docs/development/current/main/investigations/normal-main0-tx0-i0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-source-plan0-design-stop-2026-07-26.md
---

# SOURCE-ENTRY-VMREF-NEUTRAL0-L0

## Terminology correction

This row is neutral across published source families. It does not make the VM
reference executor backend-neutral.

Keep two explicit layers:

```text
PublishedSourceEntryInvocationV1
  source-family-neutral
  backend-neutral
  no VMValue / MirInterpreter / process policy

PreparedVmReferenceSourceEntryInvocationV1
  VM-reference-specific
  exact decode plan
  still passive in L0
```

Calling a product backend-neutral while it owns `VmSourceEntryDecodePlanV1`
is forbidden.

## Closed prerequisite

TX0 now transports lossless Unit evidence:

```text
VerifiedNormalMainThunkResultV1::Unit {
  origin: FunctionUnitOriginV1
}
```

The five current function origins remain distinct:

```text
EmptyBody
ImplicitFallthrough
BareReturn
ExplicitVoid
ExplicitNull
```

No later adapter may re-read AST or guess `ImplicitFallthrough`.

## Durable owner boundary

The backend-neutral layer owns exactly:

```text
complete family-specific published owner
exact source-entry target
exact source-result contract
family membership evidence
```

Conceptual vocabulary:

```rust
pub(in crate::mir) struct VerifiedPublishedSourceEntryTargetV1 {
    symbol: Box<str>,
    arity: usize,
    _seal: VerifiedPublishedSourceEntryTargetSealV1,
}

pub(in crate::mir) enum PublishedSourceEntryResultContractV1 {
    Unit { origin: UnitOriginV1 },
    Integer,
    Bool,
    Float,
    String,
}

pub(in crate::mir) struct PublishedSourceEntryInvocationV1<O> {
    owner: O,
    target: VerifiedPublishedSourceEntryTargetV1,
    result: PublishedSourceEntryResultContractV1,
    membership: PublishedSourceEntryMembershipV1,
    _seal: PublishedSourceEntryInvocationSealV1,
}
```

The generic parameter is always the complete published family owner, never a
marker or evidence summary. The Raw adapter instantiates it with the complete
Raw published owner. The canonical adapter may instantiate it only after
consuming the completed candidate through the publication transition below.
Any later VM-specific type erasure must remain a closed enum whose variants
still retain those complete owners.

The Raw and canonical adapters are the only eventual producers. L0 adds no
production constructor or consumer.

## VM-reference projection

The VM-specific passive product consumes one verified published invocation:

```rust
pub(in crate::mir) struct PreparedVmReferenceSourceEntryInvocationV1 {
    published: PublishedSourceEntryInvocationV1,
    decode: VmSourceEntryDecodePlanV1,
    _seal: PreparedVmReferenceSourceEntryInvocationSealV1,
}
```

The result mapping is exhaustive and source-evidence-owned:

```text
Unit(origin) -> VmSourceEntryDecodePlanV1::Unit
Integer      -> Integer
Bool         -> Bool
Float        -> Float
String       -> String
```

For canonical Main, Unit physical return is Void and the decode contract
requires that exact physical relation. String remains outside the first
canonical Main profile, while the neutral vocabulary retains it for the
existing Raw Script family.

The mapping never observes `VMValue`, `MirType`, a physical Return, or a module
inventory.

## Identity law

Raw adapter evidence:

```text
existing invocation brand
existing selected-entry continuation
existing root target witness
existing Raw decode evidence
```

Canonical adapter evidence:

```text
CompletedNormalMainModuleCandidateV1 consumed by value
VerifiedNormalMainEntryRelationV1
VerifiedNormalMainThunkResultV1 with exact Unit origin
FunctionOwnerIdV1 membership
```

Do not invent a common Raw invocation brand for canonical Main. Raw brand
membership and canonical function-owner membership are separate variants of
one typed membership vocabulary.

Forbidden identity sources:

```text
NYASH_ENTRY
module function scan
literal "main" route inference
execute_module entry discovery
CLI/profile route reconstruction
fallback or retry
```

## Publication boundary

`CompletedNormalMainModuleCandidateV1` is currently unpublished. The later
canonical adapter must add a consuming publication transition that moves:

```text
candidate module
entry relation
source result evidence
source owner
verification receipt
```

together. `module().clone()`, a bare `MirModule` escape, or calling the
candidate published without a publication receipt is forbidden.

The Raw adapter must retain `RawPublishedInvocationV1`; it must not use
`into_compatibility_module()`, which erases Raw authority.

## Failure law

Adapter preparation failures retain the complete family owner:

```rust
pub(in crate::mir) enum SourceEntryVmNeutralStageV1 {
    Family,
    Membership,
    Route,
    Target,
    ResultContract,
}

pub(in crate::mir) struct RejectedPublishedSourceEntryInvocationV1 {
    owner: PublishedSourceEntryRejectedOwnerV1,
    stage: SourceEntryVmNeutralStageV1,
    error: SourceEntryVmNeutralErrorV1,
}
```

Public terminals are inspection and `discard(self)` only.

Forbidden:

```text
into_owner
retry / resume
alternate family selection
module or symbol scan
Legacy fallback
status construction
```

## L0 implementation order

```text
L0-A FILE-BUDGET0
  keep source_entry_vm_execution.rs unchanged and below 800 in L0
  reserve its cfg(test) extraction as the mandatory first Raw-adapter edit

L0-B PUBLISHED0
  backend-neutral target/result/membership/family vocabulary
  complete owner retention

L0-C VM-PROJECTION0
  passive Published -> VM-reference decode projection
  execution consumer zero

L0-D FAILURE0
  typed mismatch rejection and complete owner retention

L0-E G0
  extend the existing S3 execution guard/helper
  no new shell wrapper
```

## L0 fixture matrix

```text
passive success:
  Raw-like family evidence retained
  CanonicalNormal-like family evidence retained
  exact target symbol and arity
  Unit all five origins
  Integer / Bool / Float / String result contracts

passive rejection:
  target mismatch
  arity mismatch
  empty target symbol

adapter rejection reserved for later rows:
  family/membership mismatch
  result/physical relation mismatch

structural:
  move-only / non-Clone
  inspection + discard only
  production producer = 0
  production consumer = 0
```

## Structural gate

```text
PublishedSourceEntryInvocationV1 owner             = 1
VerifiedPublishedSourceEntryTargetV1 owner         = 1
PublishedSourceEntryResultContractV1 owner         = 1
PreparedVmReferenceSourceEntryInvocationV1 owner   = 1

neutral layer VMValue/MirInterpreter reference     = 0
neutral layer ProcessExitProjection/status/diagnostic = 0

RawPublishedInvocation authority erasure           = 0
canonical candidate module clone/bare escape       = 0
NYASH_ENTRY/module scan/execute_module              = 0
fallback/retry                                      = 0

L0 production producer                             = 0
L0 production consumer                             = 0
all modified/new source/check files                 < 800 lines
```

Extend `tools/checks/lib/entry_result_projection0_s3_execution_guard.py` or an
imported Python helper. Do not add a shell wrapper.

## Immediate continuation

```text
SOURCE-ENTRY-VMREF-NEUTRAL0-L0
-> SOURCE-ENTRY-VMREF-RAW-ADAPTER0-I0
-> NORMAL-MAIN0-VMREF-ADAPTER0-I0
-> NORMAL-MAIN0-VMREF0-P0
```

Raw adapter cutover must prove the existing exact target, status, diagnostic,
decoy-entry, and reuse matrix before retiring:

```text
RAW-ADAPTER0-A FILE-SPLIT0
  extract source_entry_vm_execution.rs cfg(test) body before adapter edits

PreparedRawVmReferenceActivationV1
CompletedRawVmReferenceExecutionV1
VmReferenceProjectedOwnerV1::Raw
from_raw_vm_reference
```

The canonical adapter then consumes the candidate through the same neutral
execution terminal. Fresh Rust `MirInterpreter`, `ProcessExitProjectionV1`,
and the existing diagnostic adapter remain the sole semantic-reference owners.

## Far task order

```text
SOURCE-ENTRY-VMREF-NEUTRAL0-L0
-> SOURCE-ENTRY-VMREF-RAW-ADAPTER0-I0
-> NORMAL-MAIN0-VMREF-ADAPTER0-I0
-> NORMAL-MAIN0-VMREF0-P0

-> NORMAL-CALLABLE-SOURCE0-S0
-> NORMAL-MAIN-DIRECT-CALL0-S0
-> NORMAL-HELPER-MODULE-PLAN0-S0
-> NORMAL-CALLABLE-MODULE0-A0-S0
-> NORMAL-CALLABLE-MODULE0-TX0-I0

-> NORMAL-FILE-CANONICAL-CORE0-PROFILE0-S0
-> NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0a
-> NORMAL-FILE-CANONICAL-CORE0-REUSE0-P0
-> NORMAL-FILE-CANONICAL-CORE0-CALLER0-I0
-> NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0b
-> NORMAL-FILE-CANONICAL-CORE0-G0

-> MIRBUILDER-CANONICAL-CORE-COMPLETE0-P0
-> NORMAL-ENTRY-PRODUCT-BACKEND-D0
-> NORMAL-DEFAULT-CALLER-CENSUS0-P0
-> NORMAL-ENTRY-PROMOTION-D3
-> exact selected old-caller retirement

-> NORMAL-IMPORT-BUNDLE0
-> MIRBUILDER-LEGACY-FENCE0
-> MIRBUILDER-NORMAL-COMPLETE0
-> MIRBUILDER-COMPLETE0-G0
```

## Reference-lane retirement

The VM-reference lane remains the semantic-reference interpreter during this
series. It is not the default product backend decision.

After product backend cutover parity, open:

```text
VM-REFERENCE-LANE-RETIRE0-D0
```

That decision may retire a reference CLI lane, but must not retire the shared
exact-entry executor, interpreter conformance owner, process projection, or
diagnostic authority merely because one CLI spelling is removed.

## Non-claims

```text
VM execution in L0
process projection in L0
Raw production cutover in L0
canonical publication in L0
normal/default backend cutover
helper/direct-call support
imports/using
JSON/LLVM/native
Legacy retirement
```

## Closeout

```text
Status:
  closed

Backend-neutral owner:
  PublishedSourceEntryInvocationV1<O>

Passive VM projection:
  PreparedVmReferenceSourceEntryInvocationV1<O>

Production producers / consumers:
  zero / zero

Execution / process / diagnostic authority:
  zero / zero / zero

Exact evidence:
  target symbol + arity
  complete owner by value
  Raw-brand or canonical-owner membership
  Unit physical relation
  Unit five canonical origins
  Integer / Bool / Float / String
```

Acceptance:

```text
cargo check --lib                                      = green
source_entry_published_invocation                      = 2/2
source_entry_vm_invocation                             = 1/1
normal Main transaction regression                     = 11/11
S3 execution/neutral guard                             = green
S3 owner guard                                         = green
normal-source-plan0 row guard                          = green
current-state pointer guard                            = green
all source/check files below 800                       = green
```

Next row:

```text
SOURCE-ENTRY-VMREF-RAW-ADAPTER0-I0
```
