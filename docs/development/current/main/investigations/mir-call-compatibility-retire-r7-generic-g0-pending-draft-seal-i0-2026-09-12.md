---
Status: selected__Fast__R7GenericG0PendingDraftSeal__2026-09-12
Task: MIR-CALL-COMPATIBILITY-RETIRE-R7-GENERIC-G0-PENDING-DRAFT-SEAL-I0
Date: 2026-09-12
Parent: mir-call-compatibility-retire-r7-generic-g0-pending-draft-seal-d0-2026-09-12.md
NextCard: none__R7GenericG0PendingDraftSeal__PendingCloseout
---

# R7 Generic G0 pending DraftSeal typed handoff

## Contract

The existing Generic G0 admission owner remains the sole issuer of physical
admission meaning. Its `GenericG0PhysicalEmitterAdmissionRejectV1` crosses the
existing `CanonicalFunctionSessionErrorV1` boundary without Debug formatting.
The existing DraftSeal owner remains the sole issuer of DraftSeal meaning; its
rejected live owner is consumed by `into_discarded_error()` before the typed
`DraftSeal` session error is returned. The collector, caller, and publication
route are unchanged.

The lowerer's already string-valued body errors remain the existing
`Primary(String)` terminal for this slice. No new semantic receipt, route,
fallback, retry, or body error family is introduced.

## Required implementation

- add the transport-only Generic G0 admission variant to the existing session
  error enum and preserve its Display contract;
- map the admission error in `resolved_lowering/mod.rs` without
  `format!("{error:?}")`;
- map Generic G0 DraftSeal rejection through the existing owned discarded
  payload and preserve restoration before return;
- update only the affected coarse abort matchers and focused pending tests;
- keep the existing Callable pending path and non-pending G0 lowerer behavior;
- keep every changed source below 760 lines and the hard stop below 800.

## Focused acceptance

- positive: existing Generic G0 pending caller still commits through the
  existing collector terminal;
- admission negative: the exact
  `GenericG0PhysicalEmitterAdmissionRejectV1` is retained before a child
  session/effect and maps to the typed session variant;
- DraftSeal negative: exact stage and original `FunctionDraftSealErrorV1`
  survive, the unpublished function is discarded, the parent is restored once,
  and a fresh child session can start;
- structural: no Debug conversion remains at the selected admission or
  DraftSeal bridges; no second G0 issuer or retry is added;
- existing pointer, focused owner guard, diff, formatting, and source-size
  checks remain green.

## Scope boundary

This card does not type the lowerer's pre-DraftSeal string-valued body errors,
change G0 source admission, widen cataloged methods/all-family Loop selection,
touch DirectAccum/Nested diagnostics, delete the public ambient helper, or
claim warning reduction, backend parity, R7 closure, or whole-MIRBuilder
completion. If the existing DraftSeal owner cannot be consumed while retaining
parent restoration, return to the D0 design stop rather than exposing a live
owner or adding an adapter.

## Closeout checklist

Record the changed-file inventory, focused positive/negative results, exact
typed stage/payload observation, restoration/fresh-session result, guard and
pointer output, source-size maximum, commit SHA, and pushed remote state here.
