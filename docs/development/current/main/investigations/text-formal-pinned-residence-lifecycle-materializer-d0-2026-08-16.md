---
Status: design stop; exit-ledger correction and trap-capability boundary
Date: 2026-08-16
Work mode: design_stop
Parent: TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-BRIDGE-D0 / I0
---

# TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-MATERIALIZER-D0

This card follows the proof-only exit-obligation prototype. That prototype is
historical evidence only: its copied per-exit rows are not a second exit
authority and must not be carried into the lifecycle materializer. This card
decides the corrected entry/normal-exit seam without yet executing a runtime
finish, adding a lifecycle MIR instruction, or changing DraftSeal.

## Six-line brief

```text
Decision: replace the prototype's copied exit rows with one private non-Clone PreparedTextFormalExitFinishSetV1 carrying only the same function/plan/frame/residence/session stamp; DraftSeal later consumes it beside the existing PreparedFunctionExitSetV1 in one exit iteration.
Source authority + canonical issuer: PinnedTextBackendFrameContractV1 owns compile-time plan/frame stamps, TextFormalCallResidenceV1 owns pair validation/pin/root residence, VerifiedFunctionCompletionV1 plus PreparedFunctionExitSetV1 own normal exits, and the materializer only co-seals their same-owner relation. The accepted C-prime language policy owns Fault/no-catch semantics; an exact backend no-unwind capability must be issued and verified before this route is admitted.
Non-authority: copied site/block/ValueId rows, raw slot/generation recapture, ptr/len or runtime token in common MIR/JSON, block scans, source ordinals, semantic cleanup, CheckedCallOut, superseded Stage0 prose, PinnedTextOp inference, a second Completion/Return writer, fallback, and retry.
Fail-fast boundary: missing no-unwind capability, owner/plan/frame/residence drift, entry after a Text read, partial acquire, root escape, implicit/unit exit outside the typed domain, missing/duplicate/foreign coverage in the canonical exit set, finish before operand evaluation, finish after Return, and any catch/unwind-capable trap reject before publication.
Smallest next slice: design the exact stamp-only finish capability and its DraftSeal handoff for explicit-value Single/ExactTwo only; reuse the existing exit-set iteration without copying rows or counts, and do not implement runtime/C/MIR/backend effects yet.
Non-claims: no GEP/load, PinnedTextOp lowering, Canonical session adoption, TextEq route admission, literal/StringBox origin, production caller, performance result, external fallback, or main integration.
```

## Required decision

The next design must decide the private handoff between a compile-time
finish capability and the move-only runtime residence. It must not duplicate
either authority or copy the canonical exit ledger. The corrected capability
contains only a same-function/session stamp; the existing
`PreparedFunctionExitSetV1` remains the sole site/block/value ledger and
establishes:

```text
normal entry -> every pinned read
return operand evaluation -> one finish -> DraftSeal Return
trap/unreachable -> no post-trap finish under a verified backend no-unwind capability
```

If this ordering cannot be represented without a second Return writer, a
second exit ledger, a runtime token in common MIR, or an active no-unwind
issuer, the row remains
`NoSafeSlice::PinnedTextLifecycleMaterializerUnsealed`.

## Current correction

`PinnedTextResidenceExitObligationV1` from the predecessor I0 is retained as a
prototype/test receipt only. Its site-keyed rows and expected count are not a
canonical lifecycle product. The successor must issue a private
`PreparedTextFormalExitFinishSetV1` with no copied exit rows, blocks, values,
source order, or independent count. DraftSeal must join that capability with
the existing `PreparedFunctionExitSetV1` during the same detached exit
iteration; it may not rediscover exits from MIR/JSON or infer finish needs.

## Trap-capability decision boundary

The accepted C-prime language policy owns terminal `Fault` and rejects a
catchable source `try/throw/catch` route. That semantic decision is not itself
a backend execution guarantee. The missing backend product is a future,
function-owned non-`Clone` `PinnedTextBackendNoUnwindCapabilityV1` that
co-seals:

```text
compile-invocation brand
exact function stamp
residence/frame plan stamp
target realization revision
exact entry/fault call census
nounwind + trap-noreturn verification
```

`PinnedTextBackendFrameContractV1` alone is insufficient because it owns
layout/target facts, not call/fault execution policy. LLVM attribute strings,
`llvm.trap; unreachable` emitted by a consumer, `EffectMask`, and the
superseded Stage0 inventory are non-authority. If the exact census cannot be
carried to the final contract-bound verifier, the materializer remains closed.

The corrected finish capability is therefore conceptually:

```text
PreparedTextFormalExitFinishSetV1 {
  function/session/residence/plan/frame stamp
  no_unwind capability stamp
}
```

It is passed by value beside the existing `PreparedFunctionExitSetV1` into the
future `PreparedFunctionExitPlanV1` handoff. It contains no exit rows, blocks,
values, source order, or independent count.

## Exact next design task

Before any caller-zero materializer implementation, close these four points in
this D0:

```text
1. name the sole issuer of PinnedTextBackendNoUnwindCapabilityV1
2. define the exact pre-DraftSeal call/fault census and final verifier
3. define the stamp-only PreparedTextFormalExitFinishSetV1 handoff beside
   PreparedFunctionExitSetV1
4. prove that missing/foreign/no-unwind drift rejects before publication
```

This is still a design-only BoxShape. No new receipt is implemented until all
four points have one owner and one fail-fast boundary.

## Acceptance / non-claims

```text
positive:
  one explicit-value exit, exact-two explicit-value exits, repeated pair alias

negative:
  implicit/unit, missing/duplicate/foreign canonical exit coverage, owner/stamp drift,
  partial acquire, root/pointer escape, catch/unwind, finish-before-operand,
  missing no-unwind capability, second Return writer, runtime fallback/retry

not opened:
  runtime lease code, DraftSeal Return changes, lifecycle MIR, PinnedTextOp
  lowering, GEP/load, session residence, route admission, production caller
```
