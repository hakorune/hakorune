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
Decision: choose exactly one lifetime route for every admitted ExactText call—source-backed residence spanning synchronous completion, or a mandatory opaque runtime lease—and keep caller-private prepared actualization as the sole composer; the two-lane signature remains only a preferred target.
Source authority + canonical issuer: ExactText parameter contracts supply logical BindingRef/ordinal; the route decision must name its source-backed issuer, and only the selected route may co-seal prepared actualization and runtime capture.
Non-authority: TextFormalBorrowV1 read-lock closure, HostHandleLeaseIdentityV1, DynamicV2 lease, raw HostHandle, ObjectIdentity, retain_h, KeepAlive, Completion, C validator, AST/MIR/ValueId, and runtime fallback.
Fail-fast boundary: route/source-owner loss or generation capture failure rejects before body effect; partial acquire rolls back; normal continuation finishes exactly once, while the current no-unwind trap ABI never requires a post-trap cleanup callback.
Smallest next slice: classify all admitted Text actual origins, choose one mandatory route, then design its ownership handoff, source/call target co-seal, and retirement/rollback paths; only after acceptance may a caller-zero wire/map implementation begin.
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

## Candidate owner shape (not yet selected)

The planned source receipt is intentionally opaque and non-`Clone`:

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

TextFormalCallLeaseTokenV1 { // only if source proof ends before call return
    private pair: {slot, generation},
    private finish: exactly-once discharge
}
```

The source-backed caller owner must prove that the original Text input remains
live until call completion. A raw-slot acquire is forbidden because it could
capture the current generation of an already-reused replacement. The prepared
actualizer consumes the source proof and target signature together, then either
performs atomic Text validation/generation capture immediately before the
synchronous call, or consumes the separate runtime lease token if the source
proof cannot span the call. Existing `drop_handle`, Dynamic lease retirement,
and `retain_h` are not sufficient. If the runtime lease variant is selected,
both direct retirement paths must converge on one pin-aware helper. C/LLVM
receives only a later fixed wire projection; the source receipt and token stay
caller-private.

The actualizer cannot smuggle an owned handle through an ordinary MIR
`Call`: the current Ownership SSA verifier rejects managed call operands and
results. The accepted design must therefore name either a borrow-only
capture/terminal or a dedicated ownership-aware call capability; a raw
`Vec<ValueId>` call edge or a `KeepAlive` no-op is not a lifetime proof.

This is a BoxShape decision only. Any SlotTable pin count, deferred retirement,
or C/runtime token implementation is a later BoxCount and remains unopened.

## Open route decision

The current repository has no issuer for
`VerifiedCallScopedTextOwnerLifetimeV1` and no accepted predicate deciding
when a runtime lease is required. The two candidates above are therefore not
an optional runtime branch: D0 must choose one finite route for all admitted
Text calls before any BoxCount. Creating only a runtime pin or only a compiler
receipt would otherwise introduce an unused or partial authority.

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
