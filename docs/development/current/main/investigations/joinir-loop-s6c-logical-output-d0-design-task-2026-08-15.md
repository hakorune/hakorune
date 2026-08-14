---
Status: closed; S6C owned logical JOINIR output representation and caller-zero I0 producer landed
Date: 2026-08-15
Decision: issue one source-retaining non-Clone logical-output product; keep JoinModule/MIR closed
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

The output boundary is now fixed and its bounded caller-zero producer is
landed. The product is a semantic logical transport, not a physical
preparation product. It retains the original source/Recipe/Join product so the
projection cannot become a second authority. JoinModule/MIR and production
remain closed.

## Audit decision

```text
Decision: accept one source-retaining non-Clone logical-output product for I0.
Source authority + canonical issuer: the existing combined S6C Recipe product
  is the only source/Recipe/Join authority; one S6C logical-output issuer
  consumes it by value.
Non-authority: JoinModule/JoinFunction/JoinInst, MIR ValueId, JoinValueSpace,
  JoinFuncId/JoinContId, names, AST/source order, Artifact, selector, fallback,
  retry, physical layout, and production callers.
Fail-fast boundary: exact owner, Recipe-local identity, typed rows, CallSlot
  source parity, Join transfer ownership, and terminal reject are fixed before
  output issuance.
Smallest next slice: design one product-first logical consumer boundary;
  no JoinModule, MIR, physical ID, or backend handoff.
Non-claims: no JoinModule generation, MIR lowering, backend, Artifact/
  provenance, production switch, fallback/retry, or legacy retirement.
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

## Closed D0 decisions

### 1. Owned boundary

The accepted product is a source-retaining non-Clone product:

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

Recipe-local keys are the logical identity. No new output-key issuer is
allowed. Bare `u32`, `ValueId`, `JoinFuncId`, `JoinContId`, names, selectors,
and physical IDs are forbidden.
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

The future consumer is a product-first seam inside the S6C logical-output
owner. It receives only `with_output`'s private HRTB view. The current
`LoopToJoinLowerer` remains a compatibility consumer, but its
MIR/name/Option-fallback API is not an accepted S6C path. I0 does not open a
production consumer; it only fixes the transport API for that future seam.

## Accepted I0 API contract

```rust
pub(crate) struct VerifiedS6CScanWithInitLogicalOutputV1 {
    product: VerifiedS6CScanWithInitRecipeProductV2,
    rows: S6CLogicalOutputRowsV1,
}

pub(crate) fn issue_s6c_scan_with_init_logical_output_v1(
    product: VerifiedS6CScanWithInitRecipeProductV2,
) -> Result<VerifiedS6CScanWithInitLogicalOutputV1, S6CLogicalOutputRejectV1>;

impl VerifiedS6CScanWithInitLogicalOutputV1 {
    pub(crate) fn with_output<R>(
        &self,
        callback: impl for<'rows, 'product>
            FnOnce(S6CScanWithInitLogicalOutputRefV1<'rows, 'product>) -> R,
    ) -> R;
}
```

The output owns fixed rows but does not copy Join transfer or source-call
contracts. `with_output` borrows the existing `LoopJoinLogicalTransferViewV2`
and typed call views from the retained product. No raw Recipe/JoinSig,
`into_parts`, JoinModule, or Recipe-only consumer input exists.

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

The design row is closed. Its first implementation row is a separate bounded
producer and remains caller-zero. That producer must use split files
`s6c_scan_with_init_joinir_output.rs`,
`s6c_scan_with_init_joinir_output_rows.rs`, and a focused test module; each
Rust source stays below the 760-line design trigger and 800-line hard stop.

## I0 implementation receipt (2026-08-15)

`JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-LOGICAL-OUTPUT-I0` is landed in split
`s6c_scan_with_init_joinir_output.rs` and
`s6c_scan_with_init_joinir_output_rows.rs`. It consumes the combined product
by value, issues the fixed logical rows once, retains the original product, and
lends only the private HRTB output view. Six focused S6C tests, cargo check,
format, diff, pointer, and Loop pre-cutover guards are green. It remains
caller-zero; JoinModule/MIR and physical/production handoff are not part of I0.

## Next design pointer

`JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-LOGICAL-OUTPUT-CONSUMER-D0` specifies the
future product-first logical consumer boundary. It must not re-pair Facts,
Recipe, or Join authorities.

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
