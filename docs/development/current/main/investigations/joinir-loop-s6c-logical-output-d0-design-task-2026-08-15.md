---
Status: design_stop; S6C logical JOINIR input façade is landed, but the owned logical output representation is not yet accepted
Date: 2026-08-15
Decision: keep JoinModule/MIR closed and fix one source-retaining logical-output owner before any producer
Scope: M8 LoopV0 forward ScanWithInit logical output only; no physical activation
---

# JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-LOGICAL-OUTPUT-D0

## Current capsule

The combined non-Clone `VerifiedS6CScanWithInitRecipeProductV2` and its
private `S6CScanWithInitLogicalJoinInputRefV1` façade are landed. The façade
co-checks the fixed Recipe domains, source-bound length/substring CallSlot
rows, TextEq/If, and the existing Join branch/summary/Backedge/After view.
It does not emit JoinIR, MIR, physical IDs, Artifact, a selector, or a
production caller.

The next output boundary is deliberately a design stop. An owned logical
projection may be needed by a future consumer, but its owner, identity, and
consumer dialect are not yet canonical. Do not implement a new `Verified*` or
`Prepared*` output product until this card is accepted.

## Audit decision

```text
Decision: NoSafeSlice for output implementation; accept one D0 design task.
Source authority + canonical issuer: the existing combined S6C Recipe product
  is the only source/Recipe/Join authority; the logical-output issuer is not
  yet accepted.
Non-authority: JoinModule/JoinFunction/JoinInst, MIR ValueId, JoinValueSpace,
  JoinFuncId/JoinContId, names, AST/source order, Artifact, selector, fallback,
  retry, physical layout, and production callers.
Fail-fast boundary: owner, identity, row/control vocabulary, call relation,
  and ownership/failure terminal must be fixed before output issuance.
Smallest next slice: design the source-retaining logical-output owner and its
  private view; no code, new receipt, JoinModule, or MIR handoff in D0.
Non-claims: no output producer, JoinModule generation, MIR lowering, backend,
  Artifact/provenance, production switch, fallback/retry, or legacy retirement.
```

## Rejected existing output candidates

`JoinModule`/`JoinFunction`/`JoinInst` are a compatibility dialect, not the
logical SSOT: they are mutable and `Clone`, use `VarId = MIR ValueId`, carry
method/box names and `MirType`, and feed a MIR bridge. `JoinValueSpace` issues
numeric IDs. `LoopToJoinLowerer::lower` rewalks `MirFunction`/`LoopForm`, uses
name/route selection, and returns `Option<JoinModule>` for fallback. None of
these may become the S6C output issuer.

The existing V2 Recipe and `LoopJoinLogicalTransferViewV2` remain inputs:
Recipe owns typed rows and local keys; Join owns branch, summary, Backedge,
and After transfer. The future output must borrow or co-seal these authorities,
not re-elaborate them.

## Decisions that D0 must close

### 1. Owned boundary

Choose whether the future product is a source-retaining non-Clone product,
for example:

```text
VerifiedS6CScanWithInitLogicalOutputV1 {
  original VerifiedS6CScanWithInitRecipeProductV2,
  fixed logical projection,
}
```

An output row set without the original product is not acceptable: it would
detach Facts/Recipe/Join and create a second authority. A borrow-only façade
cannot be called an owned output product. `with_output` must be the only public
read boundary; no `into_parts`, raw Recipe/JoinSig getter, or Recipe-only
consumer input is allowed.

### 2. Identity and representation

D0 must decide whether existing Recipe-local keys are sufficient or a new
branded logical output identity is required. Bare `u32`, `ValueId`,
`JoinFuncId`, `JoinContId`, names, selectors, and physical IDs are forbidden.
The exact logical vocabulary must remain typed:

```text
inputs = 3, carrier = 1, loops = 1, blocks = 3
operations = 13, If = 1, Recipe Exit = 1
CallSlot = 2, TextEq = 1
```

`Length` and `Substring` are typed call roles. Their receiver, ordered args,
result class, source owner/frame, Home, ABI, effect, and placement remain
co-sealed with the retained source-bound call relation. Method/box names must
not be reconstructed.

### 3. Control ownership

The logical output may retain fixed role witnesses for `If I8/V10` and
`Return I10/FunctionExit`, but the Join transfer view remains the authority for
branch, Return summary, Backedge, and `After = L0/B0/I64`. The callable Tail
`return -1` remains Facts/Completion authority and is never imported as a Loop
exit or Join summary.

### 4. Future consumer owner

D0 must name one product-first consumer seam and define its input/output
dialect. The current `LoopToJoinLowerer` may remain a compatibility consumer,
but its MIR/name/Option-fallback API is not an accepted S6C path. No output
producer is opened until this owner can consume the source-retaining product
without rewalking MIR or selecting by name.

## Bounded D0 deliverables

```text
D0-A  owner/issuer
      one canonical logical-output issuer and one future consumer seam
D0-B  identity/schema
      fixed key/identity policy, typed rows, exact domains and preorder
D0-C  source co-seal
      Facts + Recipe rows + source-bound calls + Join transfer relations
D0-D  ownership/API
      non-Clone source-retaining product, private HRTB view, no raw escape
D0-E  failure contract
      missing/duplicate/swap/foreign/shape drift reject before any effect
D0-F  implementation boundary
      future files split below 760 lines; no change to typed_schema_v2.rs
```

D0 is documentation and read-only census only. It must not mint a new
semantic receipt, add a Recipe kind, add a JoinModule adapter, or call a
backend. Once D0 is accepted, the first implementation row is a separate
bounded producer and remains caller-zero.

## Acceptance and negative matrix

Acceptance requires a single source-retaining issuer, exact domain coverage,
fixed operation/value/block/control representation, and an HRTB view whose
lifetime prevents output rows from escaping. The output must preserve:

```text
Length   = I1, receiver V0, args [], result V4:I64
Substring= I6, receiver V0, args [V6,V8], result V9:Text
TextEq   = I7, V9 == V1 -> V10:Bool
If       = I8, condition V10, then K2
Return   = I10, V11:I64, FunctionExit
Join     = one Return summary, one Backedge, After L0/B0/I64
Tail     = absent from Recipe/Join output
```

Required negatives include call-role/receiver/argument/result swaps, wrong
class or placement, missing/duplicate item or block, TextEq/If drift, Return
target/value drift, missing/extra summary or Backedge, After drift, foreign
owner/frame, Recipe-only or JoinSig-only input, Tail import, raw output escape,
MIR/physical ID allocation, name/selector lookup, and fallback/retry.

## Parked boundaries

Artifact/source binding, ABI publication, JoinModule generation, MIR lowering,
physical JoinIR, selector/production activation, Dynamic live cutover, warning
cleanup, and legacy retirement remain parked. The known parent-baseline test
red is evidence debt, not a reason to open this output design.
