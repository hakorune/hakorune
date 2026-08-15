---
Status: accepted BoxShape; runtime substrate closed; compiler residence is a later family
Date: 2026-08-16
Work mode: design_stop
Classification: accepted T2 BoxShape; no compiler or production caller is admitted
---

# CALLABLE-TEXT-FORMAL-CALL-RESIDENCE-D0

This child owns the lifetime split after the accepted two-lane
`ExactText -> slot,generation` callable boundary. The caller-zero runtime
lease-set substrate is closed. Compiler root residence, slices/cursors, and
physical exit coverage remain unopened.

The accepted child `text-call-actual-origin-route-d0-2026-08-15.md` fixes the
finite formal-versus-reject origin partition. Only an original ExactText
formal with no rebind is a future candidate; all other current origins reject.

## Six-line brief

```text
Decision: for the formal-only domain, one callee invocation atomically acquires one lease-set containing one pin per ExactText formal occurrence; one non-splittable residence set couples that token to immutable UTF-8 root descriptors, while slices/plans and backend ptr/len projections remain subordinate scoped views.
Source authority + canonical issuer: selected/batch identity plus complete parameter contracts issue the physical signature; a post-install exact call-edge owner joins target inventory, Installed combined Port, caller origin, and callee signature; runtime SlotTable owns atomic lease-set acquire/finish; DraftSeal private epilogue owns normal-exit insertion.
Non-authority: header/Completion as signature input, `TextFormalBorrowV1`, raw generation recapture, DynamicV2 lease, raw HostHandle, StringSpan/StringViewBox, raw ptr/len, ObjectIdentity, retain_h, KeepAlive, Completion semantic cleanup, scalar lane type, AST/MIR/ValueId, caller-side pin, and fallback.
Fail-fast boundary: preflight all pairs before any pin, root projection, BindingRef publication, or body effect; zero/missing/stale/non-Text/retiring/overflow, partial acquire, unstable UTF-8 backing, detached root/token, escaped slice, lane/target/brand drift, missing exit coverage, or duplicate/foreign finish rejects canonically.
Smallest next slice: the active physical-signature I0 closes only the package lane map; the later TEXT-FORMAL-PINNED-RESIDENCE family must close exact call edge, pair-based root projection, Canonical adoption, and DraftSeal finish coverage together.
Non-claims: no call-edge actualizer, C entry caller, root/slice ValueId, TextEq/Substring route, Canonical session emission, Builder, production caller, retry, or main integration.
```

## Runtime substrate and remaining boundary

The landed runtime lease-set atomically validates all pairs, pins every formal
occurrence, defers retirement, and finishes through one move-only token. It is
the invocation lifetime substrate, but it does not yet issue compiler-visible
root descriptors or prove stable UTF-8 backing. `TextFormalBorrowV1` remains a
probe/test carrier; production entry consumes the already-published pair lanes
directly and never recaptures generation from a raw slot.

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

TextFormalCallResidenceSetV1 {
    private lease_set: TextFormalCallLeaseSetTokenV1,
    private roots: [PinnedTextRootResidenceV1],
    private non_splittable: true
}

TextSliceRefV1<'session> {
    private root: PinnedTextRootIdV1,
    private byte_range: [start, length],
    private utf8_boundary: VerifiedUtf8SliceBoundaryV1,
    private session_brand: 'session
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
The session sees a private residence-set protocol key; it never treats
generation as a second BindingRef value. The root descriptor is valid only
while its enclosing lease set is live. It is not a raw pointer pair and cannot
be stored, returned, or passed to an unadmitted foreign call.

`TextSliceRefV1` is a transient, session-branded range over one root. Existing
`TextPlan = View1 | PiecesN | OwnedTmp` may consume such slices as the
backend-local non-Box carrier, but it never becomes the root lifetime owner.
Raw pointer and byte length values are projected only through a scoped backend
consumer. `StringSpan`, `StringViewBox`, and current helper range behavior are
runtime evidence/adapters, not the residence or CP-boundary issuer.

The later fast Loop leaf keeps both code-point index and UTF-8 byte offset in
one generic `SequentialCodePointCursorV1`. A selected source proof must fix
initial zero, `i < length(subject)`, bounds `[i,i+1)`, update `i+1`, and no
subject/needle rebind. Only then may the physical loop advance to the next
UTF-8 boundary and compare one boundary-aligned subject slice with the needle
bytes. Exact scalar-sequence equality plus valid UTF-8 makes byte equality a
conforming leaf; it does not make byte offsets the language index authority.
The admitted hot loop must have zero registry lookup, runtime TextEq/Substring
call, handle/Box birth, publication, retain/release, fallback, or retry per
iteration.

The two lanes are scalar `u64` values with Ownership-SSA `None`; this avoids
smuggling a managed handle through an ordinary MIR `Call`, which the current
verifier rejects. A raw `Vec<ValueId>` call edge still carries no origin proof,
so the mapping/actual-origin cohort and the private lease sidecar remain
mandatory. `KeepAlive` is not a lifetime proof.

The runtime BoxCount is closed caller-zero. This Decision still does not
authorize compiler residence, MIR values, or a production call edge.

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
generation through a scoped composite view. Entry acquisition will project
the root descriptor under the same validation/pin transaction when the stable
backing proof exists. A separate session-private residence-set ledger is
co-closed with Completion. DraftSeal's
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
lease/root split; unstable backing; raw ptr/len escape; slice without UTF-8/CP
boundary; one-lane adoption; raw retirement bypass; implicit/unit exit
admitted accidentally; finish before return operand evaluation; trap that may unwind;
fallback, language Fault, or retry
```

This BoxShape and its runtime substrate are closed. Compiler mapping remains
on the active physical-signature row; root residence remains stopped at
`NoSafeSlice::MissingPinnedUtf8BackingProof` until one pair-based entry issuer
can couple stable backing descriptors to the runtime lease token.

Named fast-route stops are:

```text
NoSafeSlice::PinnedRootDetachedFromLeaseSet
NoSafeSlice::PinnedRootEscapesSession
NoSafeSlice::MissingCpBoundaryProof
NoSafeSlice::ExistingStringViewSemanticDrift
NoSafeSlice::HotLoopRegistryReentry
NoSafeSlice::HotLoopPublicationRequired
NoSafeSlice::MissingNormalExitFinishCoverage
NoSafeSlice::TrapMayUnwind
NoSafeSlice::RawPointerCrossesCallableBoundary
NoSafeSlice::RuntimeFallbackRequired
```

## Bounded follow-ons

The accepted BoxShape fixes these named seams without implementing them:

```text
package actual-origin issuer
  formal ordinal + BindingRef + call-site reaching-original proof
  + whole-source target + same-brand callee row

entry residence issuer
  already-published pair lanes
  -> acquire lease-set + project immutable UTF-8 roots in one transaction
  -> non-splittable TextFormalCallResidenceSetV1

physical epilogue owner
  Canonical entry-residence-set ledger + Completion explicit-value exit projection
  -> DraftSeal detached finish+Return iteration; semantic cleanup stays empty
```

The remaining compiler work is grouped under
`TEXT-FORMAL-PINNED-RESIDENCE-D0/I0`; exact call edge, root projection,
Canonical adoption, and epilogue are ordered seams inside that family rather
than four independent authority cards. The later
`LOOP-TEXT-SLICE-EXECUTION-D0/I0` owns CP-correct slices, the generic
sequential cursor, and inline byte equality. Neither family opens a route
without tracked perf admission and a no-fallback/no-retry seal.
