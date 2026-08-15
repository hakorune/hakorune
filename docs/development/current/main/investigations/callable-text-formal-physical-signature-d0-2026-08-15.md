---
Status: accepted BoxShape; active caller-zero implementation brief
Date: 2026-08-16
Work mode: fast
Classification: T2 BoxShape accepted; next T2 BoxCount is caller-zero
Parent: LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0
---

# CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-D0

The runtime can atomically pin and retire a correct ExactText pair set. This
Decision now fixes the compiler-owned callable signature as two explicit
scalar lanes and keeps the later root residence, slice/cursor, backend
projection, and route policy outside that signature.

## Six-line brief

```text
Decision: accept logical ExactText as one formal/BindingRef mapped to two contiguous physical u64 lanes [slot,generation], while every ordinary scalar maps to one lane; logical /N and physical_formal_lane_count remain separate authorities, and the 16-byte aggregate ABI is rejected.
Source authority + canonical issuer: same-brand selected/batch callable identity plus the complete callable-parameter contract cohort are consumed by one new package-owned VerifiedCallablePhysicalSignatureCohortV1 issuer; it owns the total ordinal-to-lane map and never consumes Completion.
Non-authority: /N suffix, MirType::String, FunctionSignature alone, raw Vec<ValueId>, runtime validator argument order, TextFormalBorrowV1, Completion/header rows, AST names, Recipe keys, Dynamic leases, fallback, and retry.
Fail-fast boundary: reject missing/duplicate logical ordinal, lane gap/overlap/swap/out-of-range, foreign brand/target, logical/physical count conflation, detached pair lanes, legacy one-to-one skeleton/call projection, or any need to infer generation from a raw slot.
Smallest next slice: CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-I0 issues and transports the complete caller-zero package mapping, including one combined Installed S6C loan; skeleton/call-edge/session consumers remain later rows.
Non-claims: no call-site actualization, C/LLVM activation, session ValueId, entry acquire/root projection, Completion epilogue, Text slice/cursor, Trap lowering, TextEq route, production caller, main integration, fallback, or retry.
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
site, Completion, root residence, slice, pointer, length, or route policy.

## Boundary after the signature

The two-lane wire is the stable callable boundary, not the function-internal
Text representation. Later owners must preserve this one-way split:

```text
ExactText logical formal
  -> [slot, generation] physical signature
  -> atomic callee-entry lease-set
  -> non-splittable TextFormalCallResidenceSetV1
       lease-set token + PinnedTextRootResidenceV1[]
  -> session-branded TextSliceRefV1 / backend-local TextPlan
  -> scoped backend ptr/len projection only
```

`PinnedTextRootResidenceV1` identifies one immutable valid-UTF-8 root while
the enclosing residence set owns its lifetime. `TextSliceRefV1` is only a
bounded range over such a root with a UTF-8/code-point boundary receipt.
`TextPlan` remains the existing transient non-Box carrier. Raw `ptr,len` is a
backend projection; it is never the lifetime owner, callable ABI, BindingRef,
or independently storable common product.

Production entry consumes the already-published two lanes directly. It must
not call the probe issuer that reconstructs a generation from a raw handle.
The landed `TextFormalBorrowV1` remains validator/test evidence, not the
production call actualizer.

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
separate surfaces, so they cannot be composed by calling both. The current I0
adds one combined S6C arm that lends selected input, ExactText contracts,
package-owned S6C child, and the signature row in one HRTB callback. The later
exact call-edge issuer must consume/extend that same scoped arm rather than
open a second selected-key loan. Ordinary and Dynamic roles remain separate.

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
root residence / slice / ptr-len dependency = 0 in signature issuer
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
S6C-specific physicalizer, root/slice/pointer state in the signature product,
Builder/session inference, fallback, or retry.

## Active implementation brief

```text
Change:
  issue one non-Clone package-owned physical-signature cohort from the selected/batch identity and complete parameter-contract cohort; transport the same rows through install and one combined S6C Port loan; retire the independently consumable S6C-only signature gap.
Contract:
  ordinary scalar = one lane; ExactText = adjacent [slot,generation]; logical ordinal/BindingRef and physical lane indices are complete/disjoint; Completion, ValueId, residence, ptr/len, call edge, and route policy stay out.
Done:
  focused ordinary/ExactText/mixed positive rows; missing/duplicate/foreign/lane-gap-overlap-swap negatives; non-Clone/private-constructor/caller-zero guard; package README and ABI SSOT synchronized.
Stop:
  return to NoSafeSlice if the issuer needs header/Completion, raw function signature inference, detached lane products, two Port consumptions for one S6C key, or any Builder/runtime/production caller.
```

## Ordered successor families

After this accepted Decision:

```text
CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-I0
  caller-zero package mapping and transport

TEXT-FORMAL-PINNED-RESIDENCE-D0/I0
  ordered internal seams, not separate authority cards:
    post-install exact target/origin/signature call edge
    pair-based entry acquire + pinned UTF-8 root projection
    Canonical composite formal/residence adoption
    Completion-backed DraftSeal finish coverage

LOOP-TEXT-SLICE-EXECUTION-D0/I0
  ordered internal seams:
    pinned root -> CP-correct transient slice
    generic sequential code-point cursor
    valid-UTF8 exact-equality -> inline byte equality

LOOP-TEXT-ROUTE-PERF-R0
  exact / meso / whole evidence
  static admitted route; runtime fallback/retry = 0
```

These are two bounded implementation families after the signature row, not a
new card per type. The runtime lease I0 remains a substrate only. No family
may claim a production callable route until the common V2 envelope, admitted
route, residence, Completion epilogue, and canonical session meet at one edge.
