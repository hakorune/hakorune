---
Status: accepted target authority; activation parked behind common V2 canonical session ingress
Date: 2026-08-16
Work mode: design_stop
Parent: TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-MATERIALIZER-D0
---

# TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-AUTHORITY-R0

This row accepts the two-stage A-prime lifecycle authority and corrects its
task placement against the production pipeline. A source-side admission must
precede materialization; a final backend closure must follow all call/EH-
changing transforms. Current selected-normal production does not yet enter the
canonical session/DraftSeal owner that can issue the required exit stamp, so
the lifecycle implementation stays parked rather than extending the legacy
finalizer.

## Six-line brief

```text
Decision: accept A-prime as the canonical lifecycle target, but do not activate it on the current selected-normal legacy finalizer; resume the common V2 pre-session/session path that reaches the sole canonical SSA and DraftSeal owners.
Source authority + canonical issuer: PreparedFunctionExitSetV1 remains the sole exit ledger; a future PinnedTextLifecycleAuthorityIssuerV1 inside the common V2 canonical precommit owner issues the opaque exit stamp, stamp-only finish obligation, expected census, and materialization admission; the final contract-bound verifier alone issues the post-transform no-unwind closure.
Non-authority: legacy Return scans, copied exit rows/counts, post-seal MirFunction metadata, runtime-declaration facts alone, CheckedCallOut alone, C emitter alone, semantic Fault policy alone, JSON/MIR/name/ordinal inference, raw block/ValueId/pointer/token data, target defaults, fallback, and retry.
Fail-fast boundary: absence of a common V2 canonical DraftSeal ingress, foreign stamps/brands/target, expected/observed lifecycle drift, catch/EH/no-unwind drift, or missing final closure rejects before publication; no legacy finalizer patch or backend Return rescan may repair the gap.
Smallest next slice: resume LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0, now that installed child, single Completion ownership, physical Text signature mapping, residence substrate, and backend-frame transport prerequisites are closed; do not mix lifecycle materialization into that BoxShape.
Non-claims: no lifecycle receipt implementation, legacy finish rewrite, CFG/runtime call emission, backend observation/closure, C verifier, object-emitter change, DraftSeal finish, GEP/load, PinnedTextOp lowering, route/perf admission, production caller, literal/StringBox origin, fallback/retry, or main integration.
```

## Accepted final authority graph

```text
C-prime terminal Fault / no-source-catch policy
typed runtime declaration-effect witnesses
sealed entry-call demand
frame / plan / residence provenance
PreparedFunctionExitSetV1
  sole exit ledger + opaque preparation stamp
              |
              v
PinnedTextLifecycleAuthorityIssuerV1
  +-- PreparedTextFormalExitFinishSetV1
  |     stamp-only; no exit data or count
  +-- PreparedPinnedTextLifecycleExpectedCallFaultCensusV1
  |     role/policy obligations only
  `-- AdmittedPinnedTextLifecycleMaterializationV1
              |
              v
future lifecycle materializer
  transfers admission provenance into the unpublished candidate
              |
              v
final C/LLVM module after all call/EH-changing transforms
              |
              v
ObservedPinnedTextBackendCallFaultCensusV1
  backend observation only
              |
              v
PinnedTextBackendNoUnwindCapabilityIssuerV1
              |
              v
VerifiedPinnedTextBackendNoUnwindClosureV1
              |
              v
same contract-bound object candidate / sole emitter
```

The graph has no backward edge. The source issuer decides obligations but does
not predict LLVM realization. The backend observer enumerates actual physical
facts but does not decide source obligations. The final issuer proves exact
parity only.

## Current-HEAD production census

The required owner graph is not present on the current selected-normal route:

```text
selected static / instance lowering
  -> finalize_function_draft_with_headers
  -> legacy finalize_function_draft_with_lookup
       optional direct Return insertion
       no PreparedFunctionExitSetV1
       no OpenFunctionDraftSealV1
  -> MirFunction
  -> complete_resolved_child_with_physical_loan
       issue backend-frame contract
       collect
```

`PreparedFunctionExitSetV1` exists only on the canonical DraftSeal paths. The
frame contract currently appears after the legacy finalizer has completed.
Moving frame issue earlier on that legacy path would not create the canonical
exit set, and transporting an opaque stamp through cloneable metadata could
not place `finish` before an already-written `Return`. A later MIR/JSON Return
scan would become a second exit authority.

Therefore the following are rejected:

```text
retrofit lifecycle meaning into finalize_function_draft_with_lookup
store a non-Clone lifecycle capability in cloneable FunctionMetadata
copy site/block/ValueId rows across the legacy boundary
infer finish placement from Return instructions in Rust, JSON, C, or LLVM
issue the frame/lifecycle contract twice to simulate validation
```

The correct production ingress is the already-planned common V2 fan-in into
`CanonicalSsaFunctionSessionV2`, Completion consumption, and DraftSeal. This
keeps one CFG/SSA/PHI/Return owner and avoids a fourth finish schedule.

## Opaque exit-set identity

When the common V2 physical session is live, the canonical exit owner may mint
one opaque identity per prepared exit set:

```text
FunctionExitSetStampV1
  private
  non-Clone
  non-Copy
  constructor private to the canonical DraftSeal/exit-plan owner
  unique per PreparedFunctionExitSetV1 preparation
```

It is not derived from owner plus shape, because a retry or second preparation
for the same owner and shape must not compare equal. A scoped view may be lent
before `PreparedFunctionExitPlanV1::into_parts()`, but it cannot be returned,
stored, cloned, or used to enumerate exits.

The future finish capability contains only same-function/session/plan/frame/
residence provenance plus an opaque child proof issued from that exit seal. It
does not contain:

```text
exit count or order
Single / ExactTwo as a second classifier
source site
BasicBlockId
ValueId
return operand
finish instruction
runtime token
```

All cardinality and traversal stay behind
`PreparedFunctionExitSetV1::try_for_each_exit`.

## Expected and observed lifecycle facts

The future source-side expected product is an obligation, not a copied
callsite table:

```text
EntryAcquireExactlyOnceV1
EntrySuccessEdgeExactlyOnceV1
TerminalFaultExactlyOnceV1
FinishEveryCanonicalNormalExitV1
FaultFinishExactlyZeroV1
SourceCatchEdgeExactlyZeroV1
AllResidenceCallsNoUnwindV1 {
  permitted = ReturnableNoUnwind | TerminalNoReturnNoUnwind
}
exact runtime-call contract IDs
same function / plan / frame / target provenance
```

`FinishEveryCanonicalNormalExitV1` refers to the opaque canonical exit stamp;
it stores no exit count. Runtime declaration facts provide exact symbol/effect
witnesses, not function-wide coverage. C-prime Fault/no-catch constrains source
control shape but is not backend `nounwind`/`noreturn` proof.

The final verifier runs after lifecycle lowering and every transform that may
change calls or EH, and before object emission. It observes entry/fault/finish
roles, every call in the residence extent, effective call/declaration
attributes, final normal returns, terminal `unreachable`, target provenance,
and absence of `invoke`, `landingpad`, `resume`, `catchswitch`, `catchpad`,
`cleanuppad`, and `cleanupret`.

Only that verifier may issue
`VerifiedPinnedTextBackendNoUnwindClosureV1`. The closure consumes the
transferred admission provenance and belongs to the same unpublished module
candidate. Missing closure discards the candidate; the contracted route has
no external-emitter fallback or retry.

## Corrected task order

The lifecycle detour stops here. Resume the durable common V2 order rather
than growing the legacy finalizer:

```text
1. LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0
   same installed cohort; one Completion; physical signature sibling;
   generic operation/control envelope boundary; no Builder/session effect

2. LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0
   generic complete operation set + separate If/Exit control set + coverage

3. LOOP-S6C-COMMON-V2-PRESESSION-I0
   caller-zero installed S6C child/envelope; no physicalizer or route policy

4. existing ordered common physical coverage/session rows
   one CanonicalSsaFunctionSessionV2 + Completion + DraftSeal ingress

5. TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-AUTHORITY-I0
   opaque exit stamp + expected obligation + admission on the live owner

6. TEXT-FORMAL-PINNED-RESIDENCE-LIFECYCLE-MATERIALIZER-I0
   consume admission; create lifecycle shape; transfer provenance

7. TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-NOUNWIND-CLOSURE-I0
   observe final module; issue closure; gate contract-bound object emission
```

Rows 1--4 keep their existing task-order SSOT and must not absorb pinned-Text
lifecycle effects. Rows 5--7 reopen only after the canonical session owns the
same function, exit set, frame/plan provenance, and unpublished candidate.

## NoSafeSlice

```text
NoSafeSlice::PinnedTextCanonicalDraftSealIngressMissing
NoSafeSlice::ExitSetUnavailableAtFrameIssue
NoSafeSlice::StampOnlyCannotReachDraftSeal
NoSafeSlice::MetadataCloneDuplicatesLifecycle
NoSafeSlice::ExitLedgerRescanRequired
NoSafeSlice::PinnedTextExitFinishSetSourceMissing
NoSafeSlice::ExitStampUniquenessUnsealed
NoSafeSlice::PinnedTextEntryFaultCensusMissing
NoSafeSlice::PinnedTextBackendNoUnwindCapabilityMissing
NoSafeSlice::PinnedTextLifecycleMaterializerUnsealed
NoSafeSlice::FinalCapabilityRequiredBeforeMaterialization
NoSafeSlice::BackendVerifierRunsBeforeFinalTransforms
NoSafeSlice::PartialEvidencePromotedToAuthority
NoSafeSlice::ObjectEmissionWithoutClosure
NoSafeSlice::MirOrJsonReinferenceRequired
NoSafeSlice::FallbackOrRetryRequired
```

A-prime is the selected final design. Its implementation remains closed until
the planned common V2 canonical session makes the required owners physically
co-resident.
