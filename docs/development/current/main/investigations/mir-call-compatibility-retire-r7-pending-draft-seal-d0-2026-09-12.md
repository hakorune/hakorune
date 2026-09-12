---
Status: selected__DesignStop__R7PendingDraftSeal__2026-09-12
Task: MIR-CALL-COMPATIBILITY-RETIRE-R7-PENDING-DRAFT-SEAL-D0
Date: 2026-09-12
Parent: mir-call-compatibility-retire-r7-typed-diagnostics-i0-2026-09-12.md
---

# R7 pending draft-seal diagnostic boundary

## Six-line brief

```text
Decision: stop before changing the pending helper until its error ownership and session restoration are audited together.
Source authority + canonical issuer: FunctionDraftSeal owner and its existing typed preparation/projection errors.
Non-authority: Debug strings, String results, stage text, and borrowed error views as machine-readable reject kinds.
Fail-fast boundary: rejected pending seal must retain typed failure, discard the unpublished product, and preserve the existing parent-session restoration contract.
Smallest next slice: finite read-only inventory of RejectedFunctionDraftSeal ownership, pending callers, and exact restoration tests.
Non-claims: no enum, owned conversion, session mutation, fallback, route change, or whole-R7 completion.
```

## Census boundary

Start at `src/mir/builder/resolved_lowering/mod.rs` pending helper lines
191-207, follow the `FunctionDraftSealErrorV1` and
`RejectedFunctionDraftSealV1` owner in `draft_seal_owner.rs`, and end at each
production/test caller and its parent-session restoration assertion. Include
all Debug-to-String conversions in this pending path. Exclude the already
closed DirectAccum/Nested cutover bridge, Generic G0, unrelated diagnostics,
and public error vocabulary not reached by this helper.

## Required design questions

- Which existing typed owner can issue an owned error without duplicating
  preparation/projection authority?
- Can `RejectedFunctionDraftSealV1` expose an `into_parts()`-style move while
  retaining the discard-only and parent-session restoration invariants?
- Does the pending helper need a typed local terminal, or can the existing
  canonical outer error carry it without a new semantic authority?
- Which positive pending completion and negative rejection tests observe the
  typed payload and restoration before any publication effect?

This is a design stop. A worker audit is required before implementation.
