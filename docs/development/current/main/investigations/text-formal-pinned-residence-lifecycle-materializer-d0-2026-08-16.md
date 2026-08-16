---
Status: design stop; next lifecycle materializer boundary
Date: 2026-08-16
Work mode: design_stop
Parent: TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-BRIDGE-D0 / I0
---

# TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-MATERIALIZER-D0

This card follows the completed proof-only exit-obligation I0. It decides the
future entry/normal-exit materialization seam without yet executing a runtime
finish, adding a lifecycle MIR instruction, or changing DraftSeal.

## Six-line brief

```text
Decision: keep one function-owned lifecycle materializer over the accepted frame/plan contract, the move-only TextFormalCallResidenceV1, and the private PinnedTextResidenceExitObligationV1; choose one checked normal/trap entry and one site-keyed normal-exit finish projection before implementation.
Source authority + canonical issuer: PinnedTextBackendFrameContractV1 owns compile-time plan/frame stamps, TextFormalCallResidenceV1 owns pair validation/pin/root residence, VerifiedFunctionCompletionV1 plus PreparedFunctionExitSetV1 own normal exits, and the materializer only co-seals their same-owner relation.
Non-authority: raw slot/generation recapture, ptr/len or runtime token in common MIR/JSON, block scans, source ordinals, semantic cleanup, CheckedCallOut fault policy, PinnedTextOp inference, a second Completion/Return writer, fallback, and retry.
Fail-fast boundary: owner/plan/frame drift, entry after a Text read, partial acquire, root escape, implicit/unit exit outside the typed domain, missing/duplicate/foreign exit claim, finish before operand evaluation, finish after Return, and any catch/unwind-capable trap reject before publication.
Smallest next slice: design the exact private materializer and DraftSeal handoff for explicit-value Single/ExactTwo only, reusing Stage0 NoUnwindFailStop and the existing exit-set iteration; do not implement runtime/C/MIR/backend effects yet.
Non-claims: no GEP/load, PinnedTextOp lowering, Canonical session adoption, TextEq route admission, literal/StringBox origin, production caller, performance result, external fallback, or main integration.
```

## Required decision

The next design must decide whether the materializer owns only a compile-time
finish capability or also consumes the move-only runtime residence. It must
not duplicate either authority. The accepted proof obligation remains
site-keyed and non-`Clone`; a later implementation may consume it only after
the same `PreparedFunctionExitSetV1` has established:

```text
normal entry -> every pinned read
return operand evaluation -> one finish -> DraftSeal Return
trap/unreachable -> no post-trap finish under Stage0 NoUnwindFailStop
```

If this ordering cannot be represented without a second Return writer, a
second exit ledger, or a runtime token in common MIR, the row remains
`NoSafeSlice::PinnedTextLifecycleMaterializerUnsealed`.

## Acceptance / non-claims

```text
positive:
  one explicit-value exit, exact-two explicit-value exits, repeated pair alias

negative:
  implicit/unit, missing/duplicate/foreign site, owner/stamp drift,
  partial acquire, root/pointer escape, catch/unwind, finish-before-operand,
  second Return writer, runtime fallback/retry

not opened:
  runtime lease code, DraftSeal Return changes, lifecycle MIR, PinnedTextOp
  lowering, GEP/load, session residence, route admission, production caller
```
