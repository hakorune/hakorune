---
Status: active design stop
Date: 2026-08-15
Work mode: design_stop
Classification: T2 BoxShape
Parent: LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0
---

# CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-D0

The runtime can now atomically pin and retire a correct ExactText pair set.
It does not prove which callable owns the pairs, how logical formals expand to
physical lanes, or how caller and callee consume one mapping. This row fixes
that compiler-owned signature shape before any MIR/session or production
caller is opened.

## Six-line brief

```text
Decision: keep logical ExactText as one formal/BindingRef and map it to two contiguous physical u64 lanes [slot,generation], while every ordinary scalar maps to one lane; logical /N and physical_formal_lane_count remain separate authorities.
Source authority + canonical issuer: same-brand selected/batch callable identity plus the complete callable-parameter contract cohort are consumed by one new package-owned VerifiedCallablePhysicalSignatureCohortV1 issuer; it owns the total ordinal-to-lane map and never consumes Completion.
Non-authority: /N suffix, MirType::String, FunctionSignature alone, raw Vec<ValueId>, runtime validator argument order, TextFormalBorrowV1, Completion/header rows, AST names, Recipe keys, Dynamic leases, fallback, and retry.
Fail-fast boundary: reject missing/duplicate logical ordinal, lane gap/overlap/swap/out-of-range, foreign brand/target, logical/physical count conflation, detached pair lanes, legacy one-to-one skeleton/call projection, or any need to infer generation from a raw slot.
Smallest next slice: census the selected/batch parameter cohort, skeleton/publication signature consumers, exact static-call target handoff, and Canonical formal adoption; then name one issuer plus one scoped combined Installed Port loan and one composite callee consumer, with code still zero.
Non-claims: no signature code, call-site actualization, C/LLVM activation, session ValueId, entry acquire emission, Completion epilogue, Trap lowering, S6C/TextEq route, production caller, main integration, fallback, or retry.
```

## Target product

```rust
#[derive(Debug)]
pub(crate) struct VerifiedCallablePhysicalSignatureCohortV1 {
    // non-Clone; package-owned; fields private
    // complete same-brand callable rows
}

pub(crate) enum PhysicalFormalLaneRoleV1 {
    OrdinaryScalar,
    ExactTextSlot,
    ExactTextGeneration,
}
```

Each callable row must close:

```text
selected callable identity / catalog brand
logical_arity
physical_formal_lane_count
complete logical formal ordinal set
complete/disjoint physical lane index set

ordinary scalar ordinal -> [OrdinaryScalar]
ExactText ordinal       -> [ExactTextSlot, ExactTextGeneration]
```

Lane order is deterministic:

```text
logical formal ordinal order
  ordinary -> one lane
  ExactText -> slot immediately followed by generation
```

The product contains no `ValueId`, `BasicBlockId`, runtime token, source call
site, Completion, or route policy.

## Required owner fan-out

One package-owned row must be borrowed, never reconstructed, by three later
mechanical consumers:

```text
physical-signature row
  ├─ mapping-aware callable skeleton/publication
  ├─ post-install exact call-edge argument expansion
  └─ Canonical callee composite-formal adoption
```

The post-install call-edge issuer is distinct from the signature issuer. It
must co-seal the whole-source exact static target inventory, Installed Port,
caller original-formal/no-rebind proof, and the callee signature row. It may
project lanes but cannot change their meaning or order.

The future Installed Port must use one total exactly-once child loan. The
current S6C child and Main static-child loans consume the same selected key on
separate surfaces, so they cannot be composed by calling both. The D0 must
name a combined S6C arm that lends selected input, ExactText contracts,
package-owned S6C child, signature row, and exact call-edge view in one HRTB
callback. Ordinary and Dynamic roles remain separate arms.

## Canonical callee boundary

The future Canonical consumer receives one signature row and derives both
physical parameter `ValueId`s from the already-created physical function
parameter list:

```text
one logical BindingRef
  -> slot ValueId: ordinary Text carrier
  -> generation ValueId: private sidecar only
```

Only the slot lane is published to ordinary Binding SSA. Generation is never
an independent binding and cannot be recovered from `MirType`, raw slot, or
ordinary SSA reads. A scoped composite forward view is required for nested
calls.

## Acceptance

```text
one package-owned signature cohort issuer
Completion/header dependency = 0
logical arity and physical lane count named separately
complete/disjoint ordinal and lane coverage
ExactText [slot,generation] adjacency and role tags exact
ordinary scalar behavior preserved
ValueId / Builder / MIR = 0 in the signature product
one future combined Installed Port seam named
one future mapping-aware skeleton consumer named
one future exact call-edge consumer named
one future composite Canonical adoption consumer named
V1/Dynamic adapter = 0
fallback/retry = 0
production caller = 0
```

## NoSafeSlice

Keep:

```text
NoSafeSlice::MissingTextFormalCallableSignatureIssuer
```

if any safe design requires Completion to issue formal lanes, `/N` or
`FunctionSignature` to infer physical count, caller-supplied batch/key/header,
separate slot/generation products, two independently consumable Installed
loans for one S6C key, raw-handle generation recapture, a V2-to-V1 adapter,
S6C-specific physicalizer, Builder/session inference, fallback, or retry.

## Ordered follow-on

Only after this D0 is accepted:

```text
CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-R0/I0
  caller-zero package mapping and transport

TEXT-FORMAL-EXACT-CALL-EDGE-D0/I0
  post-install target/origin/signature co-seal

TEXT-FORMAL-ENTRY-NORMAL-EXIT-EPILOGUE-D0/I0
  Canonical entry lease-set ledger
  Completion-backed DraftSeal finish coverage
```

The runtime lease I0 remains a substrate only; none of these rows may claim a
production callable route until all three meet at one canonical session edge.
