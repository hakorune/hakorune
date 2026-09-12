---
Status: selected__Fast__R7PendingDraftSeal__2026-09-12
Task: MIR-CALL-COMPATIBILITY-RETIRE-R7-PENDING-DRAFT-SEAL-I0
Date: 2026-09-12
Parent: mir-call-compatibility-retire-r7-pending-draft-seal-d0-2026-09-12.md
---

# R7 pending DraftSeal typed handoff

## Authority and bounded change

`FunctionDraftSealErrorV1` remains the owner-issued reject payload. The
`RejectedFunctionDraftSealV1` owner consumes itself only after rejection,
discards the unpublished child, and restores the captured parent through the
existing restoration receipt. The pending callable helper transports the
owned stage/error pair through `CanonicalFunctionSessionErrorV1::DraftSeal`.

This slice covers only the CallableSingleLoop pending helper and its
cataloged-box-method production caller. It does not change Generic G0 pending,
non-pending lowerers, collector admission, session restoration mechanics, or
the other DraftSeal bridge.

## Required implementation

- add the transport-only `DiscardedFunctionDraftSealErrorV1` at the DraftSeal
  owner boundary;
- add the typed `DraftSeal` session error variant and preserve its Display
  contract without a Debug-to-String bridge at the pending helper;
- make the pending helper return the typed session error and make its caller
  preserve that error directly;
- add exact family/stage/inner-payload and restoration/reuse assertions;
- keep the live owner inaccessible and keep changed sources below 800 lines;
- add/update a stable guard for the pending helper's typed path and exclusion
  of `format!("{...:?}")` there.

## Focused acceptance

- pending CallableSingleLoop success still completes through the existing
  collector terminal;
- pending DraftSeal rejection matches `DraftSeal` + exact stage + original
  typed inner error;
- rejection leaves no unpublished current function and restores the parent;
- a fresh child session can start after rejection;
- no `format!("{...:?}")` conversion remains in the pending helper;
- Generic G0 pending retains its existing separate status;
- pointer guard, focused tests, `git diff --check`, and source-size checks pass.

## Non-claims

This card does not close all R7 diagnostic bridges, Generic G0 pending, the
warning baseline, backend parity, or whole-MIRBuilder completion.
