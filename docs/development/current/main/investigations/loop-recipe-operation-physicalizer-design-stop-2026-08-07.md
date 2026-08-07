# Loop operation physicalizer design stop

Status: `DESIGN-STOP`
Date: 2026-08-07
Parent: `LOOP-RECIPE-OPERATION-EFFECT-CROSS-PROFILE-PARITY-S0`
Authority:
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Purpose

Freeze the smallest common physicalizer boundary before any operation MIR is
emitted. This is a design-only stop after callable/G0 neutral operation/effect
parity. It fixes the physical owner, borrowed services, item dispatch,
failure discard, and Generic item-3 carrier bridge.

The next implementation may be a caller-zero, test-only one-operation canary
only after this card is accepted. No production selection or legacy retirement
opens here.

## Source authority

```text
LoopRecipeV1 / JoinSig:
  logical operation kind, operands, item placement, continuation ports

VerifiedLoopCoreProductV1:
  source-bound Recipe, JoinSig, BindingRef and effect relations

VerifiedLoopOperationEffectProductV1:
  move-only item-keyed source/effect evidence consumed by physicalization

profile adapters:
  callable/G0 source relation and Tail/After evidence; no physical owner

ReadyLoopEntryV1:
  session-local, single-use entry materialization receipt

CanonicalSsaFunctionSessionV2 and borrowed services:
  CFG, BindingSSA, PHI transaction, current function/block and completion
  owners remain in their existing canonical sessions
```

The common physicalizer must consume one move-only private product that
contains the operation/effect product and one common Loop continuation
capability. It must not receive those as unrelated arguments or reconstruct
source facts after consumption. The product may also carry a private,
non-authoritative lookup index.

## Non-authority and forbidden inputs

The physicalizer must not own or receive:

```text
AST, names, source preorder, route labels, producer/profile switches,
callable Prelude arguments, callable Tail, Generic tail reads, return ABI,
VerifiedFunctionCompletion, Return, DraftSeal, collector, module publication,
legacy scheduler, retry/fallback policy, or a second Recipe/effect catalog
```

It must not create a Loop-local CFG, BindingSSA, PHI transaction, undo log, or
semantic LoopKey-to-block/ValueKey-to-ValueId truth. Private transient indexes
are permitted only as non-authoritative lookup receipts.

## Owner and failure contract

### Stage A: preflight, no Builder mutation

Before allocating a block, ValueId, PHI, or instruction, validate all of:

```text
exact product owner/frame/Scope/Region
exact item-keyed operation and effect support
item block/loop placement
entry and preheader identity
current-block/continuation identity
supported value class and operand relation
required carrier entry availability
```

Any mismatch is `NoSafeSlice` and leaves the move-only product unconsumed when
possible.

### Stage B: emission, whole-session discard

Once physical emission begins, the enclosing unpublished function session is
poisoned on failure. `PhiTxn::abort_on_err` is only best-effort local
diagnostic cleanup. Atomicity belongs to
`CanonicalFunctionLoweringSessionV1::discard_unpublished`; caller restore is
performed once, and a fresh request is required for any later attempt.

Same-session repair, retry, fallback, reselection, ID rollback, and reuse of a
partially emitted product are forbidden.

## Generic item-3 bridge

Generic item 3 remains a normal Recipe `ReadBinding` operation in the parent
body. Its source anchor is the existing child-entry `DerivedCarrierEntry`
for carrier 2; this is not an operation relabel.

The physicalizer must:

```text
1. place item 3 in its Recipe-declared parent block;
2. emit the ordinary ReadBinding through canonical BindingSSA;
3. resolve its result BindingRef to one ValueId;
4. issue a child-entry carrier-seed receipt for carrier 2;
5. connect the seed to the existing child-entry obligation;
6. never infer placement from the source anchor or fall back to root preheader.
```

One nested fixture must assert both parent-block placement and child-entry
seed ownership. C0/C1/After/Tail reads remain outside the operation stream.

## Candidate implementation slices (not yet open)

The design must fix this order before implementation:

```text
1. one ConstI64 or ReadBinding canary through the existing emitter;
2. one BinaryI64/CompareI64 canary after the first gate;
3. nested Generic item-3 carrier-seed receipt;
4. caller-zero fresh-session failure/reuse harness.
```

No new Recipe kind, profile-specific physicalizer, or production caller is
allowed. If Generic G0 cannot issue the same common demand, stop with
`NoSafeSlice`; do not add a G0-only physical route.

## Required reject matrix

The design must define typed pre-effect rejects for missing/duplicate/foreign
items or effects, wrong owner/frame/block/loop, unsupported operation/value
class, unresolved input, missing carrier entry, terminated preheader, existing
physical block, AST/name input, route/profile switch, and any second CFG/SSA/
PHI owner. It must also define the post-emission whole-session discard receipt.

## Exit condition

This card closes only when the following are fixed in the common physical
SSOT and the implementation-ready successor task:

```text
physicalizer input type and move semantics
borrowed CFG/BindingSSA/PhiTxn service APIs
item-keyed operation dispatch matrix
Stage-A NoSafeSlice matrix
Stage-B fresh-session failure/discard harness
Generic item-3 parent-placement + carrier-seed receipt
production switch and legacy disposition (explicitly closed here)
same-commit documentation list
```

Until then, do not edit Builder lowering or emit physical operation MIR.

## Same-commit documentation obligation

The design closeout and every later physicalizer implementation slice must
update, in the same commit as the code or task transition:

```text
docs/reference/mir/loop-recipe-contract.md
docs/reference/mir/generic-loop-stage-matrix.md
docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md
src/mir/loop_recipe_contract/README.md
src/mir/builder/resolved_lowering/README.md
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
```

Reference pages must claim only the landed design/receipt. Physical,
production, backend, and legacy-deletion claims remain absent until their
own implementation receipts land.
