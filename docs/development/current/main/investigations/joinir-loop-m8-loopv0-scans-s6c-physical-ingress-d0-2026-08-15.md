---
Status: accepted design predecessor; implementation remains caller-zero
Date: 2026-08-15
Decision: one Builder-free S6C prephysical aggregate, followed by a separate physical-session design
Scope: M8 LoopV0 forward ScanWithInit only
---

# JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-PHYSICAL-INGRESS-D0

## Six-line brief

```text
Decision: accept one non-Clone VerifiedS6CPrephysicalIngressV2 which consumes the source-retaining logical output once and co-seals only existing S6C source, Recipe, Join, and Completion evidence.
Source authority + canonical issuer: retained S6C Facts/source-bound calls/Completion plus the fixed Recipe role map and V2 Join transfer; loop_recipe_contract::issue_s6c_prephysical_ingress_v2 is the sole co-seal issuer.
Non-authority: LogicalConsumer::Consumed, V1 operation-effect/continuation/demand products, a new Pure/effect taxonomy, item order, AST/name/MIR lookup, generic CompareEq, LoopToJoinLowerer, physical IDs, selector, fallback, retry.
Fail-fast boundary: exact context, 15 placements, 13 role-keyed source/execution rows, one If, one Loop Exit, two calls, V2 After, inputs/carrier, and exact-two exit evidence close before Builder/session or physical effects.
Smallest next slice: implement the caller-zero Builder-free aggregate, private HRTB façade, exact source-anchor multiplicity, and focused positive/negative evidence.
Non-claims: no TextEq physical target, ReadyEntry, host/session ownership, MIR/CFG/SSA/PHI/layout, callable Return emission, Artifact/ABI, selector, production caller, fallback, retry, or retirement.
```

## Decision

The landed chain remains the sole authority owner:

```text
resolver source seal
  -> VerifiedS6CScanWithInitFactsV1
  -> VerifiedS6CScanWithInitRecipeProductV2
  -> source-retaining VerifiedS6CScanWithInitLogicalOutputV1
```

The prephysical ingress consumes that output by value. It does not accept raw
Facts, Recipe, JoinSig, context, effects, or continuation as separate inputs.
That one-input rule prevents a foreign cohort from being re-paired after the
Facts/Recipe/Join co-seal.

The accepted product shape is:

```rust
pub(crate) struct VerifiedS6CPrephysicalIngressV2 {
    output: VerifiedS6CScanWithInitLogicalOutputV1,
    context: VerifiedLoopSemanticContextV1,
    seal: S6CPrephysicalIngressSealV2,
}

pub(crate) fn issue_s6c_prephysical_ingress_v2(
    output: VerifiedS6CScanWithInitLogicalOutputV1,
) -> Result<VerifiedS6CPrephysicalIngressV2, S6CPrephysicalIngressRejectV2>;
```

`seal` is private transport evidence. It does not own another Recipe, Join
closure, Completion, or semantic effect ledger. The original output remains in
the aggregate, so all later views are borrowed from the same non-Clone owner.

The D0 is a T2 BoxShape: it fixes a new ownership boundary without accepting a
new source shape, Recipe dialect, ABI, backend, or production route. The I0 adds
exactly one aggregate product, but it remains a caller-zero BoxShape under the
repository definition of BoxCount/BoxShape.

## Exact bounded census

The previous phrase `15 item-keyed operation/effect rows` was incorrect. The
canonical S6C logical output is:

```text
domains:
  loops      = 1
  blocks     = 3
  bindings   = 1
  inputs     = 3
  values     = 15
  items      = 15
  carriers   = 1
  exits      = 1

placements:
  operations = 13
  If         = 1  // I8
  Exit       = 1  // I10

operation-family census:
  ReadBinding  = 4  // I0, I3, I9, I11
  ConstI64     = 2  // I4, I12
  BinaryI64    = 2  // I5, I13
  CompareI64   = 1  // I2
  CallSlot     = 2  // I1, I6
  TextEq       = 1  // I7
  WriteBinding = 1  // I14

logical transfer:
  branch        = 1
  Return summary = 1
  Backedge      = 1
  After         = L0/B0/I64
```

These are four different authorities and must not be merged:

```text
operation-family census:
  Read 4 / Write 1 / Call 2 / expression operation 6

resolver BodyEffect census:
  Call 2 / Write 1

CoreMethod semantic effect:
  Length PureRead / Substring PureRead

V2 execution census:
  NonFaulting 11 / FaultBeforeNormalResult 0 / ExternallyBoundOutcome 2
```

The `4/1/2/6` split is a fixed Recipe-family census, not a new semantic effect
enum. `LoopOperationV2::execution_class_v2()` remains the execution-family
authority. Facts remain the only source effect authority. CoreMethod contracts
remain the only Home/ABI/PureRead authority for Length and Substring.

## Source-evidence co-seal

Each of the 13 operation roles must borrow its existing exact source relation.
The ingress issuer may create a private parity seal; it may not create another
source ledger or infer roles from item order.

One operation can represent more than one source occurrence. In particular:

```text
I3 body_index_read / V6
  <- substring argument 0 source site
  <- SliceEndAdd lhs source site
```

Those two source sites are distinct. A singular `source_anchor` would lose
closed source coverage. I0 must preserve the exact role-specific anchor
multiplicity, either as a borrowed fixed view or a private alias parity seal.
Missing, duplicate, foreign, or collapsed anchors reject before issuing the
aggregate.

## Private HRTB boundary

The only read API is a private HRTB façade. It lends narrow projections from the
retained output and the private seal:

```text
owner/origin/source-kind/Loop site/frame/scope context
Subject/Needle/Index input bindings and initializer/carrier relation
15 logical placements
13 role-specific source/execution views
role-wise Length/Substring call views
If and Loop Exit control view
V2 branch/Return-summary/Backedge/After view
exact-two Completion view:
  Loop Return(index)
  callable Tail(-1)
  target function and empty cleanup
```

Callable Tail is absent from the Loop item/Exit rows, but remains available as
a separate Completion subview from the same aggregate. This prevents a later
physical session from reopening Facts or constructing a second Tail route.
DraftSeal remains the only eventual MIR Return writer.

Forbidden APIs and fields:

```text
Clone / into_parts / take_* / raw output getter
raw Facts / Recipe / JoinSig / Completion getter
VerifiedLoopOperationEffectProductV1 coercion
VerifiedLoopContinuationContractV1 reconstruction from L0/B0/I64
MirBuilder / session / ValueId / BasicBlockId / physical IDs
selector / name / AST / MIR rewalk
Option fallback / retry
```

`LoopPhysicalServicesV1` remains outside the semantic aggregate. A later
physical-session owner receives it as an explicit argument only after the
prephysical product and ReadyEntry are accepted.

## TextEq boundary

S6C already has complete logical TextEq authority:

```text
ResolvedBinaryExpressionSourceV1::Equal
S6CBinaryRelationV1::TextEqual
LoopOperationV2::TextEq(Text, Text) -> Bool
LoopOperationExecutionClassV2::NonFaulting
```

There is no named physical TextEq target/emitter authority. `StringBox::equals`,
the old JoinIR evaluator, generic MIR `CompareOp::Eq`, and backend comparison
code are consumers or compatibility implementations; none proves the exact
S6C source owner, Text operand representation, or physical target.

This does not block the Builder-free ingress I0. It does block opening the
physical session. The next design stop after ingress I0 is therefore
`S6C-TEXT-EQ-PHYSICAL-CONTRACT-D0`, with
`NoSafeSlice::MissingS6CTextEqPhysicalOwner` if no neutral owner can be named.

## I0 acceptance

```text
positive:
  output moves exactly once into one non-Clone aggregate
  exact domains and 15 = 13 + If + Exit census
  all 13 roles have exact source/execution evidence
  I3 preserves two distinct source anchors
  source-bound calls are Length then Substring by role, never position alone
  owner/frame/scope/function/Loop identity is exact
  branch=1, Return summary=1, Backedge=1, After=L0/B0/I64
  Loop Return and callable Tail remain distinct exact-two Completion views
  private HRTB references cannot escape

negative:
  missing/duplicate/swapped/foreign role or placement
  If or Exit classified as operation
  BodyEffect count treated as 13
  I3 anchor missing, duplicated, or collapsed
  wrong call receiver/arguments/result/Home/ABI/PureRead
  TextEq operands/result/source placement drift
  owner/frame/scope or After drift
  Tail imported into Loop Exit rows
  V1/V2 coercion or raw constituent escape
  host/session or physical ID stored in ingress
  AST/MIR/name lookup, Option, fallback, retry, production caller
```

## Minimal remaining DAG

```text
PHYSICAL-INGRESS-D0                  // this accepted BoxShape
  -> PHYSICAL-INGRESS-I0             // one caller-zero aggregate
  -> S6C-TEXT-EQ-PHYSICAL-CONTRACT-D0
  -> S6C-PHYSICAL-SESSION-D0
       names the production ReadyEntry/prelude issuer,
       explicit LoopPhysicalServicesV1 handoff,
       topology/cursor, and exact-two Completion/DraftSeal terminal
  -> SESSION-E0 shell/prelude/topology
  -> SESSION-E1 exact 15-item cursor/control/backedge
  -> SESSION-E2 After/Tail/profile close/exact-two DraftSeal
  -> parity/canary
  -> bounded selector/caller cutover
  -> latest-HEAD integration
  -> legacy retirement
```

ReadyEntry, host, topology, operations, and Tail do not get separate competing
owner cards. They are bounded slices inside one future physical session.

## D0 close receipt

```text
accepted = corrected one-input prephysical aggregate
source shape delta = 0
Recipe/Join schema delta = 0
semantic effect delta = 0
Builder/MIR/physical ID delta = 0
production caller delta = 0
fallback/retry delta = 0
next executable row = PHYSICAL-INGRESS-I0
```
