---
Status: design stop; worker consensus recorded
Date: 2026-08-15
Decision: design one prephysical S6C ingress without opening physicalization
Scope: M8 LoopV0 forward ScanWithInit; caller-zero and Builder-free
---

# JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-PHYSICAL-INGRESS-D0

## Six-line brief

```text
Decision: define one move-only prephysical ingress view for the sealed S6C product; do not emit JoinModule, MIR, or physical layout.
Source authority + canonical issuer: S6C Facts/Recipe/Join output plus the existing resolver context, operation/effect, and continuation issuers; one private ingress co-seal may borrow them without reissuing meaning.
Non-authority: raw Facts/Recipe/JoinSig, LoopToJoinLowerer, AST/name lookup, JoinModule/MIR/ValueId/BasicBlockId, ABI/selector, fallback, retry, and production routing.
Fail-fast boundary: owner/frame/scope, exact 15 item-keyed operation/effect rows, two source-bound calls, logical transfer, After, and host capability must agree before Builder/session or physical-ID effects.
Smallest next slice: audit and specify one private HRTB physical-input façade and its typed reject/discard contract; issue no Prepared* or physical receipt in this row.
Non-claims: no physical CFG/SSA/PHI/layout, JoinModule conversion, ABI/Completion publication, Artifact/source binding, selector, production caller, fallback, retry, or legacy retirement.
```

## Current capsule

The landed S6C chain is:

```text
resolver source seal
  -> VerifiedS6CScanWithInitFactsV1
  -> VerifiedS6CScanWithInitRecipeProductV2
  -> private logical JOINIR input façade
  -> source-retaining logical output
  -> typed caller-zero logical consumer
```

The logical consumer's `Consumed` terminal is an observation of that sealed
product. It is not a physical demand, a Builder capability, or permission to
reconstruct operation/effect/continuation facts. The next boundary must therefore
start from the retained product, not from `Consumed` and not from a Recipe-only
argument.

Existing common owners already cover parts of the required physical contract:

```text
VerifiedLoopSemanticContextV1
VerifiedLoopOperationEffectProductV1
VerifiedLoopContinuationContractV1
VerifiedLoopOperationPhysicalDemandV1
PreparedLoopOperationProgramV1
PreparedLoopPhysicalLayoutV1
```

Their reuse is not yet proven for S6C. In particular, the S6C output does not yet
carry a co-sealed operation/effect product, exact owner/frame/scope context, or
the host/session capability expected by a physicalizer. The D0 decision is to
name and test that join before any physical product is issued.

## Read-only owner audit result

The current code confirms the boundary gap rather than closing it:

```text
VerifiedS6CScanWithInitRecipeProductV2
  owns Facts + Recipe + role seal + Join closure
  lends source-bound calls and LoopJoinLogicalTransferViewV2

VerifiedLoopOperationPhysicalDemandV1
  requires VerifiedLoopSemanticContextV1
  requires VerifiedLoopOperationEffectProductV1
  requires VerifiedLoopContinuationContractV1
  then may prepare a physical program/layout
```

`VerifiedS6CScanWithInitFactsV1` retains the resolver source/Completion
co-seal and exact body coverage, but does not issue or retain the neutral
semantic context, item-keyed operation/effect evidence, or continuation
capability required by `VerifiedLoopOperationPhysicalDemandV1`. The existing
callable adapter creates those products from a different callable Recipe
co-seal; it is not valid to rebuild them from S6C logical rows or to pass the
logical consumer's `Consumed` observation as a substitute.

Therefore the exact audit decision is:

```text
physical-ingress issuer/co-seal = not yet named for S6C
physical implementation I0       = NoSafeSlice
reason                            = MissingS6CPhysicalIngressIssuer
```

This is an authority gap, not a test-only or theoretical hardening issue. No
code or new semantic receipt is authorized until an existing resolver-side
issuer can provide the missing context/effect/continuation/host capability and
co-seal it with this exact S6C product without a second source or Recipe pair.
The evidence is the current split between `semantic_context.rs`,
`operation_effect.rs`, `continuation.rs`, and the callable-only adapter in
`src/mir/compiler/callable_single_loop_operation_effect.rs`.

## Canonical boundary

The future physical owner is a selected adapter under the existing resolved
lowering / loop physicalizer owner. It may eventually borrow a private view from
the S6C product and then hand physical IDs to the canonical Builder/CFG/SSA/PHI
session. It must not become a second semantic issuer.

The proposed façade is intentionally prephysical:

```rust
struct S6CPhysicalIngressRefV1<'a> {
    logical_rows: S6CScanWithInitRecipeRowsRefV2<'a>,
    calls: S6CLogicalCallPairsRefV1<'a>,
    transfer: LoopJoinLogicalTransferViewV2<'a>,
    context: S6CPhysicalContextRefV1<'a>,
    operations: S6COperationEffectRowsRefV1<'a>,
    continuation: S6CContinuationRefV1<'a>,
}

with_s6c_physical_ingress<R>(
    product: &VerifiedS6CScanWithInitLogicalOutputV1,
    callback: impl for<'a> FnOnce(S6CPhysicalIngressRefV1<'a>) -> R,
) -> Result<R, S6CPhysicalIngressRejectV1>
```

This is a shape sketch only. The exact field types and issuer are accepted only
after the existing context, operation/effect, continuation, and host/session
owners are found to co-seal without a new semantic `Verified*` authority.

The façade must retain the original combined product and lend only private
borrow views. It must not expose raw `VerifiedLoopRecipeV2`, raw JoinSig, source
AST, method names, or any physical ID. `LoopJoinLogicalTransferViewV2` remains a
borrowed logical transfer, not an owned physical CFG.

## Required design checks

Before accepting a later implementation row, the D0 audit must establish:

```text
1. one source/Recipe/Join product owner; no Facts/Recipe/Join re-pairing
2. one existing issuer for owner/frame/scope context
3. one existing issuer for all 15 item-keyed operation/effect rows
4. one existing issuer for continuation and exact After = L0/B0/I64
5. exact Length/Substring source-call parity remains borrowed and role-wise
6. logical transfer remains branch=1, Return summary=1, Backedge=1
7. callable Tail -1 remains outside the loop ingress
8. host/session capability is explicit before Builder or physical IDs
9. every rejection is typed and occurs before session/physical effects
10. no `LoopToJoinLowerer` rewalk, name dispatch, Option fallback, or retry
```

The common operation/effect product may be reused only if its owner, loop,
placement, operation set, source anchors, and effect rows can be matched to the
S6C product by existing sealed relations. S6C logical rows must never be used to
invent effects, Home, ABI, continuation, or physical demand.

## Acceptance matrix for this D0

```text
accepted:
  - exact owner/frame/scope and source target parity are named
  - existing operation/effect and continuation issuers are identified
  - one private HRTB ingress and typed reject/discard lifecycle are specified
  - physicalizer owner is named without opening its implementation

rejected:
  - missing/duplicate/swapped/foreign operation or effect row
  - owner/frame/scope drift
  - Length/Substring receiver, argument, result, or placement drift
  - branch/Return summary/Backedge/After drift
  - callable Tail imported as a loop exit
  - raw Recipe/JoinSig/AST/name/MIR re-observation
  - physical ID or Builder/session allocation before all checks
  - `LoopToJoinLowerer`, `Option`, fallback, retry, or production caller
```

## Stop line / NoSafeSlice

Keep `work_mode = design_stop` if any of these remain true:

```text
the canonical operation/effect/continuation issuer cannot be named
S6C needs a new effect/Home/ABI/Completion meaning
the retained product must be split or re-paired
the host/session capability is implicit or synthesized
physical layout or IDs are required to express the ingress
the only available path rewalks MIR/AST or uses LoopToJoinLowerer
```

The explicit stop token is:

```text
NoSafeSlice::MissingS6CPhysicalIngressIssuer
```

When the issuer/co-seal is closed, a separate `...-PHYSICAL-INGRESS-I0`
implementation card may be opened. That later row remains Builder-free until
the ingress product itself is accepted; physical CFG/SSA/PHI and production
selection require separate design decisions.

## Worker evidence

Three read-only audits agreed on the following:

```text
LoopToJoinLowerer is a compatibility consumer, not the S6C authority.
The existing physical-demand family is a candidate reuse target, not yet a
proven S6C ingress. The safe next step is a prephysical HRTB design stop;
physical implementation is NoSafeSlice until context/effect/continuation and
host/session capabilities are co-sealed.
```

No code, fixture, JoinModule, MIR, physical ID, selector, or production caller
is introduced by this D0.
