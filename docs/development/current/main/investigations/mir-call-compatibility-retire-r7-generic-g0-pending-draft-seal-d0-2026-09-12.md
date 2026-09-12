---
Status: closed__DecisionAccepted__R7GenericG0PendingDraftSeal__2026-09-12
Task: MIR-CALL-COMPATIBILITY-RETIRE-R7-GENERIC-G0-PENDING-DRAFT-SEAL-D0
Date: 2026-09-12
Parent: mir-call-compatibility-retire-r7-pending-draft-seal-i0-2026-09-12.md
---

# R7 Generic G0 pending DraftSeal boundary

## Six-line brief

```text
Decision: preserve typed Generic G0 admission and DraftSeal rejection through the existing pending session boundary before another implementation.
Source authority + canonical issuer: GenericG0PhysicalEmitterAdmissionRejectV1 at prephysical admission, and FunctionDraftSealErrorV1 at the existing DraftSeal owner.
Non-authority: Debug strings, Primary(String), stage text, module names, and the pending collector do not issue or classify semantic failure.
Fail-fast boundary: admission rejects before session/effect; lowerer rejects discard the unpublished session; DraftSeal rejects consume the live owner through restoration-preserving discard.
Smallest next slice: replace only the Generic G0 pending admission and DraftSeal Debug bridges with the existing typed session transport, then prove parent restoration and fresh-session reuse.
Non-claims: no Generic G0 source widening, new Recipe/receipt, non-pending lowerer change, DirectAccum/Nested change, fallback, retry, ambient-helper deletion, or whole-R7/MIRBuilder completion.
```

Census boundary: `MirBuilder::lower_resolved_generic_g0_function_pending_v1`
and its existing `normal_top_level_function_admission.rs` caller -> the
existing `commit_resolved_pending` collector terminal. Include the admission
reject at `resolved_lowering/mod.rs`, the pending Generic lowerer failure and
DraftSeal conversion in `generic_lowerer.rs`, and the two coarse abort
classifiers. Exclude the already-typed Callable pending path, non-pending G0
lowerer, DirectAccum/Nested cutovers, and public compatibility profiles.

## Finite outcome matrix

| outcome | authority | pre-effect behavior | terminal | decision |
| --- | --- | --- | --- | --- |
| Generic admission rejected | `GenericG0PhysicalEmitterAdmissionRejectV1` | before child session/effect | typed `CanonicalFunctionSessionErrorV1` transport | include |
| G0 body lowering rejected | existing lowerer `String` boundary | discard open unpublished session | existing `Primary(String)` | retain; no single typed owner is introduced here |
| DraftSeal rejected | `FunctionDraftSealErrorV1` plus rejected owner | consume and restore before returning | existing `DraftSeal(DiscardedFunctionDraftSealErrorV1)` | include |
| pending draft ready | existing DraftSeal commit/pending owner | no publication by this helper | `PendingFunctionSessionCloseV1` | retain |
| collector/session cleanup or publication failure | existing session/collector owners | existing cleanup/discard order | existing typed session terminal | unchanged |

The matrix deliberately keeps the existing lowerer's broad `String` body
boundary separate. Turning every internal physicalizer error into a new typed
family would require another owner decision; it is not a reason to leave the
two known Debug bridges in the selected admission/DraftSeal path.

## Accepted Decision

The Callable pending slice established a reusable transport-only
`DiscardedFunctionDraftSealErrorV1` and
`CanonicalFunctionSessionErrorV1::DraftSeal`. Reuse those owners for Generic
G0. Add one `GenericG0Admission(...)` session variant carrying the existing
`GenericG0PhysicalEmitterAdmissionRejectV1`; it must not reclassify or format
the reject. The pending Generic lowerer must call
`RejectedFunctionDraftSealV1::into_discarded_error()` before returning, so the
live function/session owner is restored exactly once and cannot be retried.

The existing `Primary(String)` path remains only for the lowerer's already
string-valued body errors in this slice. It does not authorize a fallback or a
second G0 route. `normal_top_level_function_admission.rs` continues to pass
the resulting session error to the existing collector; its caller identity and
publication boundary are unchanged.

## Worker consultation receipt

Wegener performed an independent read-only audit of HEAD `966d2d0a4c` on
2026-09-12. It confirmed that Boundary `llc_flags` and the DirectAccum/Nested
typed bridges are already landed, that the Generic G0 pending admission and
DraftSeal bridges remain, and that
`backend_codegen_request_defaults` is repo-caller-zero but still externally
reachable through the public config re-export. The worker ran no Cargo,
fixture, test, or code mutation and was closed after integration.

## Implementation handoff

The next card is
`MIR-CALL-COMPATIBILITY-RETIRE-R7-GENERIC-G0-PENDING-DRAFT-SEAL-I0`.
Required evidence is one positive pending completion, one admission negative,
one DraftSeal negative retaining exact stage/inner error and restoring the
parent, a fresh-session replay, the existing coarse abort tests, a focused
guard proving both Debug bridges are absent, and the source-size/pointer/diff
checks. Cargo remains single-process (`-j1`) if the owner gate requires it.
