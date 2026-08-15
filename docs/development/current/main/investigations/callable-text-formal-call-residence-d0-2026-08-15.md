---
Status: accepted BoxShape; runtime state-machine D0 is the next child
Date: 2026-08-15
Work mode: design_stop
Classification: accepted T2 BoxShape; no compiler or production caller is admitted
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
Decision: for the formal-only domain, one callee invocation atomically acquires one lease-set containing one pin per ExactText formal occurrence; caller forwarding adds no pin, while each nested callee entry acquires its own set. The 16-byte aggregate and source-residence-only route are not admitted.
Source authority + canonical issuer: selected/batch identity plus complete parameter contracts issue the physical signature; a post-install exact call-edge owner joins target inventory, Installed combined Port, caller origin, and callee signature; runtime SlotTable owns atomic lease-set acquire/finish; DraftSeal private epilogue owns normal-exit insertion.
Non-authority: header/Completion as signature input, `TextFormalBorrowV1`, raw generation recapture, DynamicV2 lease, raw HostHandle, ObjectIdentity, retain_h, KeepAlive, Completion semantic cleanup, scalar lane type, AST/MIR/ValueId, caller-side pin, and fallback.
Fail-fast boundary: preflight all pairs before any pin, BindingRef publication, or body effect; zero/missing/stale/non-Text/retiring/overflow, partial acquire, lane/target/brand drift, ambiguous alias multiplicity, missing implicit-exit policy, or missing/duplicate/foreign finish rejects canonically.
Smallest next slice: `TEXT-FORMAL-CALL-LEASE-RUNTIME-D0` fixes the SlotTable lifetime state machine, atomic lease-set API, pin cardinality, shared retirement terminal, and exact tests; only its accepted I0 may change runtime code.
Non-claims: no signature implementation, call-edge actualizer, physical arity activation, C entry caller, TextEq/Substring route, ValueId adoption, Canonical session emission, Builder, production caller, retry, or main integration.
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

The planned layers are intentionally opaque and non-`Clone`; the source proof
ends at callee-entry acquisition, and the runtime lease owns the call lifetime:

```text
VerifiedTextFormalActualOriginV1 {
    private formal_owner: original live Text/StringBox owner,
    private reaching_original_formal: no-rebind proof through call entry
}

PreparedTextFormalCallActualizationV1 {
    private signature_row: source-backed logical-to-physical mapping,
    private actual_origin: VerifiedTextFormalActualOriginV1,
    private pair_lanes: [slot, generation]
}

TextFormalCallLeaseSetTokenV1 {
    private occurrences: [{slot, generation}],
    private finish: exactly-once set discharge
}

TextFormalEntryLeaseSetSlotIdV1 {
    private protocol_key: session-local only
}
```

The package-owned actual-origin issuer must prove that the original formal
reaches the call site without rebind, then the target terminal emits the pair
lanes while that source owner is still live. It must not pin or recapture a
generation from a detached raw slot. At callee entry,
`acquire_text_formal_call_leases_v1(pairs)` preflights every occurrence under
one SlotTable write lock and creates one move-only call-wide lease-set only
after all validations pass. Repeated equal pairs count once per formal
occurrence: `f(text,text)` adds two pins. A nested caller forwards its scoped
pair view without a forwarding pin; each nested callee entry adds its own pins.
The session sees only the private `TextFormalEntryLeaseSetSlotIdV1`; it never
treats generation as a second BindingRef value. Existing `drop_handle`,
Dynamic lease retirement, and `retain_h` are insufficient; both direct
retirement paths must later converge on one pin-aware helper. C/LLVM receives
only the fixed two-lane projection; source proof and lease-set stay private.

The two lanes are scalar `u64` values with Ownership-SSA `None`; this avoids
smuggling a managed handle through an ordinary MIR `Call`, which the current
verifier rejects. A raw `Vec<ValueId>` call edge still carries no origin proof,
so the mapping/actual-origin cohort and the private lease sidecar remain
mandatory. `KeepAlive` is not a lifetime proof.

This is a BoxShape decision only. Any SlotTable pin count, deferred retirement,
or C/runtime token implementation is a later BoxCount and remains unopened.

## Remaining implementation boundary

The route decision is finite: only source-backed ExactText formals may become
future physical calls, and they all use the mandatory callee-entry lease-set.
The signature issuer is independent of header/Completion because S6C already
moves its Completion seed into the child. The exact call-edge issuer is a
later post-install owner; it must combine whole-source target inventory,
Installed Port, caller actual-origin, and callee signature without consuming
the same selected key through two independent Port loans.

The callable target terminal must consume the actual-origin proof through the
same package-owned physical-signature row that maps one logical `BindingRef`
to `slot` and `generation` lanes. It may not capture a pair from a detached
argument. The Canonical session will later adopt the pair as one composite
receipt, publish only the slot as ordinary BindingRef SSA, and retain the
generation through a scoped `TextFormalLanePairRefV1`. A separate
session-private lease-set ledger is co-closed with Completion. DraftSeal's
private `PreparedTextFormalLeaseEpilogueV1` consumes that non-splittable parent
and uses the same detached explicit-value exit-set iteration to emit one
finish-set immediately before each Return. Return operand evaluation completes
before finish, and DraftSeal remains the sole Return writer. The current
admitted domain is explicit-value exits; implicit/unit exits are typed
unsupported. Semantic Completion cleanup remains empty. Production also
requires a separate `NoUnwindFailStop` seal; trap/unreachable then has no
post-trap finish.

## Required negatives

```text
drop/release/rebind during call; stale generation; non-Text payload; zero slot;
second-pair failure after a valid first pair; same-pair alias multiplicity;
nested pin depth; pin overflow without mutation; duplicate/foreign finish;
lease-set escape; one-lane adoption; raw retirement bypass; implicit/unit exit
admitted accidentally; finish before return operand evaluation; trap that may unwind;
fallback, language Fault, or retry
```

This BoxShape is accepted. Runtime implementation remains stopped at
`NoSafeSlice::MissingTextFormalCallLeaseRuntimeOwner` until the child D0 fixes
the state machine and exact transition table. Compiler mapping/target/session
work remains separately stopped at
`NoSafeSlice::MissingTextFormalCallableSignatureIssuer`.

## Bounded follow-ons

The accepted BoxShape fixes these named seams without implementing them:

```text
package actual-origin issuer
  formal ordinal + BindingRef + call-site reaching-original proof
  + whole-source target + same-brand callee row

runtime lease issuer
  acquire_text_formal_call_leases_v1(pairs)
  -> TextFormalCallLeaseSetTokenV1

physical epilogue owner
  Canonical entry-lease-set ledger + Completion explicit-value exit projection
  -> DraftSeal detached finish+Return iteration; semantic cleanup stays empty
```

The first implementation child after acceptance is caller-zero runtime
`BoxCount`: pin counts, pending retirement, opaque acquire/finish, and one
retirement helper shared by `drop_handle` and
`drop_if_lease_identity_matches`. It must not claim compiler actualization or
production routing. Compiler prologue/epilogue and composite session adoption
remain later I0 rows after the signature cohort and canonical Trap owner close.
