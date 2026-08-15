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

## Six-line brief

```text
Decision: design one source-backed, move-only VerifiedTextCallSourceResidenceV1 that keeps the original Text/StringBox owner live through synchronous call completion; the two-lane signature remains only a preferred target until this owner and target handoff are accepted.
Source authority + canonical issuer: ExactText parameter contracts supply logical BindingRef/ordinal; the canonical caller/session owner supplies the strong source residence, while the host-handle SlotTable supplies only one-lock Text classification and generation capture immediately before the call.
Non-authority: TextFormalBorrowV1 read-lock closure, HostHandleLeaseIdentityV1, DynamicV2 lease, raw HostHandle, ObjectIdentity, retain_h, StringBox/as_str_fast, C validator, AST/MIR/ValueId, and runtime fallback.
Fail-fast boundary: zero/missing/stale/non-Text, pin overflow, foreign generation, owner drop before call end, duplicate finish, or any unpinned body entry rejects/traps before the target body effect; every return/trap path must discharge exactly once.
Smallest next slice: design the SlotTable pin/deferred-drop protocol, opaque residence guard, target-bound call actualizer, and cleanup contract; only after acceptance may a caller-zero wire/map implementation begin.
Non-claims: no signature issuer, physical arity change, C ABI caller, TextEq route, Substring corridor, ValueId adoption, Canonical session, Builder, production caller, fallback, retry, or main integration.
```

## Why the current runtime is insufficient

`TextFormalBorrowV1` captures `{slot,generation}` and lends text under one
read lock, but it does not keep the slot alive after the callback. The generic
`retain_h` path obtains a compatibility object and allocates another raw slot;
it neither preserves the original generation identity nor covers every
`StableText`/`StringBox` payload. DynamicV2 lease identity owns an End/drop
operation, not a callable-wide borrow. Therefore none of these can be the
callable signature's lifetime authority.

## Preferred owner shape

The planned source residence is intentionally opaque and non-`Clone`:

```text
VerifiedTextCallSourceResidenceV1 {
    private source_owner: original live Text/StringBox owner,
    private pair: {slot, generation},
    private finish: exactly-once discharge
}
```

The source-backed caller owner must prove that the original Text input remains
live until call completion; `acquire(raw_slot)` alone is forbidden because it
could capture the current generation of an already-reused replacement. After
that source proof is consumed, the runtime issuer performs Text-class
validation and generation capture in one SlotTable transition immediately
before a synchronous call. The guard cannot escape, be copied, or recreate a
generation from a raw slot. Existing `drop_handle`, Dynamic lease retirement,
and `retain_h` are not sufficient; if a runtime pin/deferred-drop table is ever
needed, it is a separate owner decision and must cover every retirement path.
C/LLVM receives only a later fixed wire projection; the residence itself
remains a compiler/runtime capability.

The callable target terminal must consume this residence through the same
package-owned physical-signature row that maps one logical `BindingRef` to
`slot` and `generation` lanes. It may not capture a pair from a detached
argument, and the callee/session must retain the pair as one composite receipt,
publishing only the slot as ordinary BindingRef SSA.

## Required negatives

```text
drop/release/rebind during call; stale generation; non-Text payload; zero slot;
duplicate finish; finish on the wrong generation; residence
escape from the HRTB/call scope; one-lane adoption; raw retain/release;
fallback to scalar/borrowed route; language Fault or retry on invariant failure
```

Until these ownership and cleanup rules have a named issuer and a focused
caller-zero proof, the parent signature row remains:

```text
NoSafeSlice::MissingTextFormalCallableSignatureIssuer
```
