---
Status: closed__Fast__R7PendingDraftSeal__2026-09-12
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

## I0 closeout evidence

Implementation landed at `5042110b6d`. The DraftSeal owner now consumes the
rejected live session before issuing the owned typed
`DiscardedFunctionDraftSealErrorV1`; the CallableSingleLoop pending helper
transports it as `CanonicalFunctionSessionErrorV1::DraftSeal`, and the
cataloged-box-method caller preserves that variant without a Debug-string
bridge. The existing collector terminal and parent restoration contract are
unchanged. Generic G0 pending and the non-pending DraftSeal bridge remain
separate.

The focused `completion_draft_seal_tests` suite passed 12/12, including
`callable_pending_draft_seal_rejection_keeps_typed_error_and_restores_parent`
with exact stage/inner-error matching, unpublished-state checks, and fresh
session reuse. The final exact filtered test passed 1/1. `cargo check -p
nyash-rust --features plugins --profile quick -j1` passed with the warning
baseline at lib=1806; warnings remain tracked by
`MIRBUILDER-WARNING-SURFACE-CENSUS-R0` /
`MIRBUILDER-WARNING-BASELINE-REFRESH-I0`.

The pending DraftSeal guard, the prior typed cutover guard, current-state
pointer guard, targeted rustfmt checks, `git diff --check`, and source-size
checks passed. All changed Rust sources remain below 800 lines; the largest
is `draft_seal_owner.rs` at 753 lines. No whole-R7, Generic G0 pending,
backend parity, or whole-MIRBuilder completion claim is made.
