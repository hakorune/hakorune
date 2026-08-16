---
Status: accepted BoxShape; proof-only exit-obligation I0 is the next row
Date: 2026-08-16
Work mode: design_stop
Parent: CALLABLE-TEXT-FORMAL-CALL-RESIDENCE-D0 / LOOP-TEXT-SLICE-DIRECT-AOT-D0
---

# TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-BRIDGE-D0

This card is the next design boundary after the scoped
`PinnedTextBackendFrameBorrowV1` child. It joins the already-landed runtime
residence substrate to the existing function Completion/DraftSeal exit owner;
it does not lower a `PinnedTextOp` or materialize a pointer.

The no-unwind condition reuses the existing Stage0 cleanup/catch boundary:
backend exception ABI and stack unwinding are closed there. This card does not
mint a second trap policy; admitted functions must satisfy that existing policy
and reject source catch/unwind shapes before any residence effect.

## Six-line brief

```text
Decision: co-seal one function-owned lifecycle bridge for entry residence, normal-exit finish, and the no-unwind trap relation; keep compile-time frame facts, runtime lease ownership, and Return writing as sibling projections of existing owners.
Source authority + canonical issuer: PinnedTextBackendFrameContractV1/borrow owns compile-time plan and target facts, TextFormalCallResidenceV1 owns atomic lease/frame entry and finish, VerifiedFunctionCompletionV1 owns the exact normal-exit set, and a bridge issuer only joins same-function stamps without reissuing meaning.
Non-authority: JSON/root numbers, raw ptr/len, slot or generation recapture, ValueId shape, StringSpan/ViewBox, semantic cleanup, Completion copies, backend CFG scans, PinnedTextOp kind inference, runtime fallback, and a second Return writer.
Fail-fast boundary: reject missing/foreign plan or frame, entry after first Text read, partial acquire/rollback, detached frame/lease, root escape, duplicate/missing/foreign normal-exit finish, finish before return-operand evaluation, implicit/unit exits outside the admitted domain, and any trap that may unwind.
Smallest next slice: design-only BoxShape for one explicit-value function: entry dominates every future Text leaf, one residence ledger owns the invocation, Completion's exact exit set drives one finish immediately before each DraftSeal Return, and trap/unreachable has no post-trap finish under NoUnwindFailStop.
Non-claims: no GEP/load, UTF-8 execution, PinnedTextOp lowering, Canonical session adoption, route admission, production caller, literal/StringBox origin, performance result, fallback/retry, or main integration.
```

## Existing owners and the bridge boundary

The bridge must not create a new lifetime authority. The existing owners remain:

```text
PinnedTextBackendFrameContractV1 / Borrow
  compile-time co-sealed plan, census, Residence ABI, target facts

TextFormalCallResidenceV1
  runtime pair validation, pin/rollback, root frame, move-only finish

VerifiedFunctionCompletionV1
  explicit-value normal exit set and implicit-body-end policy

CanonicalSsaFunctionSessionV2
  later CFG/SSA/PHI owner; not opened by this D0

DraftSeal
  sole physical Return writer; not a semantic or runtime owner
```

The candidate bridge is a private, non-`Clone` relation over one function
stamp. It may expose only scoped projections for entry and finish materializers;
it must not be stored in a module port, JSON, runtime registry, or backend-global
table. The compile-time borrow remains non-pointer metadata. The runtime frame
and lease token remain opaque and move-only.

## Required lifecycle relation

The accepted design must prove this ordering for every admitted function:

```text
co-sealed plan/census/frame contract
  -> residence entry validates every pair and publishes every root row
  -> first PinnedTextOp is reachable only from the normal entry landing
  -> return operand is fully evaluated
  -> Completion exact normal-exit claim is closed
  -> residence finish consumes the invocation owner exactly once
  -> DraftSeal writes the physical Return
```

Entry failure has no published root or partial pin. A normal exit with a
missing, duplicate, or foreign finish is rejected before object publication.
The current admitted domain is explicit value-return exits only. Implicit
body-end and unit exits remain typed unsupported until their Completion claim
has a stable exit identity. A noreturn trap is accepted only when a
`NoUnwindFailStop` capability is present; it does not require cleanup after the
trap edge and must never fall through to Return.

The missing compile-time witness is deliberately an exit obligation, not a
runtime token carrier. The candidate shape is a private, non-`Clone`
`PinnedTextResidenceExitObligationV1` containing only the function/plan/frame
stamp, the expected explicit-value exit count, and the site-keyed exit claims.
It is issued from the existing validated Completion/DraftSeal exit projection;
it cannot be constructed from a block number, return ordinal, JSON metadata, or
the runtime lease token. A later lifecycle implementation consumes this
obligation together with the move-only `TextFormalCallResidenceV1`, while the
obligation itself never owns or serializes the token.

The D0 must also name the exact materialization seam: the same
`PreparedFunctionExitSetV1` iteration that validates each detached exit must
order `return operand -> residence finish -> Return`. If the current DraftSeal
API cannot accept a private finish capability at that point without becoming a
second Return writer, the D0 remains open and no I0 is authorized.

## D0 acceptance

The BoxShape is accepted for the explicit-value `Single` and `ExactTwo`
exit-set vocabulary. `VerifiedFunctionCompletionV1` and
`PreparedFunctionExitSetV1` remain the sole site-keyed exit authorities;
Stage0's no-unwind stop is the sole trap policy. The next I0 may issue only a
proof-only exit obligation from those existing products. It may not store a
runtime token, change DraftSeal Return placement, or add a lifecycle MIR op.

## Design acceptance matrix

```text
positive:
  one ExactText root, two ExactText roots, repeated pair aliases,
  explicit value Return A/B, reverse source claim order mapped by site

negative:
  missing/foreign plan or frame, entry after first Text use,
  stale/non-Text pair, partial acquire, rollback failure,
  detached root/lease, root or pointer escape, missing/duplicate finish,
  foreign exit block, finish before operand, finish after Return,
  implicit/unit exit, trap with unwind/catch, duplicate Return writer

invariants:
  one function stamp, one residence ledger, all normal exits covered exactly
  once, semantic cleanup unchanged, no raw runtime field in compile-time borrow,
  no fallback/retry, no publication after any failed projection
```

## Ordering with direct AOT

The typed direct-AOT binder may validate the compile-time borrow projection,
but it cannot claim a live root until this lifecycle boundary is accepted. The
next implementation row, if this D0 closes, is a caller-zero lifecycle bridge
that reuses the existing runtime frame contract and adds no new wire. Only
after that row may `TEXT-FORMAL-PINNED-RESIDENCE-DIRECT-AOT-I0` lower the three
already-defined leaves. That later row still must leave CFG/SSA/PHI and Return
placement to their existing owners.

## NoSafeSlice conditions

```text
NoSafeSlice::PinnedTextLifecycleBridgeUnsealed
NoSafeSlice::ResidenceFrameNotFunctionOwned
NoSafeSlice::PinnedRootDetachedFromLeaseSet
NoSafeSlice::MissingNormalExitFinishCoverage
NoSafeSlice::FinishBeforeReturnOperand
NoSafeSlice::ImplicitOrUnitExitUnclassified
NoSafeSlice::TrapMayUnwind
NoSafeSlice::SecondReturnWriterRequired
NoSafeSlice::SemanticCleanupMutationRequired
NoSafeSlice::RawPointerCrossesCallableBoundary
NoSafeSlice::FallbackOrRetryRequired
```

This is a design-only task. Its acceptance moves the pointer to one bounded
caller-zero implementation row; it does not claim a C-speed kernel or a
production TextEq route.
