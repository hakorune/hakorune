---
Status: design stop; prerequisite child of CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-D0
Date: 2026-08-15
Work mode: design_stop
Classification: T2 BoxShape; no runtime or compiler implementation is admitted
---

# CALLABLE-TEXT-FORMAL-CALL-RESIDENCE-D0

This child names the missing lifetime-owner decision for the preferred two-lane
`ExactText -> slot,generation` callable signature. It does not accept the
signature mapping itself and does not open a caller, Builder, or session.

Its first bounded child is
`text-call-actual-origin-route-d0-2026-08-15.md`, which fixes the finite
formal-versus-reject origin partition before choosing a lifetime route.

## Six-line brief

```text
Decision: for the closed formal-only domain, choose one mandatory callee-entry lease route: every admitted ExactText call carries contiguous `slot,generation` lanes, and exactly one callee entry acquires/pins that pair; the 16-byte aggregate and source-residence-only route are not admitted here.
Source authority + canonical issuer: ExactText parameter contracts supply logical BindingRef/ordinal; the future package-owned `VerifiedCallablePhysicalSignatureCohortV1` co-seals the two lanes and entry-lease plan, while the runtime `acquire_text_formal_call_lease_v1(pair)` is the sole mechanical pin issuer.
Non-authority: `TextFormalBorrowV1` read-lock closure, raw-handle generation recapture, `HostHandleLeaseIdentityV1`, DynamicV2 lease, raw HostHandle, ObjectIdentity, retain_h, KeepAlive, Completion semantic cleanup, C validator, AST/MIR/ValueId, caller-side duplicate pin, and fallback.
Fail-fast boundary: validate/acquire every ExactText pair before BindingRef publication/body effect; zero/missing/stale/non-Text/retiring/overflow, lane/target/brand drift, partial acquire, or missing/duplicate/foreign finish rejects canonically; normal continuation finishes exactly once and noreturn trap needs no post-trap cleanup.
Smallest next slice: fix the formal-only origin partition and this one-entry-owner handoff; then a caller-zero runtime BoxCount may add pin-aware retirement and opaque acquire/finish, followed later by mapping-aware call actualization and composite Canonical adoption.
Non-claims: no source-residence issuer, signature implementation, physical arity activation, C entry caller, TextEq route, Substring corridor, ValueId adoption, Canonical session, Builder, production caller, retry, or main integration.
```

## Why the current runtime is insufficient

`TextFormalBorrowV1` captures `{slot,generation}` and lends text under one
read lock, but it does not keep the slot alive after the callback. The generic
`retain_h` path obtains a compatibility object and allocates another raw slot;
it neither preserves the original generation identity nor covers every
`StableText`/`StringBox` payload. DynamicV2 lease identity owns an End/drop
operation, not a callable-wide borrow. Therefore none of these can be the
callable signature's lifetime authority.

## Selected route shape (design-only; issuer not landed)

The planned actualizer and lease are intentionally opaque and non-`Clone`;
the source owner is only the pre-call actualization proof, not the lifetime
owner after callee entry:

```text
VerifiedCallScopedTextOwnerLifetimeV1 {
    private source_owner: original live Text/StringBox owner,
    private until_call_return: sealed synchronous-lifetime proof
}

PreparedTextFormalCallActualizationV1 {
    private signature_row: source-backed logical-to-physical mapping,
    private source_residence: VerifiedCallScopedTextOwnerLifetimeV1,
    private capture_and_call: caller-private exact transition
}

TextFormalCallLeaseTokenV1 {
    private pair: {slot, generation},
    private lease_slot: TextFormalEntryLeaseSlotIdV1,
    private finish: exactly-once discharge
}
```

The caller-side actualizer must obtain the pair from the same source-backed
formal owner and target row, but it must not pin. A raw-slot acquire is
forbidden because it could capture the current generation of an already-reused
replacement. At the callee entry, `acquire_text_formal_call_lease_v1(pair)`
atomically validates Text class/generation and creates the single move-only
lease. Nested calls forward the composite pair without another pin. The
session sees only the private `TextFormalEntryLeaseSlotIdV1`; it never treats
generation as a second BindingRef value. Existing `drop_handle`, Dynamic lease
retirement, and `retain_h` are not sufficient. Both direct retirement paths
must later converge on one pin-aware helper. C/LLVM receives only the fixed
two-lane projection; the source receipt and lease token stay private.

The actualizer cannot smuggle an owned handle through an ordinary MIR
`Call`: the current Ownership SSA verifier rejects managed call operands and
results. The accepted design must therefore name either a borrow-only
capture/terminal or a dedicated ownership-aware call capability; a raw
`Vec<ValueId>` call edge or a `KeepAlive` no-op is not a lifetime proof.

This is a BoxShape decision only. Any SlotTable pin count, deferred retirement,
or C/runtime token implementation is a later BoxCount and remains unopened.

## Remaining implementation boundary

The route decision is now finite: only source-backed ExactText formal
parameters may become future physical calls, and they all use the mandatory
callee-entry lease. The current repository still has no issuer for the
source-to-target actualizer, no pin-aware retirement, and no composite session
adoption. Those are the next bounded implementation/design rows; they are not
permission to add a second source-residence route.

The callable target terminal must consume this residence through the same
package-owned physical-signature row that maps one logical `BindingRef` to
`slot` and `generation` lanes. It may not capture a pair from a detached
argument, and the callee/session must retain the pair as one composite receipt,
publishing only the slot as ordinary BindingRef SSA.

## Required negatives

```text
drop/release/rebind during call; stale generation; non-Text payload; zero slot;
duplicate finish; finish on the wrong generation; partial-acquire leak;
residence/token escape from the HRTB/call scope; one-lane adoption; raw
retain/release; direct `drop_handle` bypass; fallback to another route;
ordinary managed `Call` with an owned handle; `KeepAlive` as a substitute;
language Fault or retry on invariant failure
```

Until these ownership and cleanup rules have a named issuer and a focused
caller-zero proof, this child remains:

```text
NoSafeSlice::MissingTextFormalCallResidenceIssuer
```

Only after this child closes does the parent resume its separate
`MissingTextFormalCallableSignatureIssuer` mapping/target/session decision.
