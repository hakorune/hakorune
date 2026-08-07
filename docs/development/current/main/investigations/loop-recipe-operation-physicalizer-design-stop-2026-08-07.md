# Loop operation physicalizer design stop

Status: `CLOSED; Decision B accepted after independent and external review`
Date: 2026-08-07
Parent: `LOOP-RECIPE-OPERATION-EFFECT-CROSS-PROFILE-PARITY-S0`
Authority:
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Decision

The first physical operation proof is a private leaf-emitter canary, not a
claim that a complete `LoopRecipeV1` has been physicalized.

```text
Decision B:
  full demand and leaf emission are separate proofs

Decision A:
  a synthetic one-operation full Recipe is not the first authority
  it may be added later as an integration fixture
```

Callable and Generic G0 fixtures contain seven and fifteen operations. Taking
only the first operation from either full product would be partial lowering
and a hidden selector. No `first_operation`, `select_operation`, filter, or
ordinal-based execution API may be added to the full demand.

## Authority layers

### Logical authority

Existing products remain authoritative:

```text
LoopRecipeV1 / LoopJoinSigV1
  logical operations, nesting, item placement, ports, and carriers

VerifiedLoopCoreProductV1
  source-bound Recipe, JoinSig, BindingRef, and effect relations

VerifiedLoopOperationEffectProductV1
  moved Core plus exact evidence for every Recipe operation
```

### Full physical demand

The private, move-only full-program input is:

```text
VerifiedLoopOperationPhysicalDemandV1 {
  context: VerifiedLoopSemanticContextV1,
  operation_effect: VerifiedLoopOperationEffectProductV1,
  continuation: VerifiedLoopContinuationContractV1,
  index: private LoopOperationPhysicalIndexV1,
}
```

The context and continuation are neutral move-only transport wrappers around
resolver/JoinSig-issued evidence; they do not reissue or clone authority. Its
only whole-program transition is conceptually:

```text
prepare_all(self)
  -> PreparedLoopOperationProgramV1
```

`PreparedLoopOperationProgramV1` retains the complete demand plus a Recipe-
derived exact schedule and coverage receipt. Semantic preflight proves every
operation is supported, every source/effect row exists, every logical
placement is unique, and the operation count is complete before Builder
mutation.

The private index is a lookup cache over existing keys. It cannot determine
execution order, filter by profile, select one operation, or become a second
Recipe. Execution order comes only from Recipe Loop/Block/Item structure; the
sorted evidence vector is not execution authority.

### Leaf emission

One leaf operation is represented separately:

```text
PreparedLoopOperationEmissionV1 {
  owner
  item
  operation
  evidence
  expected_loop
  expected_block
}
```

The first canary may use a private `cfg(test)` ConstI64 constructor. That
constructor is not issued by extracting one row from a full demand. The leaf
emitter never sees Recipe, profile identity, Tail, ABI, Completion, Return,
DraftSeal, collector, publication, or Loop continuation.

Continuation remains owned by the full orchestrator:

```text
full demand/program:
  owns continuation

leaf emission:
  owns no continuation
```

## Physical placement receipt

Topology allocation may issue one private receipt from the existing canonical
CFG owner:

```text
LoopPhysicalBlockReceiptV1 {
  owner
  preheader
  rows: logical Loop/Block/role -> physical BasicBlockId
}
```

The receipt proves only that `CanonicalCfgSessionV1` created one exact mapping.
It is not a second CFG owner. Before instruction emission, the placement binder
must match the prepared operation's owner, expected Loop, expected logical
block, preheader, and function state to one receipt row. Unconditional emission
into `current_block` is forbidden.

## Two-stage preflight

### Semantic preflight

Before any Builder effect:

```text
complete operation coverage
supported operation/value classes
operand and source/effect relations
logical placement
continuation compatibility
```

Failure is typed `NoSafeSlice` with Builder effect zero.

### Physical placement binding

After topology blocks exist but before operation instruction emission:

```text
exact owner/function/preheader
logical block -> physical block
physical block is live and unterminated
```

Failure discards the whole unpublished function session. It is not described
as a pre-effect failure because block allocation has already occurred.

## Canonical owner and atomicity

The leaf emitter borrows existing canonical services only:

```text
CanonicalCfgSessionV1
CanonicalSsaFunctionSessionV2.identity
the function session's one PhiTxn
ReadyLoopEntryV1
```

`ReadyLoopEntryV1` is explicit even for ConstI64; its exact required-input set
is empty rather than omitted. No Loop-local CFG, BindingSSA, PhiTxn, undo log,
or transaction may be created.

`CanonicalFunctionLoweringSessionV1` remains the sole transaction owner. The
caller-zero canary explicitly discards the unpublished function on success and
failure because it does not reach DraftSeal. A single harness-only failure is
injected after successful emission; production emitter code has no test flag.
The harness then opens a fresh session and repeats the same semantic emission.
ValueId and BasicBlockId numbers are not compared across sessions.

## Module boundary

Before operation emission is added, keep topology and operation emission in
separate modules. The accepted target layout is:

```text
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/
  mod.rs
  topology.rs
  operation_emitter.rs
  operation_state.rs       # only when ValueKey materialization needs it
  tests.rs
```

The module split is a behavior-neutral BoxShape row. It must not add an
accepted operation shape. `operation_state` may map Recipe ValueKey to already
materialized ValueId, but it cannot own reaching BindingRef values, assignment
state, or PHI creation.

The Builder-free demand remains under:

```text
src/mir/loop_recipe_contract/
  operation_physical_demand.rs
  operation_physical_demand_tests.rs
```

## Ordered implementation ladder

Each line is one responsibility and one acceptance claim:

```text
1. LOOP-RECIPE-OPERATION-PHYSICAL-DEMAND-P0
   full Callable/G0 demand issuance and prepare_all; Builder effect zero

2. LOOP-RECIPE-PHYSICALIZER-MODULE-SPLIT-R0
   move the flat topology module into one directory facade, delete the old
   flat file, and leave exactly one module entry; behavior unchanged

3. LOOP-RECIPE-PHYSICAL-BLOCK-RECEIPT-P0
   canonical logical-block -> physical-block receipt only

4. LOOP-RECIPE-OPERATION-EMITTER-CONST-S0
   private ConstI64 leaf emission, exact placement, discard, fresh reuse

5. LOOP-RECIPE-OPERATION-EMITTER-READ-S1
   exact source claim and canonical BindingSSA read

6. LOOP-RECIPE-OPERATION-EMITTER-BINARY-S2
7. LOOP-RECIPE-OPERATION-EMITTER-COMPARE-S3
8. LOOP-RECIPE-OPERATION-EMITTER-WRITE-S4

9. LOOP-RECIPE-OPERATION-PROGRAM-CALLABLE-P0
   all seven Callable operations exactly once, then continuation

10. LOOP-RECIPE-OPERATION-PROGRAM-GENERIC-G0-P0
    all fifteen G0 operations plus item-3 carrier-seed bridge

11. production selection, M8/M9 coverage, M10b activation, M11/M12 retirement
```

Do not combine the Builder-free demand row, BoxShape module split, block
receipt, or Const acceptance into one commit. A one-operation full Recipe may
be added only as a later integration fixture and never as a special producer
or production route.

## First canary claim

The Const leaf canary may prove only:

```text
one prepared ConstI64
-> one exact physical block
-> existing canonical Builder service
-> one emission receipt

success and post-emission injected failure
-> whole unpublished session discard
-> fresh-session semantic repeat
```

It may not claim full Loop physicalization, all operations, Callable/G0
physicalization, BindingSSA read/write, carrier PHI, continuation, Completion,
DraftSeal, module publication, backend parity/performance, production
selection, retry/fallback removal, 19-route coverage, or legacy deletion.

## Legacy disposition

The topology-only `VerifiedLoopPhysicalBoundaryV1` intentionally drops source
anchors. Its general-looking `into_physical_boundary` entry must be quarantined
or renamed as topology-canary-only during the module-split row. It cannot feed
operation physicalization.

When the full physicalizer lands, production consumes only:

```text
full demand
-> prepare all
-> topology allocation
-> bind all placements
-> emit all exactly once
-> continuation
```

Test-only leaf constructors and topology-only boundaries remain inaccessible to
production. At production cutover, switch one named caller and delete that
caller's old operation writer, CFG/PHI edge, retry, and fallback in the same
commit. All-route cutover later retires the legacy scheduler, route-local
physicalizers/PHI writers, old DirectAccum pilot input, and profile-specific G0
physical route.

## Same-commit documentation obligation

Every implementation row must update its exact landed receipt in the same
commit:

```text
docs/reference/mir/loop-recipe-contract.md
docs/reference/mir/generic-loop-stage-matrix.md
docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md
src/mir/loop_recipe_contract/README.md
src/mir/builder/resolved_lowering/README.md
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
```

References must claim only landed behavior. Every touched source/test file
stays below 800 lines. Production, backend, selector, retry/fallback, and
legacy-deletion claims remain closed until their own rows land.
